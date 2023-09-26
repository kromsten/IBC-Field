<script lang="ts">
    import { formatAddress } from "$lib/utils";
    import { address, initWeb3 } from "$lib/web3";
</script>


<div class="px-4 py-1">
    { #if $address }
        <button class="btn">{ formatAddress($address) }</button>
    {:else}
        <button class="btn" on:click={() => initWeb3()}>Connect</button>
    {/if}
</div>