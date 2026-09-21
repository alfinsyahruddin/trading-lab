import { describe, it, expect } from 'vitest';
import { render, screen, fireEvent } from '@testing-library/svelte';
import BacktestAiInsights, {
	parseInsightsTokens
} from '$lib/components/backtest/BacktestAiInsights.svelte';

describe('BacktestAiInsights', () => {
	it('renders insights dimensions, spotlight recommendation, and header', () => {
		const insights = [
			'Net return of +15.2% over 12 months outperformed benchmark.',
			'Annualized volatility of 14% indicates moderate risk exposure.',
			'Average winning hold time of 8 days shows disciplined profit capture.',
			'Consider tightening stop loss to -3.5% to improve risk-reward ratio.'
		];

		render(BacktestAiInsights, { props: { insights } });

		expect(screen.getByText('AI Insights')).toBeInTheDocument();
		expect(screen.getByText('Backtest Review')).toBeInTheDocument();

		// Dimension titles
		expect(screen.getByText('Profitability & Return')).toBeInTheDocument();
		expect(screen.queryByText('Win / Loss Performance')).not.toBeInTheDocument();
		expect(screen.getByText('Risk & Volatility Profile')).toBeInTheDocument();
		expect(screen.getByText('Holding & Execution')).toBeInTheDocument();
		expect(screen.getByText('Next Steps')).toBeInTheDocument();
		expect(screen.queryByText('Actionable')).not.toBeInTheDocument();

		// Key text parts
		expect(screen.getByText(/Net return of/)).toBeInTheDocument();
		expect(screen.getByText('+15.2%')).toBeInTheDocument();
		expect(screen.getByText('-3.5%')).toBeInTheDocument();
	});

	it('renders first 3 items in grid and last item as Next Steps when given a legacy 5-item insights', () => {
		const insights = [
			'Net return of +15.2% over 12 months.',
			'Legacy win rate point.',
			'Annualized volatility of 14%.',
			'Average winning hold time of 8 days.',
			'Recommendation to keep stop loss.'
		];

		render(BacktestAiInsights, { props: { insights } });

		// Should have Profitability, Risk, Holding in grid
		expect(screen.getByText('Profitability & Return')).toBeInTheDocument();
		expect(screen.getByText('Risk & Volatility Profile')).toBeInTheDocument();
		expect(screen.getByText('Holding & Execution')).toBeInTheDocument();
		expect(screen.queryByText('Win / Loss Performance')).not.toBeInTheDocument();

		// Next Steps is the 5th item
		expect(screen.getByText('Next Steps')).toBeInTheDocument();
		expect(screen.getByText('Recommendation to keep stop loss.')).toBeInTheDocument();
		// 4th item (index 3) is omitted from 3-card grid
		expect(screen.queryByText('Average winning hold time of 8 days.')).not.toBeInTheDocument();
	});

	it('renders nothing when insights is empty', () => {
		render(BacktestAiInsights, { props: { insights: [] } });
		expect(screen.queryByText('AI Insights')).not.toBeInTheDocument();
		expect(screen.queryByText('AI Summary')).not.toBeInTheDocument();
	});

	it('toggles collapse and expand modes', async () => {
		const insights = [
			'Net return of +15.2% over 12 months.',
			'Volatility of 14%.',
			'Hold time of 8 days.',
			'Actionable recommendation point.'
		];

		render(BacktestAiInsights, { props: { insights } });

		// Initially expanded: Bento grid dimension titles are visible
		expect(screen.getByText('Profitability & Return')).toBeInTheDocument();

		// Click collapse
		const collapseBtn = screen.getByTitle('Collapse insights');
		await fireEvent.click(collapseBtn);

		// Now collapsed: Bento grid hidden, Next Steps shown
		expect(screen.queryByText('Profitability & Return')).not.toBeInTheDocument();
		expect(screen.getByText('Next Steps')).toBeInTheDocument();
		expect(screen.getByTitle('Expand insights')).toBeInTheDocument();
		expect(screen.queryByText('Collapse')).not.toBeInTheDocument();
		expect(screen.queryByText('Expand')).not.toBeInTheDocument();

		// Click expand
		const expandBtn = screen.getByTitle('Expand insights');
		await fireEvent.click(expandBtn);
		expect(screen.getByText('Profitability & Return')).toBeInTheDocument();
	});

	describe('parseInsightsTokens', () => {
		it('parses positive (+31%), negative (-20%), and neutral (0%) percentages', () => {
			const text = 'Achieved +31.5% net return, max drawdown -20.2%, and 0% drift.';
			const tokens = parseInsightsTokens(text);

			const posToken = tokens.find((t) => t.text === '+31.5%');
			expect(posToken).toBeDefined();
			expect(posToken?.tone).toBe('positive');

			const negToken = tokens.find((t) => t.text === '-20.2%');
			expect(negToken).toBeDefined();
			expect(negToken?.tone).toBe('negative');

			const neutralToken = tokens.find((t) => t.text === '0%');
			expect(neutralToken).toBeDefined();
			expect(neutralToken?.tone).toBe('neutral');
		});

		it('parses markdown bold and italics with nested tones', () => {
			const text = 'Return of **+31%** and *moderate risk* with **-10%** dip.';
			const tokens = parseInsightsTokens(text);

			const boldPos = tokens.find((t) => t.text === '+31%');
			expect(boldPos?.bold).toBe(true);
			expect(boldPos?.tone).toBe('positive');

			const italicText = tokens.find((t) => t.text === 'moderate risk');
			expect(italicText?.italic).toBe(true);

			const boldNeg = tokens.find((t) => t.text === '-10%');
			expect(boldNeg?.bold).toBe(true);
			expect(boldNeg?.tone).toBe('negative');
		});

		it('leaves unsigned percentages neutral/plain without green or red', () => {
			const text = 'Win rate of 65% with 2.1 profit factor.';
			const tokens = parseInsightsTokens(text);

			const pctToken = tokens.find((t) => t.text === '65%');
			// 65% is not signed with +/- and not 0, so it remains in plain text chunk
			expect(pctToken).toBeUndefined();
			expect(tokens.some((t) => t.text.includes('65%'))).toBe(true);
		});
	});
});
