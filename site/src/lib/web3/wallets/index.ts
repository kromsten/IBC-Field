import { writable } from "svelte/store";
import { WalletType, type WalletConnector, type WalletAccount } from "../types";
import { connectKeplr, getKeplrAccount } from "./keplr";


export const activeWallet = writable<WalletType | undefined>();
export let activeWalletValue : WalletType | undefined;


export const detectWallet = async () : Promise<WalletType | undefined>  => {
    // default
    return WalletType.Keplr
}


export const connectWallet : WalletConnector = async (chainId: string | string[], wallet? : WalletType) => {
    if (!chainId.length) return false;
    wallet ??= await detectWallet()

    let connected = false;

    if (wallet === WalletType.Keplr) { connected =  await connectKeplr(chainId); }

    const connectedBefore = localStorage.getItem("connected");
    if (connected) {
      activeWallet.set(wallet);
      activeWalletValue = wallet;

      if (connectedBefore !== "true") localStorage.setItem("connected", "true");

    } else {
      if (connectedBefore === "true") localStorage.setItem("connected", "false");
    }

    return connected;
}




export const getAccount = async (chainId: string,  wallet : WalletType) : Promise<WalletAccount | null > => {
  if (wallet === WalletType.Keplr) {
    return await getKeplrAccount(chainId);
  }

  return null;
}