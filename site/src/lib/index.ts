import type { SecretNetworkClient } from "secretjs";
import { getBalance, getPermit, initWeb3, setupContractAndListeners } from "./web3";
import { getMainPageInfo } from "./web3/contract";
import { getOpenCellLastInteraction } from "./web3/events";
import { secretClientValue } from "./web3/clients";
import { canOpenAt } from "./state";


export const onSuccessfulIBCInteraction = async (client: SecretNetworkClient) => {
    await getBalance(client);

    getOpenCellLastInteraction(
        secretClientValue, 
        client.address
    )
    .then((last) => {
        if (last) {
            const logs = last?.arrayLog!;
            const canOpenNext = logs.find(log => 
                log.type == "wasm-cell-opening" &&
                log.key == "can_open_next_at"
            )!.value;
            canOpenAt.set(new Date(Number(canOpenNext) * 1000));
        }
    })
}


export const onConnectLogic = async (client: SecretNetworkClient) => {
    await getBalance(client);

    const permit = await getPermit(client);
    
    if (permit) {
        await getMainPageInfo(permit)
        getOpenCellLastInteraction(
            secretClientValue, 
            client.address
        )
        .then((last) => {
            
            if (last) {

                const logs = last?.arrayLog!;
                const canOpenNext = logs.find(log => 
                    log.type == "wasm-cell-opening" &&
                    log.key == "can_open_next_at"
                )!.value;

                canOpenAt.set(new Date(Number(canOpenNext) * 1000));
            }
        })
    }

}


// place files you want to import through the `$lib` alias in this folder.
export const onMountLogic = async () => {
    const connectedBefore = localStorage.getItem('connected') == "true";
    if (connectedBefore) {
        const client = await initWeb3();
        if (client) {
            await onConnectLogic(client);
        }
    }

    await setupContractAndListeners();
}