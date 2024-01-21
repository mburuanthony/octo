#![allow(unused)]
#[macro_use]
extern crate serde;
use candid::{Decode, Encode};
use ic_cdk::api::time;
use ic_stable_structures::memory_manager::{MemoryId, MemoryManager, VirtualMemory};
use ic_stable_structures::{BoundedStorable, Cell, DefaultMemoryImpl, StableBTreeMap, Storable};
use std::{borrow::Cow, cell::RefCell};

type Memory = VirtualMemory<DefaultMemoryImpl>;
type IdCell = Cell<u64, Memory>;

mod messages;

thread_local! {
    static MEMORY_MANAGER: RefCell<MemoryManager<DefaultMemoryImpl>> = RefCell::new(
            MemoryManager::init(DefaultMemoryImpl::default())
        );

    static ID_COUNTER: RefCell<IdCell> = RefCell::new(
        IdCell::init(MEMORY_MANAGER.with(|m| m.borrow().get(MemoryId::new(0))), 0)
            .expect("Cannot create a counter")
    );

    static MESSAGESSTORAGE: RefCell<StableBTreeMap<u64, messages::Message, Memory>> =
        RefCell::new(StableBTreeMap::init(
            MEMORY_MANAGER.with(|m| m.borrow().get(MemoryId::new(1)))
    ));
}

type MessagesData = (u64, messages::Message);

#[ic_cdk::query]
fn get_messages() -> Result<Vec<MessagesData>, Error> {
    match _get_messages() {
        Some(messages) => Ok(messages),
        None => Err(Error::NotFound {
            msg: format!("Messages temporarily unavailable"),
        }),
    }
}

fn _get_messages() -> Option<Vec<MessagesData>> {
    Some(MESSAGESSTORAGE.with(|s| s.borrow().iter().collect()))
}

#[ic_cdk::query]
fn get_message(id: u64) -> Result<messages::Message, Error> {
    match _get_message(&id) {
        Some(message) => Ok(message),
        None => Err(Error::NotFound {
            msg: format!("Company with id={} not found", id),
        }),
    }
}

fn _get_message(id: &u64) -> Option<messages::Message> {
    MESSAGESSTORAGE.with(|s| s.borrow().get(id))
}

#[ic_cdk::update]
fn add_message(message: messages::MessagePayload) -> Option<messages::Message> {
    let id = ID_COUNTER
        .with(|counter| {
            let current_value = *counter.borrow().get();
            counter.borrow_mut().set(current_value + 1)
        })
        .expect("cannot increment id counter");
    let message = messages::Message {
        id,
        title: message.title,
        body: message.body,
        attachment_url: message.attachment_url,
        created_at: time(),
        updated_at: None,
    };
    do_insert(message.clone());
    Some(message)
}

#[ic_cdk::update]
fn update_message(id: u64, payload: messages::MessagePayload) -> Result<messages::Message, Error> {
    match MESSAGESSTORAGE.with(|service| service.borrow().get(&id)) {
        Some(mut message) => {
            message.attachment_url = payload.attachment_url;
            message.body = payload.body;
            message.title = payload.title;
            message.updated_at = Some(time());
            do_insert(message.clone());
            Ok(message)
        }
        None => Err(Error::NotFound {
            msg: format!("unable to update a message with id {}", id),
        }),
    }
}

fn do_insert(message: messages::Message) {
    MESSAGESSTORAGE.with(|service| service.borrow_mut().insert(message.id, message.clone()));
}

#[ic_cdk::update]
fn delete_message(id: u64) -> Result<messages::Message, Error> {
    match MESSAGESSTORAGE.with(|service| service.borrow_mut().remove(&id)) {
        Some(message) => Ok(message),
        None => Err(Error::NotFound {
            msg: format!(
                "unable to delete a message with id {} message not found.",
                id
            ),
        }),
    }
}

#[derive(candid::CandidType, Deserialize, Serialize)]
enum Error {
    NotFound { msg: String },
}

ic_cdk::export_candid!();
