<script lang="ts">
	import Calculator from '$lib/Calculator.svelte';
</script>

<svelte:head>
	<title>Calcutron - Кроссплатформенный калькулятор</title>
	<meta name="description" content="Современный калькулятор на Tauri и Svelte" />
</svelte:head>

<main class="main-container">
	<div class="content-wrapper">
		<div class="app-header">
			<h1 class="app-title">Calcutron</h1>
			<p class="app-subtitle">Современный кроссплатформенный калькулятор</p>
		</div>

		<Calculator />

		<div class="help-section">
			<details class="keyboard-shortcuts">
				<summary>⌨️ Горячие клавиши</summary>
				<div class="shortcuts-grid">
					<div class="shortcut-group">
						<h4>Основные операции</h4>
						<div class="shortcut-item">
							<kbd>0-9</kbd> <span>Цифры</span>
						</div>
						<div class="shortcut-item">
							<kbd>+ - * /</kbd> <span>Арифметические операции</span>
						</div>
						<div class="shortcut-item">
							<kbd>Enter</kbd> или <kbd>=</kbd> <span>Равно</span>
						</div>
						<div class="shortcut-item">
							<kbd>.</kbd> <span>Десятичная точка</span>
						</div>
					</div>

					<div class="shortcut-group">
						<h4>Дополнительные функции</h4>
						<div class="shortcut-item">
							<kbd>%</kbd> <span>Процент</span>
						</div>
						<div class="shortcut-item">
							<kbd>R</kbd> <span>Квадратный корень</span>
						</div>
						<div class="shortcut-item">
							<kbd>^</kbd> <span>Степень</span>
						</div>
						<div class="shortcut-item">
							<kbd>F9</kbd> <span>Изменить знак</span>
						</div>
					</div>

					<div class="shortcut-group">
						<h4>Управление</h4>
						<div class="shortcut-item">
							<kbd>C</kbd> или <kbd>Esc</kbd> <span>Очистить все</span>
						</div>
						<div class="shortcut-item">
							<kbd>Delete</kbd> <span>Очистить ввод</span>
						</div>
						<div class="shortcut-item">
							<kbd>Backspace</kbd> <span>Удалить символ</span>
						</div>
						<div class="shortcut-item">
							<kbd>Ctrl+T</kbd> <span>Переключить тему</span>
						</div>
						<div class="shortcut-item">
							<kbd>Ctrl+S</kbd> <span>Настройки</span>
						</div>
						<div class="shortcut-item">
							<kbd>Ctrl+H</kbd> <span>Очистить историю</span>
						</div>
					</div>
				</div>
			</details>
		</div>
	</div>
</main>

<style>
	.main-container {
		min-height: 100vh;
		display: flex;
		align-items: center;
		justify-content: center;
		padding: 2rem 1rem;
		background: linear-gradient(135deg, #667eea 0%, #764ba2 100%);
		position: relative;
		overflow-x: hidden;
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
		gap: 2rem;
		max-width: 500px;
		width: 100%;
	}

	.app-header {
		text-align: center;
		margin-bottom: 1rem;
	}

	.app-title {
		font-size: 3.5rem;
		font-weight: 800;
		color: white;
		margin: 0;
		text-shadow: 0 4px 8px rgba(0, 0, 0, 0.3);
		letter-spacing: -0.02em;
		background: linear-gradient(135deg, #ffffff 0%, #f0f0f0 100%);
		-webkit-background-clip: text;
		-webkit-text-fill-color: transparent;
		background-clip: text;
	}

	.app-subtitle {
		font-size: 1.2rem;
		color: rgba(255, 255, 255, 0.9);
		margin: 0.5rem 0 0 0;
		font-weight: 400;
		text-shadow: 0 2px 4px rgba(0, 0, 0, 0.2);
	}

	.help-section {
		width: 100%;
		max-width: 400px;
	}

	.keyboard-shortcuts {
		background: rgba(255, 255, 255, 0.1);
		border: 1px solid rgba(255, 255, 255, 0.2);
		border-radius: 12px;
		padding: 1rem;
		backdrop-filter: blur(10px);
		-webkit-backdrop-filter: blur(10px);
	}

	.keyboard-shortcuts summary {
		cursor: pointer;
		font-weight: 600;
		color: white;
		font-size: 1.1rem;
		padding: 0.5rem;
		border-radius: 8px;
		transition: background-color 0.2s ease;
		user-select: none;
	}

	.keyboard-shortcuts summary:hover {
		background: rgba(255, 255, 255, 0.1);
	}

	.shortcuts-grid {
		display: grid;
		grid-template-columns: repeat(auto-fit, minmax(200px, 1fr));
		gap: 1.5rem;
		margin-top: 1rem;
		padding-top: 1rem;
		border-top: 1px solid rgba(255, 255, 255, 0.2);
	}

	.shortcut-group h4 {
		color: white;
		font-size: 1rem;
		font-weight: 600;
		margin: 0 0 0.75rem 0;
		padding-bottom: 0.5rem;
		border-bottom: 1px solid rgba(255, 255, 255, 0.2);
	}

	.shortcut-item {
		display: flex;
		align-items: center;
		gap: 0.75rem;
		margin-bottom: 0.5rem;
		font-size: 0.9rem;
	}

	.shortcut-item kbd {
		background: rgba(255, 255, 255, 0.2);
		border: 1px solid rgba(255, 255, 255, 0.3);
		border-radius: 4px;
		padding: 0.25rem 0.5rem;
		font-family: 'Consolas', 'Monaco', monospace;
		font-size: 0.8rem;
		color: white;
		min-width: 2rem;
		text-align: center;
		box-shadow: 0 1px 2px rgba(0, 0, 0, 0.2);
	}

	.shortcut-item span {
		color: rgba(255, 255, 255, 0.9);
		flex: 1;
	}

	@media (max-width: 768px) {
		.app-title {
			font-size: 2.5rem;
		}

		.app-subtitle {
			font-size: 1rem;
		}

		.shortcuts-grid {
			grid-template-columns: 1fr;
			gap: 1rem;
		}

		.main-container {
			padding: 1rem 0.5rem;
		}
	}

	@media (max-width: 480px) {
		.app-title {
			font-size: 2rem;
		}

		.content-wrapper {
			gap: 1rem;
		}
	}
</style>
