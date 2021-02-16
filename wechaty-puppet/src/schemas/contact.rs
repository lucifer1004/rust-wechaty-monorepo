use num::FromPrimitive;
use regex::Regex;
use serde_repr::{Deserialize_repr, Serialize_repr};
use wechaty_grpc::puppet::ContactPayloadResponse;

#[derive(Debug, Clone, PartialEq, FromPrimitive, Deserialize_repr, Serialize_repr)]
#[repr(i32)]
pub enum ContactGender {
    Unknown,
    Male,
    Female,
}

#[derive(Debug, Clone, PartialEq, FromPrimitive, Deserialize_repr, Serialize_repr)]
#[repr(i32)]
pub enum ContactType {
    Unknown,
    Individual,
    Official,
    Corporation,
}

#[derive(Debug, Clone)]
pub struct ContactPayload {
    pub id: String,
    pub gender: ContactGender,
    pub contact_type: ContactType,
    pub name: String,
    pub avatar: String,
    pub address: String,
    pub alias: String,
    pub city: String,
    pub friend: bool,
    pub province: String,
    pub signature: String,
    pub star: bool,
    pub weixin: String,
    pub corporation: String,
    pub title: String,
    pub description: String,
    pub coworker: bool,
    pub phone: Vec<String>,
}

impl From<ContactPayloadResponse> for ContactPayload {
    fn from(response: ContactPayloadResponse) -> Self {
        Self {
            id: response.id,
            gender: FromPrimitive::from_i32(response.gender).unwrap(),
            contact_type: FromPrimitive::from_i32(response.r#type).unwrap(),
            name: response.name,
            avatar: response.avatar,
            address: response.address,
            alias: response.alias,
            city: response.city,
            friend: response.friend,
            province: response.province,
            signature: response.signature,
            star: response.star,
            weixin: response.weixin,
            corporation: response.corporation,
            title: response.title,
            description: response.description,
            coworker: response.coworker,
            phone: response.phone,
        }
    }
}

#[derive(Debug, Clone)]
pub struct ContactQueryFilter {
    pub alias: Option<String>,
    pub alias_regex: Option<Regex>,
    pub id: Option<String>,
    pub name: Option<String>,
    pub name_regex: Option<Regex>,
    pub weixin: Option<String>,
}

// FIXME: trait aliases are experimental, see issue #41517 <https://github.com/rust-lang/rust/issues/41517>
// pub trait ContactPayloadFilterFunction = Fn(ContactPayload) -> bool;
//
// pub trait ContactPayloadFilterFactory = Fn(ContactQueryFilter) -> ContactPayloadFilterFunction;
