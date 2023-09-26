
export const sleep = (ms: number) => new Promise(resolve => setTimeout(resolve, ms));

export const formatAddress = (address: string, symbols: number = 4) => 
    address.slice(0, symbols+2) + "..." + address.slice(-symbols)