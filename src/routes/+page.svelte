<script lang="ts">
	import Calculator from '$lib/Calculator.svelte';
</script>

<svelte:head>
	<title>Calcutron - Кроссплатформенный калькулятор</title>
	<meta name="description" content="Современный калькулятор на Tauri и Svelte" />
</svelte:head>

<main class="main-container">
	<div class="content-wrapper">
		<Calculator />
	</div>
</main>

<style>
	.main-container {
		min-height: 100vh;
		height: 100vh;
		display: flex;
		align-items: stretch;
		justify-content: center;
		padding: 0;
		background: linear-gradient(135deg, #667eea 0%, #764ba2 100%);
		position: relative;
		overflow: hidden;
	}

	:global(.dark) .main-container {
		background: linear-gradient(135deg, #1a1a2e 0%, #16213e 50%, #0f3460 100%);
	}

	.main-container::before {
		content: '';
		position: absolute;
		top: 0;
		left: 0;
		right: 0;
		bottom: 0;
		background:
			radial-gradient(circle at 20% 80%, rgba(120, 119, 198, 0.3) 0%, transparent 50%),
			radial-gradient(circle at 80% 20%, rgba(255, 119, 198, 0.3) 0%, transparent 50%),
			radial-gradient(circle at 40% 40%, rgba(120, 219, 255, 0.2) 0%, transparent 50%);
		pointer-events: none;
	}

	.content-wrapper {
		position: relative;
		z-index: 1;
		display: flex;
		flex-direction: column;
		align-items: center;
		gap: 1rem;
		width: 100%;
		height: 100%;
		max-width: none;
	}



	@media (max-width: 768px) {
		.main-container {
			padding: 0;
		}
	}

	@media (max-width: 480px) {
		.content-wrapper {
			gap: 1rem;
		}
	}
</style>
