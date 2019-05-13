//!
//! An example use of the nano fix client
//!
//! Type the following command to execute the program:
//!
//!     cargo run --example fix_client
//!
#![allow(dead_code)]

//#[macro_use]
extern crate log;

use fixclient::FixClient;
//use nanofix_message::message::Message;
use env_logger::{Builder, Target};

//fn new_logon() -> Message {
//    Message{LOGON}
//}

pub fn main() -> Result<(), Box<std::error::Error>> {

    //const HOSTNAME: &'static str = "54.194.110.77:18888";
    const HOSTNAME: &'static str = "127.0.0.1:8888";

    let mut builder = Builder::from_default_env();
    builder.default_format_timestamp_nanos(true);
    builder.target(Target::Stdout);
    builder.init();

    let addr = HOSTNAME.parse().unwrap();
    println!("Connecting to {} ..", addr);
    let mut client = FixClient::connect(addr)?;
    println!("Connected -> {:?}", client);

    // At this point we want to create a FIX message
    let buf = b"8=XYZ\x0135=A\x01";   // missing BodyLen
    //let buf = b"8=XYZ\x019=5\x0135=A\x01";    // missing Checksum
    //let buf = b"8=XYZ\x019=5\x0134=1\x0110=000\x01";    // missing MsgSeqNum
    //"8=FIX.4.4|9=57|35=A|49=sender|56=target|34=1|52=2018-08-01-00:00:01.123|10=123|"
    // 8=FIX.4.4|9=116|35=A|34=1|49=TORA|52=20190307-00:00:00.195|56=B2C2UT|98=0|108=30|141=Y|
    // 553=daa647796e28e64c2217a402861fa594d2ffeb81|10=190|

    // Write message to stream / hidden by FixClient
    let written = client.write(buf)?;
    println!("Wrote {} bytes to stream", written);
    Ok(())
}
