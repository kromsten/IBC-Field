<script lang="ts">
    import DigDrop from "./DigDrop.svelte";
	import Cell from "./Cell.svelte";

    import { popup } from '@skeletonlabs/skeleton';
    import type { PopupSettings } from '@skeletonlabs/skeleton';


    const digDropdown: PopupSettings = {
        event: 'click',
        target: 'digDropdown',
        placement: 'bottom',
        closeQuery: ""
    };
</script>


<div data-popup="digDropdown">
    <DigDrop />
</div>



<div class="container-flex p-8">

    <div class="grid grid-cols-xl-2 sm:grid-cols-4 md:grid-cols-6 lg:grid-cols-8 xl:grid-cols-12 gap-7">

        {#each Array(60) as _, i}
            <div use:popup={digDropdown} role="button">
                <Cell open={Math.random() < 0.2}  />
            </div>
        {/each}

    </div>
    
</div>
