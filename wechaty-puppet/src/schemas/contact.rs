use regex::Regex;

#[derive(Debug, Clone, PartialEq, FromPrimitive)]
pub enum ContactGender {
    Unknown,
    Male,
    Female,
}

#[derive(Debug, Clone, PartialEq, FromPrimitive)]
pub enum ContactType {
    Unknown,
    Individual,
    Official,
    Corporation,
}

#[derive(Debug, Clone)]
pub struct ContactPayload {
    id: String,
    gender: ContactGender,
    contact_type: ContactType,
    name: String,
    avatar: String,
    address: String,
    alias: String,
    city: String,
    friend: bool,
    province: String,
    signature: String,
    star: bool,
    weixin: String,
    corporation: String,
    title: String,
    description: String,
    coworker: bool,
    phone: Vec<String>,
}

#[derive(Debug, Clone)]
pub struct ContactQueryFilter {
    alias: Option<String>,
    alias_regex: Option<Regex>,
    id: Option<String>,
    name: Option<String>,
    name_regex: Option<Regex>,
    weixin: Option<String>,
}

pub type ContactPayloadFilterFunction = fn(ContactPayload) -> bool;

pub type ContactPayloadFilterFactory = fn(ContactQueryFilter) -> ContactPayloadFilterFunction;
