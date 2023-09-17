<script lang="ts">
    import Akash from "$lib/components/graphics/Akash.svelte";
    export let 
        text : string, 
        name : string, 
        price : number,
        onBuy : (n : number) => void;
    

    let loading = false;
    let toBuy = 1

    const submit = () => {
        if (toBuy == 0) return;
        if (!loading) {
            loading = true;
            setTimeout(() => {
                onBuy(toBuy);
                loading = false;
            }, 1000);
        } 
    }
</script>



<div class="center max-w-min flex-col text-center gap-2 bg-purple-50 px-2 py-3 md:py-1 border border-2 rounded border-primary-400/50 max-w-xs">
    <h6>{name}</h6>
    <div class="text-muted text-xs">{text}</div>
    <form on:submit|preventDefault={submit} class="center gap-3 px-2">
        <div class="flex gap-2 items-center">
            <span class="text-sm">Buy</span>
            <input class="center" placeholder="1" title="1" type="number" min="1" max="10" bind:value={toBuy}>
            <span>with</span>
            <button class="mx-1 px-2 btn variant-filled-primary px-0" disabled={loading}>
                { #if !loading || true }<span>{ toBuy * price }</span>{/if}
                <Akash {loading} /> 
            </button>
        </div>
    </form>
</div> 



<style>


    input {
        width: 1rem;
        text-align: center;
        background-color: transparent;
        border: none;
        border-bottom: 2px solid #c5a3c8;
    }

    input::-webkit-inner-spin-button {
        -webkit-appearance: none;
        margin: 0;
    }

    input:focus {
        border-color: #c5a3c8;
        box-shadow: 0 0 0 0.25rem rgb(156 69 153 / 25%);
    }

    button {
        display: flex;
        justify-content: space-evenly;
        align-items: center;
    }

    button:focus  {
        background-color: rgb(255 253 247);
        box-shadow: 0 0 0 0.25rem rgb(25 135 84 / 50%);
    }

    button:hover {
        color: #42ae7c;
        transform: scale(1.1);
    }

    h6 {
        color: rgb(158 127 185);
        font-weight: bolder;
    }


</style>