use web3::{
    transports::{self, Http},
    Web3,
};

pub fn web3(url: &str) -> Web3<Http> {
    let transport = transports::Http::new(url).unwrap();
    Web3::new(transport)
}
