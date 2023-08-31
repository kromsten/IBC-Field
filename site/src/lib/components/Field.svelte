<script lang="ts">
    import DigDrop from "./graphics/DigDrop.svelte";
	import Cell from "./Cell.svelte";

    import { popup } from '@skeletonlabs/skeleton';
    import type { PopupSettings } from '@skeletonlabs/skeleton';


    const popupFeatured: PopupSettings = {
        event: 'click',
        target: 'popupFeatured',
        placement: 'bottom',
        closeQuery: ""
    };
</script>


<div data-popup="popupFeatured">
    <DigDrop />
</div>



<div class="container-flex p-8">

    <div class="grid grid-cols-xl-2 sm:grid-cols-4 md:grid-cols-6 lg:grid-cols-8 xl:grid-cols-12 gap-7">

        {#each Array(60) as _, i}
            <div use:popup={popupFeatured} role="button">
                <Cell open={Math.random() < 0.2}  />
            </div>
        {/each}

    </div>
    
</div>
