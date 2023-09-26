import { writable } from "svelte/store";
import { activeWalletValue, connectWallet, getAccount } from "./wallets";
import type { WalletType } from "./types";
import { getSigningClient } from "./clients";

import { 
  PUBLIC_CONSUMER_CHAIN_ENDPOINT, 
  PUBLIC_CONSUMER_CHAIN_ID,
  PUBLIC_CONTRACT_ADDRESS, 
} from "$env/static/public";
import { getMainPageInfo } from "./contract";
import type { Permit, SecretNetworkClient } from "secretjs";

export const connected = writable(false);
export const signer = writable<any>();
export const address = writable<string | null>(null);


export const getEnigmaUtils = async (chainId: string) => {
  const enigmaUtils = window.getEnigmaUtils!(chainId);
  return enigmaUtils;
}



export const getPermit = async (client?: SecretNetworkClient) : Promise<Permit | undefined> => {
  let permit : Permit | undefined = undefined;
  const localPermit = localStorage.getItem(`permit`);

  if (localPermit) {
    permit = JSON.parse(localPermit);
  } else if (client) {
    permit = await client.utils.accessControl.permit.sign(
      client.address,
      PUBLIC_CONSUMER_CHAIN_ID,
      "test-permit",
      [PUBLIC_CONTRACT_ADDRESS],
      ["owner"],
      true
    )
    localStorage.setItem(`permit`, JSON.stringify(permit));
  } else {
    //throw new Error(`Can't get Permit`);
  }

  return permit;
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

  const permit = await getPermit();

  await getMainPageInfo(permit);
  return false;
}