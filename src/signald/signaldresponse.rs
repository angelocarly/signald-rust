use serde::{Serialize, Deserializer};
use serde::Deserialize;
use serde_json::Value;
use std::collections::HashMap;
use crate::signald::signaldresponse::ResponseType::{ContactList, BusUpdate, Version, Unknown};

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
    Unknown(Value),
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
            "linking_uri" => {
                let data = serde_json::from_value(val.clone()).unwrap();
                return ResponseType::LinkingUri(data);
            }
            "linking_error" => {
                let data = serde_json::from_value(val.clone()).unwrap();
                return ResponseType::LinkingError(data);
            }
            "subscribed" => return ResponseType::Subscribed,
            "unsubscribed" => return ResponseType::Unsubscribed,
            _ => return Unknown(val.clone())
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_sync_message_text() {
        let message = serde_json::json!({
            "type": "message",
            "id": "test",
            "data": {
                "username": "+32000000000",
                "source": "+32000000000",
                "sourceDevice": 1,
                "type": 1,
                "timestamp": 1583863426832u64,
                "timestampISO": "2020-03-10T18:03:46.832Z",
                "serverTimestamp": 1583863428672u64,
                "hasLegacyMessage": false,
                "hasContent": true,
                "isReceipt": false,
                "isUnidentifiedSender": false,
                "syncMessage": {
                    "sent": {
                        "destination": "+32111111111",
                        "timestamp": 1583863426832u64,
                        "expirationStartTimestamp": 0,
                        "message": {
                            "timestamp": 1583863426832u64,
                                "message": "messagedata123",
                                    "expiresInSeconds": 0,
                                    "attachments": []
                                },
                            "unidentifiedStatus": {
                            "+3211111111": true
                        },
                        "isRecipientUpdate": false
                    },
                    "contactsComplete": false,
                    "stickerPackOperations": []
                }
            }
        });
        // Try to parse the message
        let result = SignaldResponse::from_value(message);
        match result.data {
            ResponseType::Message(x) => {
                assert_eq!(x.username.unwrap(), "+32000000000");

                let sync_message = x.sync_message.unwrap();
                let sent = sync_message.sent.unwrap();
                assert_eq!(sent.message.message, "messagedata123");
            }
            _ => panic!("Received wrong response type")
        }
    }

    #[test]
    fn test_parse_sync_message_read() {
        let message = serde_json::json!({
            "type": "message",
            "data": {
                "username": "+32000000000",
                "source": "+32000000000",
                "sourceDevice": 1,
                "type": 1,
                "timestamp": 1583863416850u64,
                "timestampISO": "2020-03-10T18:03:36.850Z",
                "serverTimestamp": 1583863418138u64,
                "hasLegacyMessage": false,
                "hasContent": true,
                "isReceipt": false,
                "isUnidentifiedSender": false,
                "syncMessage": {
                    "contactsComplete": false,
                    "readMessages": [{
                        "sender": "+32111111111",
                        "timestamp": 1583863416783u64
                    }],
                        "stickerPackOperations": []
                }
            }
        });
        // Try to parse the message
        let result = SignaldResponse::from_value(message);
        match result.data {
            ResponseType::Message(x) => {
                assert_eq!(x.username.unwrap(), "+32000000000");

                let sync_message = x.sync_message.unwrap();
                let read_message = sync_message.read_messages.unwrap();
                assert_eq!(read_message.get(0).unwrap().sender, "+32111111111");
            }
            _ => panic!("Received wrong response type")
        }
    }

    #[test]
    fn test_parse_data_message_text() {
        let message = serde_json::json!({
            "type": "message",
            "id": "test",
            "data": {
                "username": "+32000000000",
                "source": "+32111111111",
                "sourceDevice": 0,
                "type": 6,
                "timestamp": 1583863470594u64,
                "timestampISO": "2020-03-10T18:04:30.594Z",
                "serverTimestamp": 1583863470817u64,
                "hasLegacyMessage": false,
                "hasContent": true,
                "isReceipt": false,
                "isUnidentifiedSender": true,
                "dataMessage": {
                    "timestamp": 1583863470594u64,
                    "message": "Thanks",
                    "expiresInSeconds": 0,
                    "attachments": []
                }
            }
        });
        // Try to parse the message
        let result = SignaldResponse::from_value(message);
        match result.data {
            ResponseType::Message(x) => {
                let data_message = x.data_message.unwrap();
                assert_eq!(data_message.message, "Thanks");
                assert_eq!(data_message.timestamp, 1583863470594);
                assert_eq!(data_message.expires_in_seconds, 0);
            }
            _ => panic!("Received wrong response type")
        }
    }

    #[test]
    fn test_parse_typing_message() {
        let message = serde_json::json!({
            "type": "message",
            "data": {
                "username": "+32000000000",
                "source": "+32111111111",
                "sourceDevice": 0,
                "type": 6,
                "timestamp": 1583863467014u64,
                "timestampISO": "2020-03-10T18:04:27.014Z",
                "serverTimestamp": 1583863467091u64,
                "hasLegacyMessage": false,
                "hasContent": true,
                "isReceipt": false,
                "isUnidentifiedSender": true,
                "typing": {
                    "action": "STARTED",
                    "timestamp": 1583863467014u64
                }
            }
        });
        // Try to parse the message
        let result = SignaldResponse::from_value(message);
        match result.data {
            ResponseType::Message(x) => {
                let typing = x.typing.unwrap();
                assert_eq!(typing.action, "STARTED");
                assert_eq!(typing.timestamp, 1583863467014);
            }
            _ => panic!("Received wrong response type")
        }
    }

    #[test]
    fn test_parse_receipt_message() {
        let message = serde_json::json!({
            "type": "message",
            "data": {
                "username": "+32000000000",
                "source": "+32111111111",
                "sourceDevice": 0,
                "type": 6,
                "timestamp": 1583863428937u64,
                "timestampISO": "2020-03-10T18:03:48.937Z",
                "serverTimestamp": 1583863429257u64,
                "hasLegacyMessage": false,
                "hasContent": true,
                "isReceipt": false,
                "isUnidentifiedSender": true,
                "receipt": {
                    "type": "DELIVERY",
                    "timestamps": [
                        1583863426832u64
                    ],
                "when": 1583863428937u64
                }
            }
        });
        // Try to parse the message
        let result = SignaldResponse::from_value(message);
        match result.data {
            ResponseType::Message(x) => {
                let receipt = x.receipt.unwrap();
                assert_eq!(receipt.typ, "DELIVERY");
                assert_eq!(receipt.timestamps.get(0).unwrap().clone(), 1583863426832u64);
            }
            _ => panic!("Received wrong response type")
        }
    }

    #[test]
    fn test_parse_version_message() {
        let message = serde_json::json!({
            "type": "version",
            "data": {
                "name": "signald",
                "version":"0.9.0+git2020-03-08r1a9be52a.5",
                "branch":"master",
                "commit":"1a9be52a721b873eebbec31072908c152bc762aa"
            }
        });
        // Try to parse the message
        let result = SignaldResponse::from_value(message);
        match result.data {
            ResponseType::Version(x) => {
                assert_eq!(x.name, "signald");
                assert_eq!(x.version, "0.9.0+git2020-03-08r1a9be52a.5");
                assert_eq!(x.branch, "master");
                assert_eq!(x.commit, "1a9be52a721b873eebbec31072908c152bc762aa");
            }
            _ => panic!("Received wrong response type")
        }
    }

    #[test]
    fn test_parse_contact_list_message() {
        let message = serde_json::json!({
        "type":"contact_list",
        "data":[{
                "name":"AAAAA",
                "number":"+32111111111",
                "color":"blue_grey",
                "profileKey":"11111="
            },
            {
                "name":"BBBBB",
                "number":"+32222222222",
                "color":"purple",
                "profileKey":"22222="
            },
            {
                "name":"",
                "number":"+32000000000",
                "color":"grey",
                "profileKey":"00000="
            },
            {
                "name":"CCCCC",
                "number":"+32333333333",
                "color":"green"
            },
            {
                "name":"DDDDD",
                "number":"+32444444444",
                "color":"teal"
                ,"profileKey":"44444="
            }]
        });
        // Try to parse the message
        let result = SignaldResponse::from_value(message);
        match result.data {
            ResponseType::ContactList(x) => {
                let first_entry = x.get(0).unwrap();
                assert_eq!(first_entry.name, "AAAAA");
                assert_eq!(first_entry.color, "blue_grey");
                assert_eq!(first_entry.profile_key.clone().unwrap(), "11111=");
                assert_eq!(first_entry.number, "+32111111111");
            }
            _ => panic!("Received wrong response type")
        }
    }

    #[test]
    fn test_parse_linking_uri_message() {
        let message = serde_json::json!({
            "type": "linking_uri",
            "data": {
                "uri": "tsdevice:/?uuid=Sx9vhPhZq5KHG4nZ4w4CFQ&pub_key=BYDtS3MR5qxQnHpRZTCLXp05LvDnqulYdYfpjUqVtpxc"
            }
        });
        // Try to parse the message
        let result = SignaldResponse::from_value(message);
        match result.data {
            ResponseType::LinkingUri(x) => {
                assert_eq!(x.uri, "tsdevice:/?uuid=Sx9vhPhZq5KHG4nZ4w4CFQ&pub_key=BYDtS3MR5qxQnHpRZTCLXp05LvDnqulYdYfpjUqVtpxc");
            }
            _ => panic!("Received wrong response type")
        }
    }

    #[test]
    fn test_parse_linking_error_message() {
        let message = serde_json::json!({
            "type": "linking_error",
            "data": {
                "msg_number": 1,
                "message": "Timed out while waiting for device to link",
                "error": true,
                "request": {
                    "type": "link",
                    "expiresInSeconds": 0,
                    "when": 0
                }
            }
        });
        // Try to parse the message
        let result = SignaldResponse::from_value(message);
        match result.data {
            ResponseType::LinkingError(x) => {
                assert_eq!(x.msg_number, 1);
                assert_eq!(x.message, "Timed out while waiting for device to link");
                assert_eq!(x.error, true);
                assert_eq!(x.request.typ, "link");
                assert_eq!(x.request.expires_in_seconds, 0);
                assert_eq!(x.request.when, 0);
            }
            _ => panic!("Received wrong response type")
        }
    }
}
