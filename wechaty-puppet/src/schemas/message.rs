use regex::Regex;

#[derive(Debug, Copy, Clone, FromPrimitive)]
pub enum MessageType {
    Unknown,
    Attachment,
    Audio,
    Contact,
    ChatHistory,
    Emoticon,
    Image,
    Text,
    Location,
    MiniProgram,
    GroupNote,
    Transfer,
    RedEnvelope,
    Recalled,
    Url,
    Video,
}

#[derive(Debug, Copy, Clone, FromPrimitive)]
pub enum WechatAppMessageType {
    Text = 1,
    Img = 2,
    Audio = 3,
    Video = 4,
    Url = 5,
    Attach = 6,
    Open = 7,
    Emoji = 8,
    VoiceRemind = 9,
    ScanGood = 10,
    Good = 13,
    Emotion = 15,
    CardTicket = 16,
    RealtimeShareLocation = 17,
    ChatHistory = 19,
    MiniProgram = 33,
    Transfers = 2000,
    RedEnvelopes = 2001,
    ReaderType = 100001,
}

#[derive(Debug, Copy, Clone, FromPrimitive)]
pub enum WechatMessageType {
    Text = 1,
    Image = 3,
    Voice = 34,
    VerifyMsg = 37,
    PossibleFriendMsg = 40,
    ShareCard = 42,
    Video = 43,
    Emoticon = 47,
    Location = 48,
    App = 49,
    VoipMsg = 50,
    StatusNotify = 51,
    VoipNotify = 52,
    VoipInvite = 53,
    MicroVideo = 62,
    Transfer = 2000,
    RedEnvelope = 2001,
    MiniProgram = 2002,
    GroupInvite = 2003,
    File = 2004,
    SysNotice = 9999,
    Sys = 10000,
    Recalled = 10002,
}

#[derive(Debug, Clone)]
pub enum MessagePayload {
    Room {
        id: String,
        filename: Option<String>,
        text: Option<String>,
        timestamp: u64,
        message_type: MessageType,
        from_id: Option<String>,
        mention_ids: Vec<String>,
        room_id: String,
        to_id: Option<String>,
    },
    To {
        id: String,
        filename: Option<String>,
        text: Option<String>,
        timestamp: u64,
        message_type: MessageType,
        from_id: String,
        room_id: Option<String>,
        to_id: String,
    },
}

#[derive(Debug, Clone)]
pub struct MessageQueryFilter {
    from_id: Option<String>,
    id: Option<String>,
    room_id: Option<String>,
    text: Option<String>,
    text_regex: Option<Regex>,
    to_id: Option<String>,
    message_type: Option<MessageType>,
}

pub type MessagePayloadFilterFunction = fn(MessagePayload) -> bool;

pub type MessagePayloadFilterFactory = fn(MessageQueryFilter) -> MessagePayloadFilterFunction;
