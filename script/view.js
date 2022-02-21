const { connect, KeyPair, keyStores, utils } = require("near-api-js");
const fs = require("fs");
const path = require("path");
const homedir = require("os").homedir();

const CREDENTIALS_DIR = ".near-credentials";
const INCA_CONTRACT = "pnx.zus.testnet";
const XINCA_CONTRACT = "xpnx.testnet";
const ACCOUNT_ID = "inti02.testnet";

const credentialsPath = path.join(homedir, CREDENTIALS_DIR);
const keyStore = new keyStores.UnencryptedFileSystemKeyStore(credentialsPath);

const config = {
  keyStore,
  networkId: "testnet",
  nodeUrl: "https://rpc.testnet.near.org",
};

async function main() {
  const near = await connect(config);
  const account = await near.account(ACCOUNT_ID);
  // view INTI Token Contract
  let result = await account.viewFunction(INCA_CONTRACT, "ft_metadata");
  console.log(result);
  // view xINTI Token Contract
  result = await account.viewFunction(XINCA_CONTRACT, "ft_metadata");
  console.log(result);

  result = await account.viewFunction(XINCA_CONTRACT, "get_virtual_price");
  console.log('Price = ', result / 100000000);

}
main()
  .then(() => process.exit(0))
  .catch((error) => {
    console.error(error);
    process.exit(1);
  });