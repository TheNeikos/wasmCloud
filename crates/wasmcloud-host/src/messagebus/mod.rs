use crate::auth::Authorizer;
use crate::capability::link_cache::LinkCache;
use crate::Result;
use crate::{Invocation, WasccEntity};
use actix::dev::{MessageResponse, ResponseChannel};
use actix::prelude::*;
use std::collections::HashMap;
use wascap::prelude::{Claims, KeyPair};

use crate::messagebus::rpc_client::RpcClient;
pub use handlers::OP_BIND_ACTOR;
use std::time::Duration;

pub(crate) mod handlers;
mod hb;
pub(crate) mod nats_subscriber;
pub(crate) mod rpc_client;
pub(crate) mod rpc_subscription;
pub(crate) mod utils;

pub(crate) use nats_subscriber::{NatsMessage, NatsSubscriber};

#[derive(Default)]
pub(crate) struct MessageBus {
    nc: Option<nats::asynk::Connection>,
    namespace: Option<String>,
    subscribers: HashMap<WasccEntity, Recipient<Invocation>>,
    rpc_outbound: Option<Addr<RpcClient>>,
    link_cache: LinkCache,
    claims_cache: HashMap<String, Claims<wascap::jwt::Actor>>,
    key: Option<KeyPair>,
    authorizer: Option<Box<dyn Authorizer>>,
}

#[derive(Message)]
#[rtype(result = "QueryResponse")]
pub struct QueryActors;

#[derive(Message)]
#[rtype(result = "QueryResponse")]
pub struct QueryProviders;

#[derive(Message)]
#[rtype(result = "LinksResponse")]
pub struct QueryAllLinks;

pub struct LinksResponse {
    pub links: Vec<LinkDefinition>,
}

pub struct LinkDefinition {
    pub actor_id: String,
    pub provider_id: String,
    pub contract_id: String,
    pub link_name: String,
    pub values: std::collections::HashMap<String, String>,
}

pub struct QueryResponse {
    pub results: Vec<String>,
}

impl<A, M> MessageResponse<A, M> for QueryResponse
where
    A: Actor,
    M: Message<Result = QueryResponse>,
{
    fn handle<R: ResponseChannel<M>>(self, _: &mut A::Context, tx: Option<R>) {
        if let Some(tx) = tx {
            tx.send(self);
        }
    }
}

impl<A, M> MessageResponse<A, M> for LinksResponse
where
    A: Actor,
    M: Message<Result = LinksResponse>,
{
    fn handle<R: ResponseChannel<M>>(self, _: &mut A::Context, tx: Option<R>) {
        if let Some(tx) = tx {
            tx.send(self);
        }
    }
}

#[derive(Message)]
#[rtype(result = "()")]
pub struct Initialize {
    pub nc: Option<nats::asynk::Connection>,
    pub namespace: Option<String>,
    pub key: KeyPair,
    pub auth: Box<dyn Authorizer>,
    pub rpc_timeout: Duration,
}

#[derive(Message)]
#[rtype(result = "()")]
pub struct Subscribe {
    pub interest: WasccEntity,
    pub subscriber: Recipient<Invocation>,
}

#[derive(Message)]
#[rtype(result = "()")]
pub struct Unsubscribe {
    pub interest: WasccEntity,
}

#[derive(Message)]
#[rtype(result = "()")]
pub struct PutClaims {
    pub claims: Claims<wascap::jwt::Actor>,
}

#[derive(Message)]
#[rtype(result = "Option<String>")]
pub struct LookupLink {
    // Capability ID
    pub contract_id: String,
    pub actor: String,
    pub link_name: String,
}

#[derive(Message, Clone)]
#[rtype(result = "Result<()>")]
pub struct AdvertiseLink {
    pub contract_id: String,
    pub actor: String,
    pub link_name: String,
    pub provider_id: String,
    pub values: HashMap<String, String>,
}

#[derive(Message)]
#[rtype(result = "()")]
pub struct PutLink {
    pub contract_id: String,
    pub actor: String,
    pub link_name: String,
    pub provider_id: String,
    pub values: HashMap<String, String>,
}

#[derive(Message)]
#[rtype(result = "Result<()>")]
pub struct AdvertiseClaims {
    pub claims: Claims<wascap::jwt::Actor>,
}

#[derive(Message)]
#[rtype(result = "FindLinksResponse")]
pub struct FindLinks {
    pub provider_id: String,
    pub link_name: String,
}

#[derive(Debug)]
pub struct FindLinksResponse {
    pub links: Vec<(String, HashMap<String, String>)>,
}

#[derive(Message)]
#[rtype(result = "ClaimsResponse")]
pub struct GetClaims;

#[derive(Debug)]
pub struct ClaimsResponse {
    pub claims: HashMap<String, Claims<wascap::jwt::Actor>>,
}

#[derive(Message)]
#[rtype(result = "bool")]
pub struct CanInvoke {
    pub actor: String,
    pub contract_id: String,
    pub operation: String,
    pub provider_id: String,
    pub link_name: String,
}

#[derive(Message)]
#[rtype(result = "()")]
pub struct EnforceLocalLink {
    pub actor: String,
    pub contract_id: String,
    pub link_name: String,
}

#[derive(Message)]
#[rtype(result = "()")]
pub struct EnforceLocalActorLinks {
    pub actor: String,
}

#[derive(Message)]
#[rtype(result = "()")]
pub struct EnforceLocalProviderLinks {
    pub provider_id: String,
    pub link_name: String,
}

#[derive(Message)]
#[rtype(result = "()")]
pub struct EstablishAllLinks {}

impl<A, M> MessageResponse<A, M> for FindLinksResponse
where
    A: Actor,
    M: Message<Result = FindLinksResponse>,
{
    fn handle<R: ResponseChannel<M>>(self, _: &mut A::Context, tx: Option<R>) {
        if let Some(tx) = tx {
            tx.send(self);
        }
    }
}

impl<A, M> MessageResponse<A, M> for ClaimsResponse
where
    A: Actor,
    M: Message<Result = ClaimsResponse>,
{
    fn handle<R: ResponseChannel<M>>(self, _: &mut A::Context, tx: Option<R>) {
        if let Some(tx) = tx {
            tx.send(self);
        }
    }
}
