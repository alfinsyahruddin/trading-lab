<script lang="ts">
	import { onMount } from 'svelte';
	import { goto } from '$app/navigation';
	import Icon from '@iconify/svelte';
	import TextField from '#lib/components/TextField.svelte';
	import ComingSoonModal from '#lib/components/ComingSoonModal.svelte';
	import { register, getCaptcha } from '#lib/api.js';
	import { ApiError } from '#lib/api.js';
	import { toast } from '#lib/helpers/toast.svelte.js';
	import { isComingSoon } from '#lib/helpers/config.js';

	let name = $state('');
	let email = $state('');
	let password = $state('');
	let confirmPassword = $state('');
	let captchaId = $state('');
	let captchaImage = $state('');
	let captchaCode = $state('');
	let captchaLoading = $state(false);
	let loading = $state(false);
	let error = $state('');
	let showComingSoonModal = $state(false);

	// Field-level validation errors
	let nameError = $state('');
	let emailError = $state('');
	let passwordError = $state('');
	let confirmError = $state('');
	let captchaError = $state('');

	async function loadCaptcha() {
		captchaLoading = true;
		try {
			const res = await getCaptcha();
			captchaId = res.id;
			captchaImage = res.image;
			captchaCode = '';
			captchaError = '';
		} catch {
			toast.error('Failed to load captcha. Please refresh.');
		} finally {
			captchaLoading = false;
		}
	}

	onMount(() => {
		loadCaptcha();
	});

	function validate(): boolean {
		nameError = '';
		emailError = '';
		passwordError = '';
		confirmError = '';
		captchaError = '';
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
		if (!captchaCode.trim()) {
			captchaError = 'Captcha code is required.';
			valid = false;
		}
		return valid;
	}

	async function handleSubmit(e: Event) {
		e.preventDefault();
		error = '';

		if (!validate()) return;

		if (isComingSoon()) {
			showComingSoonModal = true;
			return;
		}

		loading = true;
		try {
			await register(
				name.trim(),
				email.trim().toLowerCase(),
				password,
				captchaId,
				captchaCode.trim()
			);
			toast.success('Account created! Please sign in.');
			await goto('/login');
		} catch (err) {
			if (err instanceof ApiError) {
				error = err.message;
			} else {
				error = 'An unexpected error occurred. Please try again.';
			}
			await loadCaptcha();
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
			class="absolute -top-40 left-1/2 size-96 -translate-x-1/2 rounded-full opacity-10 sm:size-125"
			style="background: radial-gradient(circle, var(--accent) 0%, transparent 70%);"
		></div>
	</div>

	<div class="relative z-10 w-full max-w-sm">
		<!-- Logo -->
		<div class="mb-6 flex flex-col items-center sm:mb-8">
			<a href="/" class="transition-opacity duration-150 hover:opacity-85">
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
			</a>
		</div>

		<!-- Card -->
		<div
			class="rounded-2xl border p-6 shadow-lg sm:p-8"
			style="background-color: var(--bg-card); border-color: var(--border);"
		>
			<h1 class="font-700 mb-1 text-xl" style="color: var(--fg)">Create account</h1>
			<p class="mb-6 text-sm" style="color: var(--fg-muted)">Join Trading Lab today.</p>

			<form onsubmit={handleSubmit} class="flex flex-col gap-4">
				<TextField
					label="Full Name"
					type="text"
					placeholder="Tokyo"
					bind:value={name}
					error={nameError}
					required
				/>
				<TextField
					label="Email"
					type="email"
					placeholder="tokyo@mail.com"
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

				<!-- Captcha Verification -->
				<div class="flex flex-col gap-1.5">
					<span class="font-500 text-sm" style="color: var(--fg-muted)">
						Security Check <span style="color: var(--danger)">*</span>
					</span>
					<div class="flex items-center gap-2">
						<div
							class="flex h-12 flex-1 items-center justify-center overflow-hidden rounded-lg border bg-white/5"
							style="border-color: var(--border);"
						>
							{#if captchaLoading}
								<Icon
									icon="lucide:loader-2"
									class="text-accent animate-spin"
									width="20"
									height="20"
								/>
							{:else if captchaImage}
								<img
									src={captchaImage}
									alt="Security Captcha"
									class="size-full object-contain select-none"
								/>
							{:else}
								<span class="text-xs" style="color: var(--fg-muted)">Captcha unavailable</span>
							{/if}
						</div>
						<button
							type="button"
							onclick={loadCaptcha}
							disabled={captchaLoading}
							class="btn-interactive flex size-12 shrink-0 items-center justify-center rounded-lg border transition-colors hover:border-(--accent) hover:text-(--accent)"
							style="border-color: var(--border); color: var(--fg-muted);"
							title="Refresh Captcha"
							aria-label="Refresh Captcha"
						>
							<Icon
								icon="lucide:refresh-cw"
								class={captchaLoading ? 'animate-spin' : ''}
								width="18"
								height="18"
							/>
						</button>
					</div>
					<TextField
						placeholder="Enter code above"
						bind:value={captchaCode}
						error={captchaError}
						required
					/>
				</div>

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
					class="btn-interactive font-700 active:scale-0.98 mt-1 w-full rounded-xl py-3 text-sm text-white shadow-sm hover:shadow-md"
					style="background-color: var(--accent); opacity: {loading ? '0.7' : '1'};"
				>
					{#if loading}
						<span class="flex items-center justify-center gap-2">
							<Icon icon="lucide:loader-2" class="animate-spin" width="16" height="16" />
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

	<ComingSoonModal bind:open={showComingSoonModal} />
</div>
