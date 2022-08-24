/*
 * notifier
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: 0.3.0
 * Contact: 
 * Generated by: https://openapi-generator.tech
 */




#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct NotifyBody {
    #[serde(rename = "message")]
    pub message: String,
    #[serde(rename = "user_id")]
    pub user_id: String,
    #[serde(rename = "subject")]
    pub subject: String,
}

impl NotifyBody {
    pub fn new(message: String, user_id: String, subject: String) -> NotifyBody {
        NotifyBody {
            message,
            user_id,
            subject,
        }
    }
}


