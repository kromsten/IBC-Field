import { expect, test, describe, it} from 'vitest';
import * as env from './env';
import { loadConfig, saveConfig } from './config';
import { consumerClient, secretClient } from './clients';
import { instantiateContract, uploadContract } from './contract';
import { sendIBCToken } from './ibc';
import { toHex, toUtf8 } from 'secretjs'
import { sha256 } from '@noble/hashes/sha256';
import { AllNetworkConfigResult, NetworkConfigResult } from './types';
import { sleep } from './utils';

describe('Env, IBC and Contract setup', () => {

    test('Checking environment variables', async () => {
        Object.values(env).forEach((value) => {
            expect(value).not.toBeUndefined();
            expect(typeof value).toBe('string');
            expect(value.length).toBeGreaterThan(0);
        });
    });

    describe("IBC setup existance", async () => {

        const config = loadConfig();

        test("if channel between two chain does exist", async () => {
            const response = await secretClient.query.ibc_channel.channels({});
            expect(response.channels?.length).toBeGreaterThan(0);
            const channel = response.channels!.find(c => c.state == "STATE_OPEN");
            expect(channel?.channel_id).not.toBeUndefined();

            if (config.ibc_info == undefined) {
                config.ibc_info = {
                    secret_channel: channel!.channel_id!,
                    consumer_channel: channel!.counterparty!.channel_id!
                }
                saveConfig(config);
            }
        })
        

        const response = await secretClient.query.ibc_transfer.denomTraces({});
        
        test("if denom trace exists or creatable", async () => {

            expect(( async () => {
                if (response.denom_traces?.length == 0) {
    
                    const config = loadConfig();
    
                    if (config.ibc_info == undefined) {
                        throw new Error("config.ibc_info is undefined");
                    }

                    await sendIBCToken(
                        consumerClient,
                        secretClient.address,
                        env.CONSUMER_TOKEN,
                        "1",
                        config.ibc_info!.consumer_channel!
                    )

                    await sleep(2000);

                }         
            })()).resolves.not.toThrowError();
        });
        
        it("should now definetely have denom traces", async () => {

            await sleep(6000);

            let length = response.denom_traces?.length ?? 0;
            length |= (await secretClient.query.ibc_transfer.denomTraces({})).denom_traces?.length ?? 0;
            expect(length).toBeGreaterThan(0);

            const first = response.denom_traces![0];
            const ibc_denom = "ibc/" + toHex(sha256(toUtf8(first.path + '/' + first.base_denom))).toUpperCase();

            saveConfig({
                ...config,
                ibc_info: {
                    ...config.ibc_info!,
                    ibc_denom
                }
            })
        })

    })



    describe("Contract deployableness checks", async () => {   
                
        
        test("should now have contract id and hash", async () => {
            let config = loadConfig();
            if (!config.contract_info) {
                await uploadContract(secretClient)
            }
            config = loadConfig();
            expect(config.contract_info).not.toBeUndefined();
            expect(config.contract_info!.code_id).toBeGreaterThan(0);
            expect(config.contract_info!.code_hash. length).toBeGreaterThan(0);
        })

        it("should now have contract address", async () => {
            let config = loadConfig();
            expect(config.contract_info).not.toBeUndefined();
            if (config.contract_info!.contract_address == undefined) {
                await instantiateContract(secretClient)
            }
            config = loadConfig();
            expect(config.contract_info!.contract_address!.length).toBeGreaterThan(0);
        })

    })
    

    describe("Querying network configuration", async () => {   

        
        test("if can query all config", async () => {
            await sleep(3000);
            const config = loadConfig();

            const res : AllNetworkConfigResult = await secretClient.query.compute.queryContract({
                contract_address: config.contract_info?.contract_address!,
                code_hash: config.contract_info?.code_hash!,
                query: {
                    all_network_configs: {}
                }
            })

            expect(res.length).toBe(2);
        })
    

        test("if can query remote chain config", async () => {

            const config = loadConfig();

            const res : NetworkConfigResult = await secretClient.query.compute.queryContract({
                contract_address: config.contract_info?.contract_address!,
                code_hash: config.contract_info?.code_hash!,
                query: {
                    network_config: { denom: config.ibc_info?.ibc_denom! }
                }
            })

            expect(res.channel_id).toBe(config.ibc_info?.secret_channel!);
        })
    })

});
