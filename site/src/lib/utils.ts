import { toHex } from "secretjs";
import { sha256 } from "@noble/hashes/sha256";
import { cloverSelected, fertilizerSelected, shovelSelected } from "./state";

export const sleep = (ms: number) => new Promise(resolve => setTimeout(resolve, ms));

export const formatAddress = (address: string, symbols: number = 4) => 
    address.slice(0, symbols+2) + "..." + address.slice(-symbols)

export const toSecretIBCDenom = (
    denom: string,
    channel: string,
) => {
    const hash = toHex(sha256("transfer/" + channel + "/" + denom))
    return "ibc/" + hash.toUpperCase()
}


export const toNumber = (uAmount: string) => {
    return Number(uAmount) / 1_000_000
}

export const fromNumber = (amount: number) => {
    return String(Math.round(amount * 1_000_000))
}

export const clearSelection = () => {
    shovelSelected.set(false)
    cloverSelected.set(false)
    fertilizerSelected.set(false)
}