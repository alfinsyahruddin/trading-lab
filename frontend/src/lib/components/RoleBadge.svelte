<script lang="ts">
	import type { UserRole } from '$lib/types';

	let {
		role,
		size = 'md'
	}: {
		role: UserRole;
		size?: 'sm' | 'md';
	} = $props();
</script>

<span
	class="font-600 inline-flex items-center rounded-full tracking-wide {size === 'sm'
		? 'px-2 py-0 text-[10px]'
		: 'px-2.5 py-0.5 text-xs'}"
	style:background-color={role === 'ADMIN' ? 'var(--accent-soft)' : 'rgba(100,116,139,0.12)'}
	style:color={role === 'ADMIN' ? 'var(--accent)' : 'var(--fg-muted)'}
	style:border={role === 'ADMIN' ? '1px solid rgba(48,180,201,0.3)' : '1px solid var(--border)'}
>
	{role}
</span>
