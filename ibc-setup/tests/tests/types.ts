export type ContractInfo = {
    code_id: number;
    code_hash: string;
    contract_address?: string;
}

export type IbcInfo = {
    secret_channel: string;
    consumer_channel: string;
}

export type Config = {
    contract_info?: ContractInfo;
    ibc_info?: IbcInfo
}


export type InitMsg = {}