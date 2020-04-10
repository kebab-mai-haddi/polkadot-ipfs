use clap::{load_yaml, App};
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

pub fn get_account_id_for_file_hash() {
    env_logger::init();
    let url = get_url();
    let _file_hash = get_file_hash();
    let mut api = Api::new(format!("ws://{}", url));
    let signer = AccountKeyring::Alice.pair();
    api.signer = Some(signer);
    let mut file_hash = _file_hash.to_string().into_bytes();
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

fn get_file_hash() -> String {
    let yml = load_yaml!("../configuration.yaml");
    let matches = App::from_yaml(yml).get_matches();
    let file_hash = matches
        .value_of("file-hash")
        .unwrap_or("0000000000000000000000000000000000000000000000000000000000000004");
    println!("IPFS Hash is: {:?}", file_hash);
    file_hash.to_string()
}

fn get_url() -> String {
    let yml = load_yaml!("../configuration.yaml");
    let matches = App::from_yaml(yml).get_matches();
    let node_ip = matches.value_of("node-server").unwrap_or("127.0.0.1");
    let node_port = matches.value_of("node-port").unwrap_or("9944");
    let url = format!("{}:{}", node_ip, node_port);
    url
}
