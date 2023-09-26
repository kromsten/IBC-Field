import { getLocalSecretChainInfo } from "../misc/localChainInfo";
import type { WalletAccount, WalletConnector } from "../types";
import type { Window as KeplrWindow } from "@keplr-wallet/types";

declare global {
  interface Window extends KeplrWindow {}
}


export const suggestKeplr = async (chainId : string | string[]) => {
  chainId = Array.isArray(chainId) ? chainId[0] : chainId;
  await window.keplr!.experimentalSuggestChain(getLocalSecretChainInfo(chainId))
} 


export const connectKeplr : WalletConnector = async (chainId : string | string[]) => {
    
    try {
      await suggestKeplr(chainId);
    } catch (e : any) {
      console.error(e.message)
      return false;
    }
  
  
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
