///
/// Message tags:
/// 8 - BeginString - set by user
/// 9 - BodyLen - set when sending
/// 35 - MsgType - set when created
/// 49 - SenderCompID - set by user/config
/// 56 - TargetCompID - set by user/config
/// 34 - MsgSeqNum - set when sending
/// 52 - SendingTime - set when sending
/// <Message specific tags>
/// 10 - Checksum - set when sending
///
pub struct MsgType {
    id: [u8],
    name: [u8],
}

pub struct Message {
    msg_type: MsgType
//    begin_string,
//    body_len,
//    buffer,
//    checksum
}

