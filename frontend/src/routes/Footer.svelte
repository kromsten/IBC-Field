<script>
	import scrt from '$lib/images/scrt-logo.svg';
	import github from '$lib/images/github.svg';
</script>

<footer>
	<div class="corner">
		<a href="/">
			<img src={scrt} alt="Secret" />
		</a>
	</div>

	<div class="corner">
		<a href="https://github.com/sveltejs/kit">
			<img src={github} alt="GitHub" />
		</a>
	</div>
</footer>

<style>
	footer {
		display: flex;
		justify-content: space-between;
		align-items: center;
		padding: 12px;
	}

	footer a {
		font-weight: bold;
	}

	.corner {
		width: 3em;
		height: 3em;
	}

	.corner a {
		display: flex;
		align-items: center;
		justify-content: center;
		width: 100%;
		height: 100%;
	}

	.corner img {
		width: 2em;
		height: 2em;
		object-fit: contain;
	}


	a:hover {
		color: var(--color-theme-1);
	}

	@media (min-width: 480px) {
		footer {
			padding: 12px 0;
		}
	}
</style>