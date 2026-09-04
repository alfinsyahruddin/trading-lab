import { SvelteSet } from 'svelte/reactivity';

export interface RevealOptions {
	delay?: number;
	y?: number;
}

let revealObserver: IntersectionObserver | null = null;
const revealNodes = new SvelteSet<HTMLElement>();
let scrollListenerAttached = false;
let rafId: number | null = null;

function checkReveals() {
	if (typeof window === 'undefined') return;
	const vh = window.innerHeight || document.documentElement.clientHeight;
	const triggerBottom = vh * 0.94;
	const scrollY = window.scrollY || window.pageYOffset || document.documentElement.scrollTop || 0;
	const scrollHeight = Math.max(
		document.documentElement.scrollHeight,
		document.body.scrollHeight,
		document.documentElement.offsetHeight
	);
	const isAtPageBottom = vh + scrollY >= scrollHeight - 80;

	revealNodes.forEach((node) => {
		if (!node.classList.contains('is-revealed')) {
			const rect = node.getBoundingClientRect();
			if (isAtPageBottom || (rect.top <= triggerBottom && rect.bottom >= 0)) {
				node.classList.add('is-revealed');
				revealNodes.delete(node);
				revealObserver?.unobserve(node);
			}
		}
	});

	if (revealNodes.size === 0 && scrollListenerAttached && typeof window !== 'undefined') {
		window.removeEventListener('scroll', onScrollRaf);
		scrollListenerAttached = false;
	}
}

function onScrollRaf() {
	if (rafId !== null) return;
	rafId = requestAnimationFrame(() => {
		rafId = null;
		checkReveals();
	});
}

function getRevealObserver(): IntersectionObserver | null {
	if (!revealObserver && typeof IntersectionObserver !== 'undefined') {
		revealObserver = new IntersectionObserver(
			(entries) => {
				entries.forEach((entry) => {
					if (entry.isIntersecting) {
						(entry.target as HTMLElement).classList.add('is-revealed');
						revealNodes.delete(entry.target as HTMLElement);
						revealObserver?.unobserve(entry.target);
					}
				});
			},
			{
				threshold: 0,
				rootMargin: '0px 0px -10px 0px'
			}
		);
	}
	return revealObserver;
}

export function reveal(node: HTMLElement, options: RevealOptions = {}) {
	const delay = options?.delay ?? 0;
	const y = options?.y ?? 28;

	node.classList.add('reveal-on-scroll');
	node.style.setProperty('--reveal-y', `${y}px`);
	if (delay > 0) {
		node.style.transitionDelay = `${delay}ms`;
	}

	revealNodes.add(node);

	if (typeof window !== 'undefined') {
		const obs = getRevealObserver();
		obs?.observe(node);

		if (!scrollListenerAttached) {
			window.addEventListener('scroll', onScrollRaf, { passive: true });
			scrollListenerAttached = true;
		}

		requestAnimationFrame(() => checkReveals());
	}

	return {
		destroy() {
			revealNodes.delete(node);
			revealObserver?.unobserve(node);
		}
	};
}

export function cleanupRevealEngine() {
	revealObserver?.disconnect();
	revealObserver = null;
	revealNodes.clear();
	if (scrollListenerAttached && typeof window !== 'undefined') {
		window.removeEventListener('scroll', onScrollRaf);
		scrollListenerAttached = false;
	}
	if (rafId !== null) {
		cancelAnimationFrame(rafId);
		rafId = null;
	}
}
