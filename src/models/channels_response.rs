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
pub struct ChannelsResponse {
    #[serde(rename = "channels")]
    pub channels: Vec<crate::models::ChannelResponse>,
    #[serde(rename = "user_id")]
    pub user_id: String,
}

impl ChannelsResponse {
    pub fn new(channels: Vec<crate::models::ChannelResponse>, user_id: String) -> ChannelsResponse {
        ChannelsResponse {
            channels,
            user_id,
        }
    }
}


