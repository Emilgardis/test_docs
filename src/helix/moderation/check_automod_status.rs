//! Determines whether a string message meets the channel’s AutoMod requirements.
//! [`check-automod-status`](https://dev.twitch.tv/docs/api/reference#check-automod-status)
//!
//! # Accessing the endpoint
//!
//! ## Request: [CheckAutoModStatusRequest]
//!
//! To use this endpoint, construct a [`CheckAutoModStatusRequest`] with the [`CheckAutoModStatusRequest::builder()`] method.
//!
//! ```rust, no_run
//! use twitch_api2::helix::moderation::check_automod_status;
//! let request = check_automod_status::CheckAutoModStatusRequest::builder()
//!     .broadcaster_id("1234")
//!     .build();
//! ```
//!
//! ## Body: [CheckAutoModStatusBody]
//!
//! We also need to provide a body to the request containing what we want to change.
//!
//! ```
//! # use twitch_api2::helix::moderation::check_automod_status;
//! let body = check_automod_status::CheckAutoModStatusBody::builder()
//!     .msg_id("test1")
//!     .msg_text("automod please approve this!")
//!     .user_id("1234")
//!     .build();
//! ```
//!
//! ## Response: [CheckAutoModStatus]
//!
//!
//! Send the request to receive the response with [`HelixClient::req_post()`](helix::HelixClient::req_post).
//!
//!
//! ```rust, no_run
//! use twitch_api2::helix::{self, moderation::check_automod_status};
//! # use twitch_api2::client;
//! # #[tokio::main]
//! # async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync + 'static>> {
//! # let client: helix::HelixClient<'static, client::DummyHttpClient> = helix::HelixClient::default();
//! # let token = twitch_oauth2::AccessToken::new("validtoken".to_string());
//! # let token = twitch_oauth2::UserToken::from_existing(twitch_oauth2::dummy_http_client, token, None, None).await?;
//! let request = check_automod_status::CheckAutoModStatusRequest::builder()
//!     .broadcaster_id("1234")
//!     .build();
//! let body = vec![check_automod_status::CheckAutoModStatusBody::builder()
//!     .msg_id("test1")
//!     .msg_text("automod please approve this!")
//!     .user_id("1234")
//!     .build()];
//! let response: Vec<check_automod_status::CheckAutoModStatus> = client.req_post(request, body, &token).await?.data;
//! # Ok(())
//! # }
//! ```
//!
//! You can also get the [`http::Request`] with [`request.create_request(&token, &client_id)`](helix::RequestPost::create_request)
//! and parse the [`http::Response`] with [`request.parse_response(&request.get_uri()?)`](helix::RequestPost::parse_response())

use super::*;
/// Query Parameters for [Check AutoMod Status](super::check_automod_status)
///
/// [`check-automod-status`](https://dev.twitch.tv/docs/api/reference#check-automod-status)
#[derive(PartialEq, typed_builder::TypedBuilder, Deserialize, Serialize, Clone, Debug)]
#[non_exhaustive]
pub struct CheckAutoModStatusRequest {
    /// Must match the User ID in the Bearer token.
    #[builder(setter(into))]
    pub broadcaster_id: types::UserId,
}

/// Body Parameters for [Check AutoMod Status](super::check_automod_status)
///
/// [`check-automod-status`](https://dev.twitch.tv/docs/api/reference#check-automod-status)
#[derive(PartialEq, typed_builder::TypedBuilder, Deserialize, Serialize, Clone, Debug, Default)]
#[non_exhaustive]
pub struct CheckAutoModStatusBody {
    /// Developer-generated identifier for mapping messages to results.
    #[builder(setter(into))]
    pub msg_id: String,
    /// Message text.
    #[builder(setter(into))]
    pub msg_text: String,
    /// User ID of the sender.
    #[builder(setter(into))]
    pub user_id: String,
}

/// Return Values for [Check AutoMod Status](super::check_automod_status)
///
/// [`check-automod-status`](https://dev.twitch.tv/docs/api/reference#check-automod-status)
#[derive(PartialEq, Deserialize, Debug, Clone)]
#[cfg_attr(not(feature = "allow_unknown_fields"), serde(deny_unknown_fields))]
#[non_exhaustive]
pub struct CheckAutoModStatus {
    /// The msg_id passed in the body of the POST message. Maps each message to its status.
    pub msg_id: String,
    /// Indicates if this message meets AutoMod requirements.
    pub is_permitted: bool,
}

impl helix::Request for CheckAutoModStatusRequest {
    type Response = Vec<CheckAutoModStatus>;

    const PATH: &'static str = "moderation/enforcements/status";
    #[cfg(feature = "twitch_oauth2")]
    const SCOPE: &'static [twitch_oauth2::Scope] = &[twitch_oauth2::Scope::ModerationRead];
}

impl helix::RequestPost for CheckAutoModStatusRequest {
    type Body = Vec<CheckAutoModStatusBody>;

    fn body(&self, body: &Self::Body) -> Result<String, helix::BodyError> {
        #[derive(Serialize)]
        struct InnerBody<'a> {
            data: &'a Vec<CheckAutoModStatusBody>,
        }

        serde_json::to_string(&InnerBody { data: &body }).map_err(Into::into)
    }
}

#[test]
fn test_request() {
    use helix::*;
    let req = CheckAutoModStatusRequest::builder()
        .broadcaster_id("198704263".to_string())
        .build();

    // From twitch docs
    let data = br#"
{
   "data": [
     {
       "msg_id": "123",
       "is_permitted": true
     },
     {
       "msg_id": "393",
       "is_permitted": false
     }
   ]
}
"#
    .to_vec();

    let http_response = http::Response::builder().body(data).unwrap();

    let uri = req.get_uri().unwrap();
    assert_eq!(
        uri.to_string(),
        "https://api.twitch.tv/helix/moderation/enforcements/status?broadcaster_id=198704263"
    );

    dbg!(CheckAutoModStatusRequest::parse_response(Some(req), &uri, http_response).unwrap());
}
