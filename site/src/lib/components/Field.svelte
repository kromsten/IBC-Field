<script lang="ts">
    import DigDrop from "./DigDrop.svelte";
	import Cell from "./Cell.svelte";

    import { popup } from '@skeletonlabs/skeleton';
    import type { PopupSettings } from '@skeletonlabs/skeleton';
    import { cells } from "$lib/state";
    import { onDestroy, onMount } from "svelte";

    let currentId: number;

    const digDropdown: PopupSettings = {
        event: 'click',
        target: 'digDropdown',
        placement: 'bottom',
        closeQuery: "p"
    };

    let now = new Date();

    const intervalId = setInterval(() => {
        now = new Date();
    }, 5_000);

    onMount(() => {
        if ($cells.length) [
            currentId = $cells[0].id
        ]
    })

    onDestroy(() => {
        clearInterval(intervalId);
    });

</script>


<div data-popup="digDropdown">
    <DigDrop {currentId} />
</div>



<div class="container-flex p-8">
    <div class="grid grid-cols-xl-2 sm:grid-cols-4 md:grid-cols-6 lg:grid-cols-8 xl:grid-cols-12 gap-7">

        {#each $cells as cell (cell.id)}
            <!-- svelte-ignore a11y-interactive-supports-focus -->
            <!-- svelte-ignore a11y-click-events-have-key-events -->
            <div use:popup={digDropdown} role="button" on:click={() => currentId = cell.id }>
                <Cell open={cell.open_at > now}  />
            </div>
        {/each}

    </div>
</div>


<style>
    div.container-flex {
        background-image: url("/background.jpg");
    }
</style>