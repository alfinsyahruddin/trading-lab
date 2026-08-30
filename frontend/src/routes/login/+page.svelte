<script lang="ts">
	import { onMount } from 'svelte';
	import { goto } from '$app/navigation';
	import Icon from '@iconify/svelte';
	import TextField from '$lib/components/TextField.svelte';
	import RoleBadge from '$lib/components/RoleBadge.svelte';
	import { login, ApiError } from '$lib/api';
	import {
		persistSession,
		getRememberedAccounts,
		saveRememberedAccount,
		removeRememberedAccount
	} from '$lib/helpers/session';
	import { toast } from '$lib/helpers/toast.svelte';
	import type { RememberedAccount } from '$lib/types';

	let email = $state('');
	let password = $state('');
	let rememberMe = $state(false);
	let loading = $state(false);
	let error = $state('');
	let loggingInEmail = $state('');
	let rememberedAccounts = $state<RememberedAccount[]>([]);

	onMount(() => {
		rememberedAccounts = getRememberedAccounts();
	});

	async function handleQuickLogin(acc: RememberedAccount) {
		if (loading) return;
		email = acc.email;
		error = '';

		if (!acc.password) {
			const pwdInput = document.getElementById('field-password') as HTMLInputElement | null;
			pwdInput?.focus();
			return;
		}

		password = acc.password;
		loading = true;
		loggingInEmail = acc.email;

		try {
			const res = await login(acc.email.trim().toLowerCase(), acc.password);
			persistSession(res.tokens.access_token, res.tokens.refresh_token, res.user);
			toast.success('Welcome back, ' + res.user.name + '!');
			await goto('/dashboard');
		} catch (err) {
			if (err instanceof ApiError) {
				error = err.message;
			} else {
				error = 'An unexpected error occurred. Please try again.';
			}
		} finally {
			loading = false;
			loggingInEmail = '';
		}
	}

	function handleRemoveAccount(e: MouseEvent, accountEmail: string) {
		e.stopPropagation();
		removeRememberedAccount(accountEmail);
		rememberedAccounts = getRememberedAccounts();
	}

	async function handleSubmit(e: Event) {
		e.preventDefault();
		error = '';
		loading = true;

		try {
			const res = await login(email.trim().toLowerCase(), password);
			persistSession(res.tokens.access_token, res.tokens.refresh_token, res.user);

			if (rememberMe) {
				saveRememberedAccount({
					name: res.user.name,
					email: res.user.email,
					role: res.user.role,
					password
				});
			}

			toast.success('Welcome back, ' + res.user.name + '!');
			await goto('/dashboard');
		} catch (err) {
			if (err instanceof ApiError) {
				error = err.message;
			} else {
				error = 'An unexpected error occurred. Please try again.';
			}
		} finally {
			loading = false;
		}
	}
</script>

<div
	class="flex min-h-screen items-center justify-center px-4 py-8 sm:px-6 sm:py-12"
	style="background: linear-gradient(135deg, var(--bg) 0%, var(--bg-card) 100%);"
>
	<!-- Background glow -->
	<div class="pointer-events-none absolute inset-0 overflow-hidden" aria-hidden="true">
		<div
			class="absolute -top-40 left-1/2 h-96 w-96 sm:h-125 sm:w-125 -translate-x-1/2 rounded-full opacity-10"
			style="background: radial-gradient(circle, var(--accent) 0%, transparent 70%);"
		></div>
	</div>

	<div class="relative z-10 w-full max-w-sm">
		<!-- Logo -->
		<div class="mb-6 sm:mb-8 flex flex-col items-center">
			<img
				src="/logo-dark.svg"
				alt="Trading Lab"
				class="logo-dark-theme h-7"
				style="max-width: 160px;"
			/>
			<img
				src="/logo-light.svg"
				alt="Trading Lab"
				class="logo-light-theme h-7"
				style="max-width: 160px;"
			/>
		</div>

		<!-- Card -->
		<div
			class="rounded-2xl border p-6 sm:p-8 shadow-lg"
			style="background-color: var(--bg-card); border-color: var(--border);"
		>
			<h1 class="mb-1 text-xl font-700" style="color: var(--fg)">Sign in</h1>
			<p class="mb-6 text-sm" style="color: var(--fg-muted)">Enter your credentials to continue.</p>

			<form onsubmit={handleSubmit} class="flex flex-col gap-4">
				<TextField
					label="Email"
					type="email"
					placeholder="tokyo@mail.com"
					bind:value={email}
					required
				/>
				<TextField
					label="Password"
					type="password"
					placeholder="••••••••"
					bind:value={password}
					required
				/>

				<!-- Remember me checkbox -->
				<label
					class="flex items-center gap-2 text-xs font-500 cursor-pointer select-none"
					style="color: var(--fg-muted)"
				>
					<input
						type="checkbox"
						bind:checked={rememberMe}
						class="h-4 w-4 rounded border transition-colors cursor-pointer accent-(--accent)"
						style="border-color: var(--border-strong);"
					/>
					<span>Remember me</span>
				</label>

				{#if error}
					<div
						class="rounded-lg border px-4 py-3 text-sm"
						style="background-color: rgba(239,68,68,0.08); border-color: rgba(239,68,68,0.3); color: var(--danger);"
					>
						{error}
					</div>
				{/if}

				<button
					type="submit"
					disabled={loading}
					class="btn-interactive mt-1 w-full rounded-xl py-3 text-sm font-700 text-white shadow-sm hover:shadow-md active:scale-[0.98]"
					style="background-color: var(--accent); opacity: {loading ? '0.7' : '1'};"
				>
					{#if loading}
						<span class="flex items-center justify-center gap-2">
							<Icon icon="lucide:loader-2" class="animate-spin" width="16" height="16" />
							Signing in…
						</span>
					{:else}
						Sign In
					{/if}
				</button>
			</form>

			<!-- Quick Login Section (hidden if empty) -->
			{#if rememberedAccounts.length > 0}
				<div class="mt-6 border-t pt-5" style="border-color: var(--border);">
					<p class="mb-3 text-xs font-600 uppercase tracking-wider" style="color: var(--fg-muted)">
						Quick Login
					</p>
					<div class="flex flex-col gap-2">
						{#each rememberedAccounts as acc (acc.email)}
							<div
								class="btn-interactive group flex items-center justify-between gap-3 rounded-xl border p-2.5 transition-all duration-150 hover:bg-(--bg-card-hover) cursor-pointer"
								class:opacity-60={loading && loggingInEmail !== acc.email}
								style="border-color: var(--border); background-color: var(--bg-input, var(--bg));"
								onclick={() => handleQuickLogin(acc)}
								role="button"
								tabindex="0"
								onkeydown={(e) => {
									if (e.key === 'Enter' || e.key === ' ') handleQuickLogin(acc);
								}}
							>
								<!-- Avatar + Email + Badge role -->
								<div class="flex items-center gap-2.5 min-w-0 flex-1">
									<div
										class="avatar-glass flex h-7 w-7 shrink-0 items-center justify-center rounded-full border shadow-xs text-xs font-600"
										style="color: var(--fg);"
									>
										{#if loggingInEmail === acc.email}
											<Icon icon="lucide:loader-2" class="animate-spin" width="13" height="13" />
										{:else}
											<Icon icon="lucide:user" width="13" height="13" />
										{/if}
									</div>
									<div class="flex items-center gap-2 min-w-0 flex-1">
										<span class="truncate text-xs font-600" style="color: var(--fg)">
											{acc.email}
										</span>
										<RoleBadge role={acc.role} size="sm" />
									</div>
								</div>

								<!-- X button to remove -->
								<button
									type="button"
									onclick={(e) => handleRemoveAccount(e, acc.email)}
									disabled={loading}
									class="btn-interactive flex h-6 w-6 shrink-0 items-center justify-center rounded-md text-sm transition-colors duration-150 hover:bg-(--bg-card) opacity-60 hover:opacity-100 disabled:pointer-events-none"
									style="color: var(--fg-muted);"
									title="Remove saved account"
									aria-label={`Remove ${acc.email} from saved accounts`}
								>
									<Icon icon="lucide:x" width="14" height="14" />
								</button>
							</div>
						{/each}
					</div>
				</div>
			{/if}
		</div>

		<!-- Register link -->
		<p class="mt-6 text-center text-sm" style="color: var(--fg-muted)">
			Don't have an account?
			<a
				href="/register"
				class="font-600 transition-colors duration-150"
				style="color: var(--accent);"
			>
				Create one
			</a>
		</p>
	</div>
</div>
