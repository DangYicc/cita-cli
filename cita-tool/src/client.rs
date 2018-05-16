use tokio_core::reactor::Core;
use std::{io, str, u64};
use hyper::{self, Body, Client as HyperClient, Method, Request, Uri};
use std::{convert::Into, str::FromStr};
use serde_json;
use futures::{Future, Stream, future::JoinAll, future::join_all};
use super::{JsonRpcParams, JsonRpcResponse, ParamsValue, PrivKey, ResponseValue, Transaction};
use hex::{decode, encode};
use protobuf::Message;
use uuid::Uuid;

const CITA_BLOCK_BUMBER: &str = "cita_blockNumber";
const CITA_GET_META_DATA: &str = "cita_getMetaData";

/// Jsonrpc client, Only to one chain
#[derive(Debug)]
pub struct Client {
    id: u64,
    url: Vec<String>,
    core: Core,
    chain_id: Option<u32>,
}

impl Client {
    /// Create a client for CITA
    pub fn new() -> io::Result<Self> {
        let core = Core::new()?;
        Ok(Client {
            id: 0,
            url: Vec::new(),
            core: core,
            chain_id: None,
        })
    }

    /// Add node url
    pub fn add_url<T: Into<String>>(mut self, url: T) -> Self {
        self.url.push(url.into());
        self
    }

    /// Send requests
    pub fn send_request(
        &mut self,
        method: &str,
        params: JsonRpcParams,
    ) -> Result<Vec<JsonRpcResponse>, ()> {
        self.id = self.id.overflowing_add(1).0;

        let params = params.insert("id", ParamsValue::Int(self.id));
        let reqs = self.make_requests_with_all_url(params);

        match method {
            CITA_BLOCK_BUMBER => Ok(self.run(reqs)),
            CITA_GET_META_DATA => Ok(self.run(reqs)),
            _ => Err(()),
        }
    }

    fn make_requests_with_all_url(
        &self,
        params: JsonRpcParams,
    ) -> JoinAll<Vec<Box<Future<Item = hyper::Chunk, Error = hyper::error::Error>>>> {
        let client = HyperClient::new(&self.core.handle());
        let mut reqs = Vec::new();
        for url in self.url.as_slice() {
            let uri = Uri::from_str(url).unwrap();
            let mut req: Request<Body> = Request::new(Method::Post, uri);
            req.set_body(serde_json::to_string(&params).unwrap());
            let future: Box<Future<Item = hyper::Chunk, Error = hyper::error::Error>> =
                Box::new(client.request(req).and_then(|res| res.body().concat2()));
            reqs.push(future);
        }
        join_all(reqs)
    }

    #[allow(dead_code)]
    fn make_requests_with_params_list<T: Iterator<Item = JsonRpcParams>>(
        &mut self,
        params: T,
    ) -> JoinAll<Vec<Box<Future<Item = hyper::Chunk, Error = hyper::error::Error>>>> {
        let url = self.url.as_slice()[0].clone();
        let client = HyperClient::new(&self.core.handle());
        let mut reqs = Vec::new();
        if !self.url.is_empty() {
            for params in params.map(|param| {
                self.id = self.id.overflowing_add(1).0;
                param.insert("id", ParamsValue::Int(self.id))
            }) {
                let uri = Uri::from_str(&url).unwrap();
                let mut req: Request<Body> = Request::new(Method::Post, uri);
                req.set_body(serde_json::to_string(&params).unwrap());
                let future: Box<
                    Future<Item = hyper::Chunk, Error = hyper::error::Error>,
                > = Box::new(client.request(req).and_then(|res| res.body().concat2()));
                reqs.push(future);
            }
        }
        join_all(reqs)
    }

    /// Constructing a UnverifiedTransaction hex string
    pub fn generate_transaction(
        &mut self,
        code: &str,
        address: String,
        pv: &PrivKey,
        current_height: u64,
        chain_id: Option<u32>,
    ) -> String {
        let data = decode(code).unwrap();

        let mut tx = Transaction::new();
        tx.set_data(data);
        // Create a contract if the target address is empty
        tx.set_to(address);
        tx.set_nonce(encode(Uuid::new_v4().as_bytes()));
        tx.set_valid_until_block(current_height + 88);
        tx.set_quota(1000000);
        tx.set_chain_id(chain_id.unwrap_or(self.get_chain_id()));
        encode(
            tx.sign(*pv)
                .take_transaction_with_sig()
                .write_to_bytes()
                .unwrap(),
        )
    }

    fn get_chain_id(&mut self) -> u32 {
        if self.chain_id.is_some() {
            self.chain_id.unwrap()
        } else {
            let params = JsonRpcParams::new()
                .insert(
                    "params",
                    ParamsValue::List(vec![ParamsValue::String(String::from("latest"))]),
                )
                .insert(
                    "method",
                    ParamsValue::String(String::from("cita_getMetaData")),
                );
            if let Some(ResponseValue::Map(mut value)) =
                self.send_request("cita_getMetaData", params)
                    .unwrap()
                    .pop()
                    .unwrap()
                    .result()
            {
                match value.remove("chainId").unwrap() {
                    ParamsValue::Int(chain_id) => {
                        self.chain_id = Some(chain_id as u32);
                        return chain_id as u32;
                    }
                    _ => return 0,
                }
            } else {
                0
            }
        }
    }

    /// Start run
    fn run(
        &mut self,
        reqs: JoinAll<Vec<Box<Future<Item = hyper::Chunk, Error = hyper::error::Error>>>>,
    ) -> Vec<JsonRpcResponse> {
        let responses = self.core.run(reqs).unwrap();
        responses
            .into_iter()
            .map(|response| serde_json::from_slice::<JsonRpcResponse>(&response).unwrap())
            .collect::<Vec<JsonRpcResponse>>()
    }
}
