use clap::{load_yaml, App};
use hex;
use keyring::AccountKeyring;
use sp_core::blake2_256;
use sp_core::crypto::Pair;
use sp_core::crypto::Ss58Codec;
use sp_runtime::AccountId32;
use std;
use std::io::{self, Write};
use std::process::Command;
use std::str;
use substrate_api_client::{compose_extrinsic, utils::hexstr_to_hash, Api};

/// Slices a vector into array
fn from_slice(vector: Vec<u8>) -> [u8; 32] {
    let mut array = [0; 32];
    let bytes = &vector[..array.len()]; // panics if not enough data
    array.copy_from_slice(bytes);
    array
}

/// Gets file hash from the command line
fn get_file_hash() -> String {
    let yml = load_yaml!("../configuration.yaml");
    let matches = App::from_yaml(yml).get_matches();
    let file_hash = matches.value_of("file-hash").unwrap();
    file_hash.to_string()
}

/// Gets URL of the running substrate node from the command line
fn get_url() -> String {
    let yml = load_yaml!("../configuration.yaml");
    let matches = App::from_yaml(yml).get_matches();
    let node_ip = matches.value_of("node-server").unwrap_or("127.0.0.1");
    let node_port = matches.value_of("node-port").unwrap_or("9944");
    let url = format!("{}:{}", node_ip, node_port);
    url
}

/// fetch account id from substrate chain
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

fn write_to_polkadot() {
    env_logger::init();
    let url = get_url();
    let from = AccountKeyring::Alice.pair();
    let mut api = Api::new(format!("ws://{}", url)).set_signer(from);
    let mut _file_hash = get_file_hash().to_string();
    let _file_hash_as_bytes = _file_hash.as_bytes();
    let _blake_hashed_file_hash = Box::new(blake2_256(_file_hash_as_bytes)).to_vec();
    let file_prefix = "0x".to_string();
    let hex_encoded_blake_hash = file_prefix + &hex::encode(&_blake_hashed_file_hash);
    println!(
        "Blake Hashed File hash has become: {:?}",
        hex_encoded_blake_hash
    );
    let _substrate_reqd_file_hash = hexstr_to_hash(hex_encoded_blake_hash).unwrap();
    println!(
        "Substrate required File hash has become: {:?}",
        _substrate_reqd_file_hash
    );
    // create an RPC in JSON format
    let xt = compose_extrinsic!(
        api.clone(),
        "Substratekitties",
        "set_value",
        _substrate_reqd_file_hash
    );
    println!("[+] Composed Extrinsic:\n {:?}\n", xt);
    let signer = AccountKeyring::Alice.pair();
    api.signer = Some(signer);
    // step to write to polkadot
    let tx_hash = api.send_extrinsic(xt.hex_encode()).unwrap();
    println!("[+] Transaction got finalized. Hash: {:?}", tx_hash);
}

pub fn determine_read_or_write_operation() {
    let yml = load_yaml!("../configuration.yaml");
    let matches = App::from_yaml(yml).get_matches();
    let mode_of_operation = matches.value_of("mode").unwrap_or("r");
    if mode_of_operation == "u" {
        upload_the_file();
    }
    if mode_of_operation == "w" {
        write_to_polkadot();
    }
    if mode_of_operation == "r" {
        get_account_id_for_file_hash();
    }
    if mode_of_operation == "d" {
        download_the_file();
    }
}

fn download_the_file() {
    println!("Please enter the checksum of the file: ");
    let mut checksum = String::new();
    io::stdin()
        .read_line(&mut checksum)
        .expect("Failed to read checksum.");
    println!("The checksum entered is: {}", checksum);
    let ipfs_output = Command::new("ipfs")
        .arg("get")
        .arg(checksum.trim_end())
        .output()
        .expect("ipfs command failed to download the file.");
    println!("The file is downloaded now!");
    io::stdout().write_all(&ipfs_output.stdout).unwrap();
}

fn upload_the_file() {
    println!("Please enter the file path to upload on IPFS: ");
    let mut file_path = String::new();
    io::stdin()
        .read_line(&mut file_path)
        .expect("Failed to read file path.");
    println!("The file path entered is: {}", file_path);
    let ipfs_output = Command::new("ipfs")
        .arg("add")
        .arg(file_path.trim_end())
        .output()
        .expect("ipfs command failed to start");
    println!("Hash digest is: ");
    io::stdout().write_all(&ipfs_output.stdout).unwrap();
}
