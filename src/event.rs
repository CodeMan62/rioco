use std::{collections::HashMap, io};

use mio::{
    Interest, Poll, Token,
    event::{Event, Source},
};

pub enum Action {
    Add {
        token: Token,
        interests: Interest,
        handler: Box<dyn EventHandler>,
    },
    Remove(Token),
}

pub trait EventHandler {
    fn handle_event(&mut self, event: &Event) -> io::Result<Vec<Action>>;
    fn source(&mut self) -> &mut dyn Source;
}
#[warn(dead_code)]
pub struct EventLoop {
    poll: Poll,
    handlers: HashMap<Token, Box<dyn EventHandler>>,
}

impl EventLoop {
    pub fn new() -> io::Result<Self> {
        Ok(Self {
            poll: Poll::new()?,
            handlers: HashMap::new(),
        })
    }
    pub fn add_handler(
        &mut self,
        token: Token,
        interest: Interest,
        mut handler: Box<dyn EventHandler>,
    ) -> io::Result<()> {
        let source = handler.source();
        self.poll.registry().register(source, token, interest)?;
        self.handlers.insert(token, handler);
        Ok(())
    }
    pub fn run(&mut self) -> io::Result<()> {
        Ok(())
    }
}
