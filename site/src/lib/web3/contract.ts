import { PUBLIC_CONSUMER_CHANNEL, PUBLIC_CONSUMER_TOKEN, PUBLIC_CONTRACT_ADDRESS, PUBLIC_CONTRACT_CODE_HASH, PUBLIC_SECRET_CHAIN_ENDPOINT, PUBLIC_SECRET_CHAIN_ID, PUBLIC_SECRET_CHANNEL } from "$env/static/public";
import { coinFromString, type Permit, type SecretNetworkClient } from "secretjs";
import { getClient } from "./clients";

import { toNumber, toSecretIBCDenom } from "$lib/utils";
import { cells, cloverCount, fertilizerCount, shovelCount } from "$lib/state";
import { parseConfig } from "./game";
import { Powerup, type Cell, type MainResult } from "$lib/types";


export const hookWrapper = (msg: object) => {
    return JSON.stringify({
        wasm: {
            contract: PUBLIC_CONTRACT_ADDRESS,
            msg
        }
    })
}

export const executeOverIBC = async (
    client: SecretNetworkClient,
    contractMsg: object,
    amount: string
) => {
    return await client.tx.ibc.transfer({
        sender: client.address,
        receiver: PUBLIC_CONTRACT_ADDRESS,
        token: coinFromString(amount + PUBLIC_CONSUMER_TOKEN),
        source_port: "transfer",
        source_channel: PUBLIC_CONSUMER_CHANNEL,
        memo: hookWrapper(contractMsg),
        timeout_timestamp: String(Math.floor(Date.now()/1000) + 90)
    })
}


export const openCell = async (
    client: SecretNetworkClient,
    cell_id: number,
    permit: Permit,
    powerups: [],
    amount: string
) => {

    const contractMsg = {
        open_cell: {
            cell_id,
            powerups,
            permit
        }
    }


    const res =  await executeOverIBC(
        client,
        contractMsg,
        amount
    )

    const ibcRes = await res.ibcResponses[0];

    console.log("IBC RES:", ibcRes)

    return res;
}


export const queryContract = async (
    query: object
  ) : Promise<any> => {
  
    const client = getClient(
      PUBLIC_SECRET_CHAIN_ID,
      PUBLIC_SECRET_CHAIN_ENDPOINT
    );
  
    return await client.query.compute.queryContract({
      contract_address: PUBLIC_CONTRACT_ADDRESS,
      code_hash: PUBLIC_CONTRACT_CODE_HASH ?? "",
      query
    })
  
}





export const getMainPageInfo = async (
    permit?: Permit
) => {
    
    const res : MainResult = await queryContract({
        main: {
            permit
        }
    })

    console.log("MAIN PAGE INFO:", res)


    const tokenOnSecret = toSecretIBCDenom(
        PUBLIC_CONSUMER_TOKEN,
        PUBLIC_SECRET_CHANNEL
    )

    const foundConfig = res.network_configs.find(
        (nc) => nc[0] === tokenOnSecret
    )

    if (foundConfig) {
        parseConfig(foundConfig[1])
    } else {
        console.error("FATAL: The contract has no config fot this chain")
    }


    cells.set(
        res.cells
        .map((c : Cell, i: number) => ({ id: i+1, open_at: new Date(c.open_at * 1000)  }))
    )

    if (res.powerups) {
        for (const [pup, count] of res.powerups) {
            if (pup == Powerup.Clover) {
                cloverCount.set(toNumber(count))
            } else if (pup == Powerup.Fertilizer) {
                fertilizerCount.set(toNumber(count))
            } else if (pup == Powerup.Shovel) {
                shovelCount.set(toNumber(count))
            }
        }
    }

    return res
}


export const queryConfig = async () => {

    const tokenOnSecret = toSecretIBCDenom(
        PUBLIC_CONSUMER_TOKEN,
        PUBLIC_SECRET_CHANNEL
    )

}