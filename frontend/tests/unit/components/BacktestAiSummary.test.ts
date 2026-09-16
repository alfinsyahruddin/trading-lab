import { describe, it, expect, vi } from 'vitest';
import { render, screen, fireEvent } from '@testing-library/svelte';
import BacktestAiSummary, {
	parseSummaryTokens
} from '$lib/components/backtest/BacktestAiSummary.svelte';

describe('BacktestAiSummary', () => {
	it('renders summary dimensions, spotlight recommendation, and header', () => {
		const summary = [
			'Net return of +15.2% over 12 months outperformed benchmark.',
			'Win rate of 65% coupled with 2.1 profit factor demonstrates strong expectancy.',
			'Annualized volatility of 14% indicates moderate risk exposure.',
			'Average winning hold time of 8 days shows disciplined profit capture.',
			'Consider tightening stop loss to -3.5% to improve risk-reward ratio.'
		];

		render(BacktestAiSummary, { props: { summary } });

		expect(screen.getByText('AI Summary')).toBeInTheDocument();
		expect(screen.getByText('Backtest Review')).toBeInTheDocument();
		expect(screen.queryByText('Gemini Insights')).not.toBeInTheDocument();

		// Dimension titles
		expect(screen.getByText('Profitability & Return')).toBeInTheDocument();
		expect(screen.getByText('Win / Loss Performance')).toBeInTheDocument();
		expect(screen.getByText('Risk & Volatility Profile')).toBeInTheDocument();
		expect(screen.getByText('Holding & Execution')).toBeInTheDocument();
		expect(screen.getByText('Next Steps')).toBeInTheDocument();
		expect(screen.queryByText('Actionable')).not.toBeInTheDocument();

		// Key text parts
		expect(screen.getByText(/Net return of/)).toBeInTheDocument();
		expect(screen.getByText('+15.2%')).toBeInTheDocument();
		expect(screen.getByText('-3.5%')).toBeInTheDocument();
	});

	it('renders nothing when summary is empty', () => {
		render(BacktestAiSummary, { props: { summary: [] } });
		expect(screen.queryByText('AI Summary')).not.toBeInTheDocument();
	});

	it('toggles collapse and expand modes', async () => {
		const summary = [
			'Net return of +15.2% over 12 months.',
			'Win rate of 65%.',
			'Volatility of 14%.',
			'Hold time of 8 days.',
			'Actionable recommendation point.'
		];

		render(BacktestAiSummary, { props: { summary } });

		// Initially expanded: Bento grid dimension titles are visible
		expect(screen.getByText('Profitability & Return')).toBeInTheDocument();

		// Click collapse
		const collapseBtn = screen.getByTitle('Collapse summary');
		await fireEvent.click(collapseBtn);

		// Now collapsed: Bento grid hidden, Next Steps shown
		expect(screen.queryByText('Profitability & Return')).not.toBeInTheDocument();
		expect(screen.getByText('Next Steps')).toBeInTheDocument();
		expect(screen.getByTitle('Expand summary')).toBeInTheDocument();

		// Click expand
		const expandBtn = screen.getByTitle('Expand summary');
		await fireEvent.click(expandBtn);
		expect(screen.getByText('Profitability & Return')).toBeInTheDocument();
	});

	it('handles copy summary action', async () => {
		const writeTextMock = vi.fn().mockResolvedValue(undefined);
		Object.assign(navigator, {
			clipboard: {
				writeText: writeTextMock
			}
		});

		const summary = [
			'Net return +20%.',
			'Win rate 60%.',
			'Sharpe 1.5.',
			'Hold 5 days.',
			'Keep SL at -5%.'
		];

		render(BacktestAiSummary, { props: { summary } });

		const copyBtn = screen.getByTitle('Copy AI summary in Markdown');
		await fireEvent.click(copyBtn);

		expect(writeTextMock).toHaveBeenCalledOnce();
		expect(writeTextMock.mock.calls[0][0]).toContain('### AI Summary');
		expect(writeTextMock.mock.calls[0][0]).toContain('Profitability & Return');
		expect(writeTextMock.mock.calls[0][0]).toContain('Keep SL at -5%.');
	});

	describe('parseSummaryTokens', () => {
		it('parses positive (+31%), negative (-20%), and neutral (0%) percentages', () => {
			const text = 'Achieved +31.5% net return, max drawdown -20.2%, and 0% drift.';
			const tokens = parseSummaryTokens(text);

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
			const tokens = parseSummaryTokens(text);

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
			const tokens = parseSummaryTokens(text);

			const pctToken = tokens.find((t) => t.text === '65%');
			// 65% is not signed with +/- and not 0, so it remains in plain text chunk
			expect(pctToken).toBeUndefined();
			expect(tokens.some((t) => t.text.includes('65%'))).toBe(true);
		});
	});
});
