import { client } from "./client";
import { detectWallets } from "./connection";

export const initDapp = async () => {
    console.log('initDapp');
    await detectWallets();
    const clientt = client
}
