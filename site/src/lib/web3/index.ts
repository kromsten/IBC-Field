import { writable } from "svelte/store";
import { activeWalletValue, connectWallet, getAccount } from "./wallets";
import type { WalletType } from "./types";
import { getSigningClient } from "./clients";

import { 
  PUBLIC_CONSUMER_CHAIN_ENDPOINT, 
  PUBLIC_CONSUMER_CHAIN_ID, 
} from "$env/static/public";
import { queryConfig, getMainPageInfo } from "./contract";

export const connected = writable(false);
export const signer = writable<any>();
export const address = writable<string | null>(null);


export const getEnigmaUtils = async (chainId: string) => {
  const enigmaUtils = window.getEnigmaUtils!(chainId);
  return enigmaUtils;
}



export const initWeb3 = async (chainId?: string | string[], wallet? : WalletType) => {
    chainId ??= PUBLIC_CONSUMER_CHAIN_ID;

    const walletConnected = await connectWallet(chainId)
    connected.set(walletConnected);

    chainId = Array.isArray(chainId) ? chainId[0] : chainId;

    if (walletConnected) {
      const account = await getAccount(chainId, activeWalletValue!);
      if (account) {
        signer.set(account.signer);
        address.set(account.address);

        await getSigningClient(
          chainId,
          PUBLIC_CONSUMER_CHAIN_ENDPOINT,
          await getEnigmaUtils(chainId),
          account.address,
          account.signer
        );

        return true;
      }
    }

    return false;
}




export const setupContractAndListeners = async () => {
  await getMainPageInfo();
  return false;
}