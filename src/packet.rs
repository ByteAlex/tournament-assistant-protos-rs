#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Acknowledgement {
    #[prost(string, tag="1")]
    pub packet_id: ::prost::alloc::string::String,
    #[prost(enumeration="acknowledgement::AcknowledgementType", tag="2")]
    pub r#type: i32,
}
/// Nested message and enum types in `Acknowledgement`.
pub mod acknowledgement {
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum AcknowledgementType {
        MessageReceived = 0,
        FileDownloaded = 1,
    }
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Command {
    #[prost(enumeration="command::CommandTypes", tag="1")]
    pub command_type: i32,
}
/// Nested message and enum types in `Command`.
pub mod command {
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum CommandTypes {
        Heartbeat = 0,
        ReturnToMenu = 1,
        ScreenOverlayShowPng = 2,
        ScreenOverlayShowGreen = 3,
        DelayTestFinish = 4,
    }
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Connect {
    #[prost(enumeration="connect::ConnectTypes", tag="1")]
    pub client_type: i32,
    #[prost(string, tag="2")]
    pub name: ::prost::alloc::string::String,
    #[prost(string, tag="3")]
    pub password: ::prost::alloc::string::String,
    #[prost(string, tag="4")]
    pub user_id: ::prost::alloc::string::String,
    #[prost(int32, tag="5")]
    pub client_version: i32,
}
/// Nested message and enum types in `Connect`.
pub mod connect {
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum ConnectTypes {
        Player = 0,
        Coordinator = 1,
        TemporaryConnection = 2,
    }
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Response {
    #[prost(enumeration="response::ResponseType", tag="1")]
    pub r#type: i32,
    #[prost(string, tag="2")]
    pub message: ::prost::alloc::string::String,
}
/// Nested message and enum types in `Response`.
pub mod response {
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum ResponseType {
        Fail = 0,
        Success = 1,
    }
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ConnectResponse {
    #[prost(message, optional, tag="1")]
    pub response: ::core::option::Option<Response>,
    #[prost(message, optional, tag="2")]
    pub self_: ::core::option::Option<super::model::User>,
    #[prost(message, optional, tag="3")]
    pub state: ::core::option::Option<super::model::State>,
    #[prost(int32, tag="4")]
    pub server_version: i32,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct File {
    #[prost(string, tag="1")]
    pub file_id: ::prost::alloc::string::String,
    #[prost(enumeration="file::Intentions", tag="2")]
    pub intent: i32,
    #[prost(bool, tag="3")]
    pub compressed: bool,
    #[prost(bytes="vec", tag="4")]
    pub data: ::prost::alloc::vec::Vec<u8>,
}
/// Nested message and enum types in `File`.
pub mod file {
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum Intentions {
        None = 0,
        ///Image will be stored in the StreamSyncController and displayed when the DelayTest_Trigger command is received
        SetPngToShowWhenTriggered = 1,
        ///Image will be immediately displayed if the StreamSyncController is active
        ShowPngImmediately = 2,
    }
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LoadedSong {
    #[prost(message, optional, tag="1")]
    pub level: ::core::option::Option<super::model::PreviewBeatmapLevel>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Event {
    #[prost(oneof="event::ChangedObject", tags="1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13")]
    pub changed_object: ::core::option::Option<event::ChangedObject>,
}
/// Nested message and enum types in `Event`.
pub mod event {
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct PlayerAddedEvent {
        #[prost(message, optional, tag="1")]
        pub player: ::core::option::Option<super::super::model::Player>,
    }
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct PlayerUpdatedEvent {
        #[prost(message, optional, tag="1")]
        pub player: ::core::option::Option<super::super::model::Player>,
    }
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct PlayerLeftEvent {
        #[prost(message, optional, tag="1")]
        pub player: ::core::option::Option<super::super::model::Player>,
    }
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct CoordinatorAddedEvent {
        #[prost(message, optional, tag="1")]
        pub coordinator: ::core::option::Option<super::super::model::Coordinator>,
    }
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct CoordinatorLeftEvent {
        #[prost(message, optional, tag="1")]
        pub coordinator: ::core::option::Option<super::super::model::Coordinator>,
    }
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct MatchCreatedEvent {
        #[prost(message, optional, tag="1")]
        pub r#match: ::core::option::Option<super::super::model::Match>,
    }
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct MatchUpdatedEvent {
        #[prost(message, optional, tag="1")]
        pub r#match: ::core::option::Option<super::super::model::Match>,
    }
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct MatchDeletedEvent {
        #[prost(message, optional, tag="1")]
        pub r#match: ::core::option::Option<super::super::model::Match>,
    }
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct QualifierCreatedEvent {
        #[prost(message, optional, tag="1")]
        pub event: ::core::option::Option<super::super::model::QualifierEvent>,
    }
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct QualifierUpdatedEvent {
        #[prost(message, optional, tag="1")]
        pub event: ::core::option::Option<super::super::model::QualifierEvent>,
    }
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct QualifierDeletedEvent {
        #[prost(message, optional, tag="1")]
        pub event: ::core::option::Option<super::super::model::QualifierEvent>,
    }
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct HostAddedEvent {
        #[prost(message, optional, tag="1")]
        pub server: ::core::option::Option<super::super::model::CoreServer>,
    }
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct HostDeletedEvent {
        #[prost(message, optional, tag="1")]
        pub server: ::core::option::Option<super::super::model::CoreServer>,
    }
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum ChangedObject {
        #[prost(message, tag="1")]
        PlayerAddedEvent(PlayerAddedEvent),
        #[prost(message, tag="2")]
        PlayerUpdatedEvent(PlayerUpdatedEvent),
        #[prost(message, tag="3")]
        PlayerLeftEvent(PlayerLeftEvent),
        #[prost(message, tag="4")]
        CoordinatorAddedEvent(CoordinatorAddedEvent),
        #[prost(message, tag="5")]
        CoordinatorLeftEvent(CoordinatorLeftEvent),
        #[prost(message, tag="6")]
        MatchCreatedEvent(MatchCreatedEvent),
        #[prost(message, tag="7")]
        MatchUpdatedEvent(MatchUpdatedEvent),
        #[prost(message, tag="8")]
        MatchDeletedEvent(MatchDeletedEvent),
        #[prost(message, tag="9")]
        QualifierCreatedEvent(QualifierCreatedEvent),
        #[prost(message, tag="10")]
        QualifierUpdatedEvent(QualifierUpdatedEvent),
        #[prost(message, tag="11")]
        QualifierDeletedEvent(QualifierDeletedEvent),
        #[prost(message, tag="12")]
        HostAddedEvent(HostAddedEvent),
        #[prost(message, tag="13")]
        HostDeletedEvent(HostDeletedEvent),
    }
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LoadSong {
    #[prost(string, tag="1")]
    pub level_id: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub custom_host_url: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PlaySong {
    #[prost(message, optional, tag="1")]
    pub gameplay_parameters: ::core::option::Option<super::model::GameplayParameters>,
    #[prost(bool, tag="3")]
    pub floating_scoreboard: bool,
    #[prost(bool, tag="4")]
    pub stream_sync: bool,
    #[prost(bool, tag="5")]
    pub disable_fail: bool,
    #[prost(bool, tag="6")]
    pub disable_pause: bool,
    #[prost(bool, tag="7")]
    pub disable_scoresaber_submission: bool,
    #[prost(bool, tag="8")]
    pub show_normal_notes_on_stream: bool,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SubmitScore {
    #[prost(message, optional, tag="1")]
    pub score: ::core::option::Option<super::model::Score>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SongFinished {
    #[prost(message, optional, tag="1")]
    pub player: ::core::option::Option<super::model::Player>,
    #[prost(message, optional, tag="2")]
    pub beatmap: ::core::option::Option<super::model::Beatmap>,
    #[prost(enumeration="song_finished::CompletionType", tag="3")]
    pub r#type: i32,
    #[prost(int32, tag="4")]
    pub score: i32,
}
/// Nested message and enum types in `SongFinished`.
pub mod song_finished {
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum CompletionType {
        Passed = 0,
        Failed = 1,
        Quit = 2,
    }
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SendBotMessage {
    #[prost(message, optional, tag="1")]
    pub channel: ::core::option::Option<super::discord::Channel>,
    #[prost(string, tag="2")]
    pub message: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ScoreRequestResponse {
    #[prost(message, repeated, tag="1")]
    pub scores: ::prost::alloc::vec::Vec<super::model::Score>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ScoreRequest {
    #[prost(string, tag="1")]
    pub event_id: ::prost::alloc::string::String,
    #[prost(message, optional, tag="2")]
    pub parameters: ::core::option::Option<super::model::GameplayParameters>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Packet {
    #[prost(string, tag="1")]
    pub id: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub from: ::prost::alloc::string::String,
    #[prost(oneof="packet::Packet", tags="3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18")]
    pub packet: ::core::option::Option<packet::Packet>,
}
/// Nested message and enum types in `Packet`.
pub mod packet {
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Packet {
        #[prost(message, tag="3")]
        Acknowledgement(super::Acknowledgement),
        #[prost(message, tag="4")]
        Command(super::Command),
        #[prost(message, tag="5")]
        Connect(super::Connect),
        #[prost(message, tag="6")]
        Response(super::Response),
        #[prost(message, tag="7")]
        ConnectResponse(super::ConnectResponse),
        #[prost(message, tag="8")]
        File(super::File),
        #[prost(message, tag="9")]
        LoadedSong(super::LoadedSong),
        #[prost(message, tag="10")]
        Event(super::Event),
        #[prost(message, tag="11")]
        LoadSong(super::LoadSong),
        #[prost(message, tag="12")]
        PlaySong(super::PlaySong),
        #[prost(message, tag="13")]
        SubmitScore(super::SubmitScore),
        #[prost(message, tag="14")]
        SongFinished(super::SongFinished),
        #[prost(message, tag="15")]
        SendBotMessage(super::SendBotMessage),
        #[prost(message, tag="16")]
        ScoreRequestResponse(super::ScoreRequestResponse),
        #[prost(message, tag="17")]
        ScoreRequest(super::ScoreRequest),
        #[prost(message, tag="18")]
        ForwardingPacket(::prost::alloc::boxed::Box<super::ForwardingPacket>),
    }
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ForwardingPacket {
    #[prost(string, repeated, tag="1")]
    pub forward_to: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    #[prost(message, optional, boxed, tag="2")]
    pub packet: ::core::option::Option<::prost::alloc::boxed::Box<Packet>>,
}
