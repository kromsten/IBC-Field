<script lang="ts">
    import Akash from "$lib/components/graphics/Akash.svelte";
    //import { getNotificationsContext } from 'svelte-notifications';
    
    export let labelled = "";
    export let text : string, name : string, price : number;
    
    export let count : number;

    //const { addNotification } = getNotificationsContext();
    
    let loading = false;
    
    let toBuy = 1

    const submit = () => {

        if (toBuy == 0) return;
        
        if (!loading) {
            loading = true;
            setTimeout(() => {

                count += toBuy;
                loading = false;

                /* addNotification({
                    position: 'top-center',
                    removeAfter: 4000,
                    type: "purchase",
                    count: toBuy,
                    text : "Check",
                    name : name
                }) */

            }, 5000);
        } 
    }


</script>

<div class="dropdown-menu p-2 pl-5 pr-5" aria-labelledby={labelled}>
    
    <h6>{name}</h6>
    <div class="mb-2 text-muted">{text}</div>
    
    <form on:submit|preventDefault={submit} class="pl-3 pr-3 ">
        
        <div class="form-group">
            <label for="buy-{name}">Buy</label>
            
            <div class="input-group input-group-sm d-flex justify-content-around">
                <input id="buy-{name}" class="form-control col" type="number" min="1" max="99" bind:value={toBuy} disabled={loading}/>
                <button type="submit" class="btn btn-outline-success btn-sm col" >
                    <Akash {loading} /> 
                    { #if !loading}<span>{ toBuy * price }</span>{/if}
                </button>
            </div>
        </div>
    </form>
</div>

<style>

    .dropdown-menu {
        max-width: 10vw;
        font-size: 0.6em;
        border: 1px solid rgb(72 45 18);
        background-color: rgb(255 253 247);
    }

    .input-group {
        gap: 1.0em;
    }

    form input {
        flex-grow: 3;
    }

    form button {
        flex-grow: 2;
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

    button:focus:hover {
        color: #42ae7c;
    }

    :global(button:hover:not(:focus) .st0) {
        fill: white;
    }

    h6 {
        color: rgb(158 127 185);
        font-weight: bolder;
    }


</style>