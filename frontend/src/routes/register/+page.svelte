<script lang="ts">
	import { goto } from '$app/navigation';
	import TextField from '$lib/components/TextField.svelte';
	import { register } from '$lib/api';
	import { ApiError } from '$lib/api';
	import { toast } from '$lib/helpers/toast';

	let name = $state('');
	let email = $state('');
	let password = $state('');
	let confirmPassword = $state('');
	let loading = $state(false);
	let error = $state('');

	// Field-level validation errors
	let nameError = $state('');
	let emailError = $state('');
	let passwordError = $state('');
	let confirmError = $state('');

	function validate(): boolean {
		nameError = '';
		emailError = '';
		passwordError = '';
		confirmError = '';
		let valid = true;

		if (!name.trim()) {
			nameError = 'Name is required.';
			valid = false;
		}
		if (!email.trim() || !email.includes('@')) {
			emailError = 'A valid email is required.';
			valid = false;
		}
		if (password.length < 8) {
			passwordError = 'Password must be at least 8 characters.';
			valid = false;
		}
		if (password !== confirmPassword) {
			confirmError = 'Passwords do not match.';
			valid = false;
		}
		return valid;
	}

	async function handleSubmit(e: Event) {
		e.preventDefault();
		error = '';

		if (!validate()) return;

		loading = true;
		try {
			await register(name.trim(), email.trim().toLowerCase(), password);
			toast.success('Account created! Please sign in.');
			await goto('/login');
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
	class="flex min-h-screen items-center justify-center px-6 py-12"
	style="background: linear-gradient(135deg, var(--bg) 0%, var(--bg-card) 100%);"
>
	<!-- Background glow -->
	<div class="pointer-events-none absolute inset-0 overflow-hidden" aria-hidden="true">
		<div
			class="absolute -top-40 left-1/2 h-125 w-125 -translate-x-1/2 rounded-full opacity-10"
			style="background: radial-gradient(circle, var(--accent) 0%, transparent 70%);"
		></div>
	</div>

	<div class="relative z-10 w-full max-w-sm">
		<!-- Logo -->
		<div class="mb-8 flex flex-col items-center">
			<img src="/favicon.svg" alt="Trading Lab" class="mb-4 h-14 w-14" />
			<img
				src="/logo-dark.svg"
				alt="Trading Lab"
				class="hidden h-7 dark:block"
				style="max-width: 160px;"
			/>
			<img
				src="/logo-light.svg"
				alt="Trading Lab"
				class="block h-7 dark:hidden"
				style="max-width: 160px;"
			/>
		</div>

		<!-- Card -->
		<div
			class="rounded-2xl border p-8 shadow-lg"
			style="background-color: var(--bg-card); border-color: var(--border);"
		>
			<h1 class="mb-1 text-xl font-700" style="color: var(--fg)">Create account</h1>
			<p class="mb-6 text-sm" style="color: var(--fg-muted)">Join Trading Lab today.</p>

			<form onsubmit={handleSubmit} class="flex flex-col gap-4">
				<TextField
					label="Full Name"
					type="text"
					placeholder="John Doe"
					bind:value={name}
					error={nameError}
					required
				/>
				<TextField
					label="Email"
					type="email"
					placeholder="you@example.com"
					bind:value={email}
					error={emailError}
					required
				/>
				<TextField
					label="Password"
					type="password"
					placeholder="Min. 8 characters"
					bind:value={password}
					error={passwordError}
					required
				/>
				<TextField
					label="Confirm Password"
					type="password"
					placeholder="Repeat your password"
					bind:value={confirmPassword}
					error={confirmError}
					required
				/>

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
					class="mt-1 w-full rounded-xl py-3 text-sm font-700 text-white transition-opacity duration-150"
					style="background-color: var(--accent); opacity: {loading ? '0.7' : '1'};"
				>
					{#if loading}
						<span class="flex items-center justify-center gap-2">
							<svg class="h-4 w-4 animate-spin" viewBox="0 0 24 24" fill="none">
								<circle
									class="opacity-25"
									cx="12"
									cy="12"
									r="10"
									stroke="currentColor"
									stroke-width="4"
								></circle>
								<path
									class="opacity-75"
									fill="currentColor"
									d="M4 12a8 8 0 018-8V0C5.373 0 0 5.373 0 12h4z"
								></path>
							</svg>
							Creating account…
						</span>
					{:else}
						Create Account
					{/if}
				</button>
			</form>
		</div>

		<!-- Login link -->
		<p class="mt-6 text-center text-sm" style="color: var(--fg-muted)">
			Already have an account?
			<a
				href="/login"
				class="font-600 transition-colors duration-150"
				style="color: var(--accent);"
			>
				Sign in
			</a>
		</p>
	</div>
</div>
