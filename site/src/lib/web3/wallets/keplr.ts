import type { WalletAccount, WalletConnector } from "../types";
import type { Window as KeplrWindow } from "@keplr-wallet/types";

declare global {
  interface Window extends KeplrWindow {}
}

export const connectKeplr : WalletConnector = async (chainId : string | string[]) => {
    try {
        await window.keplr!.enable(chainId);
        return true;
      } catch (e : any) {
        console.error(e.message)
        return false;
      }
  }
  
export const getKeplrAccount = async (chainId: string) : Promise<WalletAccount> => {
    const offlineSigner = window.keplr!.getOfflineSigner(chainId);
    const accounts = await offlineSigner.getAccounts();
    return {
        signer: offlineSigner,
        address: accounts[0].address,
    }
}
