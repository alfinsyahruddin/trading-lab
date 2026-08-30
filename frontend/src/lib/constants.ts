export const APP_NAME = 'Trading Lab';
export const LS_TOKEN = 'trading_lab_token';
export const LS_REFRESH = 'trading_lab_refresh_token';
export const LS_USER = 'trading_lab_user';
export const LS_THEME = 'trading_lab_theme';
export const LS_REMEMBERED_ACCOUNTS = 'trading_lab_remembered_accounts';

export interface StrategyOperatorOption {
	value: string;
	symbol: string;
	label: string;
	display: string;
}

export const STRATEGY_OPERATORS: StrategyOperatorOption[] = [
	{ value: '=', symbol: '[=]', label: 'equals', display: '[=] equals' },
	{ value: '!=', symbol: '[!=]', label: 'not equals', display: '[!=] not equals' },
	{ value: '>', symbol: '[>]', label: 'greater than', display: '[>] greater than' },
	{ value: '<', symbol: '[<]', label: 'less than', display: '[<] less than' },
	{
		value: '>=',
		symbol: '[>=]',
		label: 'greater than or equals',
		display: '[>=] greater than or equals'
	},
	{
		value: '<=',
		symbol: '[<=]',
		label: 'less than or equals',
		display: '[<=] less than or equals'
	},
	{ value: '~~', symbol: '[~~]', label: 'like operator', display: '[~~] like operator' },
	{ value: 'in', symbol: '[in]', label: 'in operator', display: '[in] in operator' },
	{ value: 'is', symbol: '[is]', label: 'null / true / false', display: '[is] null / true / false' }
];

export interface StrategyVariableOption {
	code: string;
	name: string;
	description: string;
	category: string;
	isHistorical: boolean;
}

export const STRATEGY_VARIABLES: StrategyVariableOption[] = [
	// ── 1. Price & Market ── (Strictly only price and volume)
	{
		code: 'price',
		name: 'Price',
		description: 'Current or closing market price.',
		category: 'Price & Market',
		isHistorical: false
	},
	{
		code: 'volume',
		name: 'Volume',
		description: 'Trading volume in shares.',
		category: 'Price & Market',
		isHistorical: false
	},

	// ── 2. Valuation Ratios ──
	{
		code: 'pe',
		name: 'P/E Ratio',
		description: 'Price-to-earnings ratio.',
		category: 'Valuation Ratios',
		isHistorical: true
	},
	{
		code: 'pb',
		name: 'P/B Ratio',
		description: 'Price-to-book ratio.',
		category: 'Valuation Ratios',
		isHistorical: true
	},
	{
		code: 'ps',
		name: 'P/S Ratio',
		description: 'Price-to-sales ratio.',
		category: 'Valuation Ratios',
		isHistorical: true
	},
	{
		code: 'pcf',
		name: 'P/CF Ratio',
		description: 'Price-to-cash-flow ratio.',
		category: 'Valuation Ratios',
		isHistorical: true
	},
	{
		code: 'peg',
		name: 'PEG Ratio',
		description: 'Price/earnings-to-growth ratio.',
		category: 'Valuation Ratios',
		isHistorical: true
	},
	{
		code: 'enterprise_to_ebitda',
		name: 'EV / EBITDA',
		description: 'Enterprise value to EBITDA.',
		category: 'Valuation Ratios',
		isHistorical: true
	},
	{
		code: 'enterprise_to_revenue',
		name: 'EV / Revenue',
		description: 'Enterprise value to revenue.',
		category: 'Valuation Ratios',
		isHistorical: true
	},
	{
		code: 'pb_peer_avg',
		name: 'P/B Peer Average',
		description: 'Peer average price-to-book ratio.',
		category: 'Valuation Ratios',
		isHistorical: true
	},
	{
		code: 'pe_peer_avg',
		name: 'P/E Peer Average',
		description: 'Peer average price-to-earnings ratio.',
		category: 'Valuation Ratios',
		isHistorical: true
	},
	{
		code: 'ps_peer_avg',
		name: 'P/S Peer Average',
		description: 'Peer average price-to-sales ratio.',
		category: 'Valuation Ratios',
		isHistorical: true
	},

	// ── 3. Income Statement ──
	{
		code: 'revenue',
		name: 'Revenue',
		description: 'Annual total revenue in IDR.',
		category: 'Income Statement',
		isHistorical: true
	},
	{
		code: 'cost_of_revenue',
		name: 'Cost of Revenue',
		description: 'Cost of goods sold / cost of revenue in IDR.',
		category: 'Income Statement',
		isHistorical: true
	},
	{
		code: 'gross_profit',
		name: 'Gross Profit',
		description: 'Revenue minus cost of revenue in IDR.',
		category: 'Income Statement',
		isHistorical: true
	},
	{
		code: 'operating_expense',
		name: 'Operating Expenses',
		description: 'Total operating expenses in IDR.',
		category: 'Income Statement',
		isHistorical: true
	},
	{
		code: 'operating_pnl',
		name: 'Operating PnL',
		description: 'Operating profit/loss (revenue minus operating expenses) in IDR.',
		category: 'Income Statement',
		isHistorical: true
	},
	{
		code: 'ebit',
		name: 'EBIT',
		description: 'Earnings before interest and tax in IDR.',
		category: 'Income Statement',
		isHistorical: true
	},
	{
		code: 'ebitda',
		name: 'EBITDA',
		description: 'Earnings before interest, tax, depreciation and amortisation in IDR.',
		category: 'Income Statement',
		isHistorical: true
	},
	{
		code: 'earnings_before_tax',
		name: 'Earnings Before Tax',
		description: 'Earnings before income tax in IDR.',
		category: 'Income Statement',
		isHistorical: true
	},
	{
		code: 'tax',
		name: 'Income Tax',
		description: 'Income tax expense in IDR.',
		category: 'Income Statement',
		isHistorical: true
	},
	{
		code: 'earnings',
		name: 'Net Profit / Earnings',
		description: 'Annual net profit/loss in IDR.',
		category: 'Income Statement',
		isHistorical: true
	},
	{
		code: 'non_operating_income_or_loss',
		name: 'Non-Operating Income / Loss',
		description: 'Income or losses outside core operations in IDR.',
		category: 'Income Statement',
		isHistorical: true
	},
	{
		code: 'premium_income',
		name: 'Premium Income',
		description: 'Gross insurance premium income in IDR.',
		category: 'Income Statement',
		isHistorical: true
	},
	{
		code: 'net_premium_income',
		name: 'Net Premium Income',
		description: 'Net insurance premium income in IDR.',
		category: 'Income Statement',
		isHistorical: true
	},
	{
		code: 'premium_expense',
		name: 'Premium Expense',
		description: 'Insurance premium expenses in IDR.',
		category: 'Income Statement',
		isHistorical: true
	},

	// ── 4. Profitability & Returns ──
	{
		code: 'net_profit_margin',
		name: 'Net Profit Margin',
		description: 'Net profit as a percentage of revenue.',
		category: 'Profitability & Returns',
		isHistorical: true
	},
	{
		code: 'gross_profit_margin',
		name: 'Gross Profit Margin',
		description: 'Gross profit as a percentage of revenue.',
		category: 'Profitability & Returns',
		isHistorical: true
	},
	{
		code: 'operating_profit_margin',
		name: 'Operating Profit Margin',
		description: 'Operating profit as a percentage of revenue.',
		category: 'Profitability & Returns',
		isHistorical: true
	},
	{
		code: 'roa',
		name: 'ROA',
		description: 'Return on assets.',
		category: 'Profitability & Returns',
		isHistorical: true
	},
	{
		code: 'roe',
		name: 'ROE',
		description: 'Return on equity.',
		category: 'Profitability & Returns',
		isHistorical: true
	},
	{
		code: 'operating_cash_flow_margin',
		name: 'Operating CF Margin',
		description: 'Operating cash flow as a percentage of revenue.',
		category: 'Profitability & Returns',
		isHistorical: true
	},

	// ── 5. Dividends & Per Share ──
	{
		code: 'eps',
		name: 'EPS',
		description: 'Earnings per share.',
		category: 'Dividends & Per Share',
		isHistorical: true
	},
	{
		code: 'eps_growth',
		name: 'EPS Growth',
		description: 'Year-over-year EPS growth rate.',
		category: 'Dividends & Per Share',
		isHistorical: true
	},
	{
		code: 'total_dividend',
		name: 'Total Dividend',
		description: 'Total dividends paid per share.',
		category: 'Dividends & Per Share',
		isHistorical: true
	},
	{
		code: 'total_yield',
		name: 'Dividend Yield',
		description: 'Total dividend yield.',
		category: 'Dividends & Per Share',
		isHistorical: true
	},
	{
		code: 'outstanding_shares',
		name: 'Outstanding Shares',
		description: 'Total shares outstanding.',
		category: 'Dividends & Per Share',
		isHistorical: true
	},

	// ── 6. Cash Flow ──
	{
		code: 'operating_cash_flow',
		name: 'Operating Cash Flow',
		description: 'Net cash generated from core operations in IDR.',
		category: 'Cash Flow',
		isHistorical: true
	},
	{
		code: 'investing_cash_flow',
		name: 'Investing Cash Flow',
		description: 'Net cash from investing activities in IDR.',
		category: 'Cash Flow',
		isHistorical: true
	},
	{
		code: 'financing_cash_flow',
		name: 'Financing Cash Flow',
		description: 'Net cash from financing activities in IDR.',
		category: 'Cash Flow',
		isHistorical: true
	},
	{
		code: 'net_cash_flow',
		name: 'Net Cash Flow',
		description: 'Net change in cash for the period in IDR.',
		category: 'Cash Flow',
		isHistorical: true
	},
	{
		code: 'free_cash_flow',
		name: 'Free Cash Flow',
		description: 'Operating cash flow minus capex in IDR.',
		category: 'Cash Flow',
		isHistorical: true
	},
	{
		code: 'capital_expenditure',
		name: 'Capital Expenditure',
		description: 'Capital expenditure in IDR.',
		category: 'Cash Flow',
		isHistorical: true
	},
	{
		code: 'cash_inflow',
		name: 'Cash Inflow',
		description: 'Total cash inflow in IDR.',
		category: 'Cash Flow',
		isHistorical: true
	},
	{
		code: 'cash_outflow',
		name: 'Cash Outflow',
		description: 'Total cash outflow in IDR.',
		category: 'Cash Flow',
		isHistorical: true
	},
	{
		code: 'end_cash_position',
		name: 'End Cash Position',
		description: 'Ending cash position from the cash flow statement in IDR.',
		category: 'Cash Flow',
		isHistorical: true
	},
	{
		code: 'cash_and_equivalents',
		name: 'Cash & Equivalents',
		description: 'Cash and cash equivalents in IDR.',
		category: 'Cash Flow',
		isHistorical: true
	},
	{
		code: 'cash_only',
		name: 'Cash Only',
		description: 'Cash excluding equivalents in IDR.',
		category: 'Cash Flow',
		isHistorical: true
	},
	{
		code: 'total_cash_and_due_from_banks',
		name: 'Cash & Due From Banks',
		description: 'Cash and amounts due from other banks in IDR.',
		category: 'Cash Flow',
		isHistorical: true
	},

	// ── 7. Balance Sheet & Assets ──
	{
		code: 'total_assets',
		name: 'Total Assets',
		description: 'Total assets on the balance sheet in IDR.',
		category: 'Balance Sheet & Assets',
		isHistorical: true
	},
	{
		code: 'current_assets',
		name: 'Current Assets',
		description: 'Total current assets in IDR.',
		category: 'Balance Sheet & Assets',
		isHistorical: true
	},
	{
		code: 'fixed_assets',
		name: 'Fixed Assets',
		description: 'Net property, plant and equipment in IDR.',
		category: 'Balance Sheet & Assets',
		isHistorical: true
	},
	{
		code: 'inventories',
		name: 'Inventories',
		description: 'Inventories on the balance sheet in IDR.',
		category: 'Balance Sheet & Assets',
		isHistorical: true
	},
	{
		code: 'prepaid_assets',
		name: 'Prepaid Assets',
		description: 'Prepaid expenses and other current assets in IDR.',
		category: 'Balance Sheet & Assets',
		isHistorical: true
	},
	{
		code: 'non_loan_assets',
		name: 'Non-Loan Assets',
		description: 'Total assets excluding loans in IDR.',
		category: 'Balance Sheet & Assets',
		isHistorical: true
	},
	{
		code: 'total_equity',
		name: 'Total Equity',
		description: 'Total shareholders equity in IDR.',
		category: 'Balance Sheet & Assets',
		isHistorical: true
	},
	{
		code: 'retained_earnings',
		name: 'Retained Earnings',
		description: 'Cumulative retained earnings on balance sheet in IDR.',
		category: 'Balance Sheet & Assets',
		isHistorical: true
	},
	{
		code: 'realized_capital_goods_investment',
		name: 'Capital Goods Investment',
		description: 'Realised investment in capital goods in IDR.',
		category: 'Balance Sheet & Assets',
		isHistorical: true
	},

	// ── 8. Liabilities & Solvency ──
	{
		code: 'total_liabilities',
		name: 'Total Liabilities',
		description: 'Total liabilities on the balance sheet in IDR.',
		category: 'Liabilities & Solvency',
		isHistorical: true
	},
	{
		code: 'current_liabilities',
		name: 'Current Liabilities',
		description: 'Total current liabilities in IDR.',
		category: 'Liabilities & Solvency',
		isHistorical: true
	},
	{
		code: 'non_current_liabilities',
		name: 'Non-Current Liabilities',
		description: 'Long-term liabilities in IDR.',
		category: 'Liabilities & Solvency',
		isHistorical: true
	},
	{
		code: 'total_debt',
		name: 'Total Debt',
		description: 'Total interest-bearing debt in IDR.',
		category: 'Liabilities & Solvency',
		isHistorical: true
	},
	{
		code: 'non_interest_bearing_liabilities',
		name: 'Non-Interest Liabilities',
		description: 'Liabilities that do not accrue interest in IDR.',
		category: 'Liabilities & Solvency',
		isHistorical: true
	},
	{
		code: 'other_interest_bearing_liabilities',
		name: 'Other Interest Liabilities',
		description: 'Other interest-bearing liabilities excluding deposits in IDR.',
		category: 'Liabilities & Solvency',
		isHistorical: true
	},
	{
		code: 'debt_to_asset_ratio',
		name: 'Debt to Asset Ratio',
		description: 'Total debt divided by total assets.',
		category: 'Liabilities & Solvency',
		isHistorical: true
	},
	{
		code: 'debt_to_equity_ratio',
		name: 'Debt to Equity Ratio',
		description: 'Total debt divided by shareholders equity.',
		category: 'Liabilities & Solvency',
		isHistorical: true
	},
	{
		code: 'cash_flow_to_debt_ratio',
		name: 'Cash Flow to Debt',
		description: 'Operating cash flow divided by total debt.',
		category: 'Liabilities & Solvency',
		isHistorical: true
	},
	{
		code: 'interest_coverage_ratio',
		name: 'Interest Coverage Ratio',
		description: 'EBIT divided by interest expense.',
		category: 'Liabilities & Solvency',
		isHistorical: true
	},
	{
		code: 'current_ratio',
		name: 'Current Ratio',
		description: 'Current assets divided by current liabilities.',
		category: 'Liabilities & Solvency',
		isHistorical: true
	},

	// ── 9. Banking & Regulatory ──
	{
		code: 'total_deposit',
		name: 'Total Deposits',
		description: 'Total customer deposits in IDR.',
		category: 'Banking & Regulatory',
		isHistorical: true
	},
	{
		code: 'current_account',
		name: 'Current Account Deposits',
		description: 'Current account deposits in IDR.',
		category: 'Banking & Regulatory',
		isHistorical: true
	},
	{
		code: 'savings_account',
		name: 'Savings Account Deposits',
		description: 'Savings account deposits in IDR.',
		category: 'Banking & Regulatory',
		isHistorical: true
	},
	{
		code: 'time_deposit',
		name: 'Time Deposits',
		description: 'Time deposit liabilities in IDR.',
		category: 'Banking & Regulatory',
		isHistorical: true
	},
	{
		code: 'gross_loan',
		name: 'Gross Loans',
		description: 'Gross loan portfolio before allowances in IDR.',
		category: 'Banking & Regulatory',
		isHistorical: true
	},
	{
		code: 'net_loan',
		name: 'Net Loans',
		description: 'Net loans after allowances in IDR.',
		category: 'Banking & Regulatory',
		isHistorical: true
	},
	{
		code: 'non_loan_earning_assets',
		name: 'Non-Loan Earning Assets',
		description: 'Interest-earning assets excluding loans in IDR.',
		category: 'Banking & Regulatory',
		isHistorical: true
	},
	{
		code: 'non_loan_non_earning_assets',
		name: 'Non-Loan Non-Earning Assets',
		description: 'Non-earning assets excluding loans in IDR.',
		category: 'Banking & Regulatory',
		isHistorical: true
	},
	{
		code: 'allowance_for_loans',
		name: 'Allowance for Loans',
		description: 'Allowance for loan losses in IDR.',
		category: 'Banking & Regulatory',
		isHistorical: true
	},
	{
		code: 'provision',
		name: 'Loan Loss Provision',
		description: 'Provision for loan losses or liabilities in IDR.',
		category: 'Banking & Regulatory',
		isHistorical: true
	},
	{
		code: 'special_mention_loan',
		name: 'Special Mention Loans',
		description: 'Special mention (watch-list) loans in IDR.',
		category: 'Banking & Regulatory',
		isHistorical: true
	},
	{
		code: 'non_performing_loan',
		name: 'NPL (Non-Performing)',
		description: 'Non-performing loans (NPL) in IDR.',
		category: 'Banking & Regulatory',
		isHistorical: true
	},
	{
		code: 'restructured_loan_current',
		name: 'Restructured Loans (Current)',
		description: 'Restructured loans currently performing in IDR.',
		category: 'Banking & Regulatory',
		isHistorical: true
	},
	{
		code: 'interest_income',
		name: 'Interest Income',
		description: 'Total interest income in IDR.',
		category: 'Banking & Regulatory',
		isHistorical: true
	},
	{
		code: 'interest_expense',
		name: 'Interest Expense',
		description: 'Total interest expense in IDR.',
		category: 'Banking & Regulatory',
		isHistorical: true
	},
	{
		code: 'interest_expense_non_operating',
		name: 'Non-Operating Interest Expense',
		description: 'Non-operating interest expense in IDR.',
		category: 'Banking & Regulatory',
		isHistorical: true
	},
	{
		code: 'net_interest_income',
		name: 'Net Interest Income',
		description: 'Interest income minus interest expense in IDR.',
		category: 'Banking & Regulatory',
		isHistorical: true
	},
	{
		code: 'non_interest_income',
		name: 'Non-Interest Income',
		description: 'Fee and commission income outside of interest in IDR.',
		category: 'Banking & Regulatory',
		isHistorical: true
	},
	{
		code: 'net_interest_margin',
		name: 'Net Interest Margin (NIM)',
		description: 'Net interest income as a percentage of earning assets.',
		category: 'Banking & Regulatory',
		isHistorical: true
	},
	{
		code: 'cost_to_income_ratio',
		name: 'Cost to Income Ratio',
		description: 'Operating costs divided by operating income.',
		category: 'Banking & Regulatory',
		isHistorical: true
	},
	{
		code: 'efficiency_ratio',
		name: 'Efficiency Ratio',
		description: 'Operating expenses divided by net revenue.',
		category: 'Banking & Regulatory',
		isHistorical: true
	},
	{
		code: 'loan_to_deposit_ratio',
		name: 'Loan to Deposit Ratio (LDR)',
		description: 'Net loans divided by total deposits.',
		category: 'Banking & Regulatory',
		isHistorical: true
	},
	{
		code: 'casa_ratio',
		name: 'CASA Ratio',
		description: 'Current and savings account deposits as a share of total deposits.',
		category: 'Banking & Regulatory',
		isHistorical: true
	},
	{
		code: 'high_quality_liquid_asset',
		name: 'HQLA',
		description: 'High-quality liquid assets (HQLA) held in IDR.',
		category: 'Banking & Regulatory',
		isHistorical: true
	},
	{
		code: 'liquidity_coverage_ratio',
		name: 'Liquidity Coverage Ratio (LCR)',
		description: 'HQLA divided by net cash outflows over 30 days.',
		category: 'Banking & Regulatory',
		isHistorical: true
	},
	{
		code: 'core_capital_tier1',
		name: 'Core Capital (Tier 1)',
		description: 'Tier 1 core capital in IDR.',
		category: 'Banking & Regulatory',
		isHistorical: true
	},
	{
		code: 'supplementary_capital_tier2',
		name: 'Supplementary Capital (Tier 2)',
		description: 'Tier 2 supplementary capital in IDR.',
		category: 'Banking & Regulatory',
		isHistorical: true
	},
	{
		code: 'total_capital',
		name: 'Total Capital',
		description: 'Total regulatory capital in IDR.',
		category: 'Banking & Regulatory',
		isHistorical: true
	},
	{
		code: 'credit_rwa',
		name: 'Credit RWA',
		description: 'Credit risk-weighted assets in IDR.',
		category: 'Banking & Regulatory',
		isHistorical: true
	},
	{
		code: 'market_rwa',
		name: 'Market RWA',
		description: 'Market risk-weighted assets in IDR.',
		category: 'Banking & Regulatory',
		isHistorical: true
	},
	{
		code: 'operational_rwa',
		name: 'Operational RWA',
		description: 'Operational risk-weighted assets in IDR.',
		category: 'Banking & Regulatory',
		isHistorical: true
	},
	{
		code: 'total_risk_weighted_asset',
		name: 'Total RWA',
		description: 'Total risk-weighted assets in IDR.',
		category: 'Banking & Regulatory',
		isHistorical: true
	},
	{
		code: 'capital_adequacy_ratio',
		name: 'Capital Adequacy Ratio (CAR)',
		description: 'Regulatory capital as a percentage of risk-weighted assets.',
		category: 'Banking & Regulatory',
		isHistorical: true
	},
	{
		code: 'leverage_ratio',
		name: 'Leverage Ratio',
		description: 'Tier 1 capital divided by total exposure.',
		category: 'Banking & Regulatory',
		isHistorical: true
	},

	// ── 10. Efficiency & Forecasts ──
	{
		code: 'fixed_asset_turnover',
		name: 'Fixed Asset Turnover',
		description: 'Revenue divided by net fixed assets.',
		category: 'Efficiency & Forecasts',
		isHistorical: true
	},
	{
		code: 'total_asset_turnover',
		name: 'Total Asset Turnover',
		description: 'Revenue divided by total assets.',
		category: 'Efficiency & Forecasts',
		isHistorical: true
	},
	{
		code: 'forecast_eps_growth',
		name: 'Forecast EPS Growth',
		description: 'Analyst consensus EPS growth forecast.',
		category: 'Efficiency & Forecasts',
		isHistorical: true
	},
	{
		code: 'forecast_revenue_growth',
		name: 'Forecast Revenue Growth',
		description: 'Analyst consensus revenue growth forecast.',
		category: 'Efficiency & Forecasts',
		isHistorical: true
	},
	{
		code: 'forecast_eps_estimate',
		name: 'Forecast EPS Estimate',
		description: 'Analyst consensus EPS estimate in IDR.',
		category: 'Efficiency & Forecasts',
		isHistorical: true
	},
	{
		code: 'forecast_revenue_estimate',
		name: 'Forecast Revenue Estimate',
		description: 'Analyst consensus revenue estimate in IDR.',
		category: 'Efficiency & Forecasts',
		isHistorical: true
	}
];

export const VARIABLE_CATEGORIES = [
	'Price & Market',
	'Valuation Ratios',
	'Income Statement',
	'Profitability & Returns',
	'Dividends & Per Share',
	'Cash Flow',
	'Balance Sheet & Assets',
	'Liabilities & Solvency',
	'Banking & Regulatory',
	'Efficiency & Forecasts'
];

export function formatRiskReward(tp: number, sl: number): string {
	if (sl <= 0 || isNaN(sl) || isNaN(tp)) return '—';
	const ratio = tp / sl;
	const formattedRatio = Number(ratio.toFixed(2));
	return `1 : ${formattedRatio}`;
}

export function formatTimeAgo(isoString: string): string {
	if (!isoString) return '—';
	const date = new Date(isoString);
	if (isNaN(date.getTime())) return '—';
	const now = new Date();
	const diffInSeconds = Math.floor((now.getTime() - date.getTime()) / 1000);

	if (diffInSeconds < 60) {
		return 'just now';
	}
	const minutes = Math.floor(diffInSeconds / 60);
	if (minutes < 60) {
		return `${minutes} ${minutes === 1 ? 'minute' : 'minutes'} ago`;
	}
	const hours = Math.floor(minutes / 60);
	if (hours < 24) {
		return `${hours} ${hours === 1 ? 'hour' : 'hours'} ago`;
	}
	const days = Math.floor(hours / 24);
	if (days < 30) {
		return `${days} ${days === 1 ? 'day' : 'days'} ago`;
	}
	const months = Math.floor(days / 30);
	if (months < 12) {
		return `${months} ${months === 1 ? 'month' : 'months'} ago`;
	}
	const years = Math.floor(days / 365);
	return `${years} ${years === 1 ? 'year' : 'years'} ago`;
}
