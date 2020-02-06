# Why do we need to bridge polkadot and ipfs?
IPFS is a distributed file system storage. Polkadot or any blockchain for that matter needs IPFS because you should not store large files on the blockchain due to cost. Click [here](https://itnext.io/build-a-simple-ethereum-interplanetary-file-system-ipfs-react-js-dapp-23ff4914ce4e) to see how much does it cost to store huge files on Ethereum.

There are certain applications that leverage IPFS and are built on blockchain. Two such applications are [Origin Protocol](https://www.originprotocol.com/en) and [Joystream](https://www.joystream.org/).

Hence, there is a need for a standardized bridge protocol that connects IPFS and Polkadot so that people can build DAO like Airbnb, Uber, etc.

I am taking a case study of Airbnb(Listing places). User is using a decentralized application DAirbnb(say) and they want to get listings of the hopuses in the region of their choice.

The use cases are as follows:
i) Enter region
ii) On blockchain, the app gets all the places for that particular region from a hash table H(say).
iii) Show the JSON response to the user
iv) User selects a particular place(flat, villa, etc) and gets to see all the details of that place such as: owner, address, price, all images of that place.

Now, all the images are fetched from IPFS. And this is how the parachain would work:
i) use the hash of all the images and get all of them from IPFS,
ii) embed the web link to display the image to the user.


This is what H looks like:
regions: {
    region_1: {
        place_1: {
            name: '',
            add: '',
            owner: '',
            images: {
                image_1_name: <IPFS hash>,
                image_2_name: <IPFS hash>,
                .,
                .,
                image_3_name: <IPFS hash>
            }
        },
        place_2: {

        },
        .,
        .,
        .,
        place_n: {

        }
    },
    region_2: {

    },
    .,
    .,
    .,
    region_n: {

    }
}