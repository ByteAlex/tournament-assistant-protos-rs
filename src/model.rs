#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Characteristic {
    #[prost(string, tag="1")]
    pub serialized_name: ::prost::alloc::string::String,
    #[prost(int32, repeated, tag="2")]
    pub difficulties: ::prost::alloc::vec::Vec<i32>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Beatmap {
    #[prost(string, tag="1")]
    pub name: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub level_id: ::prost::alloc::string::String,
    #[prost(message, optional, tag="3")]
    pub characteristic: ::core::option::Option<Characteristic>,
    #[prost(int32, tag="4")]
    pub difficulty: i32,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PreviewBeatmapLevel {
    #[prost(string, tag="1")]
    pub level_id: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub name: ::prost::alloc::string::String,
    #[prost(message, repeated, tag="3")]
    pub characteristics: ::prost::alloc::vec::Vec<Characteristic>,
    #[prost(bool, tag="4")]
    pub loaded: bool,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct User {
    #[prost(string, tag="2")]
    pub id: ::prost::alloc::string::String,
    #[prost(string, tag="3")]
    pub name: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Coordinator {
    #[prost(message, optional, tag="1")]
    pub user: ::core::option::Option<User>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GameplayModifiers {
    #[prost(enumeration="gameplay_modifiers::GameOptions", tag="1")]
    pub options: i32,
}
/// Nested message and enum types in `GameplayModifiers`.
pub mod gameplay_modifiers {
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum GameOptions {
        None = 0,
        ///Negative modifiers
        NoFail = 1,
        NoBombs = 2,
        NoArrows = 4,
        NoObstacles = 8,
        SlowSong = 16,
        ///Positive Modifiers
        InstaFail = 32,
        FailOnClash = 64,
        BatteryEnergy = 128,
        FastNotes = 256,
        FastSong = 512,
        DisappearingArrows = 1024,
        GhostNotes = 2048,
        ///1.12.2 Additions
        DemoNoFail = 4096,
        DemoNoObstacles = 8192,
        StrictAngles = 16384,
        ///1.13.4 Additions
        ProMode = 32768,
        ZenMode = 65536,
        SmallCubes = 131072,
        SuperFastSong = 262144,
    }
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PlayerSpecificSettings {
    #[prost(float, tag="1")]
    pub player_height: f32,
    #[prost(float, tag="2")]
    pub sfx_volume: f32,
    #[prost(float, tag="3")]
    pub saber_trail_intensity: f32,
    #[prost(float, tag="4")]
    pub note_jump_start_beat_offset: f32,
    #[prost(float, tag="5")]
    pub note_jump_fixed_duration: f32,
    #[prost(enumeration="player_specific_settings::PlayerOptions", tag="6")]
    pub options: i32,
    #[prost(enumeration="player_specific_settings::NoteJumpDurationTypeSettings", tag="7")]
    pub note_jump_duration_type_settings: i32,
}
/// Nested message and enum types in `PlayerSpecificSettings`.
pub mod player_specific_settings {
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum PlayerOptions {
        None = 0,
        LeftHanded = 1,
        StaticLights = 2,
        NoHud = 4,
        AdvancedHud = 8,
        ReduceDebris = 16,
        AutoPlayerHeight = 32,
        NoFailEffects = 64,
        AutoRestart = 128,
        HideNoteSpawnEffect = 256,
        AdaptiveSfx = 512,
    }
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum NoteJumpDurationTypeSettings {
        Dynamic = 0,
        Static = 1,
    }
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GameplayParameters {
    #[prost(message, optional, tag="1")]
    pub beatmap: ::core::option::Option<Beatmap>,
    #[prost(message, optional, tag="2")]
    pub player_settings: ::core::option::Option<PlayerSpecificSettings>,
    #[prost(message, optional, tag="3")]
    pub gameplay_modifiers: ::core::option::Option<GameplayModifiers>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Team {
    #[prost(string, tag="1")]
    pub id: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub name: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ServerSettings {
    #[prost(string, tag="1")]
    pub server_name: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub password: ::prost::alloc::string::String,
    #[prost(bool, tag="3")]
    pub enable_teams: bool,
    #[prost(message, repeated, tag="4")]
    pub teams: ::prost::alloc::vec::Vec<Team>,
    #[prost(int32, tag="5")]
    pub score_update_frequency: i32,
    #[prost(string, repeated, tag="6")]
    pub banned_mods: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SongList {
    #[prost(message, repeated, tag="1")]
    pub levels: ::prost::alloc::vec::Vec<PreviewBeatmapLevel>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Player {
    #[prost(message, optional, tag="1")]
    pub user: ::core::option::Option<User>,
    #[prost(string, tag="2")]
    pub user_id: ::prost::alloc::string::String,
    #[prost(message, optional, tag="3")]
    pub team: ::core::option::Option<Team>,
    #[prost(enumeration="player::PlayStates", tag="4")]
    pub play_state: i32,
    #[prost(enumeration="player::DownloadStates", tag="5")]
    pub download_state: i32,
    #[prost(int32, tag="6")]
    pub score: i32,
    #[prost(int32, tag="7")]
    pub combo: i32,
    #[prost(double, tag="8")]
    pub accuracy: f64,
    #[prost(double, tag="9")]
    pub song_position: f64,
    #[prost(message, optional, tag="10")]
    pub song_list: ::core::option::Option<SongList>,
    #[prost(string, repeated, tag="11")]
    pub mod_list: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    #[prost(message, optional, tag="12")]
    pub stream_screen_coordinates: ::core::option::Option<player::Point>,
    #[prost(int64, tag="13")]
    pub stream_delay_ms: i64,
    #[prost(int64, tag="14")]
    pub stream_sync_start_ms: i64,
}
/// Nested message and enum types in `Player`.
pub mod player {
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Point {
        #[prost(int32, tag="1")]
        pub x: i32,
        #[prost(int32, tag="2")]
        pub y: i32,
    }
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum PlayStates {
        Waiting = 0,
        InGame = 1,
    }
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum DownloadStates {
        None = 0,
        Downloading = 1,
        Downloaded = 2,
        DownloadError = 3,
    }
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Match {
    #[prost(string, tag="1")]
    pub guid: ::prost::alloc::string::String,
    #[prost(message, repeated, tag="2")]
    pub players: ::prost::alloc::vec::Vec<Player>,
    #[prost(message, optional, tag="3")]
    pub leader: ::core::option::Option<User>,
    #[prost(message, optional, tag="5")]
    pub selected_level: ::core::option::Option<PreviewBeatmapLevel>,
    #[prost(message, optional, tag="6")]
    pub selected_characteristic: ::core::option::Option<Characteristic>,
    #[prost(int32, tag="7")]
    pub selected_difficulty: i32,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QualifierEvent {
    #[prost(string, tag="1")]
    pub event_id: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub name: ::prost::alloc::string::String,
    #[prost(message, optional, tag="3")]
    pub guild: ::core::option::Option<super::discord::Guild>,
    #[prost(message, optional, tag="4")]
    pub info_channel: ::core::option::Option<super::discord::Channel>,
    #[prost(message, repeated, tag="5")]
    pub qualifier_maps: ::prost::alloc::vec::Vec<GameplayParameters>,
    #[prost(bool, tag="6")]
    pub send_scores_to_info_channel: bool,
    #[prost(int32, tag="7")]
    pub flags: i32,
}
/// Nested message and enum types in `QualifierEvent`.
pub mod qualifier_event {
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum EventSettings {
        None = 0,
        HideScoresFromPlayers = 1,
        DisableScoresaberSubmission = 2,
        EnableLeaderboardMessage = 3,
    }
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CoreServer {
    #[prost(string, tag="1")]
    pub name: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub address: ::prost::alloc::string::String,
    #[prost(int32, tag="3")]
    pub port: i32,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct State {
    #[prost(message, optional, tag="1")]
    pub server_settings: ::core::option::Option<ServerSettings>,
    #[prost(message, repeated, tag="2")]
    pub players: ::prost::alloc::vec::Vec<Player>,
    #[prost(message, repeated, tag="3")]
    pub coordinators: ::prost::alloc::vec::Vec<Coordinator>,
    #[prost(message, repeated, tag="4")]
    pub matches: ::prost::alloc::vec::Vec<Match>,
    #[prost(message, repeated, tag="5")]
    pub events: ::prost::alloc::vec::Vec<QualifierEvent>,
    #[prost(message, repeated, tag="6")]
    pub known_hosts: ::prost::alloc::vec::Vec<CoreServer>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Score {
    #[prost(string, tag="1")]
    pub event_id: ::prost::alloc::string::String,
    #[prost(message, optional, tag="2")]
    pub parameters: ::core::option::Option<GameplayParameters>,
    #[prost(string, tag="3")]
    pub user_id: ::prost::alloc::string::String,
    #[prost(string, tag="4")]
    pub username: ::prost::alloc::string::String,
    #[prost(int32, tag="5")]
    pub score: i32,
    #[prost(bool, tag="6")]
    pub full_combo: bool,
    #[prost(string, tag="7")]
    pub color: ::prost::alloc::string::String,
}
