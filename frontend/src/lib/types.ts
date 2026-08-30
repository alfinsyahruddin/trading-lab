export type UserRole = 'ADMIN' | 'MEMBER';

export interface UserResponse {
	id: string;
	name: string;
	email: string;
	role: UserRole;
	created_at: string;
	updated_at: string;
}

export interface RememberedAccount {
	email: string;
	name: string;
	role: UserRole;
	password?: string;
}

export interface TokenResponse {
	access_token: string;
	refresh_token: string;
}

export interface LoginResponse {
	user: UserResponse;
	tokens: TokenResponse;
}

export interface SegmentOption<V> {
	value: V;
	label: string;
	icon?: string;
}

export type RuleConnector = 'AND' | 'OR';

export interface StrategyRuleCondition {
	id: string;
	variable: string;
	operator: string;
	value: string;
	connector_to_next?: RuleConnector | null;
}

export interface StrategyRuleGroup {
	id: string;
	connector_to_next?: RuleConnector | null;
	conditions: StrategyRuleCondition[];
}

export interface TradingStrategy {
	id: string;
	user_id: string;
	name: string;
	description: string | null;
	is_public: boolean;
	tp_percentage: number;
	sl_percentage: number;
	risk_reward_ratio: number | null;
	max_holding_period_days: number;
	rules: StrategyRuleGroup[];
	created_at: string;
	updated_at: string;
}

export interface CreateStrategyPayload {
	name: string;
	description?: string | null;
	is_public?: boolean;
	tp_percentage: number;
	sl_percentage: number;
	max_holding_period_days: number;
	rules?: StrategyRuleGroup[];
}

export interface UpdateStrategyPayload {
	name?: string;
	description?: string | null;
	is_public?: boolean;
	tp_percentage?: number;
	sl_percentage?: number;
	max_holding_period_days?: number;
	rules?: StrategyRuleGroup[];
}

export type BacktestStatus = 'PENDING' | 'PROCESSING' | 'DONE' | 'FAILED';
export type ExitReason = 'STOP_LOSS' | 'TAKE_PROFIT' | 'MAX_HOLDING_TIME';

export interface BacktestJob {
	id: string;
	user_id: string;
	strategy_id: string;
	strategy_name: string;
	name: string;
	year: number;
	initial_cash: number;
	max_holding_stocks: number;
	backtest_duration_months: number;
	buy_fee_percentage: number;
	sell_fee_percentage: number;
	status: BacktestStatus;
	error_message: string | null;
	result: BacktestResult | null;
	portfolio_history: PortfolioHistoryEntry[] | null;
	most_traded: MostTradedEntry[] | null;
	top_gainers: TopEntry[] | null;
	top_losers: TopEntry[] | null;
	trade_history: TradeHistoryEntry[] | null;
	created_at: string;
	updated_at: string;
}

export interface BacktestResult {
	available_cash: number;
	trades_processed: number;
	net_pnl: number;
	net_pnl_percentage: number;
	gross_pnl: number;
	gross_pnl_percentage: number;
	win_rate: number;
	profit_factor: number;
	wins: number;
	losses: number;
	sharpe_ratio: number;
	max_profit: number;
	max_profit_percentage: number;
	max_loss: number;
	max_loss_percentage: number;
	avg_profit: number;
	avg_profit_percentage: number;
	avg_loss: number;
	avg_loss_percentage: number;
	avg_hold_time_days: number;
	total_fees: number;
	avg_win_hold_days: number;
	avg_loss_hold_days: number;
	portfolio_volatility: number;
}

export interface PortfolioHistoryEntry {
	date: string;
	net_value: number;
	gross_value: number;
}

export interface MostTradedEntry {
	code: string;
	total: number;
	pnl: number;
	pnl_percentage: number;
}

export interface TopEntry {
	code: string;
	pnl: number;
	pnl_percentage: number;
}

export interface TradeHistoryEntry {
	code: string;
	pnl: number;
	pnl_percentage: number;
	exit_reason: ExitReason;
	lot: number;
	buy_price: number;
	buy_value: number;
	sell_price: number;
	sell_value: number;
	buy_fee: number;
	sell_fee: number;
	buy_date: string;
	sell_date: string;
}

export interface CreateBacktestPayload {
	strategy_id: string;
	name: string;
	year: number;
	initial_cash: number;
	max_holding_stocks: number;
	backtest_duration_months: number;
	buy_fee_percentage: number;
	sell_fee_percentage: number;
}
