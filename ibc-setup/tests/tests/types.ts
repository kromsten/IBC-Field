export type ContractInfo = {
    code_id: number;
    code_hash: string;
    contract_address?: string;
}

export type IbcInfo = {
    secret_channel: string;
    consumer_channel: string;
    ibc_denom?: string;
}

export type Config = {
    contract_info?: ContractInfo;
    ibc_info?: IbcInfo
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

export type InitMsg = {
    network_configs:[string, NetworkConfig][]
}