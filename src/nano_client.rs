#[macro_use]
extern crate log;
//
////use {localhost, TryRead, TryWrite};
////use mio::{Events, Poll, Ready, PollOpt, Token};
////use mio::net::TcpStream;
//
//use std::io::{Error, ErrorKind, Result};
//use std::net::{Shutdown, SocketAddr};
//
//use tokio::net::TcpStream;
//
////const CLIENT_TOKEN: Token = Token(100);
//
//#[derive(PartialEq)]
//enum SessionState {
//    Disconnected,
//    Connected,
//    LogonSent,
//    LogonReceived,
//    LoggedOn,
//    LogoutSent,
//    LogoutReceived,
//    LoggedOut,
//}
//
//#[allow(dead_code)]
//struct NanoFixClient {
//    address: SocketAddr,
//    socket: TcpStream,
//    state: SessionState,
//}
//
//impl NanoFixClient {
//    fn connect(url: &str) -> Result<NanoFixClient> {
//        match url.parse() {
//            Ok(addr) => {
//                let socket = TcpStream::connect(&addr)?;
////                let socket = TcpStream::connect(&addr).and_then(|stream| {
////                    info!("Successfully connected to ...");
////                    Ok(())
////                });
////                    .map_err(|err| {
////                        //state: SessionState::Disconnected,
////                        warn!("Connection to {} failed: {}", url, err);
////                    });
//
//                info!("Successfully connected to ...");
//                let client = NanoFixClient {
//                    address: addr,
//                    socket: socket,
//                    state: SessionState::Connected
//                };
//                Ok(client)
//            }
//            Err(_err) => return Err(Error::from(ErrorKind::InvalidInput)),
//        }
//    }
//
//    fn disconnect(&self) -> Result<()> {
//        self.socket.shutdown(Shutdown::Both)?;
//        Ok(())
//    }
//
//    fn write(&mut self, _poll: &mut Poll) -> Result<()> {
//        /*
//                let buf: &[u8] = "Hello world".as_bytes_mut();
//                match self.socket.try_write(&buf) {
//                    Ok(None) => {
//                        // TODO make this a debug!
//                        println!("client flushing buf; WOULDBLOCK");
//                        self.interest.insert(Ready::writable());
//                    }
//                    Ok(Some(r)) => {
//                        // TODO make this a debug!
//                        println!("CLIENT : we wrote {} bytes!", r);
//                        self.interest.insert(Ready::readable());
//                        self.interest.remove(Ready::writable());
//                    }
//                    // TODO make this a debug!
//                    Err(e) => println!("not implemented; client err={:?}", e)
//                }
//
//                if self.interest.is_readable() || self.interest.is_writable() {
//                    poll.reregister(
//                        &self.socket,
//                        CLIENT_TOKEN,
//                        self.interest,
//                        PollOpt::edge() | PollOpt::oneshot())?;
//                }
//        */
//
//        Ok(())
//    }
//}
//
fn main() {
    env_logger::init();
    info!("Starting nanofix client ..");
//    let mut client = NanoFixClient::connect("10.0.13.255:18099").unwrap();
//    match run_client(&mut client) {
//        Ok(()) => {}
//        Err(err) => warn!("Unable to run: {:?}", err)
//    }
}
//
//fn run_client(client: &mut NanoFixClient) -> Result<()> {
//
//    // Construct a new `Poll` handle as well as the `Events` we'll store into
//    let mut poll = Poll::new()?;
//    let mut events = Events::with_capacity(1024);
//
//    // Register the stream with `Poll`
//    poll.register(&client.socket, CLIENT_TOKEN, Ready::readable() | Ready::writable(), PollOpt::edge())?;
//
//
//    // Wait for the socket to become ready.
//    // This has to happens in a loop to handle spurious wakeups.
//    while client.state == SessionState::Connected {
//        poll.poll(&mut events, None)?;
//
//        for event in &events {
//            debug!("event {:?}", event);
//
//            // The socket connected (probably, it could still be a spurious wakeup)
//            if event.token() == CLIENT_TOKEN && event.readiness().is_writable() {
//                info!("Connected to '{}'", client.address);
//                client.write(&mut poll).unwrap();
//            }
//        }
//    }
//
//    client.disconnect()?;
//    Ok(())
//}
