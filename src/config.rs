use std::net::{SocketAddrV4};
use clap::ArgMatches;
use serde::{Serialize, Deserialize};
use crate::util::Result;
use tui::style::Color;
#[derive(Serialize, Deserialize, Debug)]
pub enum NodeType {
    Client{
        server_addr:SocketAddrV4
    },
    Server{
        port: u16,
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Config {
    pub discovery_addr: SocketAddrV4,
    pub node_type: NodeType, //if empty is a server
    pub user_name: String,
    pub terminal_bell: bool,
    pub theme: Theme,
    pub boot: bool,
}

impl Default for Config {
    fn default() -> Self {
        //Server by default
        Config {
            discovery_addr: "238.255.0.1:5877".parse().unwrap(),
            node_type: NodeType::Server{port: "2424".parse().unwrap()},
            user_name: whoami::username(),
            terminal_bell: true,
            theme: Theme::default(),
            boot: false,
        }
    }
}

impl Config {
    /// Try to read config file from disk
    /// If it does not exist, create it with default config values, and return that
    /// If it fails for any other reason return None
    fn from_config_file() -> Option<Self> {
        let config_dir_path = dirs_next::config_dir()?.join("termchat");
        if let Err(e) = std::fs::create_dir_all(&config_dir_path) {
            if e.kind() != std::io::ErrorKind::AlreadyExists {
                return None
            }
        }
        let config_file_path = config_dir_path.join("config");

        let create_config = |config_file_path| -> Result<Config> {
            let config = Config::default();
            std::fs::write(config_file_path, toml::to_string(&config)?)?;
            Ok(config)
        };

        match std::fs::read_to_string(&config_file_path) {
            Ok(config) => toml::from_str(&config).ok(),
            Err(e) if e.kind() == std::io::ErrorKind::NotFound => {
                // Config file was not found -> create it with default_values
                match create_config(&config_file_path) {
                    Ok(config) => Some(config),
                    Err(_) => None,
                }
            }
            Err(_) => None,
        }
    }

    /// Read configuration file from disk
    /// If it fails for any reason use default Config value
    /// If the user uses the cli arguments they will override the default values
    pub fn from_matches(matches: & ArgMatches) -> Self {
        let mut config = Config::from_config_file().unwrap_or_default();

        // the next unwrap are safe because we use clap validator
        if let Some(discovery_addr) = matches.value_of("discovery") {
            config.discovery_addr = discovery_addr.parse().unwrap();
        }
        if let Some(port) = matches.value_of("table") {
            config.node_type = NodeType::Server{port: port.parse().unwrap() };
            config.boot = true;
        }
        if let Some(server_addr) = matches.value_of("player") {
            config.node_type = NodeType::Client{server_addr: server_addr.parse::<SocketAddrV4>().unwrap()};
        }
        if let Some(user_name) = matches.value_of("username") {
            config.user_name = user_name.parse().unwrap();
        }
        if matches.is_present("quiet-mode") {
            config.terminal_bell = false;
        }
        if let Some(theme) = matches.value_of("theme") {
            if theme == "dark" {
                config.theme = Theme::dark_theme();
            }
            else {
                config.theme = Theme::light_theme();
            }
        }

        config
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Theme {
    pub message_colors: Vec<Color>,
    pub my_user_color: Color,
    pub date_color: Color,
    pub system_info_color: (Color, Color),
    pub system_warning_color: (Color, Color),
    pub system_error_color: (Color, Color),
    pub chat_panel_color: Color,
    pub progress_bar_color: Color,
    pub command_color: Color,
    pub input_panel_color: Color,
}

impl Default for Theme {
    fn default() -> Self {
        Self::dark_theme()
    }
}

impl Theme {
    fn dark_theme() -> Self {
        Self {
            message_colors: vec![Color::Blue, Color::Yellow, Color::Cyan, Color::Magenta],
            my_user_color: Color::Green,
            date_color: Color::DarkGray,
            system_info_color: (Color::Cyan, Color::LightCyan),
            system_warning_color: (Color::Yellow, Color::LightYellow),
            system_error_color: (Color::Red, Color::LightRed),
            chat_panel_color: Color::White,
            progress_bar_color: Color::LightGreen,
            command_color: Color::LightYellow,
            input_panel_color: Color::White,
        }
    }

    fn light_theme() -> Self {
        Self {
            message_colors: vec![Color::Blue, Color::Yellow, Color::Cyan, Color::Magenta],
            my_user_color: Color::Green,
            date_color: Color::DarkGray,
            system_info_color: (Color::Cyan, Color::LightCyan),
            system_warning_color: (Color::Yellow, Color::LightYellow),
            system_error_color: (Color::Red, Color::LightRed),
            chat_panel_color: Color::Black,
            progress_bar_color: Color::LightGreen,
            command_color: Color::LightYellow,
            input_panel_color: Color::Black,
        }
    }
}
