const nearAPI = require("near-api-js");

async function main() {
    console.log("hello")

    // creates keyStore from a private key string
    // you can define your key here or use an environment variable

    const { keyStores, KeyPair, connect, WalletConnection, providers } = nearAPI;
    const keyStore = new keyStores.InMemoryKeyStore();
    const PRIVATE_KEY = "3XnoQybjR5nuVuP61nQenUyXNtoFwxSXdi9iTrFafre3s1ra7ozxEgvpBcZgekvwJiSQU3XAKAuq23VEsEjEs8oY";
    // creates a public / private key pair using the provided private key
    const keyPair = KeyPair.fromString(PRIVATE_KEY);
    // adds the keyPair you created to keyStore
    await keyStore.setKey("testnet", "pnx_token.zus.testnet", keyPair);


    const config = {
        networkId: "testnet",
        keyStore, // optional if not signing transactions
        nodeUrl: "https://rpc.testnet.near.org",
        walletUrl: "https://wallet.testnet.near.org",
        helperUrl: "https://helper.testnet.near.org",
        explorerUrl: "https://explorer.testnet.near.org",
    };
    const near = await connect(config);

    const account = await near.account("pnx_token.zus.testnet");
    let balance = await account.getAccountBalance();

    console.log("balance", balance);
    console.log("Detail", await account.getAccountDetails());

    const provider = new providers.JsonRpcProvider(
        "https://archival-rpc.testnet.near.org"
    );

    const TX_HASH = "A6utqRDa5CisafD1975MBk9to7t8RS1U4g78TvDmGKGK";

    const result = await provider.txStatus(TX_HASH, "pnx_token.zus.testnet");
    console.log("Result: ", result);

}
main()
    .then(() => process.exit(0))
    .catch((error) => {
        console.error(error);
        process.exit(1);
    });
