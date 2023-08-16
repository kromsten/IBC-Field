import { writable } from "svelte/store";
import { waitUntil } from "./utils";

export enum SupportedWallets {
    Keplr = "Keplr",
}


export const keplrDetected = () => {
    return Boolean(window.keplr);
}


export const detectWallets = () => {
    waitUntil(keplrDetected)
    .then(() => {
        detectedWallets.update((wallets) => {
            return [...wallets, SupportedWallets.Keplr];
        })
    })
    .catch((err) => {
        console.log(err);
    });
}

export const detectedWallets = writable<SupportedWallets[]>([]);