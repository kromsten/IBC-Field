import { test, describe} from 'vitest';
import * as env from './env';
import { loadConfig } from './config';
import { consumerClient, secretClient } from './clients';
import { sendIBCToken } from './ibc';
import { sleep } from './utils';

describe('Execute remote contract', () => {

    test("If can trigger contract on Secret using hooks", async () => {

        const config = loadConfig();

        const contract_address = config.contract_info!.contract_address!;

        const permit =  await consumerClient.utils.accessControl.permit.sign(
            consumerClient.address,
            env.CONSUMER_CHAIN_ID,
            "my-permit",
            [contract_address],
            ["owner"],
            false
        )



        const msg = {
            wasm: {
                contract: contract_address!,
                msg: {
                    update_my_random_number: {
                        permit
                    }
                }
            }
        }

        await sendIBCToken(
            consumerClient,
            contract_address,
            env.CONSUMER_TOKEN,
            "2",
            config.ibc_info?.consumer_channel!,
            JSON.stringify(msg)
        )


        await sleep(500);

        try {
            const res = await secretClient.query.compute.queryContract({
                contract_address: config.contract_info!.contract_address!,
                code_hash: config.contract_info!.code_hash,
                query: {
                    get_my_random_number: { permit }
                }
            })
            console.log("query rarndom number:", res)
        } catch (e) {
            console.log("query rn error:", e)
        }

    })

});
