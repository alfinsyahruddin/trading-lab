<script lang="ts">
	import { page } from '$app/state';
	import { goto } from '$app/navigation';
	import Icon from '@iconify/svelte';
	import ThemeToggle from './ThemeToggle.svelte';
	import RoleBadge from './RoleBadge.svelte';
	import EditProfileModal from './EditProfileModal.svelte';
	import ChangePasswordModal from './ChangePasswordModal.svelte';
	import { getUser, clearSession, getToken } from '#lib/helpers/session.js';
	import { logout } from '#lib/api.js';
	import { onMount } from 'svelte';
	import type { UserResponse } from '#lib/types.js';

	let user = $state<UserResponse | null>(null);
	let mobileMenuOpen = $state(false);
	let userMenuOpen = $state(false);
	let loggingOut = $state(false);

	let editProfileOpen = $state(false);
	let changePasswordOpen = $state(false);

	let navElement = $state<HTMLElement | null>(null);
	let indicatorStyle = $state<{ left: number; width: number; opacity: number }>({
		left: 0,
		width: 0,
		opacity: 0
	});

	function updateIndicator() {
		if (!navElement) return;
		const activeEl = navElement.querySelector<HTMLElement>('a.active');
		if (activeEl && activeEl.offsetWidth > 0) {
			indicatorStyle = {
				left: activeEl.offsetLeft,
				width: activeEl.offsetWidth,
				opacity: 1
			};
		} else if (activeEl) {
			requestAnimationFrame(() => {
				const retryEl = navElement?.querySelector<HTMLElement>('a.active');
				if (retryEl && retryEl.offsetWidth > 0) {
					indicatorStyle = {
						left: retryEl.offsetLeft,
						width: retryEl.offsetWidth,
						opacity: 1
					};
				}
			});
		} else {
			indicatorStyle = {
				left: 0,
				width: 0,
				opacity: 0
			};
		}
	}

	$effect(() => {
		// Track route and nav item dependencies
		const _path = page.url.pathname;
		const _items = navItems;
		if (typeof window !== 'undefined') {
			requestAnimationFrame(() => {
				updateIndicator();
				requestAnimationFrame(() => {
					updateIndicator();
				});
			});
		}
	});

	onMount(() => {
		user = getUser();
		updateIndicator();

		let resizeObserver: ResizeObserver | null = null;
		if (typeof window !== 'undefined') {
			if (document.fonts) {
				document.fonts.ready.then(() => {
					updateIndicator();
				});
			}
			if (navElement && typeof ResizeObserver !== 'undefined') {
				resizeObserver = new ResizeObserver(() => {
					updateIndicator();
				});
				resizeObserver.observe(navElement);
				for (const child of navElement.children) {
					resizeObserver.observe(child);
				}
			}
		}

		return () => {
			resizeObserver?.disconnect();
		};
	});

	const navItems = $derived([
		{ href: '/dashboard', label: 'Dashboard', icon: 'lucide:home' },
		{
			href: '/dashboard/strategies',
			label: 'Trading Strategy',
			icon: 'lucide:candlestick-chart'
		},

		{
			href: '/dashboard/backtests',
			label: 'Backtest',
			icon: 'lucide:flask-conical'
		},

		...(user?.role === 'ADMIN'
			? [
					{ href: '/dashboard/users', label: 'Users', icon: 'lucide:users' },
					{ href: '/dashboard/settings', label: 'Settings', icon: 'lucide:settings' }
				]
			: [])
	]);

	function isActive(href: string): boolean {
		if (href === '/dashboard') {
			return page.url.pathname === '/dashboard';
		}
		return page.url.pathname.startsWith(href);
	}

	function toggleMobileMenu(e?: MouseEvent) {
		e?.stopPropagation();
		mobileMenuOpen = !mobileMenuOpen;
		if (mobileMenuOpen) userMenuOpen = false;
	}

	function toggleUserMenu(e?: MouseEvent) {
		e?.stopPropagation();
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
		const target = e.target as HTMLElement | null;
		if (!target) return;

		const path = e.composedPath ? e.composedPath() : [];
		const isMobileClick =
			path.some(
				(el) =>
					el instanceof HTMLElement &&
					(el.hasAttribute('data-mobile-menu') || el.hasAttribute('data-mobile-menu-btn'))
			) ||
			Boolean(target.closest('[data-mobile-menu]')) ||
			Boolean(target.closest('[data-mobile-menu-btn]'));

		if (!isMobileClick) {
			mobileMenuOpen = false;
		}

		const isUserClick =
			path.some(
				(el) =>
					el instanceof HTMLElement &&
					(el.hasAttribute('data-user-menu') || el.hasAttribute('data-user-menu-btn'))
			) ||
			Boolean(target.closest('[data-user-menu]')) ||
			Boolean(target.closest('[data-user-menu-btn]'));

		if (!isUserClick) {
			userMenuOpen = false;
		}
	}

	function handleKeydown(e: KeyboardEvent) {
		if (e.key === 'Escape') {
			mobileMenuOpen = false;
			userMenuOpen = false;
		}
	}
	function handleResize() {
		updateIndicator();
		if (typeof window !== 'undefined' && window.innerWidth >= 1024) {
			mobileMenuOpen = false;
		}
	}
</script>

<svelte:window onclick={handleWindowClick} onkeydown={handleKeydown} onresize={handleResize} />

<header
	class="sticky top-0 z-40 border-b backdrop-blur-md"
	style="background-color: var(--bg-card); border-color: var(--border);"
>
	<div class="mx-auto flex h-16 max-w-7xl items-center justify-between gap-2 px-4 sm:gap-6 sm:px-6">
		<!-- Left: Logo + Theme toggle -->
		<div class="flex items-center gap-2 sm:gap-3">
			<!-- Logo (no hover background) -->
			<a href="/" class="shrink-0 bg-transparent transition-opacity duration-150 hover:opacity-85">
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

		<!-- Center: Desktop Nav with Sliding Rounded Bottom Indicator -->
		<nav
			bind:this={navElement}
			class="desktop-nav relative hidden h-full items-stretch gap-1 self-stretch lg:flex lg:gap-1.5"
			aria-label="Main Navigation"
		>
			{#each navItems as item (item.href)}
				<a
					href={item.href}
					class="desktop-nav-link flex items-center gap-2 px-3 text-sm font-semibold transition-colors duration-150 lg:px-3.5"
					class:active={isActive(item.href)}
					aria-current={isActive(item.href) ? 'page' : undefined}
				>
					<Icon icon={item.icon} width="16" height="16" class="nav-icon" />
					<span>{item.label}</span>
				</a>
			{/each}

			<!-- Sliding Rounded Bottom Accent Indicator -->
			<div
				class="nav-sliding-indicator"
				style="left: {indicatorStyle.left}px; width: {indicatorStyle.width}px; opacity: {indicatorStyle.opacity};"
			></div>
		</nav>

		<!-- Right: User info + Avatar Popover + Mobile hamburger -->
		<div class="flex items-center gap-2 sm:gap-3">
			<!-- User info: Name + compact RoleBadge below (desktop only) -->
			{#if user}
				<div class="hidden flex-col items-end gap-1 text-right lg:flex">
					<p class="font-600 text-sm leading-none" style="color: var(--fg)">{user.name}</p>
					<RoleBadge role={user.role} size="sm" />
				</div>
			{/if}

			<!-- Avatar Popover Wrapper (Desktop only) -->
			<div class="relative hidden lg:block">
				<!-- Avatar Button (Neutral Glass Style) -->
				<button
					type="button"
					onclick={toggleUserMenu}
					data-user-menu-btn
					class="avatar-glass btn-interactive flex size-9 shrink-0 cursor-pointer items-center justify-center rounded-full border shadow-xs transition-all duration-150 active:scale-95"
					title={user ? `${user.name} (${user.email})` : 'User menu'}
					aria-label={user ? `User menu for ${user.name}` : 'User menu'}
					aria-haspopup="menu"
					aria-expanded={userMenuOpen}
				>
					<Icon icon="lucide:user" width="16" height="16" class="pointer-events-none" />
				</button>

				<!-- User Popover Menu -->
				{#if userMenuOpen}
					<div
						data-user-menu
						role="menu"
						class="animate-dropdown absolute top-full right-0 z-50 mt-2 w-60 overflow-hidden rounded-xl border py-1 shadow-xl"
						style="background-color: var(--bg-card); border-color: var(--border-strong);"
					>
						<!-- User Details Header -->
						{#if user}
							<div class="border-b px-4 py-3" style="border-color: var(--border);">
								<p class="font-600 truncate text-sm" style="color: var(--fg)">{user.name}</p>
								<p class="font-400 mt-0.5 truncate text-xs" style="color: var(--fg-muted)">
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
								class="btn-interactive font-500 flex w-full items-center gap-2.5 px-4 py-2 text-left text-sm transition-colors duration-150 hover:bg-(--bg-card-hover)"
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
								class="btn-interactive font-500 flex w-full items-center gap-2.5 px-4 py-2 text-left text-sm transition-colors duration-150 hover:bg-(--bg-card-hover)"
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

						<div class="my-1 border-t" style="border-color: var(--border);"></div>

						<!-- Logout Action -->
						<div class="py-1">
							<button
								type="button"
								role="menuitem"
								onclick={handleLogout}
								disabled={loggingOut}
								class="btn-interactive font-500 flex w-full items-center gap-2.5 px-4 py-2 text-left text-sm transition-colors duration-150 hover:bg-(--bg-card-hover) disabled:opacity-50"
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
				class="btn-interactive flex size-9 items-center justify-center rounded-lg border transition-all duration-150 hover:bg-(--bg-card-hover) active:scale-90 lg:hidden"
				style="border-color: var(--border); color: var(--fg);"
				aria-label="Toggle navigation menu"
				aria-expanded={mobileMenuOpen}
			>
				<Icon
					icon={mobileMenuOpen ? 'lucide:x' : 'lucide:menu'}
					width="18"
					height="18"
					class="pointer-events-none"
				/>
			</button>
		</div>
	</div>

	<!-- Mobile Navigation Drawer / Dropdown -->
	{#if mobileMenuOpen}
		<div
			data-mobile-menu
			class="animate-dropdown border-t px-4 py-3 shadow-lg lg:hidden"
			style="background-color: var(--bg-card); border-color: var(--border);"
		>
			<nav class="mobile-nav flex flex-col gap-1">
				{#each navItems as item (item.href)}
					<a
						href={item.href}
						onclick={() => (mobileMenuOpen = false)}
						class="btn-interactive font-500 flex items-center gap-3 rounded-lg px-3 py-2.5 text-sm transition-colors duration-150"
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
						<span class="font-600 text-sm" style="color: var(--fg)">{user.name}</span>
						<span class="text-xs" style="color: var(--fg-muted)">{user.email}</span>
					</div>
					<RoleBadge role={user.role} size="sm" />
				</div>

				<div class="mt-3 flex flex-col gap-1 border-t pt-2" style="border-color: var(--border);">
					<button
						type="button"
						onclick={openEditProfile}
						class="btn-interactive font-500 flex items-center gap-2.5 rounded-lg px-3 py-2 text-left text-sm transition-colors duration-150 hover:bg-(--bg-card-hover)"
						style="color: var(--fg);"
					>
						<Icon icon="lucide:user-pen" width="16" height="16" style="color: var(--fg-muted);" />
						<span>Edit Profile</span>
					</button>

					<button
						type="button"
						onclick={openChangePassword}
						class="btn-interactive font-500 flex items-center gap-2.5 rounded-lg px-3 py-2 text-left text-sm transition-colors duration-150 hover:bg-(--bg-card-hover)"
						style="color: var(--fg);"
					>
						<Icon icon="lucide:key-round" width="16" height="16" style="color: var(--fg-muted);" />
						<span>Change Password</span>
					</button>

					<button
						type="button"
						onclick={handleLogout}
						disabled={loggingOut}
						class="btn-interactive font-500 flex items-center gap-2.5 rounded-lg px-3 py-2 text-left text-sm transition-colors duration-150 hover:bg-(--bg-card-hover) disabled:opacity-50"
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

	/* Desktop Nav - Clean sliding rounded bottom accent sticking to the bottom of the topbar */

	.desktop-nav-link {
		position: relative;
		display: flex;
		height: 100%;
		align-items: center;
		color: var(--fg-muted);
		background-color: transparent !important;
		font-weight: 600;
		text-decoration: none;
	}

	.desktop-nav-link :global(svg) {
		stroke-width: 2.35;
	}

	.desktop-nav-link.active {
		color: var(--accent);
		background-color: transparent !important;
	}

	.nav-sliding-indicator {
		position: absolute;
		bottom: -1px;
		height: 3px;
		border-radius: 9999px;
		background-color: var(--accent);
		pointer-events: none;
		will-change: left, width;
		transition:
			left 0.28s cubic-bezier(0.32, 0.72, 0, 1),
			width 0.28s cubic-bezier(0.32, 0.72, 0, 1),
			opacity 0.15s ease;
	}

	/* Mobile drawer navigation links */
	.mobile-nav a {
		color: var(--fg-muted);
		background-color: transparent;
	}

	.mobile-nav a.active {
		color: var(--accent);
		background-color: var(--accent-soft);
	}

	.mobile-nav a:not(.active):hover {
		color: var(--fg);
		background-color: var(--bg-card-hover);
	}
</style>
