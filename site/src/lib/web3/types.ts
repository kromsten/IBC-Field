export enum WalletType {
    Keplr
}

export type WalletConnector = (chainId: string | string[]) => Promise<boolean>

export type WalletAccount = {
    signer: any,
    address: string,
}