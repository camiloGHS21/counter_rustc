// src/interface/message.rs

#[derive(Debug, Clone)]
pub enum Message {
    Increment,
    Decrement,
    Exit,
}
