import { test, describe} from 'vitest';
import { loadConfig } from './config';
import { consumerClient, secretClient } from './clients';
import { sendIBCToken } from './ibc';
import { getPermit, sleep } from './utils';
import { CONSUMER_CHAIN_ID, CONSUMER_TOKEN } from './env';
import { NetworkConfigResult } from './types';

describe('Execute remote contract', () => {

    test("If can trigger contract on Secret using hooks", async () => {

        const config = loadConfig();

        const contract_address = config.contract_info!.contract_address!;

        const permit =  await getPermit(
            consumerClient,
            contract_address,
            CONSUMER_CHAIN_ID
        )

        const network_config : NetworkConfigResult = await secretClient.query.compute.queryContract({
            contract_address: config.contract_info?.contract_address!,
            code_hash: config.contract_info?.code_hash!,
            query: {
                network_config: { denom: config.ibc_info?.ibc_denom! }
            }
        })

        console.log("network config:", network_config)

        const msg = {
            wasm: {
                contract: contract_address!,
                msg: {
                    open_cell: {
                        permit,
                        cell_id: 1,
                        powerups: [],
                    }
                }
            }
        }

        await sendIBCToken(
            consumerClient,
            contract_address,
            CONSUMER_TOKEN,
            network_config.to_open,
            config.ibc_info?.consumer_channel!,
            JSON.stringify(msg)
        )


        


        await sleep(500);

    })

});
