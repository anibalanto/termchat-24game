use termchat::application::run_app;
use termchat::config::Config;

use clap::{App, Arg};

use std::net::{SocketAddrV4};
use std::env;

fn main() {
    env::set_var("RUST_BACKTRACE", "1");
    let matches = App::new("app")
        .arg(
            Arg::with_name("discovery")
                .long("discovery")
                .short('d')
                .takes_value(true)
                .validator(|addr| match addr.parse::<SocketAddrV4>() {
                    Ok(_) => Ok(()),
                    Err(_) => Err(""),
                })
                .help("Multicast address to found others 'termchat' applications"),
        )
        .arg(
            Arg::with_name("player")
                .long("player")
                .short('p')
                .takes_value(true)
                /*.validator(|addr| match addr.parse::<SocketAddrV4>() {
                    Ok(_) => Ok(()),
                    Err(_) => Err("Isn't a socket addr".into()),
                })*/
                .help("Tcp server addr used when communicating with table game"),
        )
        .arg(
            Arg::with_name("table")
                .long("table")
                .short('t')
                .takes_value(true)
                /*.validator(|port| match port.parse::<u16>() {
                    Ok(_) => Ok(()),
                    Err(_) => Err("The value must be in range 0..65535".into()),
                })*/
                .help("Tcp port used when communicating with other player instances"),
        )
        .arg(
            Arg::with_name("username")
                .long("username")
                .takes_value(true)
                .short('u')
                .help("Name used as user idenfication"),
        )
        .arg(
            Arg::with_name("quiet-mode")
                .long("quiet-mode")
                .short('q')
                .help("Disable the terminal bell sound"),
        )
        .arg(
            Arg::with_name("theme")
                .long("theme")
                /*.validator(|theme| match theme.to_lowercase().as_str() {
                    "dark" | "light" => Ok(()),
                    _ => Err("Theme accepts only dark and light as value".into()),
                */
                .takes_value(true)
                .help("Choose which theme should termchat use, values are dark and light"),
        )
        .get_matches();

    // The next unwraps are safe because we specified a default value and a validator
    let config = Config::from_matches(& matches);


    run_app(config);

    


    /*let result = match Application::new(&config) {
        Ok(mut app) => app.run(std::io::stdout()),
        Err(e) => Err(e),
    };

    if let Err(e) = result {
        // app is now dropped we can print to stderr safely
        eprintln!("termchat exited with error: {}", e);
    }*/

}

