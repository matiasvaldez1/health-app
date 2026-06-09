<script lang="ts">
	import { onMount } from 'svelte';
	import { getSettings, saveSettings } from '$lib/api';

	let apiKey = $state('');
	let maskedKey = $state('');
	let saving = $state(false);
	let saved = $state(false);
	let loading = $state(true);

	onMount(async () => {
		const settings = await getSettings();
		maskedKey = settings.api_key ?? '';
		loading = false;
	});

	async function handleSave() {
		if (!apiKey.trim()) return;
		saving = true;
		saved = false;
		try {
			await saveSettings({ api_key: apiKey.trim() });
			maskedKey = apiKey.length > 8
				? `${apiKey.slice(0, 4)}...${apiKey.slice(-4)}`
				: apiKey;
			apiKey = '';
			saved = true;
			setTimeout(() => (saved = false), 3000);
		} finally {
			saving = false;
		}
	}
</script>

<div class="max-w-xl">
	<h2 class="text-2xl font-bold mb-6">Configuración</h2>

	{#if loading}
		<p class="text-text-muted text-sm">Cargando...</p>
	{:else}
		<div class="bg-bg-secondary rounded-lg p-5 border border-border">
			<h3 class="text-sm font-medium text-text-primary mb-4">Clave de API de Claude</h3>
			<p class="text-xs text-text-muted mb-4">
				Requerida para los tips de salud con IA. Conseguí tu clave en la consola de Anthropic.
			</p>

			{#if maskedKey}
				<div class="flex items-center gap-2 mb-4">
					<span class="text-sm text-text-secondary">Clave actual:</span>
					<code class="text-sm bg-bg-tertiary px-2 py-0.5 rounded text-accent">{maskedKey}</code>
				</div>
			{/if}

			<form onsubmit={handleSave} class="flex gap-3">
				<input
					type="password"
					bind:value={apiKey}
					placeholder="sk-ant-..."
					class="flex-1 bg-bg-tertiary border border-border rounded px-3 py-2 text-sm text-text-primary focus:outline-none focus:border-accent"
				/>
				<button
					type="submit"
					disabled={saving || !apiKey.trim()}
					class="bg-accent hover:bg-accent-hover text-white px-4 py-2 rounded text-sm font-medium disabled:opacity-50 transition-colors"
				>
					{saving ? 'Guardando...' : 'Guardar'}
				</button>
			</form>

			{#if saved}
				<p class="text-sm text-accent mt-2">Clave de API guardada correctamente.</p>
			{/if}
		</div>

		<div class="bg-bg-secondary rounded-lg p-5 border border-border mt-4">
			<h3 class="text-sm font-medium text-text-primary mb-2">Acerca de</h3>
			<p class="text-xs text-text-muted">Health Tracker v0.1.0</p>
			<p class="text-xs text-text-muted mt-1">Los datos se guardan localmente en tu máquina.</p>
			<p class="text-xs text-text-muted mt-1">Acceso por navegador: <code class="text-accent">http://localhost:3333</code></p>
		</div>
	{/if}
</div>
