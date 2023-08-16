import { SecretNetworkClient as IBCClient, Wallet } from "secretjs";
import { env } from '$env/dynamic/public'

const wallet = new Wallet(env.PUBLIC_MNEMONIC)

export const client = new IBCClient({
    chainId: "akashnet-2",
    url: "https://rpc-akash.ecostake.com:443",
    wallet,
    walletAddress: wallet.address
})

import { ibc, chains } from 'chain-registry';


const akashChainInfo = chains.find(({chain_name})=>chain_name==='akash');
console.log("chains:", akashChainInfo);

const { channels } = ibc.find(({chain_1, chain_2})=>
    chain_1.chain_name==='akash' && 
    chain_2.chain_name==='secretnetwork'
)!;

const channel = channels.find(({ version })=> version === 'ics20-1' )!;

const akashSide = channel.chain_1;

console.log("ibcl:", channels);


client.tx.ibc.transfer({
    receiver: env.PUBLIC_CONTRACT_ADDRESS,
    memo: JSON.stringify({
        wasm: {
            contract: env.PUBLIC_CONTRACT_ADDRESS,
            msg: {}
        }
    }),
    source_port: akashSide.port_id,
    source_channel: akashSide.channel_id,
    token: {
        denom: "uakt",
        amount: "0"
    },
    sender: wallet.address
})