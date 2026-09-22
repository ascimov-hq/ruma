//! `POST /_matrix/client/unstable/com.beeper.yeet/rooms/{roomId}/delete`
//!
//! Beeper's room-yeeting extension: instruct the homeserver to fully purge a
//! room, rather than just emptying it. Called by mautrix bridgev2's
//! `DeleteRoom` when the homeserver advertises `com.beeper.room_yeeting` in
//! `/_matrix/client/versions` `unstable_features`.

pub mod unstable {
    //! `/unstable/` (Beeper's `com.beeper.yeet` room-yeeting extension, no
    //! published MSC)

    use ruma_common::{
        OwnedRoomId,
        api::{auth_scheme::AppserviceToken, request, response},
        metadata,
    };

    metadata! {
        method: POST,
        rate_limited: false,
        authentication: AppserviceToken,
        history: {
            unstable => "/_matrix/client/unstable/com.beeper.yeet/rooms/{room_id}/delete",
        }
    }

    /// Request type for the `delete_room_yeet` endpoint.
    #[request]
    pub struct Request {
        /// The room to purge.
        #[ruma_api(path)]
        pub room_id: OwnedRoomId,
    }

    /// Response type for the `delete_room_yeet` endpoint.
    #[response]
    pub struct Response {}

    impl Request {
        /// Creates a new `Request` for the given room ID.
        pub fn new(room_id: OwnedRoomId) -> Self {
            Self { room_id }
        }
    }

    impl Response {
        /// Creates a new `Response`.
        pub fn new() -> Self {
            Self {}
        }
    }
}
