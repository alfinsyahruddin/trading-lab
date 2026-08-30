<script lang="ts">
	import type { UserRole } from '$lib/types';

	let { role }: { role: UserRole } = $props();
</script>

<span
	class="inline-flex items-center rounded-full px-2.5 py-0.5 text-xs font-600 tracking-wide"
	style:background-color={role === 'ADMIN' ? 'var(--accent-soft)' : 'rgba(100,116,139,0.12)'}
	style:color={role === 'ADMIN' ? 'var(--accent)' : 'var(--fg-muted)'}
	style:border={role === 'ADMIN' ? '1px solid rgba(48,180,201,0.3)' : '1px solid var(--border)'}
>
	{role}
</span>
