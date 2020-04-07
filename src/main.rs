use hex;
use keyring::AccountKeyring;
use sp_core::crypto::Ss58Codec;
use sp_runtime::AccountId32;
use std;
use std::str;
use substrate_api_client::Api;

fn from_slice(vector: Vec<u8>) -> [u8; 32] {
    let mut array = [0; 32];
    let bytes = &vector[..array.len()]; // panics if not enough data
    array.copy_from_slice(bytes);
    array
}

fn main() {
    env_logger::init();
    let url = get_node_url_from_cli();
    let mut api = Api::new(format!("ws://{}", url));
    let signer = AccountKeyring::Alice.pair();
    api.signer = Some(signer);
    let mut file_hash = "0000000000000000000000000000000000000000000000000000000000000002"
        .to_string()
        .into_bytes();
    file_hash = hex::decode(file_hash).unwrap();
    let mut result_str = api
        .get_storage("KittyStorage", "Value", Some(file_hash))
        .unwrap();
    result_str.pop();
    println!("Result str is: {:?}", &result_str);
    let account_id: &str = &result_str[3..];
    let account: [u8; 32] = from_slice(hex::decode(account_id).unwrap());
    println!("Account after hex decode is: {:?}", account);
    let _account_id: AccountId32 = account.into();
    println!("Account is: {:?}", _account_id.to_ss58check());
}

pub fn get_node_url_from_cli() -> String {
    let node_ip = "127.0.0.1";
    let node_port = "9944";
    let url = format!("{}:{}", node_ip, node_port);
    println!("Interacting with node on {}\n", url);
    url
}
