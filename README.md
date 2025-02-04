[![](https://img.shields.io/crates/v/termchat)](https://crates.io/crates/termchat)
[![](https://img.shields.io/crates/l/termchat)](https://www.apache.org/licenses/LICENSE-2.0.txt)
[![](https://img.shields.io/github/workflow/status/lemunozm/termchat/termchat%20ci)](https://github.com/lemunozm/termchat/actions?query=workflow%3A%22termchat+ci%22)

<p align="center">
  <img src="https://docs.google.com/drawings/d/e/2PACX-1vTCUOY5x1FQ-zWJdagKPLVWLTWDO3QCg9brYPOHZ6qqK6LndPTDM3sfp0599w1F4VatZfLITTZM33JW/pub?w=712&h=164"/>
</p>

A distributed LAN chat application in the terminal (without needing a server!).
Run the application in your terminal and write into the LAN!

Built on top of [tui-rs](https://github.com/fdehau/tui-rs) to create the terminal UI and
[message-io](https://github.com/lemunozm/message-io) to make the network connections.

![](./screenshot.png)

# Installation
You can use the [cargo][cargo] package manager in order to install it.

```
$ cargo install termchat --all-features
```

If you have `~/.cargo/bin` in your PATH (or similar in your OS), you will be able to use *termchat* everywhere in your computer!

Also, you can download the last release for your machine from the [releases](https://github.com/lemunozm/termchat/releases).

## Arch Linux

`termchat` can be installed from available [AUR packages](https://aur.archlinux.org/packages/?O=0&SeB=b&K=termchat&outdated=&SB=n&SO=a&PP=50&do_Search=Go) using an [AUR helper](https://wiki.archlinux.org/index.php/AUR_helpers). For example,

```sh
$ yay -S termchat
```

If you prefer, you can clone the [AUR packages](https://aur.archlinux.org/packages/?O=0&SeB=b&K=termchat&outdated=&SB=n&SO=a&PP=50&do_Search=Go) and then compile them with [makepkg](https://wiki.archlinux.org/index.php/Makepkg). For example,

```sh
$ git clone https://aur.archlinux.org/termchat.git && cd termchat && makepkg -si
```

[cargo]: https://doc.rust-lang.org/cargo/getting-started/installation.html

# How it works?
To not saturate the network, *termchat* uses only one multicast message at startup to find other *termchat* applications on the network.
Once a new application has been found by multicast, a TCP connection is created between them.

## Usage
Simply write:
```
$ termchat
```

to open the application in your terminal.

By default, your computer user name is used. You can use a different username with `-u <name>`

You can modify the multicast discovery address with `-d <address>`

You can set a custom tcp sever port with `-t <port>`

(see the application help for more info `--help`).

### Commands
Termchat treats messages containings the following commands in a special way:

- **`?send <$path_to_file>`**: sends the specified file to everyone on the network,
  example: `?send ./myfile`

  Note: The received files can be found in `/tmp/termchat/<termchat-username>/<file_name>`

- **`?startstream`**/**`?stopstream`**: starts/stops video stream and send it to all peers. Currently this is only supported on linux, the other platforms can only receive the video.

### Config
Termchat store its configuration in a simple file located at `$ConfigDir/termchat/config`

**Default config:**
```
discovery_addr = "238.255.0.1:5877"
tcp_server_port = 0
user_name = "my_awesome_user_name"
terminal_bell = true

[theme]
message_colors = ["Blue", "Yellow", "Cyan", "Magenta"]
my_user_color = "Green"
date_color = "DarkGray"
system_info_color = ["Cyan", "LightCyan"]
system_warning_color = ["Yellow", "LightYellow"]
system_error_color = ["Red", "LightRed"]
chat_panel_color = "White"
progress_bar_color = "LightGreen"
command_color = "LightYellow"
input_panel_color = "White"
```

## Frequently Asked Questions

***Q:*** **Hosts are not disoverable**

***A:***

- Make sure that no firewall is running (example: ufw), and if that's the case either stop it or add termchat ports to the white list.

- By default you need to allow port `5877/udp` and `port X/tcp`, `X` is a different with each run. Note that you can specify a custom tcp port as mentioned above and add it to the firewall whitelist.

***Q:*** **Can I silence the terminal bell when I received a message?**

***A:*** Yeah! You can run `termchat` passing the flag `--quiet-mode` or simple `-q`.

***Q:*** **I can't see anything on my light themed desktop!!!**

***A:*** You can use `termchat --theme light`, also you can customize colors individually via the config file.
