use serde::Serialize;
use serde::Deserialize;
use serde_json::Value;
use std::collections::HashMap;
use crate::signaldresponse::ResponseType::{Version, Unknown};

/// Indicates which kind of Signald message is received
#[derive(Clone)]
pub enum ResponseType {
    BusUpdate,
    Message(MessageData),
    Version(VersionData),
    ContactList(Vec<Account>),
    LinkingUri(LinkingUri),
    LinkingError(LinkingError),
    Subscribed,
    Unsubscribed,
    /// An unknown response
    /// String parameter is the type
    /// Value parameter is a Value of the data
    Unknown(String, Value),
}
impl ResponseType {
    /// Create a ResponseType from response data
    pub fn new(typ: &str, val: &Value) -> ResponseType {
        return match typ {
            "contact_list" => {
                let data = serde_json::from_value(val.clone()).unwrap();
                ResponseType::ContactList(data)
            }
            "version" => {
                let data = serde_json::from_value(val.clone()).unwrap();
                Version(data)
            }
            "message" => {
                let data = serde_json::from_value(val.clone()).unwrap();
                ResponseType::Message(data)
            }
            "linking_uri" => {
                let data = serde_json::from_value(val.clone()).unwrap();
                ResponseType::LinkingUri(data)
            }
            "linking_error" => {
                let data = serde_json::from_value(val.clone()).unwrap();
                ResponseType::LinkingError(data)
            }
            "subscribed" => ResponseType::Subscribed,
            "unsubscribed" => ResponseType::Unsubscribed,
            _ => Unknown(typ.to_string(), val.clone())
        }

    }
}

/// A Signald response
#[derive(Clone)]
pub struct SignaldResponse {
    pub id: Option<String>,
    pub data: ResponseType,
}
impl SignaldResponse {
    pub fn from_value(val: Value) -> SignaldResponse {
        let id = val["id"].as_str().map(|x| x.to_string());

        let data: ResponseType = ResponseType::new(val["type"].as_str().unwrap(), &val["data"]);

        SignaldResponse {
            id,
            data
        }

    }
}

pub trait ResponseData {}

// ========================================= VERSION ===============================================
#[derive(Serialize, Deserialize, Default, Clone)]
pub struct VersionData {
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "version")]
    pub version: String,
    #[serde(rename = "branch")]
    pub branch: String,
    #[serde(rename = "commit")]
    pub commit: String,
}

// ========================================= MESSAGE ===============================================
#[derive(Serialize, Deserialize, Default, Clone)]
pub struct MessageData {
    #[serde(rename = "username")]
    pub username: Option<String>,
    #[serde(rename = "uuid")]
    pub uuid: Option<String>,
    #[serde(rename = "source")]
    pub source: Option<String>,
    #[serde(rename = "sourceDevice")]
    pub source_device: Option<i32>,
    #[serde(rename = "type")]
    pub typ: i32,
    #[serde(rename = "timestamp")]
    pub timestamp: i64,
    #[serde(rename = "timestampISO")]
    pub timestamp_iso: String,
    #[serde(rename = "serverTimestamp")]
    pub server_timestamp: i64,
    #[serde(rename = "hasLegacyMessage")]
    pub has_legacy_message: bool,
    #[serde(rename = "hasContent")]
    pub has_content: bool,
    #[serde(rename = "isSignalMessage")]
    pub is_signal_message: Option<bool>,
    #[serde(rename = "isPrekeySignalMessage")]
    pub is_prekey_signal_message: Option<bool>,
    #[serde(rename = "isReceipt")]
    pub is_receipt: bool,
    #[serde(rename = "isUnidentifiedSender")]
    pub is_unidentified_sender: bool,
    #[serde(rename = "syncMessage")]
    pub sync_message: Option<SyncMessage>,
    #[serde(rename = "dataMessage")]
    pub data_message: Option<Message>,
    #[serde(rename = "typing")]
    pub typing: Option<Typing>,
    #[serde(rename = "receipt")]
    pub receipt: Option<Receipt>,
}

#[derive(Serialize, Deserialize, Default, Clone)]
pub struct SyncMessage {
    #[serde(rename = "sent")]
    pub sent: Option<SentMessage>,
    // #[serde(rename = "contacts")]
    // pub contacts: Option<Contacts>,
    #[serde(rename = "contactsComplete")]
    pub contacts_complete: bool,
    #[serde(rename = "readMessages")]
    pub read_messages: Option<Vec<ReadMessage>>,
    #[serde(rename = "stickerPackOperations")]
    pub sticker_pack_operations: Option<Vec<String>>,
    #[serde(rename = "unidentifiedStatus")]
    pub unidentified_status: Option<HashMap<String, bool>>,
    // #[serde(rename = "isRecipientUpdate")]
    // pub is_recipient_update: bool,
}

#[derive(Serialize, Deserialize, Default, Clone)]
pub struct Message {
    #[serde(rename = "timestamp")]
    pub timestamp: i64,
    #[serde(rename = "message")]
    pub message: String,
    #[serde(rename = "expiresInSeconds")]
    pub expires_in_seconds: i32,
}
#[derive(Serialize, Deserialize, Default, Clone)]
pub struct SentMessage {
    #[serde(rename = "destination")]
    pub destination: String,
    #[serde(rename = "timestamp")]
    pub timestamp: i64,
    #[serde(rename = "expirationStartTimestamp")]
    pub expiration_start_timestamp: i64,
    #[serde(rename = "message")]
    pub message: Message,
    #[serde(rename = "unidentifiedStatus")]
    pub unidentified_status: HashMap<String, bool>,
    #[serde(rename = "isRecipientUpdate")]
    pub is_recipient_update: bool,
}
#[derive(Serialize, Deserialize, Default, Clone)]
pub struct ReadMessage {
    #[serde(rename = "sender")]
    pub sender: String,
    #[serde(rename = "timestamp")]
    pub timestamp: i64,
}
#[derive(Serialize, Deserialize, Default, Clone)]
pub struct Typing {
    #[serde(rename = "action")]
    pub action: String,
    #[serde(rename = "timestamp")]
    pub timestamp: i64,
}
#[derive(Serialize, Deserialize, Default, Clone)]
pub struct Receipt {
    #[serde(rename = "type")]
    pub typ: String,
    #[serde(rename = "timestamps")]
    pub timestamps: Vec<u64>,
    #[serde(rename = "when")]
    pub when: u64,
}

// ==================================== CONTACT LIST ===============================================
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

// ========================================= LINK ==================================================
#[derive(Serialize, Deserialize, Default, Clone)]
pub struct LinkingUri {
    pub uri: String,
}
#[derive(Serialize, Deserialize, Default, Clone)]
pub struct LinkingError {
    pub msg_number: u32,
    pub message: String,
    pub error: bool,
    pub request: Request,
}
#[derive(Serialize, Deserialize, Default, Clone)]
pub struct Request {
    #[serde(rename = "type")]
    pub typ: String,
    #[serde(rename = "expiresInSeconds")]
    pub expires_in_seconds: u32,
    pub when: u64,
}

