<script lang="ts">
	import { page } from '$app/stores';
	import { goto } from '$app/navigation';
	import Icon from '@iconify/svelte';
	import ThemeToggle from './ThemeToggle.svelte';
	import RoleBadge from './RoleBadge.svelte';
	import EditProfileModal from './EditProfileModal.svelte';
	import ChangePasswordModal from './ChangePasswordModal.svelte';
	import { getUser, clearSession, getToken } from '$lib/helpers/session';
	import { logout } from '$lib/api';
	import { onMount } from 'svelte';
	import type { UserResponse } from '$lib/types';

	let user = $state<UserResponse | null>(null);
	let mobileMenuOpen = $state(false);
	let userMenuOpen = $state(false);
	let loggingOut = $state(false);

	let editProfileOpen = $state(false);
	let changePasswordOpen = $state(false);

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
		if (mobileMenuOpen) userMenuOpen = false;
	}

	function toggleUserMenu() {
		userMenuOpen = !userMenuOpen;
		if (userMenuOpen) mobileMenuOpen = false;
	}

	function openEditProfile() {
		userMenuOpen = false;
		mobileMenuOpen = false;
		editProfileOpen = true;
	}

	function openChangePassword() {
		userMenuOpen = false;
		mobileMenuOpen = false;
		changePasswordOpen = true;
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

	// Close menus when clicking outside or pressing Escape
	function handleWindowClick(e: MouseEvent) {
		const target = e.target as HTMLElement;
		if (!target.closest('[data-mobile-menu]') && !target.closest('[data-mobile-menu-btn]')) {
			mobileMenuOpen = false;
		}
		if (!target.closest('[data-user-menu]') && !target.closest('[data-user-menu-btn]')) {
			userMenuOpen = false;
		}
	}

	function handleKeydown(e: KeyboardEvent) {
		if (e.key === 'Escape') {
			mobileMenuOpen = false;
			userMenuOpen = false;
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

		<!-- Right: User info + Avatar Popover + Mobile hamburger -->
		<div class="flex items-center gap-2 sm:gap-3">
			<!-- User info: Name + compact RoleBadge below (desktop only) -->
			{#if user}
				<div class="hidden flex-col items-end gap-1 sm:flex text-right">
					<p class="text-sm font-600 leading-none" style="color: var(--fg)">{user.name}</p>
					<RoleBadge role={user.role} size="sm" />
				</div>
			{/if}

			<!-- Avatar Popover Wrapper -->
			<div class="relative">
				<!-- Avatar Button (Neutral Glass Style) -->
				<button
					type="button"
					onclick={toggleUserMenu}
					data-user-menu-btn
					class="avatar-glass btn-interactive flex h-8 w-8 sm:h-9 sm:w-9 shrink-0 items-center justify-center rounded-full border shadow-xs transition-all duration-150 active:scale-95 cursor-pointer"
					title={user ? `${user.name} (${user.email})` : 'User menu'}
					aria-label={user ? `User menu for ${user.name}` : 'User menu'}
					aria-haspopup="menu"
					aria-expanded={userMenuOpen}
				>
					<Icon icon="lucide:user" width="16" height="16" />
				</button>

				<!-- User Popover Menu -->
				{#if userMenuOpen}
					<div
						data-user-menu
						role="menu"
						class="animate-dropdown absolute right-0 top-full mt-2 w-60 rounded-xl border shadow-xl z-50 overflow-hidden py-1"
						style="background-color: var(--bg-card); border-color: var(--border-strong);"
					>
						<!-- User Details Header -->
						{#if user}
							<div class="border-b px-4 py-3" style="border-color: var(--border);">
								<p class="text-sm font-600 truncate" style="color: var(--fg)">{user.name}</p>
								<p class="text-xs truncate font-400 mt-0.5" style="color: var(--fg-muted)">
									{user.email}
								</p>
								<div class="mt-2">
									<RoleBadge role={user.role} size="sm" />
								</div>
							</div>
						{/if}

						<!-- Menu Actions -->
						<div class="py-1">
							<button
								type="button"
								role="menuitem"
								onclick={openEditProfile}
								class="btn-interactive flex w-full items-center gap-2.5 px-4 py-2 text-sm font-500 transition-colors duration-150 hover:bg-(--bg-card-hover) text-left"
								style="color: var(--fg);"
							>
								<Icon
									icon="lucide:user-pen"
									width="16"
									height="16"
									style="color: var(--fg-muted);"
								/>
								<span>Edit Profile</span>
							</button>

							<button
								type="button"
								role="menuitem"
								onclick={openChangePassword}
								class="btn-interactive flex w-full items-center gap-2.5 px-4 py-2 text-sm font-500 transition-colors duration-150 hover:bg-(--bg-card-hover) text-left"
								style="color: var(--fg);"
							>
								<Icon
									icon="lucide:key-round"
									width="16"
									height="16"
									style="color: var(--fg-muted);"
								/>
								<span>Change Password</span>
							</button>
						</div>

						<div class="border-t my-1" style="border-color: var(--border);"></div>

						<!-- Logout Action -->
						<div class="py-1">
							<button
								type="button"
								role="menuitem"
								onclick={handleLogout}
								disabled={loggingOut}
								class="btn-interactive flex w-full items-center gap-2.5 px-4 py-2 text-sm font-500 transition-colors duration-150 hover:bg-(--bg-card-hover) text-left disabled:opacity-50"
								style="color: var(--danger);"
							>
								{#if loggingOut}
									<Icon icon="lucide:loader-2" class="animate-spin" width="16" height="16" />
									<span>Logging out…</span>
								{:else}
									<Icon icon="lucide:log-out" width="16" height="16" />
									<span>Logout</span>
								{/if}
							</button>
						</div>
					</div>
				{/if}
			</div>

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

				<div class="mt-3 flex flex-col gap-1 border-t pt-2" style="border-color: var(--border);">
					<button
						type="button"
						onclick={openEditProfile}
						class="btn-interactive flex items-center gap-2.5 rounded-lg px-3 py-2 text-sm font-500 transition-colors duration-150 hover:bg-(--bg-card-hover) text-left"
						style="color: var(--fg);"
					>
						<Icon icon="lucide:user-pen" width="16" height="16" style="color: var(--fg-muted);" />
						<span>Edit Profile</span>
					</button>

					<button
						type="button"
						onclick={openChangePassword}
						class="btn-interactive flex items-center gap-2.5 rounded-lg px-3 py-2 text-sm font-500 transition-colors duration-150 hover:bg-(--bg-card-hover) text-left"
						style="color: var(--fg);"
					>
						<Icon icon="lucide:key-round" width="16" height="16" style="color: var(--fg-muted);" />
						<span>Change Password</span>
					</button>

					<button
						type="button"
						onclick={handleLogout}
						disabled={loggingOut}
						class="btn-interactive flex items-center gap-2.5 rounded-lg px-3 py-2 text-sm font-500 transition-colors duration-150 hover:bg-(--bg-card-hover) text-left disabled:opacity-50"
						style="color: var(--danger);"
					>
						{#if loggingOut}
							<Icon icon="lucide:loader-2" class="animate-spin" width="16" height="16" />
							<span>Logging out…</span>
						{:else}
							<Icon icon="lucide:log-out" width="16" height="16" />
							<span>Logout</span>
						{/if}
					</button>
				</div>
			{/if}
		</div>
	{/if}
</header>

<!-- Modals -->
<EditProfileModal
	bind:open={editProfileOpen}
	{user}
	onsuccess={(updated) => {
		user = updated;
	}}
/>

<ChangePasswordModal bind:open={changePasswordOpen} />

<style>
	/* Avatar neutral glass style (no blur) */
	.avatar-glass {
		background: rgba(0, 0, 0, 0.04);
		border-color: var(--border-strong);
		color: var(--fg);
		box-shadow:
			inset 0 1px 1px 0 rgba(255, 255, 255, 0.5),
			0 1px 2px 0 rgba(0, 0, 0, 0.05);
	}

	:global(.dark) .avatar-glass,
	:root.dark .avatar-glass {
		background: rgba(255, 255, 255, 0.08);
		border-color: rgba(255, 255, 255, 0.18);
		color: var(--fg);
		box-shadow:
			inset 0 1px 1px 0 rgba(255, 255, 255, 0.15),
			0 1px 2px 0 rgba(0, 0, 0, 0.25);
	}

	.avatar-glass:hover,
	.avatar-glass:focus-visible {
		background: rgba(0, 0, 0, 0.08);
		color: var(--fg);
	}

	:global(.dark) .avatar-glass:hover,
	:root.dark .avatar-glass:hover,
	:global(.dark) .avatar-glass:focus-visible,
	:root.dark .avatar-glass:focus-visible {
		background: rgba(255, 255, 255, 0.14);
		color: var(--fg);
	}

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
