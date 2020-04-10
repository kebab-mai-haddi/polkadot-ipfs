use clap::{load_yaml, App};
use hex;
use keyring::AccountKeyring;
use sp_core::blake2_256;
use sp_core::crypto::Pair;
use sp_core::crypto::Ss58Codec;
use sp_runtime::AccountId32;
use std;
use std::str;
use substrate_api_client::{compose_extrinsic, utils::hexstr_to_hash, Api};

fn from_slice(vector: Vec<u8>) -> [u8; 32] {
    let mut array = [0; 32];
    let bytes = &vector[..array.len()]; // panics if not enough data
    array.copy_from_slice(bytes);
    array
}

fn get_account_id_for_file_hash() {
    env_logger::init();
    let url = get_url();
    let mut api = Api::new(format!("ws://{}", url));
    let signer = AccountKeyring::Alice.pair();
    api.signer = Some(signer);
    let _file_hash = get_file_hash().to_string();
    let _file_hash_as_bytes = _file_hash.as_bytes();
    let _blake_hashed_file_hash = Box::new(blake2_256(_file_hash_as_bytes)).to_vec();
    println!("Blake hash is: {:?}", hex::encode(&_blake_hashed_file_hash));
    let mut result_str = api
        .get_storage("KittyStorage", "Value", Some(_blake_hashed_file_hash))
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
    let file_hash = matches.value_of("file-hash").unwrap();
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

fn write_to_polkadot() {
    let url = get_url();
    let from = AccountKeyring::Alice.pair();
    // address of our local blackchain
    let mut api = Api::new(format!("ws://{}", url)).set_signer(from);
    // convert string formatted hash from IPFS
    let file_hash_string = "0x0000000000000000000000000000000000000000000000000000000000000003";
    let file_hash = hexstr_to_hash(file_hash_string.to_string()).unwrap();
    println!("File hash has become: {:?}", file_hash);
    // create an RPC in JSON format
    let xt = compose_extrinsic!(api.clone(), "Substratekitties", "set_value", file_hash);
    println!("[+] Composed Extrinsic:\n {:?}\n", xt);
    let signer = AccountKeyring::Alice.pair();
    api.signer = Some(signer);
    // step to write to polkadot
    let tx_hash = api.send_extrinsic(xt.hex_encode()).unwrap();
    println!("[+] Transaction got finalized. Hash: {:?}", tx_hash);
}

pub fn determine_read_or_write_operation(){
    let yml = load_yaml!("../configuration.yaml");
    let matches = App::from_yaml(yml).get_matches();
    let mode_of_operation = matches.value_of("mode").unwrap_or("r");
    if mode_of_operation == "w"{
        write_to_polkadot();
    }
    if mode_of_operation == "r"{
        get_account_id_for_file_hash();
    }
}