use std::collections::HashMap;
use std::sync::Mutex;

lazy_static! {
    static ref STATE: Mutex<State> = Mutex::new(State {
        topic: None,
        orders: HashMap::new(),
    });
}

type PersonName = String;
type PersonOrder = String;

#[derive(Debug)]
pub struct State {
    pub topic: Option<String>,
    pub orders: HashMap<PersonName, PersonOrder>,
}

impl State {
    pub fn clear() {
        let mut state = STATE.lock().unwrap();
        state.topic = None;
        state.orders.clear();
    }

    pub fn get_topic() -> Option<String> {
        let state = STATE.lock().unwrap();
        state.topic.clone()
    }

    pub fn set_topic(topic: &str) {
        let mut state = STATE.lock().unwrap();
        state.topic = Some(topic.to_string());
    }

    pub fn set_order(person: &str, order: &str) {
        let mut state = STATE.lock().unwrap();
        state.orders.insert(person.to_owned(), order.to_owned());
    }

    pub fn get_order_summary() -> String {
        let state = STATE.lock().unwrap();
        let mut result = String::new();
        let mut is_first = true;
        if let Some(topic) = &state.topic {
            result += topic;
            result += ": ";
        } else {
            result += "(No topic) ";
        }

        for (person, order) in &state.orders {
            if is_first { is_first = false; }
            else { result += ", "; }
            result += person;
            result += " wants ";
            result += order;
        }
        result
    }
}