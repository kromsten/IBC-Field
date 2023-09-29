<script lang="ts">
    import Nav from "./Nav.svelte";
    import { onMountLogic } from "$lib";
    import { onMount } from "svelte";
    import Account from "./Account.svelte";
    import Status from "./Status.svelte";
    onMount(onMountLogic)
</script>

<header class="grid grid-cols-3 items-center w-full">
    <Status />
    <Nav />
    <Account />
</header>


<style>
    header {
        background: linear-gradient( 14deg, #8ac396 0%, #b78ac3 100%);
        border-bottom: 0.5px solid #618869;
    }
</style>