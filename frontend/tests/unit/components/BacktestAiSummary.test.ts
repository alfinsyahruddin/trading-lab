import { describe, it, expect } from 'vitest';
import { render, screen } from '@testing-library/svelte';
import BacktestAiSummary from '$lib/components/backtest/BacktestAiSummary.svelte';

describe('BacktestAiSummary', () => {
	it('renders summary keypoints with index numbers and header', () => {
		const summary = [
			'Net return of +15.2% over 12 months outperformed benchmark.',
			'Win rate of 65% coupled with 2.1 profit factor demonstrates strong expectancy.',
			'Annualized volatility of 14% indicates moderate risk exposure.',
			'Average winning hold time of 8 days shows disciplined profit capture.',
			'Consider tightening stop loss to 3.5% to improve risk-reward ratio.'
		];

		render(BacktestAiSummary, { props: { summary } });

		expect(screen.getByText('AI Summary')).toBeInTheDocument();
		expect(screen.queryByText('Gemini Insights')).not.toBeInTheDocument();
		expect(screen.getByText(summary[0])).toBeInTheDocument();
		expect(screen.getByText(summary[4])).toBeInTheDocument();
	});

	it('renders nothing when summary is empty', () => {
		render(BacktestAiSummary, { props: { summary: [] } });
		expect(screen.queryByText('AI Summary')).not.toBeInTheDocument();
	});
});
