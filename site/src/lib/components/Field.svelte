<script lang="ts">
    import DigDrop from "./DigDrop.svelte";
	import Cell from "./Cell.svelte";

    import { popup } from '@skeletonlabs/skeleton';
    import type { PopupSettings } from '@skeletonlabs/skeleton';
    import { cells } from "$lib/state";
    import { onDestroy } from "svelte";


    const digDropdown: PopupSettings = {
        event: 'click',
        target: 'digDropdown',
        placement: 'bottom',
        closeQuery: ""
    };

    let now = new Date();

    const intervalId = setInterval(() => {
        now = new Date();
    }, 5_000);

    onDestroy(() => {
        clearInterval(intervalId);
    });

</script>


<div data-popup="digDropdown">
    <DigDrop />
</div>



<div class="container-flex p-8">

    <div class="grid grid-cols-xl-2 sm:grid-cols-4 md:grid-cols-6 lg:grid-cols-8 xl:grid-cols-12 gap-7">

        {#each $cells as cell (cell.id)}
            <div use:popup={digDropdown} role="button">
                <Cell open={cell.open_at > now}  />
            </div>
        {/each}

    </div>
    
</div>
