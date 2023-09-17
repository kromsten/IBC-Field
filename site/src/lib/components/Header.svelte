<script lang="ts">
    import { onMount } from "svelte";
    import { connected } from "$lib/web3";


    const connect = () => {}


    onMount(async () => {
        const connectedBefore = localStorage.getItem('connected') == "true";
        if (connected) await connect();
    })


    import Nav from "./Nav.svelte";
</script>

<Nav />