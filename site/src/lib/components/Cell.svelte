<script lang="ts">
    import Dug from "./graphics/Dug.svelte";
    import Ground from "./graphics/Ground.svelte";

    export let open : boolean = false;
</script>

<div class="max-w-md">
    { #if open }
        <Dug />
    { :else }
        <Ground />
    { /if }
</div>