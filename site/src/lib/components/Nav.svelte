<script lang="ts">
  import { onMount } from "svelte";
  import Fertilizer from "./graphics/Fertilizer.svelte";
  import Shovel from "./graphics/Shovel.svelte";
  import Clover from "./graphics/Clover.svelte";
  import ItemDrop from "./ItemDrop.svelte";
  import { popup, type PopupSettings } from "@skeletonlabs/skeleton";
  import { cloverCount, cloverPrice, cloverSelected, fertilizerCount, fertilizerPrice, fertilizerSelected, shovelCount, shovelPrice, shovelSelected } from "$lib/state";


  let connected = false;
  let address = "123456";


  const connect = () => {

  }

  onMount(async () => {
    connected = localStorage.getItem('connected') == "true";
    if (connected) await connect();
  })



  $: counts  = {
      shovel : {
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
        closeQuery: "",
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


  nav {
    position: sticky;
    top: 0;
    display: flex;
    flex-wrap: wrap;
    padding: 0.3em;
    background: linear-gradient( 14deg, #8ac396 0%, #b78ac3 100%);
    border-bottom: 0.5px solid #618869;
    border-bottom-left-radius: 7%;
    border-bottom-right-radius: 7%;
  }

  li {
    padding: 0.2em;
    display: flex;
    gap: 0.2em;
    max-width: 25%;
    justify-content: center;
  }

  li.selected {
    background-color: #f5f5f5;
    border-radius: 0.5em;
  }

  
  li:hover {
    cursor: pointer;
  }


  nav {
      z-index: 1;
  }

</style>
