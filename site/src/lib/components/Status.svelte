<script lang="ts">
  import { canOpenAt, winAmount } from "$lib/state";
  import { toNumber } from "$lib/utils";
  import Trophy from "./graphics/Trophy.svelte";
  import Clock from "./graphics/Clock.svelte";
  import Akash from "./graphics/Akash.svelte";
  import { popup, type PopupSettings } from "@skeletonlabs/skeleton";

  const popupHover: PopupSettings = {
	    event: 'hover',
	    target: 'prize',
	    placement: 'top'
    };

</script>


<div class="bg-purple-200 p-1 rounded" data-popup="prize">
	<p>Prize amount</p>
	<div class="arrow variant-filled-secondary" />
</div>

<div class="bg-purple-200 p-1 rounded" data-popup="time">
	<p>Can open next at</p>
	<div class="arrow variant-filled-secondary" />
</div>


<div class="ps-3 flex gap-7">
    <div class="flex gap-2" use:popup={popupHover}>
        <Trophy />
        <span class="flex gap-2"> {toNumber($winAmount)} <Akash /></span>
    </div>

    { #if $canOpenAt && $canOpenAt > new Date() }
        <div class="flex gap-2" use:popup={{...popupHover, target: "time"}}>
            <Clock />
            { $canOpenAt.toLocaleString()  }
        </div>
    {/if}
</div>