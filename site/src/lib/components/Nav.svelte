<script lang="ts">
  import Fertilizer from "./graphics/Fertilizer.svelte";
  import Shovel from "./graphics/Shovel.svelte";
  import Clover from "./graphics/Clover.svelte";
  import ItemDrop from "./ItemDrop.svelte";
  import { popup, type PopupSettings } from "@skeletonlabs/skeleton";
  import { cloverCount, cloverPrice, cloverSelected, fertilizerCount, fertilizerPrice, fertilizerSelected, shovelCount, shovelPrice, shovelSelected } from "$lib/state";
  import { Powerup } from "$lib/types";


  $: counts  = {
      shovel : {
        type: Powerup.Shovel,
        text: "Extra shovel let you ignore the time limit and start digging sooner",
        name: "Shovel",
        icon: Shovel,
        count: $shovelCount,
        price: $shovelPrice,
        selected: $shovelSelected,
        onBuy: (n : number) => {
          shovelCount.update(c => c + n);
        }
      },
      clover : {
        type: Powerup.Clover,
        text: "Increases your luck and the  probability to find something valuable",
        name: "Clover",
        icon: Clover,
        count: $cloverCount,
        price: $cloverPrice,
        selected: $cloverSelected,
        onBuy: (n : number) => {
          cloverCount.update(c => c + n);
        }
      },
      fertilizer : {
        type: Powerup.Fertilizer,
        text: "Speed up the growing process and let you dig a recovering hole earlier",
        name: "Fertilizer",
        count: $fertilizerCount,
        price: $fertilizerPrice,
        selected: $fertilizerSelected,
        icon: Fertilizer,
        onBuy: (n : number) => {
          fertilizerCount.update(c => c + n);
        }
      },
    }

    const itemDropdownPop: PopupSettings = {
        event: 'click',
        target: 'itemDropdown',
        placement: 'bottom',
        closeQuery: "p",
    };

</script>





<nav class="center py-5">

  <div class="center gap-5 gap-x-8">

    { #each Object.entries(counts) as [key, value] }
      
        <li class="center gap-3" class:selected={value.selected} use:popup={{
            ...itemDropdownPop,
            target: `itemDropdowm-${key}`
          }}>
          
          <svelte:component this={value.icon} />
          <span>x</span>
          <span>{value.count}</span>
        </li>

        <div data-popup="itemDropdowm-{key}">
          <ItemDrop {...value} />
        </div>  

    {/each}

  </div>
</nav>


<style>



  li {
    padding: 0.2em;
    display: flex;
    gap: 0.2em;
    max-width: 25%;
    justify-content: center;
  }

  li.selected {
    background-color: #ffeff4;
    border-radius: 0.5em;
  }

  
  li:hover {
    cursor: pointer;
  }


  nav {
      z-index: 1;
  }

</style>
