import { initWeb3, setupContractAndListeners } from "./web3";

// place files you want to import through the `$lib` alias in this folder.
export const onMountLogic = async () => {
    const connectedBefore = localStorage.getItem('connected') == "true";
    if (connectedBefore) await initWeb3();

    await setupContractAndListeners();
}