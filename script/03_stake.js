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
    console.log('getAccountDetails', await account.getAccountDetails());
    const storage_balance = await account.viewFunction(XINCA_CONTRACT, "storage_balance_of", {"account_id": ACCOUNT_ID});
    console.log('storage_balance', storage_balance);
    if (storage_balance == null) {
        const storage_deposit = await account.functionCall({
            contractId: XINCA_CONTRACT,
            methodName: 'storage_deposit',
            args: {"account_id": ACCOUNT_ID, "registration_only": true},
            attachedDeposit: amountInYocto
        })
        console.log('storage_deposit', storage_deposit);
    }

    const result = await account.functionCall({
        contractId: INCA_CONTRACT,
        methodName: "ft_transfer_call",
        args: {
            receiver_id: XINCA_CONTRACT,
            amount: '10000000000000000000',
            msg: ''
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
