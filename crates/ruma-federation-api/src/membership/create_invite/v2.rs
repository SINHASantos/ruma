//! `/v2/` ([spec])
//!
//! [spec]: https://spec.matrix.org/latest/server-server-api/#put_matrixfederationv2inviteroomideventid

#[cfg(feature = "unstable-msc4125")]
use ruma_common::OwnedServerName;
use ruma_common::{
    api::{request, response, Metadata},
    metadata,
    serde::Raw,
    OwnedEventId, OwnedRoomId, RoomVersionId,
};
use ruma_events::AnyStrippedStateEvent;
use serde_json::value::RawValue as RawJsonValue;

const METADATA: Metadata = metadata! {
    method: PUT,
    rate_limited: false,
    authentication: ServerSignatures,
    history: {
        1.0 => "/_matrix/federation/v2/invite/{room_id}/{event_id}",
    }
};

/// Request type for the `create_invite` endpoint.
#[request]
pub struct Request {
    /// The room ID that the user is being invited to.
    #[ruma_api(path)]
    pub room_id: OwnedRoomId,

    /// The event ID for the invite event, generated by the inviting server.
    #[ruma_api(path)]
    pub event_id: OwnedEventId,

    /// The version of the room where the user is being invited to.
    pub room_version: RoomVersionId,

    /// The invite event which needs to be signed.
    pub event: Box<RawJsonValue>,

    /// An optional list of simplified events to help the receiver of the invite identify the room.
    pub invite_room_state: Vec<Raw<AnyStrippedStateEvent>>,

    /// An optional list of servers the invited homeserver should attempt to join or leave via,
    /// according to [MSC4125](https://github.com/matrix-org/matrix-spec-proposals/pull/4125).
    ///
    /// If present, it must not be empty.
    #[cfg(feature = "unstable-msc4125")]
    #[serde(skip_serializing_if = "Option::is_none", rename = "org.matrix.msc4125.via")]
    pub via: Option<Vec<OwnedServerName>>,
}

/// Response type for the `create_invite` endpoint.
#[response]
pub struct Response {
    /// The signed invite event.
    pub event: Box<RawJsonValue>,
}

impl Request {
    /// Creates a new `Request` with the given room ID, event ID, room version, event and invite
    /// room state.
    pub fn new(
        room_id: OwnedRoomId,
        event_id: OwnedEventId,
        room_version: RoomVersionId,
        event: Box<RawJsonValue>,
        invite_room_state: Vec<Raw<AnyStrippedStateEvent>>,
    ) -> Self {
        Self {
            room_id,
            event_id,
            room_version,
            event,
            invite_room_state,
            #[cfg(feature = "unstable-msc4125")]
            via: None,
        }
    }
}

impl Response {
    /// Creates a new `Response` with the given invite event.
    pub fn new(event: Box<RawJsonValue>) -> Self {
        Self { event }
    }
}
