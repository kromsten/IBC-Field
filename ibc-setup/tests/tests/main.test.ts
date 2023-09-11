import { expect, test, describe, it} from 'vitest';
import * as env from './env';

describe('Init', () => {
    test('Checking environment variables', async () => {
        Object.values(env).forEach((value) => {
            expect(value).not.toBeUndefined();
            expect(typeof value).toBe('string');
            expect(value.length).toBeGreaterThan(0);
        });
        /* await loadContracts();
        expect(Object.keys(config.contracts).length)
        .toBeGreaterThanOrEqual(readdirSync(WASM_PATH).length);
        */
    });
});
