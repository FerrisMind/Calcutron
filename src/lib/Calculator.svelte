<script lang="ts">
	import { onMount } from 'svelte';
	import { browser } from '$app/environment';

	// Состояние калькулятора с использованием Svelte 5 runes
	let display = $state('0');
	let previousValue = $state<number | null>(null);
	let operation = $state<string | null>(null);
	let waitingForOperand = $state(false);
	let history = $state<string[]>([]);

	// Настройки темы и UI
	let isDarkMode = $state(false);
	let isSystemTheme = $state(true);
	let showSettings = $state(false);

	// Анимации и эффекты
	let lastPressedButton = $state<string | null>(null);

	// Вычисляемое значение для отображения истории
	let historyDisplay = $derived(history.slice(-5).reverse());

	// Функции для управления темой
	function detectSystemTheme() {
		if (!browser) return false;
		return window.matchMedia('(prefers-color-scheme: dark)').matches;
	}

	function updateTheme() {
		if (!browser) return;

		if (isSystemTheme) {
			isDarkMode = detectSystemTheme();
		}

		if (isDarkMode) {
			document.documentElement.classList.add('dark');
		} else {
			document.documentElement.classList.remove('dark');
		}

		// Сохраняем настройки
		localStorage.setItem('calcutron-theme', JSON.stringify({ isDarkMode, isSystemTheme }));
	}

	function toggleTheme() {
		isSystemTheme = false;
		isDarkMode = !isDarkMode;
		updateTheme();
	}

	function toggleSystemTheme() {
		isSystemTheme = !isSystemTheme;
		if (isSystemTheme) {
			isDarkMode = detectSystemTheme();
		}
		updateTheme();
	}

	// Улучшенные математические функции
	function percentage() {
		const value = parseFloat(display);
		if (previousValue !== null && operation) {
			// Процент от предыдущего значения
			const result = (previousValue * value) / 100;
			display = String(result);
		} else {
			// Просто деление на 100
			display = String(value / 100);
		}
		waitingForOperand = true;
	}

	function squareRoot() {
		const value = parseFloat(display);
		if (value < 0) {
			display = 'Ошибка';
			return;
		}
		const result = Math.sqrt(value);
		display = String(result);
		waitingForOperand = true;

		// Добавляем в историю
		history = [...history, `√${value} = ${result}`];
	}

	function power() {
		performOperation('^');
	}

	function toggleSign() {
		if (display === '0') return;
		if (display.startsWith('-')) {
			display = display.slice(1);
		} else {
			display = '-' + display;
		}
	}

	function clearEntry() {
		display = '0';
		waitingForOperand = true;
	}

	// Функция для анимации нажатия кнопки
	function animateButton(buttonId: string) {
		lastPressedButton = buttonId;
		setTimeout(() => {
			lastPressedButton = null;
		}, 150);
	}

	// Функция для ввода цифр
	function inputDigit(digit: string) {
		if (waitingForOperand) {
			display = digit;
			waitingForOperand = false;
		} else {
			display = display === '0' ? digit : display + digit;
		}
	}

	// Функция для ввода десятичной точки
	function inputDecimal() {
		if (waitingForOperand) {
			display = '0.';
			waitingForOperand = false;
		} else if (display.indexOf('.') === -1) {
			display = display + '.';
		}
	}

	// Функция для очистки
	function clear() {
		display = '0';
		previousValue = null;
		operation = null;
		waitingForOperand = false;
	}

	// Функция для удаления последнего символа
	function backspace() {
		if (display.length > 1) {
			display = display.slice(0, -1);
		} else {
			display = '0';
		}
	}

	// Функция для выполнения операций
	function performOperation(nextOperation: string) {
		const inputValue = parseFloat(display);

		if (previousValue === null) {
			previousValue = inputValue;
		} else if (operation) {
			const currentValue = previousValue || 0;
			let result: number;

			switch (operation) {
				case '+':
					result = currentValue + inputValue;
					break;
				case '-':
					result = currentValue - inputValue;
					break;
				case '*':
					result = currentValue * inputValue;
					break;
				case '/':
					if (inputValue === 0) {
						display = 'Ошибка';
						previousValue = null;
						operation = null;
						waitingForOperand = true;
						return;
					}
					result = currentValue / inputValue;
					break;
				case '^':
					result = Math.pow(currentValue, inputValue);
					break;
				default:
					return;
			}

			// Добавляем в историю
			const historyEntry = `${currentValue} ${operation} ${inputValue} = ${result}`;
			history = [...history, historyEntry];

			display = String(result);
			previousValue = result;
		}

		waitingForOperand = true;
		operation = nextOperation;
	}

	// Функция для вычисления результата
	function calculate() {
		performOperation('=');
		operation = null;
		previousValue = null;
		waitingForOperand = true;
	}

	// Обработка клавиатуры
	function handleKeydown(event: KeyboardEvent) {
		const { key, ctrlKey, altKey } = event;

		// Горячие клавиши с модификаторами
		if (ctrlKey) {
			if (key === 't') {
				event.preventDefault();
				toggleTheme();
				return;
			}
			if (key === 's') {
				event.preventDefault();
				showSettings = !showSettings;
				return;
			}
			if (key === 'h') {
				event.preventDefault();
				// Очистить историю
				history = [];
				return;
			}
		}

		if (key >= '0' && key <= '9') {
			event.preventDefault();
			animateButton(key);
			inputDigit(key);
		} else if (key === '.') {
			event.preventDefault();
			animateButton('decimal');
			inputDecimal();
		} else if (key === '+' || key === '-' || key === '*' || key === '/') {
			event.preventDefault();
			animateButton(key);
			performOperation(key);
		} else if (key === '^') {
			event.preventDefault();
			animateButton('power');
			power();
		} else if (key === 'Enter' || key === '=') {
			event.preventDefault();
			animateButton('equals');
			calculate();
		} else if (key === 'Escape' || key === 'c' || key === 'C') {
			event.preventDefault();
			animateButton('clear');
			clear();
		} else if (key === 'Backspace') {
			event.preventDefault();
			animateButton('backspace');
			backspace();
		} else if (key === 'Delete') {
			event.preventDefault();
			animateButton('clear-entry');
			clearEntry();
		} else if (key === '%') {
			event.preventDefault();
			animateButton('percentage');
			percentage();
		} else if (key === 'r' || key === 'R') {
			event.preventDefault();
			animateButton('sqrt');
			squareRoot();
		} else if (key === 'F9') {
			event.preventDefault();
			animateButton('sign');
			toggleSign();
		}
	}

	// Подключение обработчика клавиатуры при монтировании
	onMount(() => {
		let handleSystemThemeChange: ((e: MediaQueryListEvent) => void) | null = null;

		// Загружаем сохраненные настройки темы
		if (browser) {
			const savedTheme = localStorage.getItem('calcutron-theme');
			if (savedTheme) {
				try {
					const { isDarkMode: savedDark, isSystemTheme: savedSystem } = JSON.parse(savedTheme);
					isDarkMode = savedDark;
					isSystemTheme = savedSystem;
				} catch (e) {
					// Если ошибка парсинга, используем системную тему
					isSystemTheme = true;
					isDarkMode = detectSystemTheme();
				}
			} else {
				// Первый запуск - используем системную тему
				isSystemTheme = true;
				isDarkMode = detectSystemTheme();
			}
			updateTheme();

			// Слушаем изменения системной темы
			const mediaQuery = window.matchMedia('(prefers-color-scheme: dark)');
			handleSystemThemeChange = (e: MediaQueryListEvent) => {
				if (isSystemTheme) {
					isDarkMode = e.matches;
					updateTheme();
				}
			};
			mediaQuery.addEventListener('change', handleSystemThemeChange);
		}

		window.addEventListener('keydown', handleKeydown);

		return () => {
			window.removeEventListener('keydown', handleKeydown);
			if (browser && handleSystemThemeChange) {
				const mediaQuery = window.matchMedia('(prefers-color-scheme: dark)');
				mediaQuery.removeEventListener('change', handleSystemThemeChange);
			}
		};
	});
</script>

<div class="calculator-container">
	<!-- Панель настроек -->
	{#if showSettings}
		<div class="settings-panel">
			<div class="settings-header">
				<h3>Настройки</h3>
				<button
					class="close-btn"
					onclick={() => (showSettings = false)}
					aria-label="Закрыть настройки"><i class="ph ph-x"></i></button
				>
			</div>
			<div class="settings-content">
				<div class="setting-item">
					<label>
						<input type="checkbox" bind:checked={isSystemTheme} onchange={toggleSystemTheme} />
						Использовать системную тему
					</label>
				</div>
				{#if !isSystemTheme}
					<div class="setting-item">
						<button class="theme-toggle-btn" onclick={toggleTheme}>
							<i class="ph {isDarkMode ? 'ph-moon' : 'ph-sun'}"></i>
							{isDarkMode ? 'Темная тема' : 'Светлая тема'}
						</button>
					</div>
				{/if}
				<div class="setting-item">
					<button class="clear-history-btn" onclick={() => (history = [])}>
						<i class="ph ph-eraser"></i> Очистить историю
					</button>
				</div>
			</div>
		</div>
	{/if}

	<div class="calculator">
		<!-- Заголовок с кнопками управления -->
		<div class="calculator-header">
			<div class="title">Calcutron</div>
			<div class="header-controls">
				<button
					class="control-btn"
					onclick={() => (showSettings = !showSettings)}
					title="Настройки (Ctrl+S)"
					aria-label="Открыть настройки"
				>
					<i class="ph ph-gear"></i>
				</button>
				<button
					class="control-btn"
					onclick={toggleTheme}
					title="Переключить тему (Ctrl+T)"
					aria-label="Переключить тему"
				>
					<i class="ph {isDarkMode ? 'ph-moon' : 'ph-sun'}"></i>
				</button>
			</div>
		</div>

		<!-- Дисплей -->
		<div class="display">
			<div class="display-screen">
				{display}
			</div>
		</div>

		<!-- История вычислений -->
		{#if historyDisplay.length > 0}
			<div class="history">
				<div class="history-header">
					<span>История</span>
					<button
						class="clear-history-mini"
						onclick={() => (history = [])}
						title="Очистить историю (Ctrl+H)"
						aria-label="Очистить историю"><i class="ph ph-eraser"></i></button
					>
				</div>
				<div class="history-list">
					{#each historyDisplay as entry, index (index)}
						<div class="history-entry">{entry}</div>
					{/each}
				</div>
			</div>
		{/if}

		<!-- Кнопки калькулятора -->
		<div class="buttons">
			<!-- Первый ряд - функциональные кнопки -->
			<div class="button-row">
				<button
					class="btn function-btn {lastPressedButton === 'percentage' ? 'pressed' : ''}"
					onclick={() => {
						animateButton('percentage');
						percentage();
					}}
					title="Процент (%)"
					aria-label="Процент"
				>
					<i class="ph ph-percent"></i>
				</button>
				<button
					class="btn function-btn {lastPressedButton === 'clear-entry' ? 'pressed' : ''}"
					onclick={() => {
						animateButton('clear-entry');
						clearEntry();
					}}
					title="Очистить ввод (Delete)"
				>
					CE
				</button>
				<button
					class="btn clear-btn {lastPressedButton === 'clear' ? 'pressed' : ''}"
					onclick={() => {
						animateButton('clear');
						clear();
					}}
					title="Очистить все (C/Esc)"
				>
					C
				</button>
				<button
					class="btn function-btn {lastPressedButton === 'backspace' ? 'pressed' : ''}"
					onclick={() => {
						animateButton('backspace');
						backspace();
					}}
					title="Удалить символ (Backspace)"
					aria-label="Удалить символ"
				>
					<i class="ph ph-backspace"></i>
				</button>
			</div>

			<!-- Второй ряд -->
			<div class="button-row">
				<button
					class="btn function-btn {lastPressedButton === 'sqrt' ? 'pressed' : ''}"
					onclick={() => {
						animateButton('sqrt');
						squareRoot();
					}}
					title="Квадратный корень (√)"
					aria-label="Квадратный корень"
				>
					<i class="ph ph-radical"></i>
				</button>
				<button
					class="btn function-btn {lastPressedButton === 'power' ? 'pressed' : ''}"
					onclick={() => {
						animateButton('power');
						power();
					}}
					title="Возведение в квадрат (x²)"
					aria-label="Возведение в квадрат"
				>
					<i class="ph ph-function"></i>
				</button>
				<button
					class="btn function-btn {lastPressedButton === 'toggleSign' ? 'pressed' : ''}"
					onclick={() => {
						animateButton('toggleSign');
						toggleSign();
					}}
					title="Изменить знак (±)"
					aria-label="Изменить знак"
				>
					<i class="ph ph-plus-minus"></i>
				</button>
				<button
					class="btn operator-btn {lastPressedButton === '/' ? 'pressed' : ''}"
					onclick={() => {
						animateButton('/');
						performOperation('/');
					}}
					title="Деление (/)"
					aria-label="Деление"
				>
					<i class="ph ph-divide"></i>
				</button>
			</div>

			<!-- Третий ряд -->
			<div class="button-row">
				<button
					class="btn number-btn {lastPressedButton === '7' ? 'pressed' : ''}"
					onclick={() => {
						animateButton('7');
						inputDigit('7');
					}}
				>
					7
				</button>
				<button
					class="btn number-btn {lastPressedButton === '8' ? 'pressed' : ''}"
					onclick={() => {
						animateButton('8');
						inputDigit('8');
					}}
				>
					8
				</button>
				<button
					class="btn number-btn {lastPressedButton === '9' ? 'pressed' : ''}"
					onclick={() => {
						animateButton('9');
						inputDigit('9');
					}}
				>
					9
				</button>
				<button
					class="btn operator-btn {lastPressedButton === '*' ? 'pressed' : ''}"
					onclick={() => {
						animateButton('*');
						performOperation('*');
					}}
					title="Умножение (*)"
					aria-label="Умножение"
				>
					<i class="ph ph-x"></i>
				</button>
			</div>

			<!-- Четвертый ряд -->
			<div class="button-row">
				<button
					class="btn number-btn {lastPressedButton === '4' ? 'pressed' : ''}"
					onclick={() => {
						animateButton('4');
						inputDigit('4');
					}}
				>
					4
				</button>
				<button
					class="btn number-btn {lastPressedButton === '5' ? 'pressed' : ''}"
					onclick={() => {
						animateButton('5');
						inputDigit('5');
					}}
				>
					5
				</button>
				<button
					class="btn number-btn {lastPressedButton === '6' ? 'pressed' : ''}"
					onclick={() => {
						animateButton('6');
						inputDigit('6');
					}}
				>
					6
				</button>
				<button
					class="btn operator-btn {lastPressedButton === '-' ? 'pressed' : ''}"
					onclick={() => {
						animateButton('-');
						performOperation('-');
					}}
					title="Вычитание (-)"
					aria-label="Вычитание"
				>
					<i class="ph ph-minus"></i>
				</button>
			</div>

			<!-- Пятый ряд -->
			<div class="button-row">
				<button
					class="btn number-btn {lastPressedButton === '1' ? 'pressed' : ''}"
					onclick={() => {
						animateButton('1');
						inputDigit('1');
					}}
				>
					1
				</button>
				<button
					class="btn number-btn {lastPressedButton === '2' ? 'pressed' : ''}"
					onclick={() => {
						animateButton('2');
						inputDigit('2');
					}}
				>
					2
				</button>
				<button
					class="btn number-btn {lastPressedButton === '3' ? 'pressed' : ''}"
					onclick={() => {
						animateButton('3');
						inputDigit('3');
					}}
				>
					3
				</button>
				<button
					class="btn operator-btn {lastPressedButton === '+' ? 'pressed' : ''}"
					onclick={() => {
						animateButton('+');
						performOperation('+');
					}}
					title="Сложение (+)"
					aria-label="Сложение"
				>
					<i class="ph ph-plus"></i>
				</button>
			</div>

			<!-- Шестой ряд -->
			<div class="button-row">
				<button
					class="btn number-btn zero-btn {lastPressedButton === '0' ? 'pressed' : ''}"
					onclick={() => {
						animateButton('0');
						inputDigit('0');
					}}
				>
					0
				</button>
				<button
					class="btn number-btn {lastPressedButton === 'decimal' ? 'pressed' : ''}"
					onclick={() => {
						animateButton('decimal');
						inputDecimal();
					}}
					title="Десятичная точка (.)"
				>
					.
				</button>
				<button
					class="btn equals-btn {lastPressedButton === 'equals' ? 'pressed' : ''}"
					onclick={() => {
						animateButton('equals');
						calculate();
					}}
					title="Равно (Enter/=)"
					aria-label="Равно"
				>
					<i class="ph ph-equals"></i>
				</button>
			</div>
		</div>
	</div>
</div>

<style>
	/* Основные переменные для темы */
	:global(:root) {
		--bg-primary: #ffffff;
		--bg-secondary: #f8f9fa;
		--bg-tertiary: #e9ecef;
		--text-primary: #212529;
		--text-secondary: #6c757d;
		--border-color: #dee2e6;
		--shadow-light: 0 2px 8px rgba(0, 0, 0, 0.1);
		--shadow-medium: 0 4px 16px rgba(0, 0, 0, 0.15);
		--shadow-heavy: 0 8px 32px rgba(0, 0, 0, 0.2);
		--accent-blue: #0078d4;
		--accent-blue-hover: #106ebe;
		--accent-green: #107c10;
		--accent-red: #d13438;
		--accent-orange: #ff8c00;
		--glass-bg: rgba(255, 255, 255, 0.7);
		--glass-border: rgba(255, 255, 255, 0.2);
	}

	:global(.dark) {
		--bg-primary: #1e1e1e;
		--bg-secondary: #2d2d30;
		--bg-tertiary: #3e3e42;
		--text-primary: #ffffff;
		--text-secondary: #cccccc;
		--border-color: #464647;
		--shadow-light: 0 2px 8px rgba(0, 0, 0, 0.3);
		--shadow-medium: 0 4px 16px rgba(0, 0, 0, 0.4);
		--shadow-heavy: 0 8px 32px rgba(0, 0, 0, 0.5);
		--accent-blue: #60cdff;
		--accent-blue-hover: #4cc2ff;
		--accent-green: #6ccb5f;
		--accent-red: #ff6b6b;
		--accent-orange: #ffb347;
		--glass-bg: rgba(30, 30, 30, 0.7);
		--glass-border: rgba(255, 255, 255, 0.1);
	}

	/* Контейнер калькулятора */
	.calculator-container {
		position: relative;
		display: flex;
		flex-direction: column;
		align-items: center;
		gap: 1rem;
		max-width: 400px;
		margin: 0 auto;
	}

	/* Панель настроек */
	.settings-panel {
		position: absolute;
		top: -10px;
		left: 50%;
		transform: translateX(-50%);
		z-index: 1000;
		background: var(--glass-bg);
		border: 1px solid var(--glass-border);
		border-radius: 12px;
		padding: 1.5rem;
		min-width: 280px;
		box-shadow: var(--shadow-heavy);
		backdrop-filter: blur(20px);
		-webkit-backdrop-filter: blur(20px);
		animation: slideDown 0.3s ease-out;
	}

	@keyframes slideDown {
		from {
			opacity: 0;
			transform: translateX(-50%) translateY(-20px);
		}
		to {
			opacity: 1;
			transform: translateX(-50%) translateY(0);
		}
	}

	.settings-header {
		display: flex;
		justify-content: space-between;
		align-items: center;
		margin-bottom: 1rem;
	}

	.settings-header h3 {
		margin: 0;
		color: var(--text-primary);
		font-size: 1.2rem;
		font-weight: 600;
	}

	.close-btn {
		background: none;
		border: none;
		font-size: 1.5rem;
		cursor: pointer;
		color: var(--text-secondary);
		padding: 0;
		width: 30px;
		height: 30px;
		display: flex;
		align-items: center;
		justify-content: center;
		border-radius: 6px;
		transition: all 0.2s ease;
	}

	.close-btn:hover {
		background: var(--bg-tertiary);
		color: var(--text-primary);
	}

	.setting-item {
		margin-bottom: 1rem;
	}

	.setting-item label {
		display: flex;
		align-items: center;
		gap: 0.5rem;
		color: var(--text-primary);
		font-size: 0.9rem;
		cursor: pointer;
	}

	.theme-toggle-btn,
	.clear-history-btn {
		width: 100%;
		padding: 0.75rem;
		border: none;
		border-radius: 8px;
		background: var(--bg-secondary);
		color: var(--text-primary);
		font-size: 0.9rem;
		cursor: pointer;
		transition: all 0.2s ease;
		border: 1px solid var(--border-color);
	}

	.theme-toggle-btn:hover,
	.clear-history-btn:hover {
		background: var(--bg-tertiary);
		transform: translateY(-1px);
		box-shadow: var(--shadow-light);
	}

	/* Основной калькулятор */
	.calculator {
		background: var(--glass-bg);
		border: 1px solid var(--glass-border);
		border-radius: 16px;
		padding: 1.5rem;
		box-shadow: var(--shadow-heavy);
		backdrop-filter: blur(20px);
		-webkit-backdrop-filter: blur(20px);
		width: 100%;
		max-width: 380px;
		min-height: 600px;
		animation: fadeIn 0.5s ease-out;
	}

	@keyframes fadeIn {
		from {
			opacity: 0;
			transform: scale(0.95);
		}
		to {
			opacity: 1;
			transform: scale(1);
		}
	}

	/* Заголовок калькулятора */
	.calculator-header {
		display: flex;
		justify-content: space-between;
		align-items: center;
		margin-bottom: 1.5rem;
		padding-bottom: 1rem;
		border-bottom: 1px solid var(--border-color);
	}

	.title {
		font-size: 1.5rem;
		font-weight: 700;
		color: var(--text-primary);
		margin: 0;
	}

	.header-controls {
		display: flex;
		gap: 0.5rem;
	}

	.control-btn {
		width: 40px;
		height: 40px;
		border: none;
		border-radius: 8px;
		background: var(--bg-secondary);
		color: var(--text-primary);
		font-size: 1.2rem;
		cursor: pointer;
		transition: all 0.2s ease;
		display: flex;
		align-items: center;
		justify-content: center;
		border: 1px solid var(--border-color);
	}

	.control-btn:hover {
		background: var(--bg-tertiary);
		transform: translateY(-1px);
		box-shadow: var(--shadow-light);
	}

	/* Дисплей */
	.display {
		margin-bottom: 1.5rem;
	}

	.display-screen {
		background: var(--bg-primary);
		border: 2px solid var(--border-color);
		border-radius: 12px;
		padding: 1.5rem;
		text-align: right;
		font-family:
			'Segoe UI',
			'SF Pro Display',
			-apple-system,
			BlinkMacSystemFont,
			sans-serif;
		font-size: 2.5rem;
		font-weight: 300;
		color: var(--text-primary);
		min-height: 80px;
		display: flex;
		align-items: center;
		justify-content: flex-end;
		box-shadow: inset 0 2px 4px rgba(0, 0, 0, 0.1);
		word-break: break-all;
		overflow-wrap: break-word;
	}

	/* История */
	.history {
		margin-bottom: 1.5rem;
		background: var(--bg-secondary);
		border: 1px solid var(--border-color);
		border-radius: 10px;
		padding: 1rem;
		max-height: 120px;
		overflow-y: auto;
	}

	.history-header {
		display: flex;
		justify-content: space-between;
		align-items: center;
		margin-bottom: 0.5rem;
		font-size: 0.85rem;
		font-weight: 600;
		color: var(--text-secondary);
	}

	.clear-history-mini {
		background: none;
		border: none;
		font-size: 0.9rem;
		cursor: pointer;
		color: var(--text-secondary);
		padding: 2px 6px;
		border-radius: 4px;
		transition: all 0.2s ease;
	}

	.clear-history-mini:hover {
		background: var(--bg-tertiary);
		color: var(--text-primary);
	}

	.history-list {
		display: flex;
		flex-direction: column;
		gap: 0.25rem;
	}

	.history-entry {
		font-family: 'Consolas', 'Monaco', monospace;
		font-size: 0.8rem;
		color: var(--text-secondary);
		padding: 0.25rem 0;
		border-bottom: 1px solid var(--border-color);
	}

	.history-entry:last-child {
		border-bottom: none;
	}

	/* Кнопки */
	.buttons {
		display: flex;
		flex-direction: column;
		gap: 0.75rem;
	}

	.button-row {
		display: grid;
		grid-template-columns: repeat(4, 1fr);
		gap: 0.75rem;
	}

	.button-row:last-child {
		grid-template-columns: 2fr 1fr 1fr;
	}

	.btn {
		height: 60px;
		border: none;
		border-radius: 12px;
		font-size: 1.2rem;
		font-weight: 600;
		cursor: pointer;
		transition: all 0.15s ease;
		display: flex;
		align-items: center;
		justify-content: center;
		position: relative;
		overflow: hidden;
		border: 1px solid var(--border-color);
		user-select: none;
	}

	.btn:active {
		transform: scale(0.95);
	}

	.btn.pressed {
		transform: scale(0.95);
		box-shadow: inset 0 2px 4px rgba(0, 0, 0, 0.2);
	}

	/* Типы кнопок */
	.number-btn {
		background: var(--bg-primary);
		color: var(--text-primary);
		border: 2px solid var(--border-color);
	}

	.number-btn:hover {
		background: var(--bg-secondary);
		transform: translateY(-2px);
		box-shadow: var(--shadow-medium);
	}

	.operator-btn {
		background: var(--accent-blue);
		color: white;
		border: 2px solid var(--accent-blue);
	}

	.operator-btn:hover {
		background: var(--accent-blue-hover);
		transform: translateY(-2px);
		box-shadow: var(--shadow-medium);
	}

	.function-btn {
		background: var(--bg-secondary);
		color: var(--text-primary);
		border: 2px solid var(--border-color);
	}

	.function-btn:hover {
		background: var(--bg-tertiary);
		transform: translateY(-2px);
		box-shadow: var(--shadow-medium);
	}

	.clear-btn {
		background: var(--accent-red);
		color: white;
		border: 2px solid var(--accent-red);
	}

	.clear-btn:hover {
		background: #b02a2e;
		transform: translateY(-2px);
		box-shadow: var(--shadow-medium);
	}

	.equals-btn {
		background: var(--accent-green);
		color: white;
		border: 2px solid var(--accent-green);
		grid-column: span 2;
	}

	.equals-btn:hover {
		background: #0e6e0e;
		transform: translateY(-2px);
		box-shadow: var(--shadow-medium);
	}

	.zero-btn {
		grid-column: span 1;
	}

	/* Адаптивность */
	@media (max-width: 480px) {
		.calculator {
			padding: 1rem;
			max-width: 100%;
			margin: 0 1rem;
		}

		.display-screen {
			font-size: 2rem;
			padding: 1rem;
		}

		.btn {
			height: 50px;
			font-size: 1rem;
		}

		.settings-panel {
			min-width: 250px;
			padding: 1rem;
		}
	}

	/* Скроллбар для истории */
	.history::-webkit-scrollbar {
		width: 6px;
	}

	.history::-webkit-scrollbar-track {
		background: var(--bg-tertiary);
		border-radius: 3px;
	}

	.history::-webkit-scrollbar-thumb {
		background: var(--text-secondary);
		border-radius: 3px;
	}

	.history::-webkit-scrollbar-thumb:hover {
		background: var(--text-primary);
	}

	/* Анимация ripple эффекта */
	.btn::before {
		content: '';
		position: absolute;
		top: 50%;
		left: 50%;
		width: 0;
		height: 0;
		border-radius: 50%;
		background: rgba(255, 255, 255, 0.3);
		transform: translate(-50%, -50%);
		transition:
			width 0.3s ease,
			height 0.3s ease;
	}

	.btn:active::before {
		width: 100px;
		height: 100px;
	}
</style>
