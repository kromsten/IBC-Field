<script lang="ts">
  
  import MenuDrop from "$lib/components/MenuDrop.svelte";
  
  import { fly } from 'svelte/transition';

  import { page } from '$app/stores';

  import { onMount } from "svelte";
  import Fertilizer from "./graphics/Fertilizer.svelte";
  import Shovel from "./graphics/Shovel.svelte";
  import Clover from "./graphics/Clover.svelte";
  import ItemDrop from "./ItemDrop.svelte";
  import { popup, type PopupSettings } from "@skeletonlabs/skeleton";

  

  let connected = false;
  let address = "123456";


  const connect = () => {

  }

  onMount(async () => {
    connected = localStorage.getItem('connected') == "true";
    if (connected) connect();
  })

  type CountValue = {
    count: number,
    price: number,
    text: string,
    name: string
  }

  const counts : {[key: string] : CountValue} = {
      plus : {
        count: 0,
        price: 5,
        text: "Extra shovel let you ignore the time limit and start digging sooner",
        name: "Shovel"
      },
      ignore : {
        count: 0,
        price: 10,
        text: "Increases your luck and the  probability to find something valuable",
        name: "Clover"
      },
      fert : {
        count: 0,
        price: 8,
        text: "Speed up the growing process and let you dig a recovering hole earlier",
        name: "Fertilizer"
      },
    }

    const itemDropdownPop: PopupSettings = {
        event: 'click',
        target: 'itemDropdowm',
        placement: 'bottom',
        closeQuery: "",
    };

</script>





<nav class="center py-5">

  <div class="center gap-5 gap-x-8">
    { #each Object.entries(counts) as [key, value] }
      
        <li id="dropdownMenuButton-{key}" class="center gap-3" use:popup={itemDropdownPop}>
          
          { #if key == "fert"}
            <Fertilizer />
          { :else if key == "plus"}
            <Shovel/>
          {:else}
            <Clover/>
          {/if}
          <span>x</span>
          <span>{value.count}</span>
        </li>

        <div data-popup="itemDropdowm">
          <ItemDrop count={value.count} labelled={"dropdownMenuButton-" + key } {...value} />
        </div>
      
      
          {/each}
  </div>
</nav>


<style>

  .item-container {
    flex-grow: 1;
    justify-content: space-around;
    display: flex;
    list-style-type: none;
    padding-inline-start: 0;
    margin-bottom: 0;
    height: 2em;
  }

  .connector {
    margin: 0 0.2em;
    padding: 0.2em 0.45em;
    display: flex;
    align-items: center;
  }

  .connector:hover {
    cursor: pointer;
    transform: scale(1.1);
  }

  .account {
    filter: drop-shadow(0 0 0.1em #888);
  }

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

  
  li:hover {
    cursor: pointer;
  }

  .nothome {
    padding-inline-start: 5%;
    justify-content: start;
  }

  :global(li > svg) {
    height: 100%;
  }

  :global(li > svg:hover) {
    transform: scale(1.1);
    filter: drop-shadow(0 0 0.1em #888);
  }


  nav {
      z-index: 1;
  }

  a {
    color: black;
    filter: drop-shadow(0 0 0.1em #888);
    text-decoration: none;
  }

  a:hover {
    cursor: pointer;
    transform: scale(1.1);
  }

</style>
