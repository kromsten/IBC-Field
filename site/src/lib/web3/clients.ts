import { SecretNetworkClient } from "secretjs";
import { writable } from "svelte/store";


export const consumerSigningClient = writable<SecretNetworkClient>();
export let consumerSigningClientValue : SecretNetworkClient;


export const secretClient = writable<SecretNetworkClient>();
export let secretClientValue : SecretNetworkClient;


export const getClient = (
    chainId: string,
    url: string,
) : SecretNetworkClient => {

    if (secretClientValue) return secretClientValue;

    const client =  new SecretNetworkClient({
        chainId,
        url,
    });

    secretClient.set(client);
    secretClientValue = client;

    return client;
}

export const getSigningClient = (
    chainId: string,
    url: string,
    encryptionUtils: any,
    walletAddress: string,
    wallet: any
) : SecretNetworkClient => {

    const client = new SecretNetworkClient({
        chainId,
        url,
        encryptionUtils,
        walletAddress,
        wallet
    });

    consumerSigningClient.set(client);
    consumerSigningClientValue = client;

    return client;
}