// externs
//pub mod fix_parser;
//pub mod protocol;

// System allocator default from 1.32
//use std::alloc::System;
//#[global_allocator]
//static A: System = System;

use std::fs::File;
use std::io;
use std::io::{BufRead, BufReader, Write};
use std::process::exit;
use std::str;

use clap::{App, Arg, crate_authors, crate_version};
use colored::*;

use fixv::fix_parser;
use fixv::fix_parser::{Consumer, ConsumerError, TagValue};

#[derive(Clone, Debug, PartialEq, Eq)]
struct TagValueConsumerWriter {
    highlight_tags: Vec<u32>,
    exclude_msg_types: Vec<u8>,
}

impl<'a> Consumer<TagValue<'a>> for TagValueConsumerWriter {
    fn accept(&self, tv: TagValue) -> Result<(), ConsumerError> {
        let mut stdout = io::stdout();
        let tag_str = tv.tag.to_string();
        let buffer_slice: &[u8] = &tv.buffer[tv.offset..tv.offset + tv.length];
        let tag_value = str::from_utf8(buffer_slice).unwrap();
        write!(stdout, "{}", tag_str.bright_blue())?;
        write!(stdout, "{}", "=".white())?;
        if self.highlight_tags.contains(&tv.tag) {
            write!(stdout, "{}", tag_value.bright_red())?;
        } else {
            write!(stdout, "{}", tag_value.green())?;
        }
        write!(stdout, "{}", "|".white())?;
        Ok(())
    }
}

fn main() {
    //let arg_matches = app_from_crate!()
    let arg_matches = App::new("fixv")
        .version(crate_version!())
        .author(crate_authors!())
        .about("FIX protocol log file viewer")
        .arg(Arg::with_name("tags")
            .short("t")
            .long("tags")
            .require_equals(true)
            .takes_value(true)
            .value_name("tags")
            .help("Highlight individual tags"))
        .arg(Arg::with_name("exclude")
            .short("x")
            .long("exclude")
            .multiple(true)
            .require_equals(true)
            .takes_value(true)
            .default_value("Heartbeat")
            .value_name("msg types")
            .help("MsgTypes to exclude"))
        .arg(Arg::with_name("source")
            .index(1)
            .help("Source file name or omit to read from stdin"))
        .get_matches();

    // Source of data to be read
    let path = arg_matches.value_of("source").unwrap_or("-");

    // Construct MsgType exclusion policy
    let exclude_value = arg_matches.value_of("exclude").unwrap_or("0");
    let exclude_list: Vec<u8> = exclude_value.split(',').map(|x| x.as_bytes()[0]).collect();

    // Determine which tags are to be highlighted
    let tags_value = arg_matches.value_of("tags").unwrap_or("49,56");
    let tags: Vec<u32> = tags_value.split(",").map(|x| x.parse::<u32>().unwrap()).collect();
    let tv_consumer = TagValueConsumerWriter { highlight_tags: tags, exclude_msg_types: exclude_list };

    match parse(path, tv_consumer) {
        Ok(()) => {
            let mut stdout = io::stdout();
            if let Err(_) = stdout.flush() {
                exit(0);
            }
        }
        Err(err) => println!("Unable to read '{}': {}", path, err)
    }
}

fn parse(source: &str, tv_consumer: TagValueConsumerWriter) -> Result<(), std::io::Error> {
    match source {
        "-" => parse_stdin(tv_consumer),
        _ => parse_file(source, tv_consumer)
    }
}

fn parse_stdin(tv_consumer: TagValueConsumerWriter) -> Result<(), std::io::Error> {
    loop {
        let mut line = String::new();
        let bytes_read = io::stdin().read_line(&mut line)?;
        if bytes_read == 0 {
            break;
        } else {
            parse_line(&line.trim().to_string(), &tv_consumer);
        }
    }
    return Ok(());
}

fn parse_file(path: &str, tv_consumer: TagValueConsumerWriter) -> Result<(), std::io::Error> {
    let file = File::open(path)?;
    let buffered = BufReader::new(file);
    for result in buffered.lines() {
        let line = result?;
        parse_line(&line, &tv_consumer);
    }
    return Ok(());
}

fn parse_line(line: &String, tv_consumer: &TagValueConsumerWriter) {
    // find start of FIX message
    // TODO improve MsgType exclusion logic to avoid extra buffer iteration
    // by delaying output until MsgType has been determined
    match line.find("8=FIX") {
        None => {}
        Some(start_offset) => {
            let msg = &line.as_bytes()[start_offset..];
            let mut should_display = tv_consumer.exclude_msg_types.len() == 0;

            // See if msg type exclusion policy should be activated
            if tv_consumer.exclude_msg_types.len() > 0 {
                let offset = find_subsequence(msg, "35=".as_bytes());
                if offset != std::usize::MAX {
                    let msg_type = msg[offset+3];
                    should_display = !(tv_consumer.exclude_msg_types.contains(&msg_type));
                }
            }
            // Go ahead and parse the message if policy dictates
            if should_display {
                match fix_parser::iterate_tags(msg, tv_consumer) {
                    Ok(_) => writeln!(io::stdout()).unwrap_or_default(),
                    _ => () // ignore FIX parser errors
                }
            }
        }
    };
}

/// Finds the given sequence withing the provided array of bytes
fn find_subsequence(bytes:&[u8], sequence:&[u8]) -> usize {
    bytes.windows(sequence.len())
        .position(|w| w == sequence)
        .unwrap_or(std::usize::MAX)
}
