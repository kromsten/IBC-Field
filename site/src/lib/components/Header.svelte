<script lang="ts">
    import Nav from "./Nav.svelte";
    import { onMountLogic } from "$lib";
    import { onMount } from "svelte";
    import Account from "./Account.svelte";

    onMount(onMountLogic)
</script>

<header class="flex justify-between items-center w-full">
    <div></div>
    <Nav />
    <Account />
</header>


<style>
    header {
        background: linear-gradient( 14deg, #8ac396 0%, #b78ac3 100%);
        border-bottom: 0.5px solid #618869;
        border-bottom-left-radius: 7%;
        border-bottom-right-radius: 7%;
    }
</style>