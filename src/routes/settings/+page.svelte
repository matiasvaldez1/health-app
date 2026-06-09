<script lang="ts">
	import { onMount } from 'svelte';
	import { getSettings, saveSettings, exportData } from '$lib/api';

	let apiKey = $state('');
	let maskedKey = $state('');
	let weightGoal = $state<number | undefined>(undefined);
	let saving = $state(false);
	let saved = $state(false);
	let loading = $state(true);
	let exporting = $state(false);

	onMount(async () => {
		const settings = await getSettings();
		maskedKey = settings.api_key ?? '';
		weightGoal = settings.weight_goal ?? undefined;
		loading = false;
	});

	async function handleSaveApiKey() {
		if (!apiKey.trim()) return;
		saving = true;
		saved = false;
		try {
			await saveSettings({ api_key: apiKey.trim(), weight_goal: weightGoal });
			maskedKey = apiKey.length > 8 ? `${apiKey.slice(0, 4)}...${apiKey.slice(-4)}` : apiKey;
			apiKey = '';
			saved = true;
			setTimeout(() => (saved = false), 3000);
		} finally {
			saving = false;
		}
	}

	async function handleSaveGoal() {
		saving = true;
		try {
			await saveSettings({ weight_goal: weightGoal || undefined });
			saved = true;
			setTimeout(() => (saved = false), 3000);
		} finally {
			saving = false;
		}
	}

	async function handleExportJSON() {
		exporting = true;
		try {
			const data = await exportData();
			const blob = new Blob([JSON.stringify(data, null, 2)], { type: 'application/json' });
			downloadBlob(blob, 'health-tracker-export.json');
		} finally {
			exporting = false;
		}
	}

	async function handleExportCSV() {
		exporting = true;
		try {
			const data = await exportData();
			let csv = '';

			csv += '--- PESO ---\nFecha,Peso (kg),Notas\n';
			for (const w of data.weights) {
				csv += `${w.date},${w.weight_kg},"${(w.notes ?? '').replace(/"/g, '""')}"\n`;
			}

			csv += '\n--- MEDITACION ---\nFecha,Hora,Duracion (min),Notas\n';
			for (const m of data.meditations) {
				csv += `${m.date},${m.time},${m.duration_min},"${(m.notes ?? '').replace(/"/g, '""')}"\n`;
			}

			csv += '\n--- ESTADO DE ANIMO ---\nFecha,Animo,Contenido\n';
			for (const f of data.feelings) {
				csv += `${f.date},${f.mood_score ?? ''},"${f.content.replace(/"/g, '""')}"\n`;
			}

			const blob = new Blob([csv], { type: 'text/csv' });
			downloadBlob(blob, 'health-tracker-export.csv');
		} finally {
			exporting = false;
		}
	}

	function downloadBlob(blob: Blob, filename: string) {
		const url = URL.createObjectURL(blob);
		const a = document.createElement('a');
		a.href = url;
		a.download = filename;
		a.click();
		URL.revokeObjectURL(url);
	}
</script>

<div class="max-w-xl">
	<h2 class="text-2xl font-bold mb-6">Configuración</h2>

	{#if loading}
		<p class="text-text-muted text-sm">Cargando...</p>
	{:else}
		<!-- API Key -->
		<div class="bg-bg-secondary rounded-lg p-5 border border-border mb-4">
			<h3 class="text-sm font-medium text-text-primary mb-4">Clave de API de Claude</h3>
			<p class="text-xs text-text-muted mb-4">Requerida para los tips de salud con IA.</p>
			{#if maskedKey}
				<div class="flex items-center gap-2 mb-4">
					<span class="text-sm text-text-secondary">Clave actual:</span>
					<code class="text-sm bg-bg-tertiary px-2 py-0.5 rounded text-accent">{maskedKey}</code>
				</div>
			{/if}
			<form onsubmit={handleSaveApiKey} class="flex gap-3">
				<input type="password" bind:value={apiKey} placeholder="sk-ant-..." class="flex-1 bg-bg-tertiary border border-border rounded px-3 py-2 text-sm text-text-primary focus:outline-none focus:border-accent" />
				<button type="submit" disabled={saving || !apiKey.trim()} class="bg-accent hover:bg-accent-hover text-white px-4 py-2 rounded text-sm font-medium disabled:opacity-50 transition-colors">
					{saving ? 'Guardando...' : 'Guardar'}
				</button>
			</form>
			{#if saved}
				<p class="text-sm text-accent mt-2">Clave guardada correctamente.</p>
			{/if}
		</div>

		<!-- Weight Goal -->
		<div class="bg-bg-secondary rounded-lg p-5 border border-border mb-4">
			<h3 class="text-sm font-medium text-text-primary mb-4">Meta de peso</h3>
			<div class="flex gap-3 items-end">
				<div class="flex-1">
					<label for="goal" class="block text-xs text-text-muted mb-1">Objetivo (kg)</label>
					<input id="goal" type="number" step="0.1" min="0" bind:value={weightGoal} placeholder="75.0" class="w-full bg-bg-tertiary border border-border rounded px-3 py-2 text-sm text-text-primary focus:outline-none focus:border-accent" />
				</div>
				<button onclick={handleSaveGoal} disabled={saving} class="bg-accent hover:bg-accent-hover text-white px-4 py-2 rounded text-sm font-medium disabled:opacity-50 transition-colors">
					Guardar meta
				</button>
			</div>
		</div>

		<!-- Export -->
		<div class="bg-bg-secondary rounded-lg p-5 border border-border mb-4">
			<h3 class="text-sm font-medium text-text-primary mb-4">Exportar datos</h3>
			<p class="text-xs text-text-muted mb-4">Descargá todos tus datos de peso, meditación y estado de ánimo.</p>
			<div class="flex gap-3">
				<button onclick={handleExportJSON} disabled={exporting} class="bg-bg-tertiary hover:bg-bg-hover text-text-primary px-4 py-2 rounded text-sm font-medium border border-border transition-colors disabled:opacity-50">
					{exporting ? 'Exportando...' : 'Exportar JSON'}
				</button>
				<button onclick={handleExportCSV} disabled={exporting} class="bg-bg-tertiary hover:bg-bg-hover text-text-primary px-4 py-2 rounded text-sm font-medium border border-border transition-colors disabled:opacity-50">
					{exporting ? 'Exportando...' : 'Exportar CSV'}
				</button>
			</div>
		</div>

		<!-- About -->
		<div class="bg-bg-secondary rounded-lg p-5 border border-border">
			<h3 class="text-sm font-medium text-text-primary mb-2">Acerca de</h3>
			<p class="text-xs text-text-muted">Health Tracker v0.1.0</p>
			<p class="text-xs text-text-muted mt-1">Los datos se guardan localmente en tu máquina.</p>
			<p class="text-xs text-text-muted mt-1">Acceso por navegador: <code class="text-accent">http://localhost:3333</code></p>
		</div>
	{/if}
</div>
