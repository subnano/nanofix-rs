use nanofix_rs::fix_parser;
use nanofix_rs::fix_parser::{TagValue, Consumer, ConsumerError};
use std::io;
use std::process::exit;
use std::str;
use std::fs::File;
use std::io::{BufReader, BufRead, Write};
use clap::{Arg, app_from_crate, crate_authors, crate_description, crate_name, crate_version};
use colored::*;

struct TagValueConsumerWriter {
    tags: Vec<u32>,
}

impl<'a> Consumer<TagValue<'a>> for TagValueConsumerWriter {
    fn accept(&self, tv: TagValue) -> Result<(), ConsumerError> {
        let mut stdout = io::stdout();
        let tag_str = tv.tag.to_string();
        let buffer_slice: &[u8] = &tv.buffer[tv.offset..tv.offset + tv.length];
        let tag_value = str::from_utf8(buffer_slice).unwrap();
        write!(stdout, "{}", tag_str.bright_blue())?;
        write!(stdout, "{}", "=".white())?;
        if self.tags.contains(&tv.tag) {
            write!(stdout, "{}", tag_value.bright_red())?;
        } else {
            write!(stdout, "{}", tag_value.green())?;
        }
        write!(stdout, "{}", "|".white())?;
        Ok(())
    }
}

fn main() {
    let arg_matches = app_from_crate!()
        .arg(Arg::with_name("tags")
            .short("t")
            .long("tags")
            .value_name("tags")
            .help("Highlight individual tags")
            .takes_value(true))
        .arg(Arg::with_name("source")
            .index(1)
            .help("Source file name or omit to read from stdin"))
        .get_matches();

    // Source of data to be read
    let path = arg_matches.value_of("source").unwrap_or("-");

    // Determine which tags are to be highlighted
    let tags_value = arg_matches.value_of("tags").unwrap_or("49,56");
    let tags: Vec<u32> = tags_value.split(",").map(|x| x.parse::<u32>().unwrap()).collect();
    let tv_consumer = TagValueConsumerWriter {tags};

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
    match line.find("8=FIX") {
        None => {}
        Some(start_offset) => {
            let msg = &line.as_bytes()[start_offset..];
            match fix_parser::iterate_tags(msg, tv_consumer) {
                Ok(_) => writeln!(io::stdout()).unwrap_or_default(),
                _ => () // ignore FIX parser errors
            }
        }
    };
}