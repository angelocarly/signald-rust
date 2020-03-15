use serde::{Serialize, Deserializer};
use serde::Deserialize;
use serde_json::Value;
use std::collections::HashMap;
use crate::signald::signaldresponse::ResponseType::{ContactList, BusUpdate, Version};

#[derive(Clone)]
pub enum ResponseType {
    BusUpdate,
    Message(MessageData),
    Version(VersionData),
    ContactList(Vec<Account>),
    Link,
    Unsubscribe
}
impl ResponseType {
    pub fn new(typ: &str, val: &Value) -> ResponseType {
        match typ {
            "contact_list" => {
                let data = serde_json::from_value(val.clone()).unwrap();
                return ResponseType::ContactList(data);
            }
            "version" => {
                let data = serde_json::from_value(val.clone()).unwrap();
                return Version(data);
            }
            "message" => {
                let data = serde_json::from_value(val.clone()).unwrap();
                return ResponseType::Message(data);
            }
            _ => panic!("No type found for {}", typ)
        }

    }
}

/// A Signald response
#[derive(Clone)]
pub struct SignaldResponse {
    pub _id: Option<String>,
    pub _data: ResponseType,
}
impl SignaldResponse {
    pub fn from_value(val: Value) -> SignaldResponse {
        let id = val["id"].as_str().map(|x| x.to_string());

        let data: ResponseType = ResponseType::new(val["type"].as_str().unwrap(), &val["data"]);

        SignaldResponse {
            _id: id,
            _data: data
        }

    }
}

pub trait ResponseData {}

#[derive(Serialize, Deserialize, Default, Clone)]
pub struct VersionData {
    #[serde(rename = "name")]
    pub _name: String,
    #[serde(rename = "version")]
    pub _version: String,
    #[serde(rename = "branch")]
    pub _branch: String,
    #[serde(rename = "commit")]
    pub _commit: String,
}

#[derive(Serialize, Deserialize, Default, Clone)]
pub struct MessageData {
    #[serde(rename = "username")]
    pub _username: Option<String>,
    #[serde(rename = "uuid")]
    pub _uuid: Option<String>,
    #[serde(rename = "source")]
    pub _source: Option<String>,
    #[serde(rename = "sourceDevice")]
    pub _source_device: Option<i32>,
    #[serde(rename = "type")]
    pub _type: i32,
    #[serde(rename = "timestamp")]
    pub _timestamp: i64,
    #[serde(rename = "timestampISO")]
    pub _timestamp_iso: String,
    #[serde(rename = "serverTimestamp")]
    pub _server_timestamp: i64,
    #[serde(rename = "hasContent")]
    pub _has_content: bool,
    #[serde(rename = "isReceipt")]
    pub _is_receipt: bool,
    #[serde(rename = "isUnidentifiedSender")]
    pub _is_unidentified_sender: bool,
    #[serde(rename = "syncMessage")]
    pub _sync_message: Value,
    #[serde(rename = "dataMessage")]
    pub _data_message: Option<Message>,
}

// #[derive(Serialize, Deserialize, Default, Clone)]
// pub struct SyncMessage {
//     #[serde(rename = "sent")]
//     pub _sent: Option<SentMessage>,
//     #[serde(rename = "contacts")]
//     pub _contacts: Option<Contacts>,
//     #[serde(rename = "contactsComplete")]
//     pub _contacts_complete: bool,
//     #[serde(rename = "readMessages")]
//     pub _read_messages: Vec<ReadMessage>,
//     #[serde(rename = "stickerPackOperations")]
//     pub _sticker_pack_operations: Option<Vec<String>>,
//     #[serde(rename = "unidentifiedStatus")]
//     pub _unidentified_status: Option<Vec<String>>,
//     #[serde(rename = "isRecipientUpdate")]
//     pub _is_recipient_update: bool,
// }

#[derive(Serialize, Deserialize, Default, Clone)]
pub struct Message {
    #[serde(rename = "timestamp")]
    pub _timestamp: i64,
    #[serde(rename = "message")]
    pub _message: String,
    #[serde(rename = "expiresInSeconds")]
    pub _expires_in_seconds: i32,
}
#[derive(Serialize, Deserialize, Default, Clone)]
pub struct SentMessage {
    #[serde(rename = "destination")]
    pub _destination: String,
    #[serde(rename = "timestamp")]
    pub _timestamp: i64,
    #[serde(rename = "expirationStartTimestamp")]
    pub _expiration_start_timestamp: i64,
    #[serde(rename = "unidentifiedStatus")]
    pub _unidentified_status: HashMap<String, i64>,
    #[serde(rename = "expirationStartTimestamp")]
    pub _is_recipient_update: bool,
}
#[derive(Serialize, Deserialize, Default, Clone)]
pub struct ReadMessage {
    #[serde(rename = "sender")]
    pub _sender: String,
    #[serde(rename = "timestamp")]
    pub _timestamp: i64,
}

#[derive(Serialize, Deserialize, Default, Clone)]
pub struct ContactListData {
    #[serde(flatten)]
    pub contacts: Vec<Account>
}
#[derive(Serialize, Deserialize, Default, Clone)]
pub struct Account {
    pub name: String,
    pub number: String,
    pub color: String,
    #[serde(rename = "profileKey")]
    pub profile_key: Option<String>,
}

#[derive(Serialize, Deserialize, Default, Clone)]
pub struct Receipt {
    #[serde(rename = "type")]
    pub _type: String,
    #[serde(rename = "timestamps")]
    pub _timestamps: Vec<String>,
    #[serde(rename = "when")]
    pub _when: i32,
}
