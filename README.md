# What is Blockchain?
The best read that I have found to understand the full implications of blockchain technology in simple words with no jargons: [A Blockchain Explanation That Your Parents Can Understand](https://taylorpearson.me/blockchain-for-dummies/)

# Why storing data in blockchain is expensive?
Why is storing data on the blockchain so expensive? It is because the data has to be stored by every full node on the blockchain network. When storing data on the blockchain, we do pay the base price for the transaction itself plus an amount per byte we want to store. [Source](https://www.blockchainguide.biz/storing-data-on-blockchain/)

# What is Polkadot?
[This](https://medium.com/posbakerz/what-is-polkadot-a-quick-overview-54ef264f15b9) overview is a must-read to understand what Polkadot is.

# Why do you need Polkadot to exchange data between two different blockchains? 
In Polkadot, there is no fees to exchange information between two blockchains. For eg, consider two scenarios A and B.
A: polkadot_btc and polkadot_eth are the two parachains( blockchains on Polkadot network ) and they can interact with each other for free _i.e._ there will be no transaction fees.
B: btc and eth are two entirely different blockchains and they need to interact, we would have to pay the transaction fees on both ends.

# What could go wrong if there is no Polkadot in between?
As explained in the above question, nothing goes wrong but you have to take care of the security, transaction fees, etc.

# What is IPFS?
>A peer-to-peer hypermedia protocol designed to make the web faster, safer, and more open.

In one simple line, IPFS is a decentralized, distributed data storage platform. 

If interested, you can read in depth about IPFS [here](https://hackernoon.com/understanding-ipfs-in-depth-1-5-a-beginner-to-advanced-guide-e937675a8c8a).

# Why do we need to bridge Polkadot and IPFS?
Let's consider a use case in your decentralized software that requires storing files(audio, video, etc.). Storing large files on any blockchain will cost you a lot of time and money. Read [this](https://itnext.io/build-a-simple-ethereum-interplanetary-file-system-ipfs-react-js-dapp-23ff4914ce4e) to understand how much does it cost to store a normal file on Ethereum blockchain. And if you are storing your files on services like S3, GCS, then you need to trust these centralized sotrage systems. So, we need some storage mechanism just like Blockchain - decentralized, distributed, no need to trust anyone and anything but Maths.

IPFS is a distributed file system storage. Polkadot or any blockchain for that matter needs IPFS because you should not store large files on the blockchain due to cost. There are certain applications that leverage IPFS and are built on blockchain. Two such applications are [Origin Protocol](https://www.originprotocol.com/en) and [Joystream](https://www.joystream.org/).

Hence, there is a need for a standardized bridge protocol that connects IPFS and Polkadot so that people can build DAO like Airbnb, Uber, etc.

# What’s the abstraction Polkadot/Substrate gives you to make the integration?
Polkadot provides me with the blockchain infra, I don't have to create a blockchain myself.

# Steps to bridge Polkadot and IPFS (Roadmap)
1. Set the state variables.
2. Capture the User’s file.
3. Convert the file to a buffer.
4. Send the buffered file to IPFS
5. IPFS returns a hash.
6. Get the User sign into Polkadot network
7. Send the IPFS for storage on Polkadot.
8. User will confirm the transaction to Polkadot.
9. Our app will return the transaction number.
10. The transaction hash number can be used to generate a transaction receipt with          information such as the block number.