import { detectWallets } from "./connection";

export const initDapp = async () => {
    console.log('initDapp');
    await detectWallets();
}