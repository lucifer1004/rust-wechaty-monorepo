use std::collections::HashSet;
use std::fmt;

use actix::{Actor, Addr, Context, Handler, Recipient};
use async_trait::async_trait;
use log::{debug, error, info};
use lru::LruCache;

use crate::error::PuppetError;
use crate::events::PuppetEvent;
use crate::schemas::contact::{ContactPayload, ContactQueryFilter};
use crate::schemas::friendship::FriendshipPayload;
use crate::schemas::image::ImageType;
use crate::schemas::message::{MessagePayload, MessageQueryFilter, MessageType};
use crate::schemas::mini_program::MiniProgramPayload;
use crate::schemas::payload::PayloadType;
use crate::schemas::room::{RoomMemberPayload, RoomPayload, RoomQueryFilter};
use crate::schemas::room_invitation::RoomInvitationPayload;
use crate::schemas::url_link::UrlLinkPayload;

const DEFAULT_CONTACT_CACHE_CAP: usize = 3000;
const DEFAULT_FRIENDSHIP_CACHE_CAP: usize = 300;
const DEFAULT_MESSAGE_CACHE_CAP: usize = 500;
const DEFAULT_ROOM_CACHE_CAP: usize = 500;
const DEFAULT_ROOM_MEMBER_CACHE_CAP: usize = 30000;
const DEFAULT_ROOM_INVITATION_CACHE_CAP: usize = 100;

// TODO: FileBox Implementation
pub struct FileBox {}

impl FileBox {
    pub fn to_string(&self) -> String {
        String::new()
    }
}

impl fmt::Display for FileBox {
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(fmt, "{}", self.to_string())
    }
}

impl From<String> for FileBox {
    fn from(_: String) -> Self {
        Self {}
    }
}

pub struct Puppet<T>
    where
        T: PuppetImpl,
{
    puppet_impl: T,
    addr: Addr<PuppetInner>,
    inner: PuppetInner,
    cache_contact_payload: LruCache<String, ContactPayload>,
    cache_friendship_payload: LruCache<String, FriendshipPayload>,
    cache_message_payload: LruCache<String, MessagePayload>,
    cache_room_payload: LruCache<String, RoomPayload>,
    cache_room_member_payload: LruCache<String, RoomMemberPayload>,
    cache_room_invitation_payload: LruCache<String, RoomInvitationPayload>,
    id: Option<String>,
}

#[derive(Clone)]
struct PuppetInner {
    subscribers: Vec<Recipient<PuppetEvent>>,
}

impl PuppetInner {
    fn new() -> Self {
        Self {
            subscribers: Vec::new()
        }
    }
}

impl Actor for PuppetInner {
    type Context = Context<Self>;

    fn started(&mut self, _ctx: &mut Self::Context) {
        info!("Puppet started");
    }

    fn stopped(&mut self, _ctx: &mut Self::Context) {
        info!("Puppet stopped");
    }
}

impl Handler<PuppetEvent> for PuppetInner {
    type Result = ();

    fn handle(&mut self, msg: PuppetEvent, _ctx: &mut Self::Context) -> Self::Result {
        for subscriber in self.subscribers.clone() {
            match subscriber.do_send(msg.clone()) {
                Err(e) => {
                    error!("Internal error: {}", e);
                }
                Ok(_) => {}
            }
        }
    }
}

impl<T> Puppet<T>
    where
        T: PuppetImpl,
{
    pub fn new(puppet_impl: T) -> Self {
        let inner = PuppetInner::new();

        Self {
            puppet_impl,
            addr: inner.clone().start(),
            inner,
            cache_contact_payload: LruCache::new(DEFAULT_CONTACT_CACHE_CAP),
            cache_friendship_payload: LruCache::new(DEFAULT_FRIENDSHIP_CACHE_CAP),
            cache_message_payload: LruCache::new(DEFAULT_MESSAGE_CACHE_CAP),
            cache_room_payload: LruCache::new(DEFAULT_ROOM_CACHE_CAP),
            cache_room_member_payload: LruCache::new(DEFAULT_ROOM_MEMBER_CACHE_CAP),
            cache_room_invitation_payload: LruCache::new(DEFAULT_ROOM_INVITATION_CACHE_CAP),
            id: None,
        }
    }

    pub fn self_addr(&self) -> Recipient<PuppetEvent> {
        self.addr.clone().recipient()
    }

    pub fn self_id(self) -> Option<String> {
        debug!("self_id()");
        self.id.clone()
    }

    pub fn log_on_off(self) -> bool {
        debug!("log_on_off()");
        match self.id {
            Some(_) => true,
            None => false,
        }
    }

    /*
        Contact
    */

    pub async fn contact_payload(&mut self, contact_id: String) -> Result<ContactPayload, PuppetError> {
        debug!("contact_payload(contact_id = {})", contact_id);
        if self.cache_contact_payload.contains(&contact_id) {
            Ok(self.cache_contact_payload.get(&contact_id).unwrap().clone())
        } else {
            match self.puppet_impl.contact_raw_payload(contact_id.clone()).await {
                Ok(payload) => {
                    self.cache_contact_payload.put(contact_id.clone(), payload.clone());
                    Ok(payload)
                }
                Err(e) => Err(e),
            }
        }
    }

    pub async fn contact_search_by_string(
        &mut self,
        query_str: String,
        search_id_list: Option<Vec<String>>,
    ) -> Result<Vec<String>, PuppetError> {
        debug!("contact_search_by_string(query_str = {})", query_str);
        let search_by_id = self
            .contact_search(
                ContactQueryFilter {
                    alias: None,
                    alias_regex: None,
                    id: Some(query_str.clone()),
                    name: None,
                    name_regex: None,
                    weixin: None,
                },
                search_id_list.clone(),
            )
            .await;
        let search_by_alias = self
            .contact_search(
                ContactQueryFilter {
                    alias: Some(query_str.clone()),
                    alias_regex: None,
                    id: None,
                    name: None,
                    name_regex: None,
                    weixin: None,
                },
                search_id_list,
            )
            .await;
        let mut filtered_contact_id_list = vec![];
        if let Ok(contact_id_list) = search_by_id {
            for contact_id in contact_id_list {
                filtered_contact_id_list.push(contact_id);
            }
        }
        if let Ok(contact_id_list) = search_by_alias {
            for contact_id in contact_id_list {
                filtered_contact_id_list.push(contact_id);
            }
        }
        Ok(filtered_contact_id_list
            .into_iter()
            .collect::<HashSet<String>>()
            .into_iter()
            .collect::<Vec<String>>())
    }

    pub async fn contact_search(
        &mut self,
        query: ContactQueryFilter,
        contact_id_list: Option<Vec<String>>,
    ) -> Result<Vec<String>, PuppetError> {
        debug!("contact_search(query = {:?})", query);
        let contact_id_list = match contact_id_list {
            Some(contact_id_list) => contact_id_list,
            None => match self.puppet_impl.contact_list().await {
                Ok(contact_id_list) => contact_id_list,
                Err(e) => return Err(e),
            },
        };
        debug!("contact_search(search_id_list.len() = {})", contact_id_list.len());

        let mut filtered_contact_id_list = vec![];
        let filter = self.contact_query_filter_factory(query);
        for contact_id in contact_id_list {
            if let Ok(payload) = self.contact_payload(contact_id.clone()).await {
                if filter(payload) {
                    filtered_contact_id_list.push(contact_id.clone());
                }
            } else {
                error!("Failed to get contact payload for {}", contact_id);
            }
        }

        Ok(filtered_contact_id_list)
    }

    fn contact_query_filter_factory(&mut self, query: ContactQueryFilter) -> impl Fn(ContactPayload) -> bool {
        debug!("contact_query_filter_factory(query = {:?})", query);
        move |payload| -> bool {
            if let Some(id) = query.clone().id {
                if payload.id != id {
                    return false;
                }
            }
            if let Some(name) = query.clone().name {
                if payload.name != name {
                    return false;
                }
            }
            if let Some(alias) = query.clone().alias {
                if payload.alias != alias {
                    return false;
                }
            }
            if let Some(weixin) = query.clone().weixin {
                if payload.weixin != weixin {
                    return false;
                }
            }
            if let Some(name_regex) = query.clone().name_regex {
                if !name_regex.is_match(&payload.name) {
                    return false;
                }
            }
            if let Some(alias_regex) = query.clone().alias_regex {
                if !alias_regex.is_match(&payload.alias) {
                    return false;
                }
            }
            true
        }
    }

    /*
        Message
    */

    pub async fn message_payload(&mut self, message_id: String) -> Result<MessagePayload, PuppetError> {
        debug!("message_payload(message_id = {})", message_id);
        if self.cache_message_payload.contains(&message_id) {
            Ok(self.cache_message_payload.get(&message_id).unwrap().clone())
        } else {
            match self.puppet_impl.message_raw_payload(message_id.clone()).await {
                Ok(payload) => {
                    self.cache_message_payload.put(message_id.clone(), payload.clone());
                    Ok(payload)
                }
                Err(e) => Err(e),
            }
        }
    }

    pub fn message_list(&self) -> Vec<String> {
        debug!("message_list()");
        let mut message_id_list = vec![];
        for (key, _val) in &self.cache_message_payload {
            message_id_list.push(key.clone());
        }
        message_id_list
    }

    pub async fn message_search(&mut self, query: MessageQueryFilter) -> Result<Vec<String>, PuppetError> {
        debug!("message_search(query = {:?})", query);

        let message_id_list = self.message_list();
        debug!("message_search(message_id_list.len() = {})", message_id_list.len());

        let mut filtered_message_id_list = vec![];
        let filter = self.message_query_filter_factory(query);
        for message_id in message_id_list {
            if let Ok(payload) = self.message_payload(message_id.clone()).await {
                if filter(payload) {
                    filtered_message_id_list.push(message_id.clone());
                }
            } else {
                error!("Failed to get message payload for {}", message_id);
            }
        }

        Ok(filtered_message_id_list)
    }

    fn message_query_filter_factory(&mut self, query: MessageQueryFilter) -> impl Fn(MessagePayload) -> bool {
        debug!("message_query_filter_factory(query = {:?})", query);
        move |payload| -> bool {
            if let Some(id) = query.clone().id {
                if payload.id != id {
                    return false;
                }
            }
            if let Some(message_type) = query.clone().message_type {
                if payload.message_type != message_type {
                    return false;
                }
            }
            if let Some(from_id) = query.clone().from_id {
                if payload.from_id != from_id {
                    return false;
                }
            }
            if let Some(to_id) = query.clone().to_id {
                if payload.to_id != to_id {
                    return false;
                }
            }
            if let Some(room_id) = query.clone().room_id {
                if payload.room_id != room_id {
                    return false;
                }
            }
            if let Some(text) = query.clone().text {
                if payload.text != text {
                    return false;
                }
            }
            if let Some(text_regex) = query.clone().text_regex {
                if !text_regex.is_match(&payload.text) {
                    return false;
                }
            }
            true
        }
    }

    pub async fn message_forward(
        &mut self,
        conversation_id: String,
        message_id: String,
    ) -> Result<Option<String>, PuppetError> {
        debug!(
            "message_forward(conversation_id = {}, message_id = {})",
            conversation_id, message_id
        );
        let payload = self.message_payload(message_id.clone()).await;
        match payload {
            Ok(payload) => match payload.message_type {
                MessageType::Attachment | MessageType::Audio | MessageType::Image | MessageType::Video => {
                    match self.puppet_impl.message_file(message_id).await {
                        Ok(file) => self.puppet_impl.message_send_file(conversation_id, file).await,
                        Err(e) => Err(e),
                    }
                }
                MessageType::Text => {
                    self.puppet_impl
                        .message_send_text(conversation_id, payload.text, Vec::new())
                        .await
                }
                MessageType::MiniProgram => match self.puppet_impl.message_mini_program(message_id).await {
                    Ok(mini_program_payload) => {
                        self.puppet_impl
                            .message_send_mini_program(conversation_id, mini_program_payload)
                            .await
                    }
                    Err(e) => Err(e),
                },
                MessageType::Url => match self.puppet_impl.message_url(message_id).await {
                    Ok(url_link_payload) => {
                        self.puppet_impl
                            .message_send_url(conversation_id, url_link_payload)
                            .await
                    }
                    Err(e) => Err(e),
                },
                MessageType::Contact => match self.puppet_impl.message_contact(message_id).await {
                    Ok(contact_id) => self.puppet_impl.message_send_contact(conversation_id, contact_id).await,
                    Err(e) => Err(e),
                },
                MessageType::ChatHistory
                | MessageType::Location
                | MessageType::Emoticon
                | MessageType::GroupNote
                | MessageType::Transfer
                | MessageType::RedEnvelope
                | MessageType::Recalled => Err(PuppetError::Unsupported(format!(
                    "sending {:?} messages",
                    payload.message_type
                ))),
                MessageType::Unknown => Err(PuppetError::UnknownMessageType),
            },
            Err(e) => Err(e),
        }
    }

    /*
        Friendship
    */

    /// Friendship payload getter.
    pub async fn friendship_payload(&mut self, friendship_id: String) -> Result<FriendshipPayload, PuppetError> {
        debug!("friendship_payload(friendship_id = {})", friendship_id);
        if self.cache_friendship_payload.contains(&friendship_id) {
            Ok(self.cache_friendship_payload.get(&friendship_id).unwrap().clone())
        } else {
            match self.puppet_impl.friendship_raw_payload(friendship_id.clone()).await {
                Ok(payload) => {
                    self.cache_friendship_payload
                        .put(friendship_id.clone(), payload.clone());
                    Ok(payload)
                }
                Err(e) => Err(e),
            }
        }
    }

    /// Friendship payload setter.
    pub async fn friendship_payload_set(
        &mut self,
        friendship_id: String,
        new_payload: FriendshipPayload,
    ) -> Result<(), PuppetError> {
        debug!(
            "friendship_payload_set(id = {}, new_payload = {:?})",
            friendship_id, new_payload
        );
        self.cache_friendship_payload.put(friendship_id, new_payload);
        Ok(())
    }

    /*
       Room Invitation
    */

    /// Room invitation payload getter.
    pub async fn room_invitation_payload(
        &mut self,
        room_invitation_id: String,
    ) -> Result<RoomInvitationPayload, PuppetError> {
        debug!("room_invitation_payload(room_invitation_id = {})", room_invitation_id);
        if self.cache_room_invitation_payload.contains(&room_invitation_id) {
            Ok(self
                .cache_room_invitation_payload
                .get(&room_invitation_id)
                .unwrap()
                .clone())
        } else {
            match self
                .puppet_impl
                .room_invitation_raw_payload(room_invitation_id.clone())
                .await
            {
                Ok(payload) => {
                    self.cache_room_invitation_payload
                        .put(room_invitation_id.clone(), payload.clone());
                    Ok(payload)
                }
                Err(e) => Err(e),
            }
        }
    }

    /// Room invitation payload setter.
    pub async fn room_invitation_payload_set(
        &mut self,
        room_invitation_id: String,
        new_payload: RoomInvitationPayload,
    ) -> Result<(), PuppetError> {
        debug!(
            "room_invitation_payload_set(id = {}, new_payload = {:?})",
            room_invitation_id, new_payload
        );
        self.cache_room_invitation_payload.put(room_invitation_id, new_payload);
        Ok(())
    }

    /*
       Room
    */

    pub async fn room_payload(&mut self, room_id: String) -> Result<RoomPayload, PuppetError> {
        debug!("room_payload(room_id = {})", room_id);
        if self.cache_room_payload.contains(&room_id) {
            Ok(self.cache_room_payload.get(&room_id).unwrap().clone())
        } else {
            match self.puppet_impl.room_raw_payload(room_id.clone()).await {
                Ok(payload) => {
                    self.cache_room_payload.put(room_id.clone(), payload.clone());
                    Ok(payload)
                }
                Err(e) => Err(e),
            }
        }
    }

    fn cache_key_room_member(room_id: String, contact_id: String) -> String {
        format!("{}@@@{}", contact_id, room_id)
    }

    pub async fn room_member_payload(
        &mut self,
        room_id: String,
        member_id: String,
    ) -> Result<RoomMemberPayload, PuppetError> {
        debug!("room_member_payload(room_id = {}, member_id = {})", room_id, member_id);
        let cache_key = Puppet::<T>::cache_key_room_member(room_id.clone(), member_id.clone());
        if self.cache_room_member_payload.contains(&cache_key) {
            Ok(self.cache_room_member_payload.get(&cache_key).unwrap().clone())
        } else {
            match self
                .puppet_impl
                .room_member_raw_payload(room_id.clone(), member_id.clone())
                .await
            {
                Ok(payload) => {
                    self.cache_room_member_payload.put(cache_key, payload.clone());
                    Ok(payload)
                }
                Err(e) => Err(e),
            }
        }
    }

    pub async fn room_search(&mut self, query: RoomQueryFilter) -> Result<Vec<String>, PuppetError> {
        debug!("room_search(query = {:?})", query);
        let room_id_list = match self.puppet_impl.room_list().await {
            Ok(room_id_list) => room_id_list,
            _ => Vec::new(),
        };
        debug!("room_search(room_id_list.len() = {})", room_id_list.len());

        let mut filtered_room_id_list = vec![];
        let filter = self.room_query_filter_factory(query);
        for room_id in room_id_list {
            if let Ok(payload) = self.room_payload(room_id.clone()).await {
                if filter(payload) {
                    filtered_room_id_list.push(room_id.clone());
                }
            } else {
                error!("Failed to get room payload for {}", room_id);
            }
        }

        Ok(filtered_room_id_list)
    }

    fn room_query_filter_factory(&mut self, query: RoomQueryFilter) -> impl Fn(RoomPayload) -> bool {
        debug!("room_query_filter_factory(query = {:?})", query);
        move |payload| -> bool {
            if let Some(id) = query.clone().id {
                if payload.id != id {
                    return false;
                }
            }
            if let Some(topic) = query.clone().topic {
                if payload.topic != topic {
                    return false;
                }
            }
            if let Some(topic_regex) = query.clone().topic_regex {
                if !topic_regex.is_match(&payload.topic) {
                    return false;
                }
            }
            true
        }
    }

    /*
       Dirty payload
    */

    async fn dirty_payload_message(&mut self, message_id: String) -> Result<(), PuppetError> {
        debug!("dirty_payload_message(message_id = {})", message_id);
        self.cache_message_payload.pop(&message_id);
        Ok(())
    }

    async fn dirty_payload_contact(&mut self, contact_id: String) -> Result<(), PuppetError> {
        debug!("dirty_payload_contact(contact_id = {})", contact_id);
        self.cache_contact_payload.pop(&contact_id);
        Ok(())
    }

    async fn dirty_payload_room(&mut self, room_id: String) -> Result<(), PuppetError> {
        debug!("dirty_payload_room(room_id = {})", room_id);
        self.cache_contact_payload.pop(&room_id);
        Ok(())
    }

    async fn dirty_payload_room_member(&mut self, room_id: String) -> Result<(), PuppetError> {
        debug!("dirty_payload_room_member(room_id = {})", room_id);

        match self.puppet_impl.room_member_list(room_id.clone()).await {
            Ok(contact_id_list) => {
                for contact_id in contact_id_list {
                    let cache_key = Puppet::<T>::cache_key_room_member(room_id.clone(), contact_id);
                    self.cache_room_member_payload.pop(&cache_key);
                }
                Ok(())
            }
            Err(e) => Err(e),
        }
    }

    async fn dirty_payload_friendship(&mut self, friendship_id: String) -> Result<(), PuppetError> {
        debug!("dirty_payload_friendship(friendship_id = {})", friendship_id);
        self.cache_friendship_payload.pop(&friendship_id);
        Ok(())
    }

    pub async fn dirty_payload(&mut self, payload_type: PayloadType, id: String) -> Result<(), PuppetError> {
        debug!("dirty_payload(payload_type = {:?}, id = {})", payload_type, id);

        match payload_type {
            PayloadType::Message => self.dirty_payload_message(id).await,
            PayloadType::Contact => self.dirty_payload_contact(id).await,
            PayloadType::Room => self.dirty_payload_room(id).await,
            PayloadType::RoomMember => self.dirty_payload_room_member(id).await,
            PayloadType::Friendship => self.dirty_payload_friendship(id).await,
            PayloadType::Unknown => Err(PuppetError::UnknownPayloadType),
        }
    }
}

#[async_trait]
pub trait PuppetImpl {
    async fn contact_self_name_set(&mut self, name: String) -> Result<(), PuppetError>;
    async fn contact_self_qr_code(&mut self) -> Result<String, PuppetError>;
    async fn contact_self_signature_set(&mut self, signature: String) -> Result<(), PuppetError>;

    async fn tag_contact_add(&mut self, tag_id: String, contact_id: String) -> Result<(), PuppetError>;
    async fn tag_contact_remove(&mut self, tag_id: String, contact_id: String) -> Result<(), PuppetError>;
    async fn tag_contact_delete(&mut self, tag_id: String) -> Result<(), PuppetError>;
    async fn tag_contact_list(&mut self, contact_id: String) -> Result<Vec<String>, PuppetError>;
    async fn tag_list(&mut self) -> Result<Vec<String>, PuppetError>;

    async fn contact_alias(&mut self, contact_id: String) -> Result<String, PuppetError>;
    async fn contact_alias_set(&mut self, contact_id: String, alias: String) -> Result<(), PuppetError>;
    async fn contact_avatar(&mut self, contact_id: String) -> Result<FileBox, PuppetError>;
    async fn contact_avatar_set(&mut self, contact_id: String, file: FileBox) -> Result<(), PuppetError>;
    async fn contact_phone_set(&mut self, contact_id: String, phone_list: Vec<String>) -> Result<(), PuppetError>;
    async fn contact_corporation_remark_set(
        &mut self,
        contact_id: String,
        corporation_remark: Option<String>,
    ) -> Result<(), PuppetError>;
    async fn contact_description_set(
        &mut self,
        contact_id: String,
        description: Option<String>,
    ) -> Result<(), PuppetError>;
    async fn contact_list(&mut self) -> Result<Vec<String>, PuppetError>;
    async fn contact_raw_payload(&mut self, contact_id: String) -> Result<ContactPayload, PuppetError>;

    async fn message_contact(&mut self, message_id: String) -> Result<String, PuppetError>;
    async fn message_file(&mut self, message_id: String) -> Result<FileBox, PuppetError>;
    async fn message_image(&mut self, message_id: String, image_type: ImageType) -> Result<FileBox, PuppetError>;
    async fn message_mini_program(&mut self, message_id: String) -> Result<MiniProgramPayload, PuppetError>;
    async fn message_url(&mut self, message_id: String) -> Result<UrlLinkPayload, PuppetError>;
    async fn message_send_contact(
        &mut self,
        conversation_id: String,
        contact_id: String,
    ) -> Result<Option<String>, PuppetError>;
    async fn message_send_file(
        &mut self,
        conversation_id: String,
        file: FileBox,
    ) -> Result<Option<String>, PuppetError>;
    async fn message_send_mini_program(
        &mut self,
        conversation_id: String,
        mini_program_payload: MiniProgramPayload,
    ) -> Result<Option<String>, PuppetError>;
    async fn message_send_text(
        &mut self,
        conversation_id: String,
        text: String,
        mention_id_list: Vec<String>,
    ) -> Result<Option<String>, PuppetError>;
    async fn message_send_url(
        &mut self,
        conversation_id: String,
        url_link_payload: UrlLinkPayload,
    ) -> Result<Option<String>, PuppetError>;
    async fn message_raw_payload(&mut self, message_id: String) -> Result<MessagePayload, PuppetError>;

    async fn friendship_accept(&mut self, friendship_id: String) -> Result<(), PuppetError>;
    async fn friendship_add(&mut self, contact_id: String, hello: Option<String>) -> Result<(), PuppetError>;
    async fn friendship_search_phone(&mut self, phone: String) -> Result<Option<String>, PuppetError>;
    async fn friendship_search_weixin(&mut self, weixin: String) -> Result<Option<String>, PuppetError>;
    async fn friendship_raw_payload(&mut self, friendship_id: String) -> Result<FriendshipPayload, PuppetError>;

    async fn room_invitation_accept(&mut self, room_invitation_id: String) -> Result<(), PuppetError>;
    async fn room_invitation_raw_payload(
        &mut self,
        room_invitation_id: String,
    ) -> Result<RoomInvitationPayload, PuppetError>;

    async fn room_add(&mut self, room_id: String, contact_id: String) -> Result<(), PuppetError>;
    async fn room_avatar(&mut self, room_id: String) -> Result<FileBox, PuppetError>;
    async fn room_create(&mut self, contact_id_list: Vec<String>, topic: Option<String>)
                         -> Result<String, PuppetError>;
    async fn room_del(&mut self, room_id: String, contact_id: String) -> Result<(), PuppetError>;
    async fn room_qr_code(&mut self, room_id: String) -> Result<String, PuppetError>;
    async fn room_quit(&mut self, room_id: String) -> Result<(), PuppetError>;
    async fn room_topic(&mut self, room_id: String) -> Result<String, PuppetError>;
    async fn room_topic_set(&mut self, room_id: String, topic: String) -> Result<(), PuppetError>;
    async fn room_list(&mut self) -> Result<Vec<String>, PuppetError>;
    async fn room_raw_payload(&mut self, room_id: String) -> Result<RoomPayload, PuppetError>;

    async fn room_announce(&mut self, room_id: String) -> Result<String, PuppetError>;
    async fn room_announce_set(&mut self, room_id: String, text: String) -> Result<(), PuppetError>;
    async fn room_member_list(&mut self, room_id: String) -> Result<Vec<String>, PuppetError>;
    async fn room_member_raw_payload(
        &mut self,
        room_id: String,
        contact_id: String,
    ) -> Result<RoomMemberPayload, PuppetError>;
}
