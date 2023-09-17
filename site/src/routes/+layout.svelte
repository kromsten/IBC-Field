<script lang="ts">
	import '../app.postcss';
	import { AppBar, AppShell } from '@skeletonlabs/skeleton';
	import Nav from '$lib/components/Nav.svelte';

	// Floating UI for Popups
	import { computePosition, autoUpdate, flip, shift, offset, arrow } from '@floating-ui/dom';
	import { storePopup } from '@skeletonlabs/skeleton';
  import Header from '$lib/components/Header.svelte';



	storePopup.set({ computePosition, autoUpdate, flip, shift, offset, arrow });
</script>

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
	