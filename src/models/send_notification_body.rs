/*
 * notifier
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: 0.2.2
 * Contact: 
 * Generated by: https://openapi-generator.tech
 */




#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct SendNotificationBody {
    #[serde(rename = "subject")]
    pub subject: String,
    #[serde(rename = "user_id")]
    pub user_id: String,
    #[serde(rename = "message")]
    pub message: String,
}

impl SendNotificationBody {
    pub fn new(subject: String, user_id: String, message: String) -> SendNotificationBody {
        SendNotificationBody {
            subject,
            user_id,
            message,
        }
    }
}


