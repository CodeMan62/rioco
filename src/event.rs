use mio::{
    Events, Interest, Poll, Token,
    event::{Event, Source},
};
use std::{collections::HashMap, io, time::Duration};

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
        let mut events = Events::with_capacity(1024);
        loop {
            self.poll
                .poll(&mut events, Some(Duration::from_millis(100)))?;
            let mut actions = Vec::new();

            for event in events.iter() {
                let token = event.token();
                if let Some(handler) = self.handlers.get_mut(&token) {
                    let mut new_action = handler.handle_event(event)?;
                    actions.append(&mut new_action);
                }
            }
        }
    }
}
