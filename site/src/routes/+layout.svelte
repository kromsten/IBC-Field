<script>
	import Header from './Header.svelte';
	import Footer from './Footer.svelte';
	import './styles.css';
	import { onMount } from 'svelte';
	import { initDapp } from '$lib/ts';
  	import { detectedWallets } from '$lib/ts/connection';

	onMount(() => {
		initDapp().then(() => {
			console.log('Dapp initialized');
			console.log($detectedWallets);
		});
	})
</script>

<div class="app">
	<Header />

	<main>
		<slot />
	</main>

	<Footer />
</div>

<style>
	.app {
		display: flex;
		flex-direction: column;
		min-height: 100vh;
	}

	main {
		flex: 1;
		display: flex;
		flex-direction: column;
		width: 100%;
		max-width: 64rem;
		margin: 0 auto;
		box-sizing: border-box;
	}
</style>
