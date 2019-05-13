#![allow(dead_code)]

//
////use {localhost, TryRead, TryWrite};
////use mio::{Events, Poll, Ready, PollOpt, Token};
////use mio::net::TcpStream;
//
//use std::io::{Error, ErrorKind, Result};
//use std::net::{Shutdown, SocketAddr};
//
////const CLIENT_TOKEN: Token = Token(100);
//

use std::io::{self, Result, Write};
use std::net::{SocketAddr, TcpStream};

//use futures::{Future, Poll};
//use tokio::net::tcp::{ConnectFuture as TokioConnectFuture, TcpStream};
//use tokio_io::io::WriteHalf;

enum SessionState {
    Disconnected,
    Connecting(FixClient),
    Connected,
    LogonSent,
    LogonReceived,
    LoggedOn,
    LogoutSent,
    LogoutReceived,
    LoggedOut,
//    Error(io::Error),
}

#[allow(dead_code)]
#[derive(Debug)]
pub struct FixClient {
    socket: TcpStream
}

impl FixClient {
    pub fn connect(addr: SocketAddr) -> io::Result<FixClient> {
        let stream = TcpStream::connect(&addr)?;
        Ok(FixClient{
            socket: stream
        })
    }

    pub fn write(&mut self, buf: &[u8]) -> Result<usize> {
        let bytes_written = self.socket.write(buf)?;
        Ok(bytes_written)
    }
}


//pub fn connect(addr: &SocketAddr) -> ConnectFuture {
//    let inner = TcpStream::connect(&addr).and_then(|stream| {
//        println!("Connected to {}", stream);
//        //ConnectFutureState::Connecting(FixClient {stream})
//    })
//    .map_err(|err| {
//        println!("Connection error = {:?}", err);
//        ConnectFutureState::Error(err)
//    });
//        let state = match mio::net::TcpStream::connect(addr) {
//        Ok(tcp) => Waiting(TcpStream::new(tcp)),
//        Err(e) => Error(e),
//    };
//    ConnectFuture { inner }
//}

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
