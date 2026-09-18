<script lang="ts">
	import { onMount } from 'svelte';
	import Icon from '@iconify/svelte';
	import { formatDateShort, formatBacktestDateRange } from '$lib/helpers/date';

	let {
		label = '',
		startDate = $bindable<Date | null>(null),
		endDate = $bindable<Date | null>(null),
		minDate = null,
		maxDate = null,
		disabled = false,
		required = false,
		placeholder = 'Select date range',
		error = '',
		class: className = '',
		onchange
	}: {
		label?: string;
		startDate?: Date | null;
		endDate?: Date | null;
		minDate?: Date | null;
		maxDate?: Date | null;
		disabled?: boolean;
		required?: boolean;
		placeholder?: string;
		error?: string;
		class?: string;
		onchange?: (start: Date | null, end: Date | null) => void;
	} = $props();

	let isOpen = $state(false);
	let openUpwards = $state(false);
	let containerRef = $state<HTMLDivElement | null>(null);
	let triggerRef = $state<HTMLButtonElement | null>(null);
	let hoverDate = $state<Date | null>(null);

	// Selecting state: 'start' = waiting for start click, 'end' = waiting for end click
	let selectionStep = $state<'start' | 'end'>('start');

	// Navigation state for calendar month/year
	let viewYear = $state<number>(new Date().getFullYear());
	let viewMonth = $state<number>(new Date().getMonth()); // 0-11

	const monthNames = [
		'January',
		'February',
		'March',
		'April',
		'May',
		'June',
		'July',
		'August',
		'September',
		'October',
		'November',
		'December'
	];

	const dayNames = ['Su', 'Mo', 'Tu', 'We', 'Th', 'Fr', 'Sa'];

	const pickerId = $derived(
		label ? `date-range-${label.toLowerCase().replace(/\s+/g, '-')}` : 'date-range-picker'
	);

	function normalizeDate(d: Date): number {
		return new Date(d.getFullYear(), d.getMonth(), d.getDate()).getTime();
	}

	function toggleDropdown() {
		if (disabled) return;
		isOpen = !isOpen;
		if (isOpen) {
			hoverDate = null;
			// Reset selection step based on current state
			selectionStep = !startDate || (startDate && endDate) ? 'start' : 'end';
			// Navigate to start date's month, or minDate's month, or today
			const anchor = startDate ?? minDate;
			if (anchor) {
				viewYear = anchor.getFullYear();
				viewMonth = anchor.getMonth();
			} else {
				viewYear = new Date().getFullYear();
				viewMonth = new Date().getMonth();
			}

			// Determine if we should open upward
			const anchorEl = triggerRef || containerRef;
			if (anchorEl && typeof window !== 'undefined') {
				const rect = anchorEl.getBoundingClientRect();
				const spaceBelow = window.innerHeight - rect.bottom;
				const spaceAbove = rect.top;
				openUpwards = spaceBelow < 340 && spaceAbove > spaceBelow;
			}
		}
	}

	function prevMonth() {
		if (canPrevMonth) {
			if (viewMonth === 0) {
				viewMonth = 11;
				viewYear -= 1;
			} else {
				viewMonth -= 1;
			}
		}
	}

	function nextMonth() {
		if (canNextMonth) {
			if (viewMonth === 11) {
				viewMonth = 0;
				viewYear += 1;
			} else {
				viewMonth += 1;
			}
		}
	}

	const canPrevMonth = $derived.by(() => {
		if (!minDate) return true;
		const minYear = minDate.getFullYear();
		const minMonth = minDate.getMonth();
		return viewYear > minYear || (viewYear === minYear && viewMonth > minMonth);
	});

	const canNextMonth = $derived.by(() => {
		if (!maxDate) return true;
		const maxYear = maxDate.getFullYear();
		const maxMonth = maxDate.getMonth();
		return viewYear < maxYear || (viewYear === maxYear && viewMonth < maxMonth);
	});

	interface CalendarDay {
		date: Date;
		dayNumber: number;
		isCurrentMonth: boolean;
		isDisabled: boolean;
		isStart: boolean;
		isEnd: boolean;
		isInRange: boolean;
		isInHoverRange: boolean;
	}

	const calendarDays = $derived.by<CalendarDay[]>(() => {
		const days: CalendarDay[] = [];

		const firstDayOfMonth = new Date(viewYear, viewMonth, 1).getDay();
		const daysInCurrentMonth = new Date(viewYear, viewMonth + 1, 0).getDate();
		const daysInPrevMonth = new Date(viewYear, viewMonth, 0).getDate();

		// Previous month padding days
		for (let i = firstDayOfMonth - 1; i >= 0; i--) {
			const d = new Date(viewYear, viewMonth - 1, daysInPrevMonth - i);
			days.push({
				date: d,
				dayNumber: d.getDate(),
				isCurrentMonth: false,
				isDisabled: true,
				isStart: false,
				isEnd: false,
				isInRange: false,
				isInHoverRange: false
			});
		}

		// Current month days
		const minTime = minDate ? normalizeDate(minDate) : null;
		const maxTime = maxDate ? normalizeDate(maxDate) : null;
		const startTime = startDate ? normalizeDate(startDate) : null;
		const endTime = endDate ? normalizeDate(endDate) : null;
		const hoverTime = hoverDate ? normalizeDate(hoverDate) : null;

		for (let day = 1; day <= daysInCurrentMonth; day++) {
			const d = new Date(viewYear, viewMonth, day);
			const time = normalizeDate(d);

			const isDateDisabled =
				(minTime !== null && time < minTime) || (maxTime !== null && time > maxTime);
			const isStart = startTime !== null && time === startTime;
			const isEnd = endTime !== null && time === endTime;
			const isInRange =
				startTime !== null && endTime !== null && time > startTime && time < endTime;

			// Hover preview when waiting for end date
			let isInHoverRange = false;
			if (selectionStep === 'end' && startTime !== null && endTime === null && hoverTime !== null) {
				if (hoverTime >= startTime) {
					isInHoverRange = time > startTime && time <= hoverTime;
				} else {
					isInHoverRange = time >= hoverTime && time < startTime;
				}
			}

			days.push({
				date: d,
				dayNumber: day,
				isCurrentMonth: true,
				isDisabled: isDateDisabled,
				isStart,
				isEnd,
				isInRange,
				isInHoverRange
			});
		}

		// Pad next month days to always fill exactly 42 cells (6 rows × 7 cols)
		const remaining = 42 - days.length;
		for (let day = 1; day <= remaining; day++) {
			const d = new Date(viewYear, viewMonth + 1, day);
			days.push({
				date: d,
				dayNumber: day,
				isCurrentMonth: false,
				isDisabled: true,
				isStart: false,
				isEnd: false,
				isInRange: false,
				isInHoverRange: false
			});
		}

		return days;
	});

	function handleDayClick(day: CalendarDay) {
		if (day.isDisabled || !day.isCurrentMonth) return;

		const clickedDate = day.date;
		const clickedTime = normalizeDate(clickedDate);

		if (selectionStep === 'start') {
			// First click: set start date, wait for end
			startDate = clickedDate;
			endDate = null;
			hoverDate = null;
			selectionStep = 'end';
		} else {
			// Second click: set end date
			const startTime = startDate ? normalizeDate(startDate) : 0;

			if (clickedTime < startTime) {
				// Clicked before start — swap: new start, clear end
				startDate = clickedDate;
				endDate = null;
				hoverDate = null;
				selectionStep = 'end';
			} else if (clickedTime === startTime) {
				// Same day clicked again — reset to start selection
				startDate = null;
				endDate = null;
				hoverDate = null;
				selectionStep = 'start';
			} else {
				// Valid end date
				endDate = clickedDate;
				hoverDate = null;
				isOpen = false;
				onchange?.(startDate, endDate);
			}
		}
	}

	function handleDayHover(day: CalendarDay) {
		if (selectionStep !== 'end' || day.isDisabled || !day.isCurrentMonth) {
			hoverDate = null;
			return;
		}
		hoverDate = day.date;
	}

	function handleMouseLeaveCalendar() {
		hoverDate = null;
	}

	const displayRange = $derived.by(() => {
		if (startDate && endDate) {
			return formatBacktestDateRange(startDate, endDate);
		}
		if (startDate) {
			return `${formatDateShort(startDate)} → ...`;
		}
		return placeholder;
	});

	const selectionHint = $derived.by(() => {
		if (selectionStep === 'start') return 'Select start date';
		return 'Select end date';
	});

	onMount(() => {
		function handleClickOutside(e: MouseEvent) {
			if (containerRef && !containerRef.contains(e.target as Node)) {
				isOpen = false;
			}
		}

		function handleKeyDown(e: KeyboardEvent) {
			if (e.key === 'Escape' && isOpen) {
				isOpen = false;
			}
		}

		document.addEventListener('mousedown', handleClickOutside);
		document.addEventListener('keydown', handleKeyDown);
		return () => {
			document.removeEventListener('mousedown', handleClickOutside);
			document.removeEventListener('keydown', handleKeyDown);
		};
	});
</script>

<div
	class="relative flex w-full flex-col gap-1.5 {isOpen ? 'z-50' : 'z-0'} {className}"
	bind:this={containerRef}
>
	{#if label}
		<label for={pickerId} class="font-500 text-sm" style="color: var(--fg-muted)">
			{label}{#if required}<span style="color: var(--danger)"> *</span>{/if}
		</label>
	{/if}

	<div class="relative w-full">
		<button
			bind:this={triggerRef}
			type="button"
			id={pickerId}
			aria-expanded={isOpen}
			{disabled}
			onclick={toggleDropdown}
			class="group flex h-[42px] w-full items-center justify-between rounded-lg border px-3.5 py-2.5 text-left text-base transition-all duration-150 outline-none disabled:cursor-not-allowed sm:text-sm"
			style="
				background-color: {disabled
				? 'var(--bg-input-disabled, var(--bg))'
				: 'var(--bg-input, var(--bg))'}; 
				border-color: {error
				? 'var(--danger)'
				: isOpen
					? 'var(--accent)'
					: disabled
						? 'var(--border)'
						: 'var(--border-strong)'};
				color: {disabled ? 'var(--fg-muted)' : startDate || endDate ? 'var(--fg)' : 'var(--fg-muted)'};
				box-shadow: {isOpen ? '0 0 0 3px rgba(48,180,201,0.1)' : 'none'};
			"
		>
			<div class="flex items-center gap-2 truncate">
				<Icon
					icon="lucide:calendar"
					class="shrink-0 transition-colors {isOpen ? 'text-(--accent)' : 'text-(--fg-muted)'}"
					width="16"
					height="16"
				/>
				<span class="font-500 truncate">
					{displayRange}
				</span>
			</div>
			<Icon
				icon="lucide:chevron-down"
				class="shrink-0 transition-transform duration-200 {isOpen
					? 'rotate-180 text-(--accent)'
					: 'text-(--fg-muted)'}"
				width="16"
				height="16"
			/>
		</button>

		{#if isOpen}
			<div
				class="absolute {openUpwards
					? 'bottom-full mb-1'
					: 'top-full mt-1'} right-0 z-50 w-72 overflow-hidden rounded-xl border p-3.5 shadow-xl select-none"
				style="
					background-color: var(--bg-card);
					border-color: var(--border);
					box-shadow: 0 10px 25px -5px rgba(0, 0, 0, 0.2), 0 8px 10px -6px rgba(0, 0, 0, 0.1);
				"
			>
				<!-- Month & Year Navigation Header -->
				<div class="mb-2.5 flex items-center justify-between px-1">
					<button
						type="button"
						disabled={!canPrevMonth}
						onclick={prevMonth}
						class="btn-interactive flex size-7 items-center justify-center rounded-md border transition-colors disabled:cursor-not-allowed disabled:opacity-20"
						style="border-color: var(--border); color: var(--fg);"
						aria-label="Previous Month"
					>
						<Icon icon="lucide:chevron-left" width="15" height="15" />
					</button>

					<div class="font-600 text-sm" style="color: var(--fg)">
						{monthNames[viewMonth]}
						{viewYear}
					</div>

					<button
						type="button"
						disabled={!canNextMonth}
						onclick={nextMonth}
						class="btn-interactive flex size-7 items-center justify-center rounded-md border transition-colors disabled:cursor-not-allowed disabled:opacity-20"
						style="border-color: var(--border); color: var(--fg);"
						aria-label="Next Month"
					>
						<Icon icon="lucide:chevron-right" width="15" height="15" />
					</button>
				</div>

				<!-- Days of Week Header -->
				<div class="mb-1 grid grid-cols-7 text-center">
					{#each dayNames as dayName (dayName)}
						<div class="font-600 py-1 text-xs" style="color: var(--fg-muted)">
							{dayName}
						</div>
					{/each}
				</div>

				<!-- Calendar Grid -->
				<!-- svelte-ignore a11y_no_static_element_interactions -->
				<div class="grid grid-cols-7" onmouseleave={handleMouseLeaveCalendar}>
					{#each calendarDays as day (day.date.getTime())}
						<button
							type="button"
							disabled={day.isDisabled || !day.isCurrentMonth}
							onclick={() => handleDayClick(day)}
							onmouseenter={() => handleDayHover(day)}
							class="font-500 flex h-8 w-full items-center justify-center text-xs transition-colors
								{!day.isCurrentMonth ? 'pointer-events-none invisible' : ''}
								{day.isStart ? 'font-600 rounded-l-lg text-white' : ''}
								{day.isEnd ? 'font-600 rounded-r-lg text-white' : ''}
								{day.isStart && day.isEnd ? 'rounded-lg' : ''}
								{!day.isStart && !day.isEnd && (day.isInRange || day.isInHoverRange) ? 'rounded-none' : ''}
								{!day.isStart && !day.isEnd && !day.isInRange && !day.isInHoverRange && day.isCurrentMonth
								? 'rounded-lg hover:opacity-80'
								: ''}
								{day.isDisabled ? 'cursor-not-allowed opacity-25' : 'cursor-pointer'}
							"
							style="
								background-color: {day.isStart || day.isEnd
								? 'var(--accent)'
								: day.isInRange || day.isInHoverRange
									? 'var(--accent-soft)'
									: 'transparent'};
								color: {day.isStart || day.isEnd
								? '#ffffff'
								: day.isInRange || day.isInHoverRange
									? 'var(--accent)'
									: day.isDisabled
										? 'var(--fg-muted)'
										: 'var(--fg)'};
							"
						>
							{day.dayNumber}
						</button>
					{/each}
				</div>

				<!-- Selection hint -->
				<div
					class="mt-2.5 flex items-center gap-1.5 border-t pt-2.5"
					style="border-color: var(--border);"
				>
					<div
						class="size-1.5 rounded-full"
						style="background-color: {selectionStep === 'end'
							? 'var(--accent)'
							: 'var(--fg-muted)'};"
					></div>
					<span class="text-xs" style="color: var(--fg-muted)">{selectionHint}</span>
					{#if selectionStep === 'end' && startDate}
						<span class="ml-auto text-xs font-medium" style="color: var(--accent)">
							{formatDateShort(startDate)}
						</span>
					{/if}
				</div>
			</div>
		{/if}
	</div>

	{#if error}
		<span class="font-500 text-xs" style="color: var(--danger)">{error}</span>
	{/if}
</div>
