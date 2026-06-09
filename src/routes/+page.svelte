<script lang="ts">
	import { onMount } from 'svelte';
	import { listWeights, listMeditations, listFeelings, getLatestTip } from '$lib/api';
	import { daysAgo, formatDate } from '$lib/utils';
	import type { WeightEntry, MeditationSession, FeelingEntry, AiTip } from '$lib/types';

	let weights = $state<WeightEntry[]>([]);
	let meditations = $state<MeditationSession[]>([]);
	let feelings = $state<FeelingEntry[]>([]);
	let latestTip = $state<AiTip | null>(null);

	const weekAgo = daysAgo(7);
	const monthAgo = daysAgo(30);

	let currentWeight = $derived(weights.length > 0 ? weights[0].weight_kg : null);
	let weightChange = $derived(
		weights.length >= 2 ? weights[0].weight_kg - weights[weights.length - 1].weight_kg : null
	);

	let meditationsThisWeek = $derived(
		meditations.filter((m) => m.date >= weekAgo).length
	);
	let meditationMinutesThisWeek = $derived(
		meditations.filter((m) => m.date >= weekAgo).reduce((sum, m) => sum + m.duration_min, 0)
	);

	let meditationStreak = $derived.by(() => {
		if (meditations.length === 0) return 0;
		const dates = [...new Set(meditations.map((m) => m.date))].sort().reverse();
		let streak = 0;
		const todayDate = new Date();
		for (let i = 0; i < dates.length; i++) {
			const expected = new Date(todayDate);
			expected.setDate(expected.getDate() - i);
			const expectedStr = expected.toISOString().split('T')[0];
			if (dates[i] === expectedStr) {
				streak++;
			} else {
				break;
			}
		}
		return streak;
	});

	let latestFeeling = $derived(feelings.length > 0 ? feelings[0] : null);

	let sortedWeights = $derived([...weights].reverse());
	let weightMin = $derived(sortedWeights.length > 0 ? Math.min(...sortedWeights.map(w => w.weight_kg)) : 0);
	let weightMax = $derived(sortedWeights.length > 0 ? Math.max(...sortedWeights.map(w => w.weight_kg)) : 0);
	let weightRange = $derived(weightMax - weightMin || 1);

	onMount(async () => {
		const [w, m, f, t] = await Promise.all([
			listWeights(monthAgo),
			listMeditations(monthAgo),
			listFeelings(weekAgo),
			getLatestTip()
		]);
		weights = w;
		meditations = m;
		feelings = f;
		latestTip = t;
	});
</script>

<div class="max-w-5xl">
	<h2 class="text-2xl font-bold mb-6">Inicio</h2>

	<!-- Acciones rápidas -->
	<div class="flex gap-3 mb-6">
		<a href="/weight" class="bg-accent hover:bg-accent-hover text-white px-4 py-2 rounded-lg text-sm font-medium transition-colors">
			+ Registrar Peso
		</a>
		<a href="/meditation" class="bg-accent hover:bg-accent-hover text-white px-4 py-2 rounded-lg text-sm font-medium transition-colors">
			+ Registrar Meditación
		</a>
		<a href="/feelings" class="bg-accent hover:bg-accent-hover text-white px-4 py-2 rounded-lg text-sm font-medium transition-colors">
			+ Registrar Ánimo
		</a>
		<a href="/tips" class="bg-bg-tertiary hover:bg-bg-hover text-text-secondary px-4 py-2 rounded-lg text-sm font-medium transition-colors border border-border">
			Ver Tips IA
		</a>
	</div>

	<div class="grid grid-cols-1 md:grid-cols-3 gap-4 mb-8">
		<a href="/weight" class="bg-bg-secondary rounded-lg p-5 border border-border hover:border-accent/50 transition-colors">
			<div class="flex items-center justify-between mb-3">
				<span class="text-text-muted text-sm">Peso actual</span>
				<span class="text-lg">⚖</span>
			</div>
			{#if currentWeight}
				<p class="text-3xl font-bold">{currentWeight.toFixed(1)} <span class="text-lg text-text-muted">kg</span></p>
				{#if weightChange !== null}
					<p class="text-sm mt-1 {weightChange > 0 ? 'text-warning' : weightChange < 0 ? 'text-accent' : 'text-text-muted'}">
						{weightChange > 0 ? '+' : ''}{weightChange.toFixed(1)} kg (30d)
					</p>
				{/if}
			{:else}
				<p class="text-text-muted text-sm">Sin registros aún</p>
			{/if}
		</a>

		<a href="/meditation" class="bg-bg-secondary rounded-lg p-5 border border-border hover:border-accent/50 transition-colors">
			<div class="flex items-center justify-between mb-3">
				<span class="text-text-muted text-sm">Meditación (esta semana)</span>
				<span class="text-lg">◎</span>
			</div>
			<p class="text-3xl font-bold">{meditationsThisWeek} <span class="text-lg text-text-muted">sesiones</span></p>
			<p class="text-sm text-text-secondary mt-1">
				{meditationMinutesThisWeek} min total · racha de {meditationStreak} días
			</p>
		</a>

		<a href="/feelings" class="bg-bg-secondary rounded-lg p-5 border border-border hover:border-accent/50 transition-colors">
			<div class="flex items-center justify-between mb-3">
				<span class="text-text-muted text-sm">Último estado de ánimo</span>
				<span class="text-lg">♡</span>
			</div>
			{#if latestFeeling}
				<p class="text-sm text-text-primary line-clamp-2">{latestFeeling.content}</p>
				<p class="text-xs text-text-muted mt-2">
					{formatDate(latestFeeling.date)}
					{#if latestFeeling.mood_score}
						· Ánimo: {latestFeeling.mood_score}/10
					{/if}
				</p>
			{:else}
				<p class="text-text-muted text-sm">Sin registros aún</p>
			{/if}
		</a>
	</div>

	{#if weights.length > 1}
		<div class="bg-bg-secondary rounded-lg p-5 border border-border mb-6">
			<h3 class="text-sm text-text-muted mb-4">Tendencia de peso (últimos 30 días)</h3>
			<div class="flex items-end gap-1 h-32">
				{#each sortedWeights as w}
					<div class="flex-1 flex flex-col items-center justify-end h-full" title="{formatDate(w.date)}: {w.weight_kg} kg">
						<div
							class="w-full bg-accent/60 rounded-t min-h-[4px]"
							style="height: {((w.weight_kg - weightMin) / weightRange) * 100 + 10}%"
						></div>
					</div>
				{/each}
			</div>
			<div class="flex justify-between mt-2">
				<span class="text-xs text-text-muted">{formatDate(weights[weights.length - 1].date)}</span>
				<span class="text-xs text-text-muted">{formatDate(weights[0].date)}</span>
			</div>
		</div>
	{/if}

	{#if latestTip}
		<div class="bg-bg-secondary rounded-lg p-5 border border-border">
			<div class="flex items-center justify-between mb-3">
				<h3 class="text-sm text-text-muted">Últimos Tips IA</h3>
				<a href="/tips" class="text-xs text-accent hover:text-accent-hover">Ver todos</a>
			</div>
			<p class="text-sm text-text-secondary whitespace-pre-line line-clamp-4">{latestTip.response}</p>
		</div>
	{:else}
		<div class="bg-bg-secondary rounded-lg p-5 border border-border text-center">
			<p class="text-text-muted text-sm">No hay tips IA aún.</p>
			<a href="/tips" class="text-accent text-sm hover:text-accent-hover">Generá tus primeros tips</a>
		</div>
	{/if}
</div>
