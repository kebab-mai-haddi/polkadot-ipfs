# What is PIPFS?
A cli-tool for Substrate users to store their data on IPFS and record their ownership and/or access to data on the Substrate chain - providing them a means of storing data in a distributed, decentralized manner on a tamper-proof and distributed system ( aka Blockchain ).

For reading the background, click [here](Background.md)

# Instructions
## Install IPFS
Make sure you have a running IPFS cli. Click [here](https://ipfs.io/#install) to install IPFS.
## Install Rust
Click [here](https://www.rust-lang.org/tools/install) to install Rust.
## Setup a running substrate node
If this is the first time you are going to build a Substrate node, click [here](https://substrate.dev/docs/en/overview/getting-started/) and follow the instructions. We will be modifying the code later on but it is important that you are familiar with the basics of Substrate - knowing how to run the frontend, backend, tests, etc.

Once done with the above _Getting Started_ instructions, go to the base directory of the project and checkut to a particular commit. This is an important step. We use `substrate-api-client` which is compatible only with a particular commit of the [repo](github.com/paritytech/substrate). However, upon the v2 release of Substrate, we will be compatible with the latest Substrate code. Substrate-API-Client is a trustworthy repository as it is funded by the dev grants of Paritytech.
### Clone Substrate repo
Clone the repo if you haven't already.

`git clone https://github.com/paritytech/substrate`

`git checkout 00a400f82539e2f78e8ddbcd98aea512c87c5f3c`

### Building our Storage Map on Substrate
Following are some git diffs that should be applied to our Substrate node:

1. 
```
diff --git a/bin/node/runtime/src/lib.rs b/bin/node/runtime/src/lib.rs
index 6c5b75482..8dee517be 100644
--- a/bin/node/runtime/src/lib.rs
+++ b/bin/node/runtime/src/lib.rs
@@ -60,6 +60,8 @@ pub use pallet_contracts::Gas;
 pub use frame_support::StorageValue;
 pub use pallet_staking::StakerStatus;

+mod substratekitties;
+
 /// Implementations of some helper traits passed into runtime modules as associated types.
 pub mod impls;
 use impls::{CurrencyToVoteHandler, Author, LinearWeightToFee, TargetedFeeAdjustment};
@@ -592,7 +594,7 @@ impl pallet_vesting::Trait for Runtime {
        type Currency = Balances;
        type BlockNumberToBalance = ConvertInto;
 }
-
+impl substratekitties::Trait for Runtime {}
 construct_runtime!(
        pub enum Runtime where
                Block = Block,
@@ -627,6 +629,7 @@ construct_runtime!(
                Society: pallet_society::{Module, Call, Storage, Event<T>, Config<T>},
                Recovery: pallet_recovery::{Module, Call, Storage, Event<T>},
                Vesting: pallet_vesting::{Module, Call, Storage, Event<T>, Config<T>},
+               Substratekitties: substratekitties::{Module, Call, Storage},
        }
 );
 ```

 2. We have to add a file to our runtime: `bin/node/runtime/src/substratekitties.rs`
```
➜  substrate git:(00a400f82) ✗ cat bin/node/runtime/src/substratekitties.rs
use frame_support::{decl_module, decl_storage, dispatch::result::Result, ensure, StorageMap};
use frame_system::ensure_signed;
use sp_runtime::DispatchError;

// pub trait Trait: balances::Trait {}
pub trait Trait: pallet_balances::Trait {}

decl_storage! {
    trait Store for Module<T: Trait> as KittyStorage {
        // Value: map T::Hash => Option<T::AccountId>;
        // TODO: check whether this is the appropriate datatype(hash).
        Value: map hasher(blake2_256) T::Hash => Option<T::AccountId>;
        // Balances: map hasher(blake2_256) (T::AssetId, T::AccountId) => T::Balance;
    }
}

decl_module! {
    pub struct Module<T: Trait> for enum Call where origin: T::Origin {
        fn set_value(origin, value: T::Hash) -> Result<(), DispatchError> {
            let sender = ensure_signed(origin)?;
            ensure!(!<Value<T>>::contains_key(value), "key already exists");
            <Value<T>>::insert(value, sender);
            Ok(())
        }
    }
}
```

### Build the code
`cargo build --release`

`cargo test --all`

`./target/release/substrate --dev`

See if the node is running, click [here](https://polkadot.js.org/apps/#/accounts). You have to get connected to the local node from the configurations which are described well enough in the aforementioned _Getting Started_ tutorial.

## Run PIPFS
### Clone and build the repo
```
git clone https://github.com/kebab-mai-haddi/polkadot-ipfs.git 
cargo build
```
### Run the cli tool
```
./target/debug/pipfs --help
```
### Few examples include
#### Upload a file to IPFS
```
./target/debug/pipfs -m u
```
>Please enter the file path to upload on IPFS:
```
<file_path>
```
#### Record the ownership on Substrate Chain
After uploading the file to IPFS, PIPFS return the content identifier for your file stored on IPFS. Use that hash to record file ownership on the Substrate chain by running the command
```
./target/debug/pipfs -m w -f <file_hash>
```
You will see the following message if all goes successful:
>[+] Transaction got finalized. Hash: 0xebdf6d231a5fc47367382cb3368ac4479322559348db5051e4f0a8311075ff7e

#### Determine the ownership for a given file
```
./target/debug/pipfs -m r -f <file_hash>
```
> Account is: "5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY"

#### Download the file from IPFS
```
./target/debug/pipfs -m d
```
Enter the checksum when the following prompt appears:
> Please enter the checksum of the file:

If all goes well, you will see the message:
> The file is downloaded now!
> Saving file(s) to QmNYERzV2LfD2kkfahtfv44ocHzEFK1sLBaE7zdcYT2GAZ

Please provide your feedback on [my email](mailto:hi@aviralsrivastava.com). For future work, click [here](Future_Work.md)