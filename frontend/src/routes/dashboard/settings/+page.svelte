<script lang="ts">
	import { onMount } from 'svelte';
	import { goto } from '$app/navigation';
	import Icon from '@iconify/svelte';
	import IosSwitch from '$lib/components/IosSwitch.svelte';
	import SelectField from '$lib/components/SelectField.svelte';
	import { getSettings, updateSettings, ApiError } from '$lib/api';
	import { getToken, getUser } from '$lib/helpers/session';
	import { toast } from '$lib/helpers/toast.svelte';

	let aiEnabled = $state(false);
	let loading = $state(true);
	let updating = $state(false);
	let error = $state('');

	let selectedProvider = $state('gemini');
	let selectedModel = $state('gemini-3.1-flash-lite');

	const providerOptions = [{ label: 'Gemini', value: 'gemini' }];

	const modelOptions = [{ label: 'gemini-3.1-flash-lite', value: 'gemini-3.1-flash-lite' }];

	onMount(async () => {
		const currentUser = getUser();
		if (!currentUser || currentUser.role !== 'ADMIN') {
			toast.error('Access restricted to Administrators only.');
			goto('/dashboard');
			return;
		}

		await loadSettings();
	});

	async function loadSettings() {
		loading = true;
		error = '';
		try {
			const token = getToken();
			if (!token) {
				goto('/login');
				return;
			}

			const settings = await getSettings(token);
			aiEnabled = settings.ai_enabled;
		} catch (err) {
			error = err instanceof ApiError ? err.message : 'Failed to load settings';
			toast.error(error);
		} finally {
			loading = false;
		}
	}

	async function handleToggle(newChecked: boolean) {
		if (updating) return;
		const prev = aiEnabled;
		aiEnabled = newChecked;
		updating = true;

		try {
			const token = getToken();
			if (!token) {
				goto('/login');
				return;
			}

			const updated = await updateSettings(token, { ai_enabled: newChecked });
			aiEnabled = updated.ai_enabled;
			toast.success(newChecked ? 'AI capabilities enabled.' : 'AI capabilities disabled.');
		} catch (err) {
			aiEnabled = prev;
			const msg = err instanceof ApiError ? err.message : 'Failed to update settings';
			toast.error(msg);
		} finally {
			updating = false;
		}
	}
</script>

<!-- Breadcrumb / Back Link -->
<div class="mb-4">
	<a
		href="/dashboard"
		class="btn-interactive font-600 inline-flex items-center gap-1.5 text-xs transition-colors duration-150"
		style="color: var(--fg-muted);"
	>
		<Icon icon="lucide:arrow-left" width="14" height="14" />
		<span>Back to Dashboard</span>
	</a>
</div>

<!-- Header -->
<div class="mb-6">
	<div class="flex items-center gap-3">
		<div
			class="flex size-10 items-center justify-center rounded-xl"
			style="background-color: var(--accent-soft); color: var(--accent);"
		>
			<Icon icon="lucide:settings-2" width="22" height="22" />
		</div>
		<div>
			<h1 class="font-700 text-xl sm:text-2xl" style="color: var(--fg)">System Settings</h1>
			<p class="mt-0.5 text-xs sm:text-sm" style="color: var(--fg-muted)">
				Manage platform-wide configuration and AI intelligence.
			</p>
		</div>
	</div>
</div>

{#if loading}
	<div class="flex min-h-60 items-center justify-center">
		<div class="flex flex-col items-center gap-3">
			<Icon icon="lucide:loader-2" class="text-accent animate-spin" width="32" height="32" />
			<p class="font-500 text-sm" style="color: var(--fg-muted)">Loading settings…</p>
		</div>
	</div>
{:else}
	<div class="flex flex-col gap-6">
		<div
			class="flex flex-col rounded-2xl border p-5 shadow-xs sm:p-6"
			style="background-color: var(--bg-card); border-color: var(--border);"
		>
			<!-- Row 1: AI Enabled Switch -->
			<div class="flex flex-col gap-4 sm:flex-row sm:items-center sm:justify-between">
				<div class="flex items-start gap-3.5">
					<div
						class="flex size-10 shrink-0 items-center justify-center rounded-xl"
						style="background-color: var(--accent-soft); color: var(--accent);"
					>
						<svg
							width="20"
							height="20"
							viewBox="0 0 607 607"
							fill="none"
							xmlns="http://www.w3.org/2000/svg"
							style="color: var(--accent);"
						>
							<path
								d="M279.22 488.635C295.407 525.561 303.5 565.016 303.5 607C303.5 565.016 311.34 525.561 327.021 488.635C343.208 451.709 364.959 419.589 392.274 392.274C419.589 364.959 451.709 343.461 488.635 327.78C525.561 311.593 565.016 303.5 607 303.5C565.016 303.5 525.561 295.66 488.635 279.979C452.696 264.483 420.004 242.345 392.274 214.726C364.655 186.996 342.517 154.304 327.021 118.365C311.34 81.4392 303.5 41.9842 303.5 0C303.5 41.9842 295.407 81.4392 279.22 118.365C263.539 155.291 242.041 187.411 214.726 214.726C186.996 242.345 154.304 264.483 118.365 279.979C81.4392 295.66 41.9842 303.5 0 303.5C41.9842 303.5 81.4392 311.593 118.365 327.78C155.291 343.461 187.411 364.959 214.726 392.274C242.041 419.589 263.539 451.709 279.22 488.635Z"
								fill="currentColor"
							/>
						</svg>
					</div>
					<div>
						<div class="flex items-center gap-2">
							<h2 class="font-700 text-base sm:text-lg" style="color: var(--fg)">AI Enabled</h2>
							{#if aiEnabled}
								<span
									class="font-600 inline-flex items-center gap-1 rounded-full px-2 py-0.5 text-[11px]"
									style="background-color: rgba(34, 197, 94, 0.15); color: var(--success);"
								>
									<span class="size-1.5 rounded-full bg-current"></span>
									Active
								</span>
							{:else}
								<span
									class="font-600 inline-flex items-center gap-1 rounded-full px-2 py-0.5 text-[11px]"
									style="background-color: var(--bg-card-hover); color: var(--fg-muted);"
								>
									<span class="size-1.5 rounded-full bg-current"></span>
									Disabled
								</span>
							{/if}
						</div>
						<p class="mt-0.5 text-xs sm:text-sm" style="color: var(--fg-muted)">
							Enable AI-powered executive backtest summaries and trading strategy parameter
							suggestions.
						</p>
					</div>
				</div>

				<div class="flex items-center gap-3 self-end sm:self-center">
					{#if updating}
						<Icon icon="lucide:loader-2" class="text-accent animate-spin" width="18" height="18" />
					{/if}
					<IosSwitch
						checked={aiEnabled}
						disabled={updating}
						label="AI Enabled"
						onchange={handleToggle}
					/>
				</div>
			</div>

			<!-- Divider -->
			<div class="my-5 border-t" style="border-color: var(--border);"></div>

			<!-- Row 2: Disabled LLM Model & Model Name Dropdowns -->
			<div class="grid grid-cols-1 gap-4 sm:grid-cols-2">
				<SelectField
					label="LLM Model"
					bind:value={selectedProvider}
					options={providerOptions}
					disabled
				/>

				<SelectField
					label="Model Name"
					bind:value={selectedModel}
					options={modelOptions}
					disabled
				/>
			</div>
		</div>
	</div>
{/if}
