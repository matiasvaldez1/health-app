<script lang="ts">
	import '../app.css';
	import { page } from '$app/stores';

	let { children } = $props();

	const navItems = [
		{ href: '/', label: 'Inicio', icon: '◉' },
		{ href: '/weight', label: 'Peso', icon: '⚖' },
		{ href: '/meditation', label: 'Meditación', icon: '◎' },
		{ href: '/feelings', label: 'Estado de ánimo', icon: '♡' },
		{ href: '/tips', label: 'Tips IA', icon: '✦' },
		{ href: '/settings', label: 'Configuración', icon: '⚙' }
	];
</script>

<div class="flex h-screen">
	<nav class="w-56 bg-bg-secondary border-r border-border flex flex-col shrink-0">
		<div class="p-5 border-b border-border">
			<h1 class="text-lg font-semibold text-text-primary">Health Tracker</h1>
			<p class="text-xs text-text-muted mt-1">Tu compañero de bienestar</p>
		</div>

		<div class="flex-1 py-3">
			{#each navItems as item}
				<a
					href={item.href}
					class="flex items-center gap-3 px-5 py-2.5 text-sm transition-colors
						{$page.url.pathname === item.href
						? 'bg-accent-muted text-accent border-r-2 border-accent'
						: 'text-text-secondary hover:bg-bg-hover hover:text-text-primary'}"
				>
					<span class="text-base">{item.icon}</span>
					{item.label}
				</a>
			{/each}
		</div>

		<div class="p-4 border-t border-border">
			<p class="text-xs text-text-muted">v0.1.0</p>
		</div>
	</nav>

	<main class="flex-1 overflow-y-auto p-8">
		{@render children()}
	</main>
</div>
