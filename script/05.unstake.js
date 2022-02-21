const { connect, KeyPair, keyStores, utils } = require("near-api-js");
const fs = require("fs");
const path = require("path");
const homedir = require("os").homedir();

const CREDENTIALS_DIR = ".near-credentials";
const INCA_CONTRACT = "pnx.zus.testnet";
const XINCA_CONTRACT = "xpnx.testnet";
const ACCOUNT_ID = "inti25.testnet";

const credentialsPath = path.join(homedir, CREDENTIALS_DIR);
const keyStore = new keyStores.UnencryptedFileSystemKeyStore(credentialsPath);
// converts NEAR amount into yoctoNEAR (10^-24)

const amountInYocto = utils.format.parseNearAmount("0.0125");
const ftAttachAmount = utils.format.parseNearAmount("1");

const config = {
  keyStore,
  networkId: "testnet",
  nodeUrl: "https://rpc.testnet.near.org",
};

async function main() {
  const near = await connect(config);
  const account = await near.account(ACCOUNT_ID);

  const result = await account.functionCall({
    contractId: XINCA_CONTRACT,
    methodName: "unstake",
    args: {
      amount: '10000000000000000000',
    },
    attachedDeposit: '1',
    gas: "300000000000000",
  });
  console.log(result);
}
main()
  .then(() => process.exit(0))
  .catch((error) => {
    console.error(error);
    process.exit(1);
  });