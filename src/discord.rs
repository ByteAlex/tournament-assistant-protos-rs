#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Channel {
    #[prost(int32, tag="1")]
    pub id: i32,
    #[prost(string, tag="2")]
    pub name: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DiscordUser {
    #[prost(int32, tag="1")]
    pub id: i32,
    #[prost(string, tag="2")]
    pub name: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Guild {
    #[prost(int32, tag="1")]
    pub id: i32,
    #[prost(string, tag="2")]
    pub name: ::prost::alloc::string::String,
}
