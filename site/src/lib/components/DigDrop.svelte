<script lang="ts">
  import { filter } from "@skeletonlabs/skeleton";
    import Akash from "./graphics/Akash.svelte";
    import Clover from "./graphics/Clover.svelte";
    import Fertilizer from "./graphics/Fertilizer.svelte";
    import Shovel from "./graphics/Shovel.svelte";
    import { cloverCount, cloverPrice, cloverSelected, fertilizerCount, fertilizerPrice, fertilizerSelected, openPrice, shovelCount, shovelPrice, shovelSelected } from "$lib/state";



    let loading = false;
    let totalPrice = $openPrice;

    let params = {
        autoBuy: false,
    }


    let powerups : {
        [ key: string ] : { 
            active: boolean, 
            count: number,
            price: number, 
            icon: typeof Shovel,
            toggle: () => void
        }
    }

    $: powerups = {
        shovel: {
            active: false,
            price: $shovelPrice,
            count: $shovelCount,
            icon: Shovel,
            toggle: () => shovelSelected.update(s => !s)
        },
        clover: {
            active: false,
            price: $cloverPrice,
            count: $cloverCount,
            icon: Clover,
            toggle: () => cloverSelected.update(s => !s)
        },
        fertilizer: {
            active: false,
            price: $fertilizerPrice,
            count: $fertilizerCount,
            icon: Fertilizer,
            toggle: () => fertilizerSelected.update(s => !s)
        }
    }

    const activate = (type : string) => {

        let item = powerups[type];

        if (item.count == 0) {
            if (item.active) {
                powerups[type].active = false;
                totalPrice -= item.price
            } else {
                powerups[type].active = true;
                totalPrice += item.price
            }
        } else {
            item.active = !item.active;
        }


        item.toggle();
    }

    const submit = () => {

        if (!loading) {
            loading = true;
            setTimeout(() => {
                loading = false;
                Object.keys(powerups).forEach(key => {
                    powerups[key].active = false;
                })
            }, 2000);
        }
    }


</script>


<div class="bg-purple-50 px-2 border border-1 border-primary-400/50">
    <form on:submit|preventDefault={submit} class="center gap-3 py-2 px-2">
        <span class="text-sm font-bold">Buy with</span>
        <button class="btn variant-filled-primary px-0 logo" disabled={loading}>
            { #if !loading || true }<span>{ totalPrice }</span>{/if}
            <Akash {loading} /> 
        </button>
    </form>

    <hr class="dropdown-divider">
    
    <div class="gap-3 py-2 px-2">
        <div class="center text-xs font-bold mb-2">POWERUPS TO USE</div>
        <div class="center gap-3 items"> 
            { #each Object.entries(powerups) as [key, info] (key) }
                <button class:active={info.active} on:click={() => activate(key)} type="button">
                    <svelte:component this={info.icon} />
                </button>
            {/each}
        </div>
    </div>
</div>




<style>

    hr {
        color: #402f0fd6;
    }


    button.active   {
        background-color: rgb(255 253 247);
        box-shadow: 0 0 0 0.25rem rgb(25 135 84 / 50%);
    }

    button.logo:hover {
        transform: scale(1.2);
    }


    /* button:focus  {
        background-color: #fff;
        box-shadow: 0 0 0 0.25rem rgb(25 135 84 / 50%);
    } */

    /* button:focus:hover {
        color: #c64343;
    } */
 

</style>