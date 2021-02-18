use wechaty_puppet::FriendshipPayload;

use crate::Entity;

pub type Friendship<T> = Entity<T, FriendshipPayload>;
