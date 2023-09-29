import { PUBLIC_CONSUMER_CHANNEL, PUBLIC_CONSUMER_TOKEN, PUBLIC_CONTRACT_ADDRESS, PUBLIC_CONTRACT_CODE_HASH, PUBLIC_SECRET_CHAIN_ENDPOINT, PUBLIC_SECRET_CHAIN_ID, PUBLIC_SECRET_CHANNEL } from "$env/static/public";
import { coinFromString, type IbcResponse, type Permit, type SecretNetworkClient, type TxResponse, type TxSender } from "secretjs";
import { getClient } from "./clients";

import { toNumber, toSecretIBCDenom } from "$lib/utils";
import { cells, cloverCount, fertilizerCount, shovelCount } from "$lib/state";
import { parseConfig } from "./game";
import { Powerup, type Cell, type MainResult } from "$lib/types";
import { processLocalTxResFailure, processLocalTxResSuccess, processRemoteTxResFailure, processIBCResSuccess } from "$lib/middleware";


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
    powerups: Powerup[],
    amount: string,
    autobuy: boolean,
    toastStore: any
) => {

    const contractMsg : any = {
        open_cell: {
            cell_id,
            powerups,
            permit
        }
    }

    if (autobuy) {
        contractMsg.open_cell["power_up_autopay"] = true
    }


    let res : TxResponse
    try {
        res =  await executeOverIBC(
            client,
            contractMsg,
            amount
        )
        console.log("OPEN CELL RES:", res)

    } catch (e) {
        console.error("OPEN CELL ERROR:", e)
        processLocalTxResFailure(e, toastStore);
        return;
    }

    processLocalTxResSuccess(res, toastStore);

    let ibcRes : IbcResponse
    try {
        ibcRes = await res.ibcResponses[0];
        console.log("OPEN CELL IBC RES:", ibcRes)
    } catch (e) {
        console.error("OPEN CELL IBC ERROR:", e)
        processRemoteTxResFailure(e, toastStore);
        return;
    }

    processIBCResSuccess(ibcRes, toastStore);

    return ibcRes;
}


export const buyPowerup = async (
    client: SecretNetworkClient,
    permit: Permit,
    powerups: Powerup[],
    amount: string,
    toastStore: any
) => {

    const contractMsg = {
        buy_powerups: {
            powerups,
            permit
        }
    }


    let res : TxResponse
    try {
        res =  await executeOverIBC(
            client,
            contractMsg,
            amount
        )
        console.log("BUY POWERUP RES:", res)

    } catch (e) {
        console.error("BUY POWERUP ERROR:", e)
        processLocalTxResFailure(e, toastStore);
        return;
    }

    processLocalTxResSuccess(res, toastStore);

    let ibcRes : IbcResponse
    try {
        ibcRes = await res.ibcResponses[0];
        console.log("BUY POWERUP IBC RES:", ibcRes)
    } catch (e) {
        console.error("BUY POWERUP IBC ERROR:", e)
        processRemoteTxResFailure(e, toastStore);
        return;
    }

    processIBCResSuccess(ibcRes, toastStore);

    return ibcRes;
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
    
    let res : MainResult = await queryContract({
        main: {
            permit
        }
    })

    if (typeof res === "string" && (res as string).includes("error")) {
        res = await queryContract({ main: {} })
    }

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
                cloverCount.set(count)
            } else if (pup == Powerup.Fertilizer) {
                fertilizerCount.set(count)
            } else if (pup == Powerup.Shovel) {
                shovelCount.set(count)
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