const { connect, KeyPair, keyStores, utils } = require("near-api-js");
const fs = require("fs");
const path = require("path");
const homedir = require("os").homedir();

const CREDENTIALS_DIR = ".near-credentials";
const INCA_CONTRACT = "pnx.zus.testnet";
const XINCA_CONTRACT = "xpnx.testnet";
const ACCOUNT_ID = "inti01.testnet";

const credentialsPath = path.join(homedir, CREDENTIALS_DIR);
const keyStore = new keyStores.UnencryptedFileSystemKeyStore(credentialsPath);

const config = {
  keyStore,
  networkId: "testnet",
  nodeUrl: "https://rpc.testnet.near.org",
};
//
async function main() {
  const near = await connect(config);
  const account = await near.account(XINCA_CONTRACT);
  const result = await account.functionCall({
    contractId: XINCA_CONTRACT,
    methodName: "modify_reward_per_sec",
    args: {
      'reward_per_sec': "500000000000000", // 0.0005
      'distribute_before_change': true
    },
    gas: "300000000000000",
  })
  console.log(result);
}
main()
  .then(() => process.exit(0))
  .catch((error) => {
    console.error(error);
    process.exit(1);
  });