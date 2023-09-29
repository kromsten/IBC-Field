import type { ToastSettings } from "@skeletonlabs/skeleton";
import { fromUtf8, type IbcResponse, type TxResponse } from "secretjs";
import { writable } from "svelte/store";
import { formatAddress } from "./utils";
import { MsgAcknowledgement } from "secretjs/dist/protobuf/ibc/core/channel/v1/tx";
import { permitStore, permitNonce } from "./state";


const classes = "flex justify-between items-center text-ellipsis border border-black rounded font-bold"


export const notificationStore = writable<Notification>();


export const processSubmitted = async (
    cellId: string
) => {

    
}

export const processLocalTxResSuccess = async (tx: TxResponse, toastStore: any) => {

    const t : ToastSettings = {
        background: "bg-green-50",
        classes,
        message: "Success (Local) TxHash: " + formatAddress(tx.transactionHash),
        action: {
            label: "Copy",
            response: () => {
                window.navigator.clipboard.writeText(tx.transactionHash);
            }
        },
        autohide: false
    }

    toastStore.trigger(t);
    
}

export const processLocalTxResFailure = async (e: any, toastStore: any) => {
    const t : ToastSettings = {
        background: "bg-red-50",
        classes,
        message: "Error with local Tx: " + e.message,
    }
    toastStore.trigger(t);
}


export const processIBCResSuccess = async (res: IbcResponse, toastStore: any) => {

    if (res.type === "timeout") {
        processTimeOut(res.tx.transactionHash, toastStore);
        return;
    }

    const packet = res.tx.tx.body!.messages!.at(1);
    const info = await MsgAcknowledgement.fromJSON(packet);
    const ack = JSON.parse(fromUtf8(info.acknowledgement));


    if (ack.error) {
        const t : ToastSettings = {
            background: "bg-red-50",
            classes,
            message: "Remote IBC chain returned an error" ,
            autohide: false
        }
    
        toastStore.trigger(t);
        return;

    } else if (ack.result) {

        const t : ToastSettings = {
            background: "bg-purple-50",
            classes,
            message: "Success from remote IBC chain",
            autohide: false
        }
        toastStore.trigger(t);
        permitNonce.update(n => n + 1);
        permitStore.set(undefined);
        localStorage.removeItem(`permit`);

    }   
}

export const processRemoteTxResFailure = async (e: any, toastStore: any) => {
    const t : ToastSettings = {
        background: "bg-red-50",
        classes,
        message: "Error with IBC Tx: " + e.message,
    }
    toastStore.trigger(t);
}


export const processTimeOut = async (txHash: string, toastStore: any) => {
    const t : ToastSettings = {
        background: "bg-red-50",
        classes,
        message: "Timeout while waiting for IBC Tx: : " + formatAddress(txHash),
        action: {
            label: "Copy",
            response: () => {
                window.navigator.clipboard.writeText(txHash);
            }
        }
    }
    toastStore.trigger(t);   
}