/// The maximum length of the textual size of an embed.
pub const EMBED_MAX_LENGTH: u16 = 4000;
/// The gateway version used by the library. The gateway URI is retrieved via
/// the REST API.
pub const GATEWAY_VERSION: u8 = 6;
/// The large threshold to send on identify.
pub const LARGE_THRESHOLD: u8 = 250;
/// The maximum unicode code points allowed within a message by Discord.
pub const MESSAGE_CODE_LIMIT: u16 = 2000;
/// The [UserAgent] sent along with every request.
///
/// [UserAgent]: ../hyper/header/struct.UserAgent.html
pub const USER_AGENT: &'static str = concat!("DiscordBot (https://github.com/zeyla/serenity, ", env!("CARGO_PKG_VERSION"), ")");

#[allow(dead_code)]
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum ErrorCode {
    BotsCannotUse,
    CannotSendEmptyMessage,
    CannotSendMessagesInVoice,
    CannotSendMessagesToUser,
    ChannelVerificationTooHigh,
    EditByOtherAuthor,
    EmbedDisabled,
    InvalidAccountType,
    InvalidAuthToken,
    InvalidBulkDeleteCount,
    InvalidDMChannelAction,
    InvalidOauthState,
    InvalidPinChannel,
    MaxFriendsReached,
    MaxGuildsReached,
    MaxPinsReached,
    MaxRolesReached,
    MissingAccess,
    MissingPermissions,
    NoteTooLong,
    Oauth2ApplicationLacksBot,
    Oauth2ApplicationLimitReached,
    OnlyBotsCanUse,
    ReactionBlocked,
    SearchIndexUnavailable,
    TooManyReactions,
    Unauthorized,
    UnknownAccount,
    UnknownApplication,
    UnknownChannel,
    UnknownGuild,
    UnknownEmoji,
    UnknownIntegration,
    UnknownInvite,
    UnknownMember,
    UnknownMessage,
    UnknownOverwrite,
    UnknownProvider,
    UnknownRole,
    UnknownToken,
    UnknownUser,
}

enum_number!(
    /// Enum to map gateway opcodes.
    OpCode {
        /// Dispatches an event.
        Event = 0,
        /// Used for ping checking.
        Heartbeat = 1,
        /// Used for client handshake.
        Identify = 2,
        /// Used to update the client status.
        StatusUpdate = 3,
        /// Used to join/move/leave voice channels.
        VoiceStateUpdate = 4,
        /// Used for voice ping checking.
        VoiceServerPing = 5,
        /// Used to resume a closed connection.
        Resume = 6,
        /// Used to tell clients to reconnect to the gateway.
        Reconnect = 7,
        /// Used to request guild members.
        GetGuildMembers = 8,
        /// Used to notify clients that they have an invalid session Id.
        InvalidSession = 9,
        /// Sent immediately after connection, contains heartbeat + server info.
        Hello = 10,
        /// Sent immediately following a client heartbeat that was received.
        HeartbeatAck = 11,
    }
);

impl OpCode {
    pub fn num(&self) -> u64 {
        match *self {
            OpCode::Event => 0,
            OpCode::Heartbeat => 1,
            OpCode::Identify => 2,
            OpCode::StatusUpdate => 3,
            OpCode::VoiceStateUpdate => 4,
            OpCode::VoiceServerPing => 5,
            OpCode::Resume => 6,
            OpCode::Reconnect => 7,
            OpCode::GetGuildMembers => 8,
            OpCode::InvalidSession => 9,
            OpCode::Hello => 10,
            OpCode::HeartbeatAck => 11,
        }
    }
}

enum_number!(
    /// Enum to map voice opcodes.
    VoiceOpCode {
        /// Used to begin a voice websocket connection.
        Identify = 0,
        /// Used to select the voice protocol.
        SelectProtocol = 1,
        /// Used to complete the websocket handshake.
        Hello = 2,
        /// Used to keep the websocket connection alive.
        KeepAlive = 3,
        /// Used to describe the session.
        SessionDescription = 4,
        /// Used to indicate which users are speaking.
        Speaking = 5,
        /// Used to heartbeat.
        Heartbeat = 8,
    }
);

impl VoiceOpCode {
    pub fn num(&self) -> u64 {
        match *self {
            VoiceOpCode::Identify => 0,
            VoiceOpCode::SelectProtocol => 1,
            VoiceOpCode::Hello => 2,
            VoiceOpCode::KeepAlive => 3,
            VoiceOpCode::SessionDescription => 4,
            VoiceOpCode::Speaking => 5,
            VoiceOpCode::Heartbeat => 8,
        }
    }
}
