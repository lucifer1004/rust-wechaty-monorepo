use wechaty_puppet::{PuppetImpl, RoomPayload};

use crate::{Entity, WechatyContext};

pub type Room<T> = Entity<T, RoomPayload>;
