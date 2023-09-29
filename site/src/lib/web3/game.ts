import { cloverPrice, fertilizerPrice, openPrice, shovelPrice } from "$lib/state"
import { Powerup, type NetworkConfigResult } from "$lib/types"
import { toNumber } from "$lib/utils"

export const parseConfig = (config: NetworkConfigResult) => {

    openPrice.set(config.to_open)
        
    for (const [pup, price] of config.power_ups) {
        if (pup == Powerup.Clover) {
            cloverPrice.set(toNumber(price))
        } else if (pup == Powerup.Fertilizer) {
            fertilizerPrice.set(toNumber(price))
        } else if (pup == Powerup.Shovel) {
            shovelPrice.set(toNumber(price))
        }
    }
    
    console.log("NETWORK CONFIG:", config)

}