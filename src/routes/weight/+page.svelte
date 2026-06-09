<script lang="ts">
	import { onMount } from 'svelte';
	import { listWeights, createWeight, deleteWeight } from '$lib/api';
	import { today, formatDate } from '$lib/utils';
	import type { WeightEntry } from '$lib/types';

	let entries = $state<WeightEntry[]>([]);
	let loading = $state(true);

	let date = $state(today());
	let weightKg = $state<number | undefined>(undefined);
	let notes = $state('');
	let submitting = $state(false);

	let sorted = $derived([...entries].reverse());
	let chartMin = $derived(sorted.length > 0 ? Math.min(...sorted.map(w => w.weight_kg)) : 0);
	let chartMax = $derived(sorted.length > 0 ? Math.max(...sorted.map(w => w.weight_kg)) : 0);
	let chartRange = $derived(chartMax - chartMin || 1);

	onMount(async () => {
		entries = await listWeights();
		loading = false;
	});

	async function handleSubmit() {
		if (!weightKg || weightKg <= 0) return;
		submitting = true;
		try {
			const entry = await createWeight({ weight_kg: weightKg, date, notes: notes || undefined });
			entries = [entry, ...entries];
			weightKg = undefined;
			notes = '';
			date = today();
		} finally {
			submitting = false;
		}
	}

	async function handleDelete(id: number) {
		await deleteWeight(id);
		entries = entries.filter((e) => e.id !== id);
	}
</script>

<div class="max-w-3xl">
	<h2 class="text-2xl font-bold mb-6">Registro de Peso</h2>

	<form onsubmit={handleSubmit} class="bg-bg-secondary rounded-lg p-5 border border-border mb-6">
		<div class="grid grid-cols-1 sm:grid-cols-3 gap-4">
			<div>
				<label for="date" class="block text-sm text-text-muted mb-1">Fecha</label>
				<input
					id="date"
					type="date"
					bind:value={date}
					class="w-full bg-bg-tertiary border border-border rounded px-3 py-2 text-sm text-text-primary focus:outline-none focus:border-accent"
				/>
			</div>
			<div>
				<label for="weight" class="block text-sm text-text-muted mb-1">Peso (kg)</label>
				<input
					id="weight"
					type="number"
					step="0.1"
					min="0"
					bind:value={weightKg}
					placeholder="75.0"
					class="w-full bg-bg-tertiary border border-border rounded px-3 py-2 text-sm text-text-primary focus:outline-none focus:border-accent"
				/>
			</div>
			<div>
				<label for="notes" class="block text-sm text-text-muted mb-1">Notas (opcional)</label>
				<input
					id="notes"
					type="text"
					bind:value={notes}
					placeholder="Después del desayuno..."
					class="w-full bg-bg-tertiary border border-border rounded px-3 py-2 text-sm text-text-primary focus:outline-none focus:border-accent"
				/>
			</div>
		</div>
		<button
			type="submit"
			disabled={submitting || !weightKg}
			class="mt-4 bg-accent hover:bg-accent-hover text-white px-4 py-2 rounded text-sm font-medium disabled:opacity-50 transition-colors"
		>
			{submitting ? 'Guardando...' : 'Registrar Peso'}
		</button>
	</form>

	{#if entries.length > 1}
		<div class="bg-bg-secondary rounded-lg p-5 border border-border mb-6">
			<h3 class="text-sm text-text-muted mb-4">Evolución del peso</h3>
			<div class="flex items-end gap-1 h-40">
				{#each sorted as w}
					<div class="flex-1 flex flex-col items-center justify-end h-full group" title="{formatDate(w.date)}: {w.weight_kg} kg">
						<div
							class="w-full bg-accent/60 group-hover:bg-accent rounded-t min-h-[4px] transition-colors"
							style="height: {((w.weight_kg - chartMin) / chartRange) * 100 + 10}%"
						></div>
					</div>
				{/each}
			</div>
			<div class="flex justify-between mt-2">
				<span class="text-xs text-text-muted">{formatDate(entries[entries.length - 1].date)}</span>
				<span class="text-xs text-text-muted">{formatDate(entries[0].date)}</span>
			</div>
		</div>
	{/if}

	{#if loading}
		<p class="text-text-muted text-sm">Cargando...</p>
	{:else if entries.length === 0}
		<p class="text-text-muted text-sm text-center py-8">No hay registros de peso aún. ¡Registrá tu primer peso arriba!</p>
	{:else}
		<div class="space-y-2">
			{#each entries as entry (entry.id)}
				<div class="bg-bg-secondary rounded-lg px-5 py-3 border border-border flex items-center justify-between">
					<div class="flex items-center gap-6">
						<span class="text-sm text-text-muted w-28">{formatDate(entry.date)}</span>
						<span class="text-lg font-semibold">{entry.weight_kg.toFixed(1)} kg</span>
						{#if entry.notes}
							<span class="text-sm text-text-secondary">{entry.notes}</span>
						{/if}
					</div>
					<button
						onclick={() => handleDelete(entry.id!)}
						class="text-text-muted hover:text-danger text-sm transition-colors"
					>
						Eliminar
					</button>
				</div>
			{/each}
		</div>
	{/if}
</div>
