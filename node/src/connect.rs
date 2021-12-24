use web3::{Web3,transports::{self, Http}};

pub fn web3(url:&str) -> Web3<Http> {
    let transport = transports::Http::new(url).unwrap();
    Web3::new(transport)
}

