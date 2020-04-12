
/*
This examples writes file hash in our blockchain
substrate is a framework for creating this (blockchain)

Examples: 
1. https://github.com/scs/substrate-api-client/blob/master/src/examples/example_custom_storage_struct.rs
2. https://github.com/scs/substrate-api-client/blob/master/src/examples/example_get_storage.rs
*/

use keyring::AccountKeyring;
use sp_core::crypto::Pair;
use substrate_api_client::{compose_extrinsic, utils::hexstr_to_hash, Api};

fn main() {
    env_logger::init();
    let url = get_node_url_from_cli();
    let from = AccountKeyring::Alice.pair();
    
    // address of our local blackchain
    let mut api = Api::new(format!("ws://{}", url)).set_signer(from);
    
    // convert string formatted hash from IPFS
    let file_hash_string = "0x0000000000000000000000000000000000000000000000000000000000000010";
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


pub fn get_node_url_from_cli() -> String {
    let node_ip = "127.0.0.1";
    let node_port = "9944";
    let url = format!("{}:{}", node_ip, node_port);
    println!("Interacting with node on {}\n", url);
    url
}
