<script lang="ts">
	import '../app.postcss';
	import { AppBar, AppShell, Toast } from '@skeletonlabs/skeleton';

	// Floating UI for Popups
	import { computePosition, autoUpdate, flip, shift, offset, arrow } from '@floating-ui/dom';
	import { storePopup } from '@skeletonlabs/skeleton';

	import { Modal } from '@skeletonlabs/skeleton';

	import { initializeStores } from '@skeletonlabs/skeleton';
	
	import Header from '$lib/components/Header.svelte';
	import { browser } from '$app/environment';
	import { cloverCount, fertilizerCount, shovelCount } from '$lib/state';
	import { onMount } from 'svelte';

	
	initializeStores();
	storePopup.set({ computePosition, autoUpdate, flip, shift, offset, arrow });

	if (browser) {
		cloverCount.subscribe((val) => localStorage.setItem("cloverCount", val.toString()));
		shovelCount.subscribe((val) => localStorage.setItem("shovelCount", val.toString()));
		fertilizerCount.subscribe((val) => localStorage.setItem("fertilizerCount", val.toString()));
	}

	onMount(() => {
		cloverCount.set(parseInt(localStorage.getItem("cloverCount") || "0"));
		shovelCount.set(parseInt(localStorage.getItem("shovelCount") || "0"));
		fertilizerCount.set(parseInt(localStorage.getItem("fertilizerCount") || "0"));
	})


</script>

<Modal />
<Toast position="t" />

<!-- App Shell -->
<AppShell>
	<svelte:fragment slot="header">
		<AppBar padding="p-0" background="p-0" gridColumns="0" gap="gx-3">
			<svelte:fragment slot="lead">
				<strong class="text-xl uppercase"></strong>
			</svelte:fragment>
			<svelte:fragment slot="default">
				<Header />
			</svelte:fragment>
		</AppBar>
	</svelte:fragment>
	<!-- Page Route Content -->
	<slot />
</AppShell>
	