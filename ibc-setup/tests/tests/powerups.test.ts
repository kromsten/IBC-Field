import { test, describe} from 'vitest';
import { loadConfig } from './config';
import { consumerClient } from './clients';
import { sendIBCToken } from './ibc';
import { getPermit, sleep } from './utils';
import { CONSUMER_CHAIN_ID, CONSUMER_TOKEN } from './env';
import { Powerup } from './types';

describe('Execute remote contract', () => {

    test("If can trigger contract on Secret using hooks", async () => {

        const config = loadConfig();

        const contract_address = config.contract_info!.contract_address!;

        const permit =  await getPermit(
            consumerClient,
            contract_address,
            CONSUMER_CHAIN_ID
        )


        const msg = {
            wasm: {
                contract: contract_address!,
                msg: {
                    buy_powerups: {
                        permit,
                        powerups: [
                            Powerup.Shovel
                        ],
                    }
                }
            }
        }

        await sendIBCToken(
            consumerClient,
            contract_address,
            CONSUMER_TOKEN,
            "3000000",
            config.ibc_info?.consumer_channel!,
            JSON.stringify(msg)
        )


        await sleep(500);

    })

});
