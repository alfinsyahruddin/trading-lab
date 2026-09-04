import { describe, it, expect, vi, beforeEach } from 'vitest';
import { reveal, cleanupRevealEngine } from '../../../src/lib/helpers/reveal';

const mockObserve = vi.fn();
const mockUnobserve = vi.fn();
const mockDisconnect = vi.fn();

class MockIntersectionObserver {
	observe = mockObserve;
	unobserve = mockUnobserve;
	disconnect = mockDisconnect;
}

Object.defineProperty(globalThis, 'IntersectionObserver', {
	value: MockIntersectionObserver,
	configurable: true
});

describe('reveal helper', () => {
	beforeEach(() => {
		cleanupRevealEngine();
		mockObserve.mockClear();
		mockUnobserve.mockClear();
		mockDisconnect.mockClear();
	});

	it('adds reveal-on-scroll class and observes node', () => {
		const el = document.createElement('div');
		const action = reveal(el, { delay: 100, y: 30 });

		expect(el.classList.contains('reveal-on-scroll')).toBe(true);
		expect(el.style.getPropertyValue('--reveal-y')).toBe('30px');
		expect(el.style.transitionDelay).toBe('100ms');
		expect(mockObserve).toHaveBeenCalledWith(el);

		action.destroy();
		expect(mockUnobserve).toHaveBeenCalledWith(el);
	});

	it('cleans up observer when cleanupRevealEngine is called', () => {
		const el = document.createElement('div');
		reveal(el);
		expect(mockObserve).toHaveBeenCalledWith(el);

		cleanupRevealEngine();
		expect(mockDisconnect).toHaveBeenCalled();
	});
});
