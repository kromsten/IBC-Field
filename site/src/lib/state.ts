import type { Permit } from "secretjs";
import { writable } from "svelte/store";
import type { GameCell } from "./types";

export const chainId = writable("");
export const token = writable("");

export const cells = writable<GameCell[]>([]);

export const walletConnected = writable(false);
export const walletAddress = writable("");

export const shovelCount = writable(0);
export const shovelPrice = writable(10);
export const shovelSelected = writable(false);

export const cloverCount = writable(0);
export const cloverPrice = writable(30);
export const cloverSelected = writable(false);

export const fertilizerCount = writable(0);
export const fertilizerPrice = writable(20);
export const fertilizerSelected = writable(false);


export const openPrice = writable(100);
export const winAmount = writable(10_000);

export const permit = writable<Permit>();