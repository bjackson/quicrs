use std::io;
use std::result;
use std::error::Error;
use std::fmt;
use std::string;


bitflags! {
    pub flags TransportErrorFlag: u32 {
        const QUIC_INTERNAL_ERROR                        = 0x80000001,
        const QUIC_STREAM_DATA_AFTER_TERMINATION         = 0x80000002,
        const QUIC_INVALID_PACKET_HEADER                 = 0x80000003,
        const QUIC_INVALID_FRAME_DATA                    = 0x80000004,
        const QUIC_MULTIPLE_TERMINATION_OFFSETS          = 0x80000005,
        const QUIC_STREAM_CANCELLED                      = 0x80000006,
        const QUIC_CLOSED_CRITICAL_STREAM                = 0x80000007,
        const QUIC_MISSING_PAYLOAD                       = 0x80000030,
        const QUIC_INVALID_STREAM_DATA                   = 0x8000002E,
        const QUIC_UNENCRYPTED_STREAM_DATA               = 0x8000003D,
        const QUIC_MAYBE_CORRUPTED_MEMORY                = 0x80000059,
        const QUIC_INVALID_RST_STREAM_DATA               = 0x80000006,
        const QUIC_INVALID_CONNECTION_CLOSE_DATA         = 0x80000007,
        const QUIC_INVALID_GOAWAY_DATA                   = 0x80000008,
        const QUIC_INVALID_WINDOW_UPDATE_DATA            = 0x80000039,
        const QUIC_INVALID_BLOCKED_DATA                  = 0x8000003A,
        const QUIC_INVALID_PATH_CLOSE_DATA               = 0x8000004E,
        const QUIC_INVALID_ACK_DATA                      = 0x80000009,
        const QUIC_INVALID_VERSION_NEGOTIATION_PACKET    = 0x8000000A,
        const QUIC_INVALID_PUBLIC_RST_PACKET             = 0x8000000B,
        const QUIC_DECRYPTION_FAILURE                    = 0x8000000C,
        const QUIC_ENCRYPTION_FAILURE                    = 0x8000000D,
        const QUIC_PACKET_TOO_LARGE                      = 0x8000000E,
        const QUIC_PEER_GOING_AWAY                       = 0x80000010,
        const QUIC_INVALID_STREAM_ID                     = 0x80000011,
        const QUIC_INVALID_PRIORITY                      = 0x80000031,
        const QUIC_TOO_MANY_OPEN_STREAMS                 = 0x80000012,
        const QUIC_TOO_MANY_AVAILABLE_STREAMS            = 0x8000004c,
        const QUIC_PUBLIC_RESET                          = 0x80000013,
        const QUIC_INVALID_VERSION                       = 0x80000014,
        const QUIC_INVALID_HEADER_ID                     = 0x80000016,
        const QUIC_INVALID_NEGOTIATED_VALUE              = 0x80000017,
        const QUIC_DECOMPRESSION_FAILURE                 = 0x80000018,
        const QUIC_NETWORK_IDLE_TIMEOUT                  = 0x80000019,
        const QUIC_HANDSHAKE_TIMEOUT                     = 0x80000043,
        const QUIC_ERROR_MIGRATING_ADDRESS               = 0x8000001a,
        const QUIC_ERROR_MIGRATING_PORT                  = 0x80000056,
        const QUIC_EMPTY_STREAM_FRAME_NO_FIN             = 0x80000032,
        const QUIC_FLOW_CONTROL_RECEIVED_TOO_MUCH_DATA   = 0x8000003B,
        const QUIC_FLOW_CONTROL_SENT_TOO_MUCH_DATA       = 0x8000003F,
        const QUIC_FLOW_CONTROL_INVALID_WINDOW           = 0x80000040,
        const QUIC_CONNECTION_IP_POOLED                  = 0x8000003E,
        const QUIC_TOO_MANY_OUTSTANDING_SENT_PACKETS     = 0x80000044,
        const QUIC_TOO_MANY_OUTSTANDING_RECEIVED_PACKETS = 0x80000045,
        const QUIC_CONNECTION_CANCELLED                  = 0x80000046,
        const QUIC_BAD_PACKET_LOSS_RATE                  = 0x80000047,
        const QUIC_PUBLIC_RESETS_POST_HANDSHAKE          = 0x80000049,
        const QUIC_TIMEOUTS_WITH_OPEN_STREAMS            = 0x8000004a,
        const QUIC_TOO_MANY_RTOS                         = 0x80000055,
        const QUIC_ENCRYPTION_LEVEL_INCORRECT            = 0x8000002c,
        const QUIC_VERSION_NEGOTIATION_MISMATCH          = 0x80000037,
        const QUIC_IP_ADDRESS_CHANGED                    = 0x80000050,
        const QUIC_ADDRESS_VALIDATION_FAILURE            = 0x80000051,
        const QUIC_TOO_MANY_FRAME_GAPS                   = 0x8000005d,
        const QUIC_TOO_MANY_SESSIONS_ON_SERVER           = 0x80000060,
    }
}

#[derive(Debug)]
pub enum QuicError {
    Io(io::Error),
    ParseError,
    SerializeError,
    FromUtf8Error(string::FromUtf8Error),
    PacketTooLarge,
    TransportError(TransportErrorFlag),
    StringParseError(string::ParseError)
}

pub type Result<T> = result::Result<T, QuicError>;

impl Error for QuicError {
    fn description(&self) -> &str {
        match *self {
            QuicError::Io(ref err) => err.description(),
            QuicError::FromUtf8Error(ref err) => err.description(),
            QuicError::ParseError => "Error parsing packet",
            QuicError::SerializeError => "Error serializing packet",
            QuicError::PacketTooLarge => "Packet too large",
            QuicError::TransportError(_) => "Transport error",
            QuicError::StringParseError(_) => "String parse error",
        }
    }

    fn cause(&self) -> Option<&Error> {
        match *self {
            QuicError::Io(ref err) => Some(err),
            QuicError::FromUtf8Error(ref err) => Some(err),
            QuicError::ParseError | QuicError::SerializeError
            | QuicError::PacketTooLarge | QuicError::TransportError(_)
            | QuicError::StringParseError(_) => None,
        }
    }
}

impl From<io::Error> for QuicError {
    fn from(err: io::Error) -> QuicError {
        QuicError::Io(err)
    }
}

impl From<string::FromUtf8Error> for QuicError {
    fn from(err: string::FromUtf8Error) -> QuicError {
        QuicError::FromUtf8Error(err)
    }
}

impl From<string::ParseError> for QuicError {
    fn from(err: string::ParseError) -> QuicError { QuicError::StringParseError(err) }
}

impl fmt::Display for QuicError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            QuicError::Io(ref err) => err.fmt(f),
            QuicError::FromUtf8Error(ref err) => err.fmt(f),
            QuicError::StringParseError(ref err) => err.fmt(f),
            QuicError::ParseError => write!(f, "Error parsing packet"),
            QuicError::SerializeError => write!(f, "Error serializing packet"),
            QuicError::PacketTooLarge => write!(f, "Packet too large"),
            QuicError::TransportError(ref err) => write!(f, "Transport Error: 0x{:X}", err.bits()),
        }
    }
}