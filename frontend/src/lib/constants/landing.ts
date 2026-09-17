export const LANDING_TYPEWRITER_WORDS = [
	'trading strategy',
	'stock screener',
	'methodology'
] as const;

export interface LandingHeroStat {
	value: string;
	label: string;
	target: number;
	suffix?: string;
	prefix?: string;
}

export const LANDING_HERO_STATS: LandingHeroStat[] = [
	{ value: '100+', label: 'IDX Indicators', target: 100, suffix: '+' },
	{ value: '4 Years', label: 'Historical Data', target: 4, suffix: ' Years' },
	{ value: '8+', label: 'Performance Metrics', target: 8, suffix: '+' }
];

export interface LandingFeature {
	id: string;
	title: string;
	description: string;
	bullets: string[];
	delay: number;
	isAi?: boolean;
}

export const LANDING_FEATURES: LandingFeature[] = [
	{
		id: 'strategy-builder',
		title: 'Visual Strategy Builder',
		description:
			'Compose multi-group screening rules with AND/OR logic across valuation, profitability, solvency, dividend, and technical indicators. No code, no Python.',
		bullets: [
			'P/E, P/B, ROE, DER, Dividend Yield',
			'Price, volume, and momentum signals',
			'Cross-metric comparisons'
		],
		delay: 50
	},
	{
		id: 'backtest-engine',
		title: 'Reliable Backtest Engine',
		description:
			'Run reliable backtests against real IDX daily data. Configure capital, fees, portfolio size, and duration. Get results in seconds.',
		bullets: [
			'Take-profit, stop-loss, holding limits',
			'Realistic broker fee modeling',
			'Win rate, Sharpe ratio, volatility'
		],
		delay: 150
	},
	{
		id: 'ai-powered',
		title: 'AI-Powered',
		description:
			'AI reviews your strategies, offering one-click refinements. After each backtest, get actionable AI insights.',
		bullets: [
			'Risk-reward optimization hints',
			'Rule enhancement suggestions',
			'Qualitative backtest analysis'
		],
		delay: 250,
		isAi: true
	},
	{
		id: 'deep-analytics',
		title: 'Deep Analytics',
		description:
			'Interactive equity charts (TradingView-powered), win/loss doughnut charts, trade telemetry logs, top gainers/losers, and sparkline previews on every card.',
		bullets: [
			'Net vs gross equity curve toggle',
			'Win Rate, Sharpe ratio, and volatility metrics',
			'Full trade execution history'
		],
		delay: 350
	}
];

export interface LandingScreenshot {
	id: string;
	lightSrc: string;
	darkSrc: string;
	alt: string;
	tag: string;
	description: string;
	delay: number;
	containerClass: string;
	imageBorderClass: string;
	tagClass: string;
}

export const LANDING_SCREENSHOTS: LandingScreenshot[] = [
	{
		id: 'backtest-results',
		lightSrc: '/backtest-light.png',
		darkSrc: '/backtest-dark.png',
		alt: 'Backtest results showing equity curve, win rate, and performance metrics',
		tag: 'Backtest Results',
		description: 'Equity curve, win rate, Sharpe ratio, and full trade log—everything in one view.',
		delay: 50,
		containerClass: 'max-md:mb-0 md:col-span-2 md:mb-6',
		imageBorderClass: 'border-(--border) shadow-none dark:shadow-2xl',
		tagClass: 'text-(--accent)'
	},
	{
		id: 'community-dashboard',
		lightSrc: '/dashboard-light.png',
		darkSrc: '/dashboard-dark.png',
		alt: 'Dashboard showing community leaderboard and user stats',
		tag: 'Community Dashboard',
		description: 'Discover top-performing strategies on the IDX community leaderboard.',
		delay: 100,
		containerClass: 'flex flex-col md:col-span-1 md:col-start-1 md:row-span-2 md:row-start-2',
		imageBorderClass: 'border-(--border) shadow-none dark:shadow-xl',
		tagClass: 'text-(--accent)'
	},
	{
		id: 'ai-suggestions',
		lightSrc: '/ai-suggestions-light.png',
		darkSrc: '/ai-suggestions-dark.png',
		alt: 'AI strategy suggestions with accept or ignore actions',
		tag: '✦ AI Suggestions',
		description: 'One-click accept or ignore for AI-generated parameter improvements.',
		delay: 200,
		containerClass: 'flex flex-col md:col-span-1 md:col-start-2 md:row-start-2',
		imageBorderClass: 'border-[rgba(245,200,66,0.2)] shadow-none dark:shadow-xl',
		tagClass: 'text-[#f5c842]'
	},
	{
		id: 'ai-summary',
		lightSrc: '/ai-summary-light.png',
		darkSrc: '/ai-summary-dark.png',
		alt: 'AI insights with qualitative performance breakdown',
		tag: '✦ AI Insights',
		description: 'Qualitative breakdown and actionable insights for every backtest.',
		delay: 300,
		containerClass: 'flex flex-col md:col-span-1 md:col-start-2 md:row-start-3',
		imageBorderClass: 'border-[rgba(222,152,254,0.25)] shadow-none dark:shadow-xl',
		tagClass:
			'w-fit bg-[linear-gradient(135deg,#de98fe_0%,#38c1fb_100%)] bg-clip-text text-transparent'
	}
];

export interface LandingWorkflowStep {
	step: string;
	title: string;
	description: string;
	delay: number;
}

export const LANDING_WORKFLOW_STEPS: LandingWorkflowStep[] = [
	{
		step: '01',
		title: 'Build your strategy',
		description:
			'Use the visual rule builder to set screening conditions across 100+ IDX financial indicators. Configure take-profit, stop-loss, and holding limits.',
		delay: 100
	},
	{
		step: '02',
		title: 'Run the backtest',
		description:
			'Select your year, starting capital, and broker fees. The engine simulates trades against real IDX daily data asynchronously—no waiting, no blocking.',
		delay: 200
	},
	{
		step: '03',
		title: 'Analyze and refine',
		description:
			'Review your equity curve, win rate, Sharpe ratio, and AI insights. Share your winning strategies with the community—or keep them private.',
		delay: 300
	}
];
