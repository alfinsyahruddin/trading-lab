<script lang="ts">
	import { page } from '$app/stores';
	import { goto } from '$app/navigation';
	import ThemeToggle from './ThemeToggle.svelte';
	import RoleBadge from './RoleBadge.svelte';
	import { getUser, clearSession, getToken } from '$lib/helpers/session';
	import { logout } from '$lib/api';
	import { toast } from '$lib/helpers/toast';
	import { onMount } from 'svelte';
	import type { UserResponse } from '$lib/types';

	let user = $state<UserResponse | null>(null);
	let avatarMenuOpen = $state(false);
	let loggingOut = $state(false);

	onMount(() => {
		user = getUser();
	});

	const navItems = $derived([
		{ href: '/dashboard', label: 'Dashboard' },
		...(user?.role === 'ADMIN' ? [{ href: '/dashboard/users', label: 'Users' }] : [])
	]);

	function isActive(href: string): boolean {
		if (href === '/dashboard') {
			return $page.url.pathname === '/dashboard';
		}
		return $page.url.pathname.startsWith(href);
	}

	function toggleAvatarMenu() {
		avatarMenuOpen = !avatarMenuOpen;
	}

	async function handleLogout() {
		loggingOut = true;
		try {
			const token = getToken();
			if (token) {
				await logout(token);
			}
		} catch {
			// Proceed with logout even if server call fails
		} finally {
			clearSession();
			goto('/login');
		}
	}

	// Close avatar menu when clicking outside
	function handleWindowClick(e: MouseEvent) {
		const target = e.target as HTMLElement;
		if (!target.closest('[data-avatar-menu]')) {
			avatarMenuOpen = false;
		}
	}

	// Get initials for avatar
	function getInitials(name: string): string {
		return name
			.split(' ')
			.map((n) => n[0])
			.join('')
			.toUpperCase()
			.slice(0, 2);
	}
</script>

<svelte:window onclick={handleWindowClick} />

<header
	class="sticky top-0 z-40 border-b"
	style="background-color: var(--bg); border-color: var(--border);"
>
	<div class="mx-auto flex h-16 max-w-7xl items-center gap-6 px-6">
		<!-- Logo -->
		<a href="/dashboard" class="shrink-0">
			<img
				src="/logo-dark.svg"
				alt="Trading Lab"
				class="hidden h-7 dark:block"
				style="max-width: 140px;"
			/>
			<img
				src="/logo-light.svg"
				alt="Trading Lab"
				class="block h-7 dark:hidden"
				style="max-width: 140px;"
			/>
		</a>

		<!-- Spacer -->
		<div class="flex-1"></div>

		<!-- Nav -->
		<nav class="flex items-center gap-1">
			{#each navItems as item}
				<a
					href={item.href}
					class="rounded-lg px-3.5 py-2 text-sm font-500 transition-colors duration-150"
					class:active={isActive(item.href)}
					aria-current={isActive(item.href) ? 'page' : undefined}
				>
					{item.label}
				</a>
			{/each}
		</nav>

		<!-- Spacer -->
		<div class="flex-1"></div>

		<!-- Right: User info + Theme toggle + Avatar -->
		<div class="flex items-center gap-3">
			<!-- User info -->
			{#if user}
				<div class="hidden items-center gap-2 sm:flex">
					<div class="text-right">
						<p class="text-sm font-600 leading-tight" style="color: var(--fg)">{user.name}</p>
						<p class="text-xs leading-tight" style="color: var(--fg-muted)">{user.email}</p>
					</div>
					<RoleBadge role={user.role} />
				</div>
			{/if}

			<!-- Theme toggle -->
			<ThemeToggle />

			<!-- Avatar with dropdown -->
			<div class="relative" data-avatar-menu>
				<button
					onclick={toggleAvatarMenu}
					class="flex h-9 w-9 items-center justify-center rounded-full text-sm font-700 text-white transition-opacity duration-150 hover:opacity-80"
					style="background-color: var(--accent);"
					aria-label="User menu"
					aria-expanded={avatarMenuOpen}
				>
					{user ? getInitials(user.name) : '?'}
				</button>

				<!-- Dropdown -->
				{#if avatarMenuOpen}
					<div
						class="absolute right-0 top-full mt-2 w-36 overflow-hidden rounded-xl border shadow-xl"
						style="background-color: var(--bg-card); border-color: var(--border-strong);"
					>
						<button
							onclick={handleLogout}
							disabled={loggingOut}
							class="flex w-full items-center gap-2 px-4 py-2.5 text-left text-sm font-500 transition-colors duration-150 hover:bg-(--bg-card-hover)"
							style="color: var(--danger);"
						>
							<svg
								width="14"
								height="14"
								viewBox="0 0 24 24"
								fill="none"
								stroke="currentColor"
								stroke-width="2.5"
							>
								<path d="M9 21H5a2 2 0 01-2-2V5a2 2 0 012-2h4M16 17l5-5-5-5M21 12H9" />
							</svg>
							{loggingOut ? 'Logging out…' : 'Logout'}
						</button>
					</div>
				{/if}
			</div>
		</div>
	</div>
</header>

<style>
	/* Nav link default (inactive) */
	a {
		color: var(--fg-muted);
		background-color: transparent;
	}

	/* Nav link active */
	a.active {
		color: var(--accent);
		background-color: var(--accent-soft);
	}

	/* Nav link hover (inactive only) */
	a:not(.active):hover {
		color: var(--fg);
		background-color: var(--bg-card-hover);
	}
</style>
