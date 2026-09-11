import type { Page } from '@playwright/test';

export const mockUser = {
	id: 'u-admin-e2e',
	name: 'Admin Tester',
	email: 'admin@tradinglab.id',
	role: 'ADMIN',
	created_at: '2026-01-01T00:00:00Z',
	updated_at: '2026-01-01T00:00:00Z'
};

export const mockStats = {
	total_stars_received: 12,
	total_strategies: 5,
	total_backtests: 8,
	date_joined: '2026-01-01T00:00:00Z'
};

export const mockStrategy = {
	id: 'strat-101',
	user_id: 'u-admin-e2e',
	name: 'IDX Momentum Breakout',
	description: 'Top momentum picks on IDX',
	tp_percentage: 12,
	sl_percentage: 4,
	risk_reward_ratio: 3,
	max_holding_period_days: 20,
	rules: [
		{
			id: 'group-1',
			connector_to_next: null,
			conditions: [
				{
					id: 'cond-1',
					variable: 'price',
					operator: '>',
					value: '100',
					connector_to_next: null
				}
			]
		}
	],
	created_at: '2026-08-01T00:00:00Z',
	updated_at: '2026-08-01T00:00:00Z'
};

export const mockBacktest = {
	id: 'bt-101',
	user_id: 'u-admin-e2e',
	strategy_id: 'strat-101',
	strategy_name: 'IDX Momentum Breakout',
	name: 'IDX Momentum - 2024',
	year: 2024,
	initial_cash: 100_000_000,
	max_holding_stocks: 5,
	max_stocks: 12,
	backtest_duration_months: 12,
	buy_fee_percentage: 0.15,
	sell_fee_percentage: 0.25,
	is_public: true,
	status: 'DONE',
	error_message: null,
	result: {
		net_pnl: 18_500_000,
		net_pnl_percentage: 18.5,
		gross_pnl: 20_000_000,
		gross_pnl_percentage: 20.0,
		win_rate: 68.0,
		profit_factor: 2.3,
		sharpe_ratio: 1.9,
		max_drawdown_percentage: 6.2,
		total_trades: 28,
		winning_trades: 19,
		losing_trades: 9,
		total_fees_paid: 1_500_000,
		annualized_return: 18.5,
		annualized_volatility: 11.2,
		ai_summary: null
	},
	portfolio_history: [
		{ date: '2024-01-02', net_value: 100_000_000, gross_value: 100_000_000 },
		{ date: '2024-12-30', net_value: 118_500_000, gross_value: 120_000_000 }
	],
	most_traded: [],
	top_gainers: [],
	top_losers: [],
	trade_history: [],
	created_at: '2026-08-15T00:00:00Z',
	updated_at: '2026-08-15T00:00:00Z'
};

export const mockSettings = {
	ai_enabled: true,
	updated_at: '2026-08-01T00:00:00Z'
};

export function jsonEnvelope(data: unknown, status = 200, message: string | null = null) {
	return {
		data,
		status,
		message,
		timestamp: new Date().toISOString()
	};
}

export async function authenticateUser(page: Page, user = mockUser) {
	await page.addInitScript(
		({ u }) => {
			localStorage.setItem('trading_lab_token', 'mock-e2e-token');
			localStorage.setItem('trading_lab_refresh_token', 'mock-e2e-refresh');
			localStorage.setItem('trading_lab_user', JSON.stringify(u));
		},
		{ u: user }
	);
}

export async function setupDefaultApiMocks(page: Page) {
	await page.route('**/api/**', async (route) => {
		const request = route.request();
		const url = new URL(request.url());
		const path = url.pathname;
		const method = request.method();

		if (path === '/api/dashboard/stats' && method === 'GET') {
			return route.fulfill({
				status: 200,
				contentType: 'application/json',
				body: JSON.stringify(jsonEnvelope(mockStats))
			});
		}

		if (path === '/api/dashboard/leaderboard' && method === 'GET') {
			const leaderboardEntry = {
				...mockBacktest,
				owner_name: mockUser.name,
				star_count: 5,
				is_starred_by_me: false,
				net_pnl: 18_500_000,
				net_pnl_percentage: 18.5,
				win_rate: 68.0,
				profit_factor: 2.3,
				trades_processed: 28
			};
			return route.fulfill({
				status: 200,
				contentType: 'application/json',
				body: JSON.stringify(jsonEnvelope([leaderboardEntry]))
			});
		}

		if (path === '/api/dashboard/top-stars' && method === 'GET') {
			const topStarEntry = {
				...mockBacktest,
				owner_name: mockUser.name,
				star_count: 12,
				is_starred_by_me: false,
				net_pnl: 18_500_000,
				net_pnl_percentage: 18.5,
				win_rate: 68.0,
				profit_factor: 2.3,
				trades_processed: 28
			};
			return route.fulfill({
				status: 200,
				contentType: 'application/json',
				body: JSON.stringify(jsonEnvelope([topStarEntry]))
			});
		}

		if (path === '/api/strategies' && method === 'GET') {
			return route.fulfill({
				status: 200,
				contentType: 'application/json',
				body: JSON.stringify(jsonEnvelope([mockStrategy]))
			});
		}

		if (path === '/api/strategies' && method === 'POST') {
			const body = JSON.parse(request.postData() || '{}');
			const newStrat = {
				...mockStrategy,
				id: 'strat-' + Date.now(),
				name: body.name || 'New Strategy',
				description: body.description || '',
				tp_percentage: body.tp_percentage ?? 10,
				sl_percentage: body.sl_percentage ?? 5,
				rules: body.rules || []
			};
			return route.fulfill({
				status: 200,
				contentType: 'application/json',
				body: JSON.stringify(jsonEnvelope(newStrat))
			});
		}

		if (path.startsWith('/api/strategies/') && method === 'GET') {
			return route.fulfill({
				status: 200,
				contentType: 'application/json',
				body: JSON.stringify(jsonEnvelope(mockStrategy))
			});
		}

		if (path === '/api/backtests' && method === 'GET') {
			return route.fulfill({
				status: 200,
				contentType: 'application/json',
				body: JSON.stringify(jsonEnvelope([mockBacktest]))
			});
		}

		if (path === '/api/backtests' && method === 'POST') {
			const body = JSON.parse(request.postData() || '{}');
			const newBacktest = {
				...mockBacktest,
				id: 'bt-' + Date.now(),
				name: body.name || 'New Backtest',
				status: 'PENDING'
			};
			return route.fulfill({
				status: 200,
				contentType: 'application/json',
				body: JSON.stringify(jsonEnvelope(newBacktest))
			});
		}

		if (path.startsWith('/api/backtests/') && method === 'GET') {
			return route.fulfill({
				status: 200,
				contentType: 'application/json',
				body: JSON.stringify(jsonEnvelope(mockBacktest))
			});
		}

		if (path === '/api/settings' && method === 'GET') {
			return route.fulfill({
				status: 200,
				contentType: 'application/json',
				body: JSON.stringify(jsonEnvelope(mockSettings))
			});
		}

		if (path === '/api/settings' && method === 'PATCH') {
			const body = JSON.parse(request.postData() || '{}');
			return route.fulfill({
				status: 200,
				contentType: 'application/json',
				body: JSON.stringify(jsonEnvelope({ ...mockSettings, ...body }))
			});
		}

		if (path === '/api/users' && method === 'GET') {
			return route.fulfill({
				status: 200,
				contentType: 'application/json',
				body: JSON.stringify(jsonEnvelope([mockUser]))
			});
		}

		if (path === '/api/auth/captcha' && method === 'GET') {
			return route.fulfill({
				status: 200,
				contentType: 'application/json',
				body: JSON.stringify(
					jsonEnvelope({
						id: 'mock-captcha-id',
						image:
							'data:image/svg+xml;utf8,<svg xmlns="http://www.w3.org/2000/svg" width="100" height="40"><text x="10" y="25">TEST</text></svg>'
					})
				)
			});
		}

		if (path === '/api/users/me' && method === 'GET') {
			return route.fulfill({
				status: 200,
				contentType: 'application/json',
				body: JSON.stringify(jsonEnvelope(mockUser))
			});
		}

		if (path === '/api/users/logout' && method === 'POST') {
			return route.fulfill({
				status: 200,
				contentType: 'application/json',
				body: JSON.stringify(jsonEnvelope(null, 200, 'Logged out'))
			});
		}

		// Fallback for unhandled API calls
		return route.fulfill({
			status: 200,
			contentType: 'application/json',
			body: JSON.stringify(jsonEnvelope(null))
		});
	});
}
