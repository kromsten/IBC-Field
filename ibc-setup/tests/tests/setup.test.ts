import { expect, test, describe, it} from 'vitest';
import * as env from './env';
import { loadConfig, saveConfig } from './config';
import { consumerClient, consumerWallet, secretClient } from './clients';
import { instantiateContract, uploadContract } from './contract';
import { sendIBCToken } from './ibc';
import { sleep } from './utils';

describe('Env and Contract setup', () => {

    test('Checking environment variables', async () => {
        Object.values(env).forEach((value) => {
            expect(value).not.toBeUndefined();
            expect(typeof value).toBe('string');
            expect(value.length).toBeGreaterThan(0);
        });
    });

    test("if contract config exist or its uploadable", async () => {
        const config = loadConfig();

        expect((async () => {
            if (config.contract_info == undefined) {
                return await uploadContract(secretClient)
            }
        })()).resolves.not.toThrowError();

        it("should now have contract id and hash", () => {
            expect(config.contract_info).not.toBeUndefined();
            expect(config.contract_info!.code_id).toBeGreaterThan(0);

            expect(config.contract_info!.code_hash. length).toBeGreaterThan(0);
        })

        expect((async () => {
            if (config.contract_info!.contract_address == undefined) {
                await instantiateContract(secretClient)
            }
        })()).resolves.not.toThrowError();

        it("should now have contract address", () => {
            expect(config.contract_info!.contract_address!.length).toBeGreaterThan(0);
        })
    })


    describe("IBC setup existance", async () => {

        const config = loadConfig();

        test("if channel between two chain does exist", async () => {
            const response = await secretClient.query.ibc_channel.channels({});
            expect(response.channels?.length).toBeGreaterThan(0);
            const channel = response.channels!.find(c => c.state == "STATE_OPEN");
            expect(channel?.channel_id).not.toBeUndefined();
            config.ibc_info = {
                secret_channel: channel!.channel_id!,
                consumer_channel: channel!.counterparty!.channel_id!
            }
            saveConfig(config);
        })
        

        test("if denom trace exists or creatable", async () => {

            let response = await secretClient.query.ibc_transfer.denomTraces({});

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
                        config.ibc_info?.consumer_channel!
                    )
                }         
            })()).resolves.not.toThrowError();
        
    
            it("should now definetely have denom traces", async () => {
                let length = response.denom_traces?.length ?? 0;
                length |= (await secretClient.query.ibc_transfer.denomTraces({})).denom_traces?.length ?? 0;
                expect(length).toBeGreaterThan(0);
            })
        });


    })

});
