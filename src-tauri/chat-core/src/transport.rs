use crate::data::{presence, request, response};
use async_trait::async_trait;

pub trait Transport:
    Clone
    + Send
    + Sync
    // Publish
    + Service<request::Publish, Response = response::Publish, Error = <Self as Transport>::Error>
    // Subscribe
    + Service<request::Subscribe, Response = response::Subscribe, Error = <Self as Transport>::Error>
    // Set state
    + Service<request::SetState, Response = response::SetState, Error=<Self as Transport>::Error>
    // Get state
    + Service<request::GetState, Response = response::GetState, Error = <Self as Transport>::Error>
    // Here now.
    + Service<request::HereNow<presence::respond_with::OccupancyOnly>, Response = response::HereNow<presence::respond_with::OccupancyOnly>, Error = <Self as Transport>::Error>
    + Service<request::HereNow<presence::respond_with::OccupancyAndUUIDs>, Response = response::HereNow<presence::respond_with::OccupancyAndUUIDs>, Error = <Self as Transport>::Error>
    + Service<request::HereNow<presence::respond_with::Full>, Response = response::HereNow<presence::respond_with::Full>, Error = <Self as Transport>::Error>
    // Global Here now.
    + Service<request::GlobalHereNow<presence::respond_with::OccupancyOnly>, Response = response::GlobalHereNow<presence::respond_with::OccupancyOnly>, Error = <Self as Transport>::Error>
    + Service<request::GlobalHereNow<presence::respond_with::OccupancyAndUUIDs>, Response = response::GlobalHereNow<presence::respond_with::OccupancyAndUUIDs>, Error = <Self as Transport>::Error>
    + Service<request::GlobalHereNow<presence::respond_with::Full>, Response = response::GlobalHereNow<presence::respond_with::Full>, Error = <Self as Transport>::Error>
    // Where now.
    + Service<request::WhereNow, Response = response::WhereNow, Error = <Self as Transport>::Error>
    // Heartbeat
    + Service<request::Heartbeat, Response = response::Heartbeat, Error = <Self as Transport>::Error>
    // PAMv3
    + Service<request::Grant, Response = response::Grant, Error = <Self as Transport>::Error>
    // History
    + Service<request::GetHistory, Response = response::GetHistory,Error = <Self as Transport>::Error>
    + Service<request::DeleteHistory, Response = response::DeleteHistory,Error = <Self as Transport>::Error>
    + Service<request::MessageCountsWithTimeToken, Response = response::MessageCountsWithTimetoken,Error = <Self as Transport>::Error>
    + Service<request::MessageCountsWithChannelTimeTokens, Response = response::MessageCountsWithChannelTimeTokens,Error = <Self as Transport>::Error>
{
    type Error: std::error::Error + Send + Sync;
}

#[async_trait]
pub trait Service<Request>: Send {
    /// Response given by the service.
    type Response;

    /// Error produced by the service.
    type Error;

    /// Process the request and return the response asynchronously.
    async fn call(&self, req: Request) -> Result<Self::Response, Self::Error>;
}
