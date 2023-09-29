<script lang="ts">
  import { onConnectLogic } from "$lib";
    import { accountBalance } from "$lib/state";
    import { formatAddress, toNumber } from "$lib/utils";
    import { address, initWeb3 } from "$lib/web3";
    import Akash from "./graphics/Akash.svelte";
    import Wallet from "./graphics/Wallet.svelte";

    const connect = () => {
        initWeb3()
        .then(client => {
            if (client) {
                onConnectLogic(client)
            }
        })
    }
</script>


<div class="px-4 py-1 flex justify-end">
    { #if $address }
        <div class="flex gap-2 items-center">
            <div class="flex gap-3 items-center">
                <Wallet />
                <div class="flex gap-2 items-center">
                    { toNumber($accountBalance) }
                    <Akash />
                </div>
            </div>
            <button class="btn">{ formatAddress($address) }</button>
        </div>
    {:else}
        <button class="btn" on:click={connect}>Connect</button>
    {/if}
</div>