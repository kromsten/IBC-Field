
import { readFileSync } from "fs"
import { sha256 } from "@noble/hashes/sha256";
import { toHex, type SecretNetworkClient, MsgStoreCodeParams, TxResultCode, MsgInstantiateContractParams, MsgInstantiateContractResponse } from "secretjs"
import { loadConfig, saveConfig } from "./config";
import { InitMsg, Powerup } from "./types";
import { CONSUMER_CHAIN_ID, SECRET_CHAIN_ID } from "./env";


export const uploadContract = async (
    client: SecretNetworkClient, 
    wasmPath: string = "tests/contract_code/field.wasm"
) => {

    console.log("Uploading contract...");
    
    const wasm_byte_code = readFileSync(wasmPath) as Uint8Array;
    const codeHash = toHex(sha256(wasm_byte_code)); 

    const msg : MsgStoreCodeParams = {
        sender: client.address,
        wasm_byte_code,
        source: "",
        builder: ""
    }

    const tx = await client.tx.compute.storeCode(msg, { gasLimit: 5_000_000 });

    if (tx.code !==  TxResultCode.Success) {
        throw new Error(`Error while uploading contract: ${tx.rawLog}`);
    }

    const codeId = Number(tx.arrayLog!.find(x => x.key === "code_id")!.value);

    const config = loadConfig();

    config.contract_info = {
        code_id: codeId,
        code_hash: codeHash
    }
    saveConfig(config);
}


export const instantiateContract = async (
    client: SecretNetworkClient
) => {
    
    const config = loadConfig();


    const msg : MsgInstantiateContractParams = {
        code_id: config.contract_info!.code_id,
        code_hash: config.contract_info!.code_hash,
        sender: client.address,
        label: `field-${Date.now()}`,
        init_msg: {
            network_configs: [
                [
                    'uscrt', { 
                        to_open: "1000000",
                        to_win: "15000000",
                        power_ups: [
                            [Powerup.Clover, "9000000"],
                            [Powerup.Shovel, "3000000"],
                            [Powerup.Fertilizer, "5000000"],
                        ],
                        chain_id: SECRET_CHAIN_ID

                    }
                ],

                [
                    config.ibc_info?.ibc_denom!, { 
                        to_open: "500000",
                        to_win: "15000000",
                        power_ups: [
                            [Powerup.Clover, "8000000"],
                            [Powerup.Shovel, "2000000"],
                            [Powerup.Fertilizer, "4000000"],
                        ],
                        chain_id: CONSUMER_CHAIN_ID, 
                        channel_id: config.ibc_info?.secret_channel
                    }
                ]
            ]
        } as InitMsg
    }

    const tx = await client.tx.compute.instantiateContract(msg, { gasLimit: 300_000 });

    if (tx.code !==  TxResultCode.Success) {
        throw new Error(`Error while instantiating contract: ${tx.rawLog}`);
    }

    config.contract_info!.contract_address = MsgInstantiateContractResponse.decode(tx.data[0]).address;

    saveConfig(config);
}