<script>
    let items = [
        //{ text: "Whop whop"},
        { text: "Dop dop"},

    ]
</script>

<nav class="fixed-bottom">
    { #each items as item, i }
        <div class="rounded text-light mb-1 p-2">{item.text}</div>
    {/each}
</nav>

<style>
    div {
        border: 2px solid #0d4381;
        border-radius: 0.38rem;
        background-color: #89a4c3;
    }

    nav {
        width: 70%;
        margin: 0 auto;
    }

</style>