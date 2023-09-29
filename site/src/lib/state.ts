import { localStorageStore } from '@skeletonlabs/skeleton';
import type { Permit } from "secretjs";
import { writable, type Writable } from "svelte/store";
import type { GameCell } from "./types";


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



export const permit = writable<Permit>();
export const canOpenAt = writable<Date>(new Date());

export const accountBalance = writable("0");

export const openPrice = localStorageStore('open_price', '1000000');
export const winAmount = localStorageStore('win_price', '100000000');
