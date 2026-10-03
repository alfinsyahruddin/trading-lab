import '@testing-library/jest-dom/vitest';
import { cleanup } from '@testing-library/svelte';
import { afterEach, beforeEach, vi } from 'vitest';
import { setStrategyMetadata } from '$lib/helpers/strategy-variables.svelte';
import { mockStrategyMetadata } from '../fixtures/strategy-metadata';

class MockIntersectionObserver {
	readonly root: Element | Document | null = null;
	readonly rootMargin: string = '';
	readonly thresholds: ReadonlyArray<number> = [];
	observe = vi.fn();
	unobserve = vi.fn();
	disconnect = vi.fn();
	takeRecords = vi.fn().mockReturnValue([]);
}

Object.defineProperty(window, 'IntersectionObserver', {
	writable: true,
	configurable: true,
	value: MockIntersectionObserver
});

if (typeof Element !== 'undefined' && !Element.prototype.animate) {
	Element.prototype.animate = vi.fn().mockImplementation(() => {
		const anim = {
			onfinish: null as ((this: Animation, ev: AnimationPlaybackEvent) => unknown) | null,
			cancel: vi.fn(),
			finish: vi.fn()
		};
		queueMicrotask(() => {
			if (typeof anim.onfinish === 'function') {
				anim.onfinish.call(anim as unknown as Animation, {} as AnimationPlaybackEvent);
			}
		});
		return anim as unknown as Animation;
	});
}

beforeEach(() => {
	setStrategyMetadata(mockStrategyMetadata);
});

afterEach(() => {
	cleanup();
});
