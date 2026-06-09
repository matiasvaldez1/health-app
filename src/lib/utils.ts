export function today(): string {
	return new Date().toISOString().split('T')[0];
}

export function nowTime(): string {
	return new Date().toTimeString().slice(0, 5);
}

export function daysAgo(n: number): string {
	const d = new Date();
	d.setDate(d.getDate() - n);
	return d.toISOString().split('T')[0];
}

export function formatDate(dateStr: string): string {
	const d = new Date(dateStr + 'T00:00:00');
	return d.toLocaleDateString('es-AR', { day: 'numeric', month: 'short', year: 'numeric' });
}

export function formatTime(timeStr: string): string {
	const [h, m] = timeStr.split(':').map(Number);
	const ampm = h >= 12 ? 'PM' : 'AM';
	const hour = h % 12 || 12;
	return `${hour}:${m.toString().padStart(2, '0')} ${ampm}`;
}
