use crate::protocol::{FixMessage, ProtocolError};

/// The absolute minimum length of a FIX message. 8=FIX.4.x|9=nn|35=x|10=nnn|
const MIN_MESSAGE_LEN: usize = 27;

/// The FIX message must start with the following three tags in order:
/// BeginString(8)
/// BodyLength(9)
/// MsgType(35)
/// BodyLength - is number of bytes starting from 35 (inclusive) up to the delimiter preceding the Checksum(10)
/// Checksum - is the modulo 256 over all bytes up to the delimiter preceding the Checksum(10)
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TagValue<'a> {
    // The number of the tag
    pub tag: u32,
    // The name of the tag (later)
    //pub name: Vec<u8>,
    // The buffer index of the value
    pub offset: usize,
    // The length of the value
    pub length: usize,
    // The underlying buffer
    pub buffer: &'a [u8],
}

pub struct ConsumerError;

pub trait Consumer<T> {
    fn accept(&self, arg: T) -> Result<(), ConsumerError>;
}

impl From<std::io::Error> for ConsumerError {
    fn from(_err: std::io::Error) -> ConsumerError {
        ConsumerError{}
    }
}

/// Assumes array starts with nn=value|
pub fn read_tag(msg: &[u8], offset: usize) -> Result<TagValue, ProtocolError> {
    let mut index = offset;
    let mut num: u32 = 0;
    // consume the tag
    while msg[index] >= b'0' && msg[index] <= b'9' {
        num *= 10;
        num += (msg[index] - b'0') as u32;
        index += 1;
    }
    // index should now be pointing at the = separator
    if msg[index] != b'=' {
        return Err(ProtocolError::Malformed);
    }
    // skip to next byte and assume value until reach SOH
    index += 1;
    if index == msg.len() {
        return Err(ProtocolError::Malformed);
    }
    let value_offset = index;
    while msg[index] != 1u8 {
        // if we find an = sign then barf, that's ridiculous
        if msg[index] == b'=' {
            return Err(ProtocolError::Malformed);
        }
        index += 1;
        // reached the end but no delimiter?
        if index == msg.len() {
            return Err(ProtocolError::MissingDelimiter);
        }
    }
    Ok(TagValue { tag: num, offset: value_offset, length: index - value_offset, buffer: msg })
}

pub fn iterate_tags<'a>(msg: &'a [u8], consumer: &'a Consumer<TagValue<'a>>) -> Result<FixMessage, ProtocolError> {
    let mut offset = 0;
    while offset < msg.len() {
        match read_tag(&msg, offset) {
            Ok(tag) => {
                offset = tag.offset + tag.length + 1;
                if let Err(_err) = consumer.accept(tag) {
                    break
                }
            },
            Err(e) => return Err(e),
        }
        //Ok(());
    }
    // TODO Change return type when it comes to actually consuming a FixMessage
    Ok(FixMessage { msg_length: 1 })
}

pub fn parse(msg: &[u8]) -> Result<FixMessage, ProtocolError> {
    if msg.len() < MIN_MESSAGE_LEN {
        return Err(ProtocolError::Incomplete);
    }
    let _tag = read_tag(&msg, 0)?;
    Ok(FixMessage { msg_length: 1 })
}

#[cfg(test)]
mod tests {
    use crate::protocol::*;

    use super::Consumer;
    use super::ConsumerError;
    use super::iterate_tags;
    use super::parse;
    use super::TagValue;

    struct NullTagValueConsumer;

    impl<'a> Consumer<TagValue<'a>> for NullTagValueConsumer {
        fn accept(&self, _arg: TagValue) -> Result<(), ConsumerError>{
            Ok(())
        }
    }

    #[test]
    fn parse_failures() {
        assert_eq!(parse(b"8=FIX.4.4\x01"), Err(ProtocolError::Incomplete));
    }

    #[test]
    fn iterate_tags_failures() {
        let null_consumer = NullTagValueConsumer {};
        assert_eq!(iterate_tags(b"3=abc\x014=", &null_consumer), Err(ProtocolError::Malformed));
        assert_eq!(iterate_tags(b"3=xyz=\x01", &null_consumer), Err(ProtocolError::Malformed));
        assert_eq!(iterate_tags(b"2=FIX.4.4", &null_consumer), Err(ProtocolError::MissingDelimiter));
    }

    #[test]
    fn parse_ok() {
        assert!(parse(b"8=FIX.4.4\x019=99\x0135=A\x0110=000\x01").is_ok());
    }
}