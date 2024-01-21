#![allow(unused)]
#[macro_use]
use std::borrow::Cow;
use candid::{Decode, Encode};
use ic_cdk::api::time;
use ic_stable_structures::{BoundedStorable, Storable};

#[derive(candid::CandidType, Clone, Serialize, Deserialize, Default)]
pub struct Message {
    pub id: u64,
    pub title: String,
    pub body: String,
    pub attachment_url: String,
    pub created_at: u64,
    pub updated_at: Option<u64>,
}

#[derive(candid::CandidType, Serialize, Deserialize, Default)]
pub struct MessagePayload {
    pub title: String,
    pub body: String,
    pub attachment_url: String,
}

impl Storable for Message {
    fn to_bytes(&self) -> std::borrow::Cow<[u8]> {
        Cow::Owned(Encode!(self).unwrap())
    }

    fn from_bytes(bytes: std::borrow::Cow<[u8]>) -> Self {
        Decode!(bytes.as_ref(), Self).unwrap()
    }
}

impl BoundedStorable for Message {
    const MAX_SIZE: u32 = 1024;
    const IS_FIXED_SIZE: bool = false;
}
