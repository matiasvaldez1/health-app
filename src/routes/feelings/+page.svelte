<script lang="ts">
	import { onMount } from 'svelte';
	import { listFeelings, createFeeling, updateFeeling, deleteFeeling } from '$lib/api';
	import { today, formatDate } from '$lib/utils';
	import type { FeelingEntry } from '$lib/types';

	let entries = $state<FeelingEntry[]>([]);
	let loading = $state(true);

	let date = $state(today());
	let content = $state('');
	let moodScore = $state<number | undefined>(undefined);
	let submitting = $state(false);

	// Edit
	let editingId = $state<number | null>(null);
	let editDate = $state('');
	let editContent = $state('');
	let editMood = $state<number | undefined>(undefined);

	// Delete confirm
	let confirmDeleteId = $state<number | null>(null);
	let deleteTimer = $state<ReturnType<typeof setTimeout> | null>(null);

	// Mood chart
	let moodEntries = $derived(
		[...entries].filter(e => e.mood_score != null).reverse()
	);
	let moodMin = $derived(moodEntries.length > 0 ? Math.min(...moodEntries.map(e => e.mood_score!)) : 1);
	let moodMax = $derived(moodEntries.length > 0 ? Math.max(...moodEntries.map(e => e.mood_score!)) : 10);
	let moodRange = $derived(moodMax - moodMin || 1);

	onMount(async () => {
		entries = await listFeelings();
		loading = false;
	});

	async function handleSubmit() {
		if (!content.trim()) return;
		submitting = true;
		try {
			const entry = await createFeeling({ date, content: content.trim(), mood_score: moodScore || undefined });
			entries = [entry, ...entries];
			content = '';
			moodScore = undefined;
			date = today();
		} finally {
			submitting = false;
		}
	}

	function startEdit(entry: FeelingEntry) {
		editingId = entry.id!;
		editDate = entry.date;
		editContent = entry.content;
		editMood = entry.mood_score ?? undefined;
	}

	async function saveEdit(id: number) {
		const updated = await updateFeeling(id, { date: editDate, content: editContent.trim(), mood_score: editMood || undefined });
		entries = entries.map(e => e.id === id ? updated : e);
		editingId = null;
	}

	function requestDelete(id: number) {
		confirmDeleteId = id;
		if (deleteTimer) clearTimeout(deleteTimer);
		deleteTimer = setTimeout(() => { confirmDeleteId = null; }, 3000);
	}

	async function confirmDelete(id: number) {
		if (deleteTimer) clearTimeout(deleteTimer);
		confirmDeleteId = null;
		await deleteFeeling(id);
		entries = entries.filter(e => e.id !== id);
	}
</script>

<div class="max-w-3xl">
	<h2 class="text-2xl font-bold mb-6">Diario de Estado de Ánimo</h2>

	<form onsubmit={handleSubmit} class="bg-bg-secondary rounded-lg p-5 border border-border mb-6">
		<div class="grid grid-cols-1 sm:grid-cols-2 gap-4 mb-4">
			<div>
				<label for="date" class="block text-sm text-text-muted mb-1">Fecha</label>
				<input id="date" type="date" bind:value={date} class="w-full bg-bg-tertiary border border-border rounded px-3 py-2 text-sm text-text-primary focus:outline-none focus:border-accent" />
			</div>
			<div>
				<label for="mood" class="block text-sm text-text-muted mb-1">Nivel de ánimo (opcional): {moodScore ?? '-'}/10</label>
				<input id="mood" type="range" min="1" max="10" bind:value={moodScore} class="w-full accent-accent" />
			</div>
		</div>
		<div class="mb-4">
			<label for="content" class="block text-sm text-text-muted mb-1">¿Cómo te sentís?</label>
			<textarea id="content" bind:value={content} rows="4" placeholder="Escribí sobre tu día, pensamientos, niveles de energía..." class="w-full bg-bg-tertiary border border-border rounded px-3 py-2 text-sm text-text-primary focus:outline-none focus:border-accent resize-none"></textarea>
		</div>
		<button type="submit" disabled={submitting || !content.trim()} class="bg-accent hover:bg-accent-hover text-white px-4 py-2 rounded text-sm font-medium disabled:opacity-50 transition-colors">
			{submitting ? 'Guardando...' : 'Guardar Entrada'}
		</button>
	</form>

	<!-- Mood Chart -->
	{#if moodEntries.length >= 2}
		<div class="bg-bg-secondary rounded-lg p-5 border border-border mb-6">
			<h3 class="text-sm text-text-muted mb-4">Tendencia de ánimo</h3>
			<div class="flex items-end gap-1 h-32">
				{#each moodEntries as e}
					<div class="flex-1 flex flex-col items-center justify-end h-full group" title="{formatDate(e.date)}: {e.mood_score}/10">
						<div class="w-full rounded-t min-h-[4px] transition-colors {e.mood_score! >= 7 ? 'bg-accent/60 group-hover:bg-accent' : e.mood_score! >= 4 ? 'bg-warning/60 group-hover:bg-warning' : 'bg-danger/60 group-hover:bg-danger'}" style="height: {((e.mood_score! - moodMin) / moodRange) * 100 + 10}%"></div>
					</div>
				{/each}
			</div>
			<div class="flex justify-between mt-2">
				<span class="text-xs text-text-muted">{formatDate(moodEntries[0].date)}</span>
				<span class="text-xs text-text-muted">{formatDate(moodEntries[moodEntries.length - 1].date)}</span>
			</div>
		</div>
	{/if}

	{#if loading}
		<p class="text-text-muted text-sm">Cargando...</p>
	{:else if entries.length === 0}
		<p class="text-text-muted text-sm text-center py-8">No hay entradas aún. ¡Empezá a escribir tu diario!</p>
	{:else}
		<div class="space-y-3">
			{#each entries as entry (entry.id)}
				{#if editingId === entry.id}
					<div class="bg-bg-secondary rounded-lg p-5 border border-accent">
						<div class="grid grid-cols-1 sm:grid-cols-2 gap-3 mb-3">
							<input type="date" bind:value={editDate} class="bg-bg-tertiary border border-border rounded px-2 py-1 text-sm text-text-primary" />
							<div class="flex items-center gap-2">
								<span class="text-xs text-text-muted">Ánimo: {editMood ?? '-'}/10</span>
								<input type="range" min="1" max="10" bind:value={editMood} class="flex-1 accent-accent" />
							</div>
						</div>
						<textarea bind:value={editContent} rows="3" class="w-full bg-bg-tertiary border border-border rounded px-2 py-1 text-sm text-text-primary resize-none mb-3"></textarea>
						<div class="flex gap-3">
							<button onclick={() => saveEdit(entry.id!)} class="text-accent text-sm font-medium">Guardar</button>
							<button onclick={() => (editingId = null)} class="text-text-muted text-sm">Cancelar</button>
						</div>
					</div>
				{:else}
					<div class="bg-bg-secondary rounded-lg p-5 border border-border">
						<div class="flex items-center justify-between mb-2">
							<div class="flex items-center gap-3">
								<span class="text-sm text-text-muted">{formatDate(entry.date)}</span>
								{#if entry.mood_score}
									<span class="text-xs bg-bg-tertiary text-text-secondary px-2 py-0.5 rounded-full">Ánimo: {entry.mood_score}/10</span>
								{/if}
							</div>
							<div class="flex items-center gap-3">
								<button onclick={() => startEdit(entry)} class="text-text-muted hover:text-accent text-sm transition-colors">Editar</button>
								{#if confirmDeleteId === entry.id}
									<span class="text-sm text-danger">¿Eliminar?</span>
									<button onclick={() => confirmDelete(entry.id!)} class="text-danger text-sm font-medium">Sí</button>
									<button onclick={() => (confirmDeleteId = null)} class="text-text-muted text-sm">No</button>
								{:else}
									<button onclick={() => requestDelete(entry.id!)} class="text-text-muted hover:text-danger text-sm transition-colors">Eliminar</button>
								{/if}
							</div>
						</div>
						<p class="text-sm text-text-primary whitespace-pre-line">{entry.content}</p>
					</div>
				{/if}
			{/each}
		</div>
	{/if}
</div>
