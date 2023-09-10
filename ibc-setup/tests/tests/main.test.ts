import { expect, test, describe, it} from 'vitest';
import { MNEMONIC } from './env';

describe('Init', () => {
    test('Checking configuration', async () => {

        expect(MNEMONIC.length).toBeGreaterThan(0);
        /* await loadContracts();
        expect(Object.keys(config.contracts).length)
        .toBeGreaterThanOrEqual(readdirSync(WASM_PATH).length);
        */
    });
});
