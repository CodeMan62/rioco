use basic_event_loop::event::{Action, EventHandler, EventLoop};
use mio::event::Event;
use mio::net::{TcpListener, TcpStream};
use mio::{Interest, Token};
use std::io::{self, Read, Write};
use std::net::SocketAddr;
struct AcceptHandler {
    listener: TcpListener,
    next_token: usize,
}

impl AcceptHandler {
    fn new(listener: TcpListener) -> Self {
        Self {
            listener,
            next_token: 1,
        }
    }
}

impl EventHandler for AcceptHandler {
    fn handle_event(&mut self, event: &Event) -> io::Result<Vec<Action>> {
        if event.is_readable() {
            let (stream, _) = self.listener.accept()?;
            let token = Token(self.next_token);
            self.next_token += 1;

            Ok(vec![Action::Add {
                token,
                interests: Interest::READABLE.add(Interest::WRITABLE),
                handler: Box::new(EchoHandler { stream }),
            }])
        } else {
            Ok(vec![])
        }
    }

    fn source(&mut self) -> &mut dyn mio::event::Source {
        &mut self.listener
    }
}

struct EchoHandler {
    stream: TcpStream,
}

impl EventHandler for EchoHandler {
    fn handle_event(&mut self, event: &Event) -> io::Result<Vec<Action>> {
        if event.is_readable() {
            let mut buf = [0; 1024];
            match self.stream.read(&mut buf) {
                Ok(n) => {
                    if n == 0 {
                        return Ok(vec![Action::Remove(event.token())]);
                    }
                    let data = &buf[..n];
                    self.stream.write_all(data)?;
                }
                Err(e) if e.kind() == io::ErrorKind::WouldBlock => {}
                Err(e) => return Err(e),
            }
        }
        Ok(vec![])
    }

    fn source(&mut self) -> &mut dyn mio::event::Source {
        &mut self.stream
    }
}

fn main() -> io::Result<()> {
    simple_logger::init_with_level(log::Level::Info).unwrap();
    let addr: SocketAddr = "127.0.0.1:12345".parse().unwrap();
    let listener = TcpListener::bind(addr)?;

    let mut event_loop = EventLoop::new()?;
    event_loop.add_handler(
        Token(0),
        Interest::READABLE,
        Box::new(AcceptHandler::new(listener)),
    )?;

    log::info!("Starting event loop on {}", addr);
    event_loop.run()
}
