use {
    anyhow::{anyhow, Result},
    base64::{engine::general_purpose, Engine},
    bitcoin::{Transaction, Txid},
    bitcoincore_rpc::{Auth, Client},
    hyper::{Body, Method, Request, Uri},
    serde::Deserialize,
    serde_json::{json, Value},
};

#[allow(unused)]
pub(crate) struct Fetcher {
    auth: String,
    client: Client,
    url: Uri,
}

#[derive(Deserialize, Debug)]
#[allow(unused)]
struct JsonResponse<T> {
    error: Option<JsonError>,
    id: usize,
    result: Option<T>,
}

#[derive(Deserialize, Debug)]
#[allow(unused)]
struct JsonError {
    code: i32,
    message: String,
}

pub(crate) fn auth_parse(rpc_user: Option<String>, rpc_pass: Option<String>) -> Result<Auth> {
    match (rpc_user, rpc_pass) {
        (Some(rpc_user), Some(rpc_pass)) => Ok(Auth::UserPass(rpc_user, rpc_pass)),
        (None, Some(_rpc_pass)) => Err(anyhow!("no bitcoind rpc user specified")),
        (Some(_rpc_user), None) => Err(anyhow!("no bitcoind rpc password specified")),
        _ => Err(anyhow!("no bitcoind rpc auth specified")),
    }
}

impl Fetcher {
    #[allow(unused)]
    pub(crate) fn new(
        rpc_url: String,
        rpc_user: Option<String>,
        rpc_pass: Option<String>,
    ) -> Result<Self> {
        let url = Uri::try_from(&rpc_url).map_err(|e| anyhow!("Invalid rpc url {rpc_url}: {e}"))?;

        // let (user, password) = auth(rpc_user, rpc_pass)?.get_user_pass()?;
        let auth = format!("{}:{}", rpc_user.clone().unwrap(), rpc_pass.clone().unwrap());
        let auth = format!(
            "Basic {}",
            &base64::engine::general_purpose::STANDARD.encode(auth)
        );

        match auth_parse(rpc_user, rpc_pass) {
            Ok(a) => {
                let client = Client::new(&rpc_url, a).unwrap();
                Ok(Fetcher { client, url, auth })
            }
            Err(err) => Err(err),
        }
    }

    // pub(crate) async fn get_transactions(&self, txids: Vec<Txid>) -> Result<Vec<Transaction>> {
    //     if txids.is_empty() {
    //         return Ok(Vec::new());
    //     }

    //     let mut reqs = Vec::with_capacity(txids.len());
    //     for (i, txid) in txids.iter().enumerate() {
    //         let req = json!({
    //           "jsonrpc": "2.0",
    //           "id": i, // Use the index as id, so we can quickly sort the response
    //           "method": "getrawtransaction",
    //           "params": [ txid.to_string() ]
    //         });
    //         reqs.push(req);
    //     }

    //     let body = Value::Array(reqs).to_string();

    //     let mut results: Vec<JsonResponse<String>>;
    //     let mut retries = 0;

    //     loop {
    //         results = match self.try_get_transactions(body.clone()).await {
    //             Ok(results) => results,
    //             Err(error) => {
    //                 if retries >= 5 {
    //                     return Err(anyhow!(
    //                         "failed to fetch raw transactions after 5 retries: {}",
    //                         error
    //                     ));
    //                 }

    //                 log::info!("failed to fetch raw transactions, retrying: {}", error);

    //                 tokio::time::sleep(tokio::time::Duration::from_millis(
    //                     100 * u64::pow(2, retries),
    //                 ))
    //                 .await;
    //                 retries += 1;
    //                 continue;
    //             }
    //         };
    //         break;
    //     }

    //     // Return early on any error, because we need all results to proceed
    //     if let Some(err) = results.iter().find_map(|res| res.error.as_ref()) {
    //         return Err(anyhow!(
    //             "failed to fetch raw transaction: code {} message {}",
    //             err.code,
    //             err.message
    //         ));
    //     }

    //     // Results from batched JSON-RPC requests can come back in any order, so we must sort them by id
    //     results.sort_by(|a, b| a.id.cmp(&b.id));

    //     let txs =
    //         results
    //             .into_iter()
    //             .map(|res| {
    //                 res.result
    //                     .ok_or_else(|| anyhow!("Missing result for batched JSON-RPC response"))
    //                     .and_then(|str| {
    //                         general_purpose::STANDARD.decode(str).map_err(|e| {
    //                             anyhow!("Result for batched JSON-RPC response not valid hex: {e}")
    //                         })
    //                     })
    //                     .and_then(|h| {
    //                         bitcoin::consensus::deserialize(&h).map_err(|e| {
    //           anyhow!("Result for batched JSON-RPC response not valid bitcoin tx: {e}")
    //         })
    //                     })
    //             })
    //             .collect::<Result<Vec<Transaction>>>()?;
    //     Ok(txs)
    // }

    // async fn try_get_transactions(&self, body: String) -> Result<Vec<JsonResponse<String>>> {
    //     let req = Request::builder()
    //         .method(Method::POST)
    //         .uri(&self.url)
    //         .header(hyper::header::AUTHORIZATION, &self.auth)
    //         .header(hyper::header::CONTENT_TYPE, "application/json")
    //         .body(Body::from(body))?;

    //     let response = self.client.request(req).await?;

    //     let buf = hyper::body::to_bytes(response).await?;

    //     let results: Vec<JsonResponse<String>> = match serde_json::from_slice(&buf) {
    //         Ok(results) => results,
    //         Err(e) => {
    //             return Err(anyhow!(
    //                 "failed to parse JSON-RPC response: {e}. response: {response}",
    //                 e = e,
    //                 response = String::from_utf8_lossy(&buf)
    //             ))
    //         }
    //     };

    //     Ok(results)
    // }
}

