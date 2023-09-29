export type Cell = {
    open_at: number;
}

export type GameCell = {
    id: number;
    open_at: Date;
}

export type Notification = {
    status: "success" | "error" | "timeout";
    message: string;
    showed: boolean;
}


export enum Powerup {
    Clover = "clover",
    Shovel = "shovel",
    Fertilizer = "fertilizer",
}

export type NetworkConfig = {
    chain_id: string,
    channel_id?: string,
    hrp?: string,
    to_win: string,
    to_open: string,
    power_ups: [Powerup, string][]
};

export type NetworkConfigResult = NetworkConfig;

export type AllNetworkConfigResult = [string, NetworkConfig][];


export type MainResult = {
    network_configs: AllNetworkConfigResult,
    cells: Cell[],
    powerups?: [Powerup, number][]
};