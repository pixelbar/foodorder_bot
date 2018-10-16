use state::State;
use irc::proto::{Message, Command};

type Reply = String;

pub fn handle_message(sender: &str, msg: &str) -> Reply {
    let mut split = msg.split(' ');
    match split.next() {
        Some("clear") => { 
            State::clear();
            String::from("Orders and topic cleared")
        },
        Some("topic") => {
            let remaining = split.fold(String::new(), |mut s, add| { s += " "; s += add; s });
            if remaining.trim().is_empty() {
                if let Some(topic) = State::get_topic() {
                    topic
                } else {
                    String::from("(No topic set)")
                }
            } else {
                let topic = remaining.trim();
                State::set_topic(topic);
                format!("New topic is: {}", topic)
            }
        },
        Some(_) if !msg.trim().is_empty() => {
            State::set_order(sender, msg);
            format!("{} wants {}", sender, msg)
        },
        _ => {
            State::get_order_summary()
        }
    }
}

pub fn find_owner(message: &Message) -> Option<&str> {
    let prefix = message.prefix.as_ref()?;
    let index = prefix.find('!')?;
    let name = &prefix[..index];
    if name.starts_with("slackbridge") {
        if let Command::PRIVMSG(_, message) = &message.command {
            if message.chars().next() != Some('<') { return None; }
            let end = message.find('>')?;
            if end > 1 {
                return Some(&message[1 .. end]);
            }
        }
    }
    if name.is_empty() {
        None
    } else {
        Some(name)
    }
}
