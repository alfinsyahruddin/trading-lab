<script lang="ts">
	import { page } from '$app/stores';
	import { goto } from '$app/navigation';
	import Icon from '@iconify/svelte';
	import ThemeToggle from './ThemeToggle.svelte';
	import RoleBadge from './RoleBadge.svelte';
	import { getUser, clearSession, getToken } from '$lib/helpers/session';
	import { logout } from '$lib/api';
	import { onMount } from 'svelte';
	import type { UserResponse } from '$lib/types';

	let user = $state<UserResponse | null>(null);
	let mobileMenuOpen = $state(false);
	let loggingOut = $state(false);

	onMount(() => {
		user = getUser();
	});

	const navItems = $derived([
		{ href: '/dashboard', label: 'Dashboard', icon: 'lucide:home' },
		{ href: '/dashboard/strategies', label: 'Trading Strategy', icon: 'lucide:candlestick-chart' },
		...(user?.role === 'ADMIN'
			? [{ href: '/dashboard/users', label: 'Users', icon: 'lucide:users' }]
			: [])
	]);

	function isActive(href: string): boolean {
		if (href === '/dashboard') {
			return $page.url.pathname === '/dashboard';
		}
		return $page.url.pathname.startsWith(href);
	}

	function toggleMobileMenu() {
		mobileMenuOpen = !mobileMenuOpen;
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

	// Close mobile menu when clicking outside or pressing Escape
	function handleWindowClick(e: MouseEvent) {
		const target = e.target as HTMLElement;
		if (!target.closest('[data-mobile-menu]') && !target.closest('[data-mobile-menu-btn]')) {
			mobileMenuOpen = false;
		}
	}

	function handleKeydown(e: KeyboardEvent) {
		if (e.key === 'Escape') {
			mobileMenuOpen = false;
		}
	}
</script>

<svelte:window onclick={handleWindowClick} onkeydown={handleKeydown} />

<header
	class="sticky top-0 z-40 border-b backdrop-blur-md"
	style="background-color: var(--bg-card); border-color: var(--border);"
>
	<div class="mx-auto flex h-16 max-w-7xl items-center justify-between gap-2 px-4 sm:gap-6 sm:px-6">
		<!-- Left: Logo + Theme toggle -->
		<div class="flex items-center gap-2 sm:gap-3">
			<!-- Logo (no hover background) -->
			<a
				href="/dashboard"
				class="shrink-0 bg-transparent transition-opacity duration-150 hover:opacity-85"
			>
				<img
					src="/logo-dark.svg"
					alt="Trading Lab"
					class="logo-dark-theme h-6 sm:h-7"
					style="max-width: 130px;"
				/>
				<img
					src="/logo-light.svg"
					alt="Trading Lab"
					class="logo-light-theme h-6 sm:h-7"
					style="max-width: 130px;"
				/>
			</a>

			<!-- Theme toggle -->
			<ThemeToggle />
		</div>

		<!-- Center: Desktop Nav -->
		<nav class="hidden items-center gap-1 sm:flex">
			{#each navItems as item}
				<a
					href={item.href}
					class="flex items-center gap-1.5 rounded-lg px-3.5 py-2 text-sm font-500 transition-colors duration-150"
					class:active={isActive(item.href)}
					aria-current={isActive(item.href) ? 'page' : undefined}
				>
					<Icon icon={item.icon} width="16" height="16" />
					<span>{item.label}</span>
				</a>
			{/each}
		</nav>

		<!-- Right: User info + Avatar + Logout + Mobile hamburger -->
		<div class="flex items-center gap-2 sm:gap-3">
			<!-- User info: Name + compact RoleBadge below (desktop only) -->
			{#if user}
				<div class="hidden flex-col items-end gap-1 sm:flex text-right">
					<p class="text-sm font-600 leading-none" style="color: var(--fg)">{user.name}</p>
					<RoleBadge role={user.role} size="sm" />
				</div>
			{/if}

			<!-- Avatar -->
			<div
				class="flex h-8 w-8 sm:h-9 sm:w-9 shrink-0 items-center justify-center rounded-full text-white shadow-xs"
				style="background-color: var(--accent);"
				title={user ? `${user.name} (${user.email})` : 'User avatar'}
				aria-label={user ? user.name : 'User avatar'}
			>
				<Icon icon="lucide:user" width="16" height="16" />
			</div>

			<!-- Logout Button -->
			<button
				type="button"
				onclick={handleLogout}
				disabled={loggingOut}
				class="btn-interactive flex h-8 w-8 sm:h-9 sm:w-9 items-center justify-center rounded-lg transition-colors duration-150 hover:bg-(--bg-card-hover) active:scale-95 disabled:opacity-50"
				style="color: var(--danger);"
				aria-label="Logout"
				title={loggingOut ? 'Logging out…' : 'Logout'}
			>
				{#if loggingOut}
					<Icon icon="lucide:loader-2" class="animate-spin" width="18" height="18" />
				{:else}
					<Icon icon="lucide:log-out" width="18" height="18" />
				{/if}
			</button>

			<!-- Mobile hamburger button -->
			<button
				type="button"
				onclick={toggleMobileMenu}
				data-mobile-menu-btn
				class="btn-interactive flex h-8 w-8 sm:h-9 sm:w-9 items-center justify-center rounded-lg border transition-all duration-150 sm:hidden hover:bg-(--bg-card-hover) active:scale-90"
				style="border-color: var(--border); color: var(--fg);"
				aria-label="Toggle navigation menu"
				aria-expanded={mobileMenuOpen}
			>
				<Icon icon={mobileMenuOpen ? 'lucide:x' : 'lucide:menu'} width="18" height="18" />
			</button>
		</div>
	</div>

	<!-- Mobile Navigation Drawer / Dropdown -->
	{#if mobileMenuOpen}
		<div
			data-mobile-menu
			class="animate-dropdown border-b px-4 py-3 sm:hidden shadow-lg"
			style="background-color: var(--bg-card); border-color: var(--border);"
		>
			<nav class="flex flex-col gap-1">
				{#each navItems as item}
					<a
						href={item.href}
						onclick={() => (mobileMenuOpen = false)}
						class="btn-interactive flex items-center gap-3 rounded-lg px-3 py-2.5 text-sm font-500 transition-colors duration-150"
						class:active={isActive(item.href)}
						aria-current={isActive(item.href) ? 'page' : undefined}
					>
						<Icon icon={item.icon} width="18" height="18" />
						<span>{item.label}</span>
					</a>
				{/each}
			</nav>

			{#if user}
				<div
					class="mt-3 flex items-center justify-between border-t pt-3"
					style="border-color: var(--border);"
				>
					<div class="flex flex-col">
						<span class="text-sm font-600" style="color: var(--fg)">{user.name}</span>
						<span class="text-xs" style="color: var(--fg-muted)">{user.email}</span>
					</div>
					<RoleBadge role={user.role} size="sm" />
				</div>
			{/if}
		</div>
	{/if}
</header>

<style>
	/* Nav link default (inactive) - scoped strictly to nav */
	nav a {
		color: var(--fg-muted);
		background-color: transparent;
	}

	/* Nav link active */
	nav a.active {
		color: var(--accent);
		background-color: var(--accent-soft);
	}

	/* Nav link hover (inactive only) */
	nav a:not(.active):hover {
		color: var(--fg);
		background-color: var(--bg-card-hover);
	}
</style>
