use std::collections::HashMap;

use crate::{Contact, Message};

pub struct WechatyPool {
    contacts: HashMap<String, Contact>,
    messages: HashMap<String, Message>,
}

impl WechatyPool {
    pub fn new() -> Self {
        Self {
            contacts: Default::default(),
            messages: Default::default()
        }
    }
}
