<script>
  import Akash from "./Akash.svelte";
  import Clover from "./Clover.svelte";
  import Fertilizer from "./Fertilizer.svelte";
  import Shovel from "./Shovel.svelte";


    const cost = 2;

    let loading = false;

    const powerups  = {
        shovel: false,
        clover: false,
        fertilizer: false
    }

    const submit = () => {

        /* if (!loading) {
            loading = true;
            setTimeout(() => {
                loading = false;

                Object.keys(powerups).forEach(key => {
                    powerups[key] = false;
                })

                addNotification({
                    text: 'Ok',
                    position: 'top-center',
                    removeAfter: 2500
                })

            }, 2000);
        }  */
    }


</script>


<div class="bg-red-200">
    <div class="center mb-2">
        <form on:submit|preventDefault={submit}>
            <button type="submit" class="btn btn-outline-success btn-sm">
                <Akash {loading} /> 
                { #if !loading}<span>{ cost }</span>{/if}
            </button>
        </form>
    </div>

    <hr class="dropdown-divider">

    <!-- svelte-ignore a11y-click-events-have-key-events -->
    <div class="center gap-3 items"> 
        <!-- svelte-ignore a11y-click-events-have-key-events -->
        <!-- svelte-ignore a11y-no-static-element-interactions -->
        <span class:active={powerups.shovel} on:click={() => powerups.shovel = !powerups.shovel} type="button">
            <Shovel/>
        </span>
        <button on:click={() => powerups.shovel = !powerups.shovel} type="button">
            <Shovel/>
        </button>
        <button on:click={() => powerups.shovel = !powerups.shovel} type="button">
            <Shovel/>
        </button>
    </div>
    <!-- <ul>
        <ul class="center p-5 gap-3">
            <li class:active={powerups.shovel} on:click={() => powerups.shovel = !powerups.shovel} class="col"></li>
            <li class:active={powerups.clover} on:click={() => powerups.clover = !powerups.clover} class="col"><Clover/></li>
            <li class:active={powerups.fertilizer} on:click={() => powerups.fertilizer = !powerups.fertilizer} class="col"><Fertilizer/></li>
        </ul>
    </ul> -->
</div>




<style>

    .dropdown-menu {
        max-width: 5em;
        border: 1px solid rgb(117, 26, 102);
        background-color: rgb(255, 255, 255);    
    }

    
    ul {
        list-style-type: none;
        padding-inline-start: 0;
        margin-bottom: 0;
    }

    hr {
        color: #402f0fd6;
    }


    button {
        display: flex;
        justify-content: center;
        align-items: center;
    }

    button:focus  {
        background-color: #fff;
        box-shadow: 0 0 0 0.25rem rgb(25 135 84 / 50%);
    }

    button:focus:hover {
        color: #42ae7c;
    }

    .col {
        max-width: 33%;
    }
    
    :global(button:hover:not(:focus) .st0) {
        fill: white;
    }

    :global(#context-drop li svg) {
        max-height: 1.5em;

    }

    :global(#context-drop button > svg) {
        min-height: 1.5em;
        max-height: 2em;
        margin-right: 0.1em;
    }

    :global(.items .active  svg) {
        filter: drop-shadow(0 0 0.35em rgb(90, 2, 90));
    }
    

</style>