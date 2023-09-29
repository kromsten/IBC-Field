<script lang="ts">
    import Akash from "./graphics/Akash.svelte";
    import Clover from "./graphics/Clover.svelte";
    import Fertilizer from "./graphics/Fertilizer.svelte";
    import Shovel from "./graphics/Shovel.svelte";
    import { cells, cloverCount, cloverPrice, cloverSelected, fertilizerCount, fertilizerPrice, fertilizerSelected, openPrice, permitStore, shovelCount, shovelPrice, shovelSelected } from "$lib/state";
    import { openCell } from "$lib/web3/contract";
    import { consumerSigningClient, secretClient } from "$lib/web3/clients";
    import { clearSelection, fromNumber, sleep, toNumber } from "$lib/utils";
    import { getPermit } from "$lib/web3";
    import type { Permit } from "secretjs";
    import { Powerup, type GameCell } from "$lib/types";
    import { getModalStore, getToastStore, type ModalComponent, type ModalSettings } from "@skeletonlabs/skeleton";
    import { getOpenCellLastInteraction } from "$lib/web3/events";
    import OpenedModal from "./OpenedModal.svelte";
    import { onMount } from "svelte";
    import { onSuccessfulIBCInteraction } from "$lib";

    const modalStore = getModalStore();
    const toastStore = getToastStore();

    export let currentId : number

    $: if (currentId) {
        cell = $cells.find(c => c.id == currentId)!;
        console.log("cell:", cell);
        canOpen = cell.open_at <= new Date();
    }

    let cell : GameCell;
    let canOpen : boolean;

    let loading = false;
    let totalPrice = toNumber($openPrice);



    const modal : ModalSettings = {
        type: "component",
        component: { ref: OpenedModal },
        meta: {}
    }


    let powerups : {
        [ key: string ] : { 
            active: boolean, 
            count: number,
            price: number, 
            type: Powerup
            icon: typeof Shovel,
            toggle: () => void
        }
    }

    $: powerups = {
        shovel: {
            active: false,
            price: $shovelPrice,
            count: $shovelCount,
            type: Powerup.Shovel,
            icon: Shovel,
            toggle: () => shovelSelected.update(s => !s)
        },
        clover: {
            active: false,
            price: $cloverPrice,
            count: $cloverCount,
            type: Powerup.Clover,
            icon: Clover,
            toggle: () => cloverSelected.update(s => !s)
        },
        fertilizer: {
            active: false,
            price: $fertilizerPrice,
            count: $fertilizerCount,
            type: Powerup.Fertilizer,
            icon: Fertilizer,
            toggle: () => fertilizerSelected.update(s => !s)
        }
    }

    $: selectedPowerups = Object.values(powerups).filter(pup => pup.active)
    $: autopay = selectedPowerups.filter(pup => pup.count == 0).length > 0
    $: selectedPowerupTypes = selectedPowerups.map(pup => pup.type)

    const clear = () => {
        Object.keys(powerups).forEach(key => {
            powerups[key].active = false;
        })
        clearSelection();
        loading = false;

        $fertilizerSelected = false;
        $shovelSelected = false;
        $cloverSelected = false;
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
            powerups[type].active = !item.active;
        }

        item.toggle();
    }

    const submit = async () => {

        if (!loading) {

            loading = true;

            const client = $consumerSigningClient;
            const permitValue : Permit | undefined = $permitStore ?? await getPermit($consumerSigningClient);


            if (permitValue == undefined) {
                clear();
                console.error("No permit");
                return;
            }

            openCell(
                client,
                currentId,
                permitValue,
                selectedPowerupTypes,
                fromNumber(totalPrice),
                autopay,
                toastStore
            )
            .then(() => getOpenCellLastInteraction(
                $secretClient,
                client.address,
                currentId.toString(),
            ))
            .then(last => {
                const logs = last!.arrayLog!;

                const nextOpenCell = logs.find(log => log.key == "can_open_cell_at")?.value ?? "0";


                cells.update((cells : GameCell[]) => {
                    return cells.map((c : GameCell) => {

                        if (c.id == currentId) {
                            c.open_at = new Date(Number(nextOpenCell) * 1000);
                        }
                        return c;
                    })
                })

                modalStore.trigger({
                    ...modal,
                    meta: {
                        cell,
                        logs
                    }
                });

                selectedPowerups.forEach(pup => {
                    if (pup.type == Powerup.Clover) {
                        $cloverCount -= 1;
                    } else if (pup.type == Powerup.Fertilizer) {
                        $fertilizerCount -= 1;
                    } else if (pup.type == Powerup.Shovel) {
                        $shovelCount -= 1;
                    }
                })

                onSuccessfulIBCInteraction(client)
            })
            .finally(clear)
        }
    }

    onMount(async () => {
        /* await sleep(2000);
        modalStore.trigger({
            ...modal,
            meta: {
                cell
            }
        }); */
    })

</script>


<div class="bg-purple-50 px-2 border border-1 border-primary-400/50">

    { #if cell && !canOpen }
        <div class="flex flex-col items-center justify-center py-2">
            <span class="font-bold">Can open next at</span>
            <span>{ cell.open_at.toLocaleString() }</span>
        </div>
         <hr class="dropdown-divider">

    { /if }

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

</style>