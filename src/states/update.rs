// Importa la estructura Counter
use crate::interface::counter::Counter;
use crate::interface::message::Message;

pub fn update(counter: &mut Counter, message: Message) {
    match message {
        Message::Increment => counter.increment(),
        Message::Decrement => counter.decrement(),
        Message::Exit => std::process::exit(0),
    }
}
