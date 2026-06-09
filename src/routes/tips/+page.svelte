<script lang="ts">
	import { onMount } from 'svelte';
	import { getLatestTip, generateTips, checkTipsStale, hasApiKey } from '$lib/api';
	import type { AiTip } from '$lib/types';

	let tip = $state<AiTip | null>(null);
	let loading = $state(true);
	let generating = $state(false);
	let error = $state('');
	let isStale = $state(false);
	let hasKey = $state(false);

	onMount(async () => {
		const [t, stale, key] = await Promise.all([getLatestTip(), checkTipsStale(), hasApiKey()]);
		tip = t;
		isStale = stale;
		hasKey = key;
		loading = false;
	});

	async function handleGenerate() {
		generating = true;
		error = '';
		try {
			tip = await generateTips();
			isStale = false;
		} catch (e) {
			error = e instanceof Error ? e.message : 'Error al generar tips';
		} finally {
			generating = false;
		}
	}
</script>

<div class="max-w-3xl">
	<h2 class="text-2xl font-bold mb-6">Tips de Salud con IA</h2>

	{#if loading}
		<p class="text-text-muted text-sm">Cargando...</p>
	{:else}
		{#if !hasKey}
			<div class="bg-bg-secondary rounded-lg p-6 border border-border text-center">
				<p class="text-text-secondary mb-3">Necesitás configurar tu clave de API de Claude primero.</p>
				<a href="/settings" class="text-accent hover:text-accent-hover text-sm font-medium">
					Ir a Configuración
				</a>
			</div>
		{:else}
			<div class="flex items-center gap-4 mb-6">
				<button
					onclick={handleGenerate}
					disabled={generating}
					class="bg-accent hover:bg-accent-hover text-white px-4 py-2 rounded text-sm font-medium disabled:opacity-50 transition-colors"
				>
					{generating ? 'Generando...' : isStale ? 'Generar Nuevos Tips' : 'Actualizar Tips'}
				</button>
				{#if isStale && tip}
					<span class="text-xs text-warning">Los tips pueden estar desactualizados (hay datos nuevos o pasaron 3+ días)</span>
				{/if}
			</div>

			{#if error}
				<div class="bg-danger/10 border border-danger/30 rounded-lg p-4 mb-6">
					<p class="text-sm text-danger">{error}</p>
				</div>
			{/if}

			{#if tip}
				<div class="bg-bg-secondary rounded-lg p-6 border border-border">
					<div class="flex items-center justify-between mb-4">
						<h3 class="text-sm text-text-muted">Tus Tips Personalizados</h3>
						{#if tip.created_at}
							<span class="text-xs text-text-muted">Generado: {tip.created_at}</span>
						{/if}
					</div>
					<div class="text-sm text-text-primary whitespace-pre-line leading-relaxed">
						{tip.response}
					</div>
				</div>
			{:else}
				<div class="bg-bg-secondary rounded-lg p-6 border border-border text-center">
					<p class="text-text-muted mb-2">No se generaron tips aún.</p>
					<p class="text-xs text-text-muted">
						Empezá a registrar tu peso, meditación y estado de ánimo, y después generá tips personalizados.
					</p>
				</div>
			{/if}
		{/if}
	{/if}
</div>
