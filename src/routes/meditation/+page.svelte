<script lang="ts">
	import { onMount } from 'svelte';
	import { listMeditations, createMeditation, deleteMeditation } from '$lib/api';
	import { today, nowTime, formatDate, formatTime, daysAgo } from '$lib/utils';
	import type { MeditationSession } from '$lib/types';

	let sessions = $state<MeditationSession[]>([]);
	let loading = $state(true);

	let date = $state(today());
	let time = $state(nowTime());
	let durationMin = $state<number | undefined>(undefined);
	let notes = $state('');
	let submitting = $state(false);

	let weekAgo = daysAgo(7);
	let thisWeek = $derived(sessions.filter((s) => s.date >= weekAgo));
	let totalMinutes = $derived(thisWeek.reduce((sum, s) => sum + s.duration_min, 0));

	let meditationDays = $derived(new Set(sessions.map((s) => s.date)));

	onMount(async () => {
		sessions = await listMeditations();
		loading = false;
	});

	async function handleSubmit() {
		if (!durationMin || durationMin <= 0) return;
		submitting = true;
		try {
			const session = await createMeditation({
				date,
				time,
				duration_min: durationMin,
				notes: notes || undefined
			});
			sessions = [session, ...sessions];
			durationMin = undefined;
			notes = '';
			date = today();
			time = nowTime();
		} finally {
			submitting = false;
		}
	}

	async function handleDelete(id: number) {
		await deleteMeditation(id);
		sessions = sessions.filter((s) => s.id !== id);
	}

	function getMonthDays(): { date: string; inMonth: boolean }[] {
		const now = new Date();
		const year = now.getFullYear();
		const month = now.getMonth();
		const firstDay = new Date(year, month, 1);
		const lastDay = new Date(year, month + 1, 0);
		const startPad = firstDay.getDay();
		const days: { date: string; inMonth: boolean }[] = [];

		for (let i = 0; i < startPad; i++) {
			days.push({ date: '', inMonth: false });
		}
		for (let d = 1; d <= lastDay.getDate(); d++) {
			const dateStr = `${year}-${String(month + 1).padStart(2, '0')}-${String(d).padStart(2, '0')}`;
			days.push({ date: dateStr, inMonth: true });
		}
		return days;
	}
</script>

<div class="max-w-3xl">
	<h2 class="text-2xl font-bold mb-6">Registro de Meditación</h2>

	<form onsubmit={handleSubmit} class="bg-bg-secondary rounded-lg p-5 border border-border mb-6">
		<div class="grid grid-cols-2 sm:grid-cols-4 gap-4">
			<div>
				<label for="date" class="block text-sm text-text-muted mb-1">Fecha</label>
				<input id="date" type="date" bind:value={date}
					class="w-full bg-bg-tertiary border border-border rounded px-3 py-2 text-sm text-text-primary focus:outline-none focus:border-accent" />
			</div>
			<div>
				<label for="time" class="block text-sm text-text-muted mb-1">Hora</label>
				<input id="time" type="time" bind:value={time}
					class="w-full bg-bg-tertiary border border-border rounded px-3 py-2 text-sm text-text-primary focus:outline-none focus:border-accent" />
			</div>
			<div>
				<label for="duration" class="block text-sm text-text-muted mb-1">Duración (min)</label>
				<input id="duration" type="number" min="1" bind:value={durationMin} placeholder="15"
					class="w-full bg-bg-tertiary border border-border rounded px-3 py-2 text-sm text-text-primary focus:outline-none focus:border-accent" />
			</div>
			<div>
				<label for="notes" class="block text-sm text-text-muted mb-1">Notas</label>
				<input id="notes" type="text" bind:value={notes} placeholder="Respiración guiada..."
					class="w-full bg-bg-tertiary border border-border rounded px-3 py-2 text-sm text-text-primary focus:outline-none focus:border-accent" />
			</div>
		</div>
		<button type="submit" disabled={submitting || !durationMin}
			class="mt-4 bg-accent hover:bg-accent-hover text-white px-4 py-2 rounded text-sm font-medium disabled:opacity-50 transition-colors">
			{submitting ? 'Guardando...' : 'Registrar Sesión'}
		</button>
	</form>

	<div class="grid grid-cols-1 md:grid-cols-2 gap-4 mb-6">
		<div class="bg-bg-secondary rounded-lg p-5 border border-border">
			<h3 class="text-sm text-text-muted mb-3">Esta semana</h3>
			<div class="flex gap-6">
				<div>
					<p class="text-2xl font-bold">{thisWeek.length}</p>
					<p class="text-xs text-text-muted">sesiones</p>
				</div>
				<div>
					<p class="text-2xl font-bold">{totalMinutes}</p>
					<p class="text-xs text-text-muted">minutos</p>
				</div>
			</div>
		</div>

		<div class="bg-bg-secondary rounded-lg p-5 border border-border">
			<h3 class="text-sm text-text-muted mb-3">Este mes</h3>
			<div class="grid grid-cols-7 gap-1">
				{#each ['D', 'L', 'M', 'M', 'J', 'V', 'S'] as day}
					<span class="text-center text-xs text-text-muted">{day}</span>
				{/each}
				{#each getMonthDays() as { date: d, inMonth }}
					<div
						class="aspect-square rounded-sm flex items-center justify-center text-xs
							{!inMonth ? '' : meditationDays.has(d) ? 'bg-accent/60 text-white' : 'bg-bg-tertiary text-text-muted'}"
					>
						{#if inMonth}
							{parseInt(d.split('-')[2])}
						{/if}
					</div>
				{/each}
			</div>
		</div>
	</div>

	{#if loading}
		<p class="text-text-muted text-sm">Cargando...</p>
	{:else if sessions.length === 0}
		<p class="text-text-muted text-sm text-center py-8">No hay sesiones de meditación aún. ¡Empezá tu práctica!</p>
	{:else}
		<div class="space-y-2">
			{#each sessions as session (session.id)}
				<div class="bg-bg-secondary rounded-lg px-5 py-3 border border-border flex items-center justify-between">
					<div class="flex items-center gap-6">
						<span class="text-sm text-text-muted w-28">{formatDate(session.date)}</span>
						<span class="text-sm text-text-secondary">{formatTime(session.time)}</span>
						<span class="text-lg font-semibold">{session.duration_min} min</span>
						{#if session.notes}
							<span class="text-sm text-text-secondary">{session.notes}</span>
						{/if}
					</div>
					<button
						onclick={() => handleDelete(session.id!)}
						class="text-text-muted hover:text-danger text-sm transition-colors"
					>
						Eliminar
					</button>
				</div>
			{/each}
		</div>
	{/if}
</div>
