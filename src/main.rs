extern crate chrono;
extern crate irc;
#[macro_use]
extern crate lazy_static;

mod clear_thread;
mod handler;
mod state;

use handler::{find_owner, handle_message};
use irc::client::data::Config;
use irc::client::ext::ClientExt;
use irc::client::reactor::IrcReactor;
use irc::client::Client;
use irc::proto::Command;
use std::fs;
use std::io::Write;
use std::path::PathBuf;

const CONFIG_FILE: &str = "config.json";
const COMMAND_PREFIX: &str = "#foodorder";
fn main() {
    if !PathBuf::from(CONFIG_FILE).exists() {
        write_default_file(CONFIG_FILE);
        println!("config.json written. Please modify this as you see fit and restart the bot");
        return;
    }

    let mut reactor = IrcReactor::new().expect("Could not start IRC reactor");
    let client = reactor
        .prepare_client_and_connect(&Config::load(CONFIG_FILE).expect("Could not read config file"))
        .expect("Could not create client");
    client.identify().unwrap();

    std::thread::spawn(crate::clear_thread::start_clear_thread);

    reactor.register_client_with_handler(client, |client, message| {
        if let Command::PRIVMSG(channel, text) = &message.command {
            if let Some(index) = text.find(COMMAND_PREFIX) {
                if let Some(owner) = find_owner(&message) {
                    let foodorder = text[index + COMMAND_PREFIX.len()..].trim();
                    let response = handle_message(owner, foodorder);

                    let mut index = 0;
                    while index < response.len() {
                        let length = 450.min(response.len() - index);
                        client
                            .send(Command::PRIVMSG(
                                channel.to_owned(),
                                response[index..index + length].to_owned(),
                            ))
                            .expect("Could not send reply message");
                        index += length;
                    }
                }
            }
        }

        Ok(())
    });

    reactor.run().expect("Reactor finished unexpectedly");
}

fn write_default_file(path: &str) {
    let mut file = fs::File::create(path).expect("Could not create config file");
    file.write_all(
        br##"{
    "nickname": "foodorder",
    "server": "irc.smurfnet.ch",
    "channels": [
        "#pixelbar-test"
    ]
}"##,
    )
    .expect("Could not write config file");
}
