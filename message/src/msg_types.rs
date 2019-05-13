use crate::message::MsgType;

pub const HEARTBEAT: MsgType = MsgType { id: b"0", name: b"Heartbeat" };
pub const LOGOUT: MsgType = MsgType { id: b"5", name: b"Logout" };
pub const LOGON: MsgType = MsgType { id: b"A", name: b"Logon" };
