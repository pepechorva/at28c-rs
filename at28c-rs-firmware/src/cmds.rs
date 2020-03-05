use core::{
    convert::{From, Into},
    default::Default,
    mem,
};

#[derive(Copy, Clone, PartialEq)]
#[repr(u8)]
pub enum Commands {
    #[allow(dead_code)]
    Connect = 0x00,
    #[allow(dead_code)]
    ReadByte = 0x01,
    WriteByte = 0x02,
    WritePage = 0x03,
    #[allow(dead_code)]
    Disconnect = 0x04,
    #[allow(dead_code)]
    QueryState = 0x05,
    ReadPage = 0x06,
    DisableProctetion = 0x07,
    Invalid = 0x08,
}

#[derive(Copy, Clone, PartialEq)]
#[repr(u8)]
pub enum Response {
    Connected = 0x00,
    WriteDone = 0x02,
    Idle = 0x04,
    Busy = 0x05,
    Disconnected = 0x06,
    NotValid = 0x07,
    NoResponse = 0x08,
    SendPage = 0x09,
    Error = 0x10,
}

#[derive(Copy, Clone, PartialEq)]
pub enum State {
    Idle = 0x00,
    Busy = 0x01,
    WaitingPage = 0x02,
    Disconnected = 0x03,
    Sending = 0x04,
}

impl Default for State {
    fn default() -> Self {
        Self::Disconnected
    }
}

impl Into<u8> for Response {
    fn into(self) -> u8 {
        // NOTE(unsafe), Response is repr(u8)
        unsafe { mem::transmute(self) }
    }
}

impl From<u8> for Commands {
    fn from(cmd: u8) -> Self {
        if cmd <= 0x07 {
            // NOTE(unsafe), Commands is repr(u8)
            unsafe { mem::transmute(cmd) }
        } else {
            Commands::Invalid
        }
    }
}

impl Commands {
    pub fn process(cmd: Self, state: &mut State) -> Response {
        match cmd {
            Commands::Connect => {
                if *state != State::Disconnected {
                    Response::NotValid
                } else {
                    *state = State::Idle;
                    Response::Connected
                }
            }
            Commands::ReadByte
            | Commands::WriteByte
            | Commands::DisableProctetion
            | Commands::ReadPage => {
                if *state != State::Idle {
                    Response::NotValid
                } else {
                    *state = State::Busy;
                    Response::NoResponse
                }
            }
            Commands::WritePage => {
                if *state != State::Idle {
                    Response::NotValid
                } else {
                    *state = State::WaitingPage;
                    Response::SendPage
                }
            }
            Commands::Disconnect => {
                if *state != State::Idle {
                    Response::NotValid
                } else {
                    *state = State::Disconnected;
                    Response::Disconnected
                }
            }
            Commands::QueryState => match *state {
                State::Idle => Response::Idle,
                State::Busy | State::Sending => Response::Busy,
                State::Disconnected => Response::Disconnected,
                State::WaitingPage => Response::SendPage,
            },
            Commands::Invalid => Response::NoResponse,
        }
    }
}