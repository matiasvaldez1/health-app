<script lang="ts">
	import { onMount } from 'svelte';
	import { listFeelings, createFeeling, deleteFeeling } from '$lib/api';
	import { today, formatDate } from '$lib/utils';
	import type { FeelingEntry } from '$lib/types';

	let entries = $state<FeelingEntry[]>([]);
	let loading = $state(true);

	let date = $state(today());
	let content = $state('');
	let moodScore = $state<number | undefined>(undefined);
	let submitting = $state(false);

	onMount(async () => {
		entries = await listFeelings();
		loading = false;
	});

	async function handleSubmit() {
		if (!content.trim()) return;
		submitting = true;
		try {
			const entry = await createFeeling({
				date,
				content: content.trim(),
				mood_score: moodScore || undefined
			});
			entries = [entry, ...entries];
			content = '';
			moodScore = undefined;
			date = today();
		} finally {
			submitting = false;
		}
	}

	async function handleDelete(id: number) {
		await deleteFeeling(id);
		entries = entries.filter((e) => e.id !== id);
	}
</script>

<div class="max-w-3xl">
	<h2 class="text-2xl font-bold mb-6">Diario de Estado de Ánimo</h2>

	<form onsubmit={handleSubmit} class="bg-bg-secondary rounded-lg p-5 border border-border mb-6">
		<div class="grid grid-cols-1 sm:grid-cols-2 gap-4 mb-4">
			<div>
				<label for="date" class="block text-sm text-text-muted mb-1">Fecha</label>
				<input id="date" type="date" bind:value={date}
					class="w-full bg-bg-tertiary border border-border rounded px-3 py-2 text-sm text-text-primary focus:outline-none focus:border-accent" />
			</div>
			<div>
				<label for="mood" class="block text-sm text-text-muted mb-1">
					Nivel de ánimo (opcional): {moodScore ?? '-'}/10
				</label>
				<input id="mood" type="range" min="1" max="10" bind:value={moodScore}
					class="w-full accent-accent" />
			</div>
		</div>
		<div class="mb-4">
			<label for="content" class="block text-sm text-text-muted mb-1">¿Cómo te sentís?</label>
			<textarea
				id="content"
				bind:value={content}
				rows="4"
				placeholder="Escribí sobre tu día, pensamientos, niveles de energía..."
				class="w-full bg-bg-tertiary border border-border rounded px-3 py-2 text-sm text-text-primary focus:outline-none focus:border-accent resize-none"
			></textarea>
		</div>
		<button type="submit" disabled={submitting || !content.trim()}
			class="bg-accent hover:bg-accent-hover text-white px-4 py-2 rounded text-sm font-medium disabled:opacity-50 transition-colors">
			{submitting ? 'Guardando...' : 'Guardar Entrada'}
		</button>
	</form>

	{#if loading}
		<p class="text-text-muted text-sm">Cargando...</p>
	{:else if entries.length === 0}
		<p class="text-text-muted text-sm text-center py-8">No hay entradas aún. ¡Empezá a escribir tu diario!</p>
	{:else}
		<div class="space-y-3">
			{#each entries as entry (entry.id)}
				<div class="bg-bg-secondary rounded-lg p-5 border border-border">
					<div class="flex items-center justify-between mb-2">
						<div class="flex items-center gap-3">
							<span class="text-sm text-text-muted">{formatDate(entry.date)}</span>
							{#if entry.mood_score}
								<span class="text-xs bg-bg-tertiary text-text-secondary px-2 py-0.5 rounded-full">
									Ánimo: {entry.mood_score}/10
								</span>
							{/if}
						</div>
						<button
							onclick={() => handleDelete(entry.id!)}
							class="text-text-muted hover:text-danger text-sm transition-colors"
						>
							Eliminar
						</button>
					</div>
					<p class="text-sm text-text-primary whitespace-pre-line">{entry.content}</p>
				</div>
			{/each}
		</div>
	{/if}
</div>
