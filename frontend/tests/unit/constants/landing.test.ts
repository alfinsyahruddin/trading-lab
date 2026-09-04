import { describe, it, expect } from 'vitest';
import {
	LANDING_TYPEWRITER_WORDS,
	LANDING_HERO_STATS,
	LANDING_FEATURES,
	LANDING_SCREENSHOTS,
	LANDING_WORKFLOW_STEPS
} from '../../../src/lib/constants/landing';

describe('Landing Constants', () => {
	it('has typewriter words defined', () => {
		expect(LANDING_TYPEWRITER_WORDS.length).toBeGreaterThan(0);
		expect(LANDING_TYPEWRITER_WORDS).toContain('trading strategy');
	});

	it('has hero stats defined', () => {
		expect(LANDING_HERO_STATS).toHaveLength(3);
		expect(LANDING_HERO_STATS[0].value).toBe('50+');
	});

	it('has 4 landing features with 1 AI feature', () => {
		expect(LANDING_FEATURES).toHaveLength(4);
		const aiFeatures = LANDING_FEATURES.filter((f) => f.isAi);
		expect(aiFeatures).toHaveLength(1);
	});

	it('has 4 screenshots with light and dark variants', () => {
		expect(LANDING_SCREENSHOTS).toHaveLength(4);
		LANDING_SCREENSHOTS.forEach((shot) => {
			expect(shot.lightSrc).toMatch(/-light\.png$/);
			expect(shot.darkSrc).toMatch(/-dark\.png$/);
		});
	});

	it('has 3 workflow steps in order', () => {
		expect(LANDING_WORKFLOW_STEPS).toHaveLength(3);
		expect(LANDING_WORKFLOW_STEPS.map((s) => s.step)).toEqual(['01', '02', '03']);
	});
});
