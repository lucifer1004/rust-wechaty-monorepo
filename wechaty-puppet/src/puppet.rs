use actix::{Actor, Context, Handler, Recipient};
use log::{error, trace};
use lru::LruCache;

use crate::events::PuppetEvent;
use crate::schemas::contact::ContactPayload;
use crate::schemas::friendship::FriendshipPayload;
use crate::schemas::message::MessagePayload;
use crate::schemas::room::{RoomPayload, RoomMemberPayload};
use crate::schemas::room_invitation::RoomInvitationPayload;
use crate::schemas::payload::PayloadType;

const DEFAULT_CONTACT_CACHE_CAP: usize = 3000;
const DEFAULT_FRIENDSHIP_CACHE_CAP: usize = 300;
const DEFAULT_MESSAGE_CACHE_CAP: usize = 500;
const DEFAULT_ROOM_CACHE_CAP: usize = 500;
const DEFAULT_ROOM_MEMBER_CACHE_CAP: usize = 30000;
const DEFAULT_ROOM_INVITATION_CACHE_CAP: usize = 100;

pub struct Puppet<T>
    where
        T: IntoPuppet,
{
    puppet_impl: T,
    cache_contact_payload: LruCache<String, ContactPayload>,
    cache_friendship_payload: LruCache<String, FriendshipPayload>,
    cache_message_payload: LruCache<String, MessagePayload>,
    cache_room_payload: LruCache<String, RoomPayload>,
    cache_room_member_payload: LruCache<String, RoomMemberPayload>,
    cache_room_invitation_payload: LruCache<String, RoomInvitationPayload>,
    id: Option<String>,
    subscribers: Vec<Recipient<PuppetEvent>>,
}

impl<T> Puppet<T>
    where
        T: IntoPuppet,
{
    pub fn new(puppet_impl: T) -> Self {
        Self {
            puppet_impl,
            cache_contact_payload: LruCache::new(DEFAULT_CONTACT_CACHE_CAP),
            cache_friendship_payload: LruCache::new(DEFAULT_FRIENDSHIP_CACHE_CAP),
            cache_message_payload: LruCache::new(DEFAULT_MESSAGE_CACHE_CAP),
            cache_room_payload: LruCache::new(DEFAULT_ROOM_CACHE_CAP),
            cache_room_member_payload: LruCache::new(DEFAULT_ROOM_MEMBER_CACHE_CAP),
            cache_room_invitation_payload: LruCache::new(DEFAULT_ROOM_INVITATION_CACHE_CAP),
            id: None,
            subscribers: Vec::new(),
        }
    }

    pub fn self_id(self) -> Option<String> {
        self.id.clone()
    }

    pub fn log_on_off(self) -> bool {
        match self.id {
            Some(_) => true,
            None => false,
        }
    }

    pub async fn dirty_payload_message(&mut self, message_id: String) {
        self.cache_message_payload.pop(&message_id);
    }

    pub async fn dirty_payload_contact(&mut self, contact_id: String) {
        self.cache_contact_payload.pop(&contact_id);
    }

    pub async fn dirty_payload_room(&mut self, room_id: String) {
        self.cache_contact_payload.pop(&room_id);
    }

    // TODO: Need to implement chained removal logic.
    pub async fn dirty_payload_room_member(&mut self, room_member_id: String) {
        self.cache_room_member_payload.pop(&room_member_id);
    }

    pub async fn dirty_payload_friendship(&mut self, friendship_id: String) {
        self.cache_friendship_payload.pop(&friendship_id);
    }

    pub async fn dirty_payload(&mut self, payload_type: PayloadType, id: String) {
        trace!("Dirty {:?}, Id = {}", payload_type, id);

        match payload_type {
            PayloadType::Message => self.dirty_payload_message(id).await,
            PayloadType::Contact => self.dirty_payload_contact(id).await,
            PayloadType::Room => self.dirty_payload_room(id).await,
            PayloadType::RoomMember => self.dirty_payload_room_member(id).await,
            PayloadType::Friendship => self.dirty_payload_friendship(id).await,
            PayloadType::Unknown => error!("Unknown payload type"),
        }
    }
}

impl<T> Actor for Puppet<T> {
    type Context = Context<Self>;
}

impl<T> Handler<PuppetEvent> for Puppet<T> {
    type Result = ();

    fn handle(&mut self, msg: PuppetEvent, ctx: &mut Self::Context) -> Self::Result {
        unimplemented!()
    }
}

pub trait IntoPuppet {}
