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

export interface StrategyOwner {
	id: string;
	name: string;
	email: string;
}

export interface TradingStrategy {
	id: string;
	user_id: string;
	name: string;
	description: string | null;
	tp_percentage: number;
	sl_percentage: number;
	risk_reward_ratio: number | null;
	max_holding_period_days: number;
	rules: StrategyRuleGroup[];
	owner?: StrategyOwner | null;
	created_at: string;
	updated_at: string;
}

export interface CreateStrategyPayload {
	name: string;
	description?: string | null;
	tp_percentage: number;
	sl_percentage: number;
	max_holding_period_days: number;
	rules?: StrategyRuleGroup[];
}

export interface UpdateStrategyPayload {
	name?: string;
	description?: string | null;
	tp_percentage?: number;
	sl_percentage?: number;
	max_holding_period_days?: number;
	rules?: StrategyRuleGroup[];
}

export type BacktestStatus = 'PENDING' | 'PROCESSING' | 'DONE' | 'FAILED';
export type ExitReason = 'STOP_LOSS' | 'TAKE_PROFIT' | 'MAX_HOLDING_TIME';

export interface BacktestOwner {
	id: string;
	name: string;
	email: string;
}

export interface BacktestJob {
	id: string;
	user_id: string;
	strategy_id: string;
	strategy_name: string;
	name: string;
	year: number;
	initial_cash: number;
	max_holding_stocks: number;
	max_stocks?: number;
	backtest_duration_months: number;
	buy_fee_percentage: number;
	sell_fee_percentage: number;
	is_public: boolean;
	status: BacktestStatus;
	error_message: string | null;
	owner?: BacktestOwner | null;
	result: BacktestResult | null;
	portfolio_history: PortfolioHistoryEntry[] | null;
	most_traded: MostTradedEntry[] | null;
	top_gainers: TopEntry[] | null;
	top_losers: TopEntry[] | null;
	trade_history: TradeHistoryEntry[] | null;
	start_date?: string | null;
	end_date?: string | null;
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
	ai_insights?: string[] | null;
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
	id?: string;
	code: string;
	company_name?: string | null;
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
	query_values?: Record<string, unknown> | null;
}

export interface CreateBacktestPayload {
	strategy_id: string;
	name: string;
	year: number;
	initial_cash: number;
	max_holding_stocks: number;
	max_stocks?: number;
	backtest_duration_months: number;
	buy_fee_percentage: number;
	sell_fee_percentage: number;
	is_public?: boolean;
	start_date?: string | null;
	end_date?: string | null;
}

export interface CaptchaResponse {
	id: string;
	image: string;
}

export interface UpdateBacktestPayload {
	is_public?: boolean;
}

export interface DashboardStats {
	total_stars_received: number;
	total_strategies: number;
	total_backtests: number;
	date_joined: string;
}

export interface LeaderboardEntry {
	id: string;
	name: string;
	owner_name: string;
	strategy_name: string;
	year: number;
	initial_cash: number;
	backtest_duration_months: number;
	buy_fee_percentage: number;
	sell_fee_percentage: number;
	status: BacktestStatus;
	net_pnl: number;
	net_pnl_percentage: number;
	win_rate: number;
	profit_factor: number;
	trades_processed: number;
	star_count: number;
	is_starred_by_me: boolean;
	portfolio_history: LeaderboardPortfolioPoint[];
	created_at: string;
}

export interface LeaderboardPortfolioPoint {
	date: string;
	net_value: number;
}

export interface AppSettings {
	ai_enabled: boolean;
}

export type StrategySuggestionType = 'PARAMETER' | 'RULE';
export type StrategyRuleAction = 'ADD_CONDITION' | 'EDIT_CONDITION';

export interface StrategyRulePayload {
	group_index: number;
	condition_index?: number | null;
	variable: string;
	operator: string;
	value: string;
	connector_to_next?: RuleConnector | null;
}

export type StrategySuggestionField = 'tp_percentage' | 'sl_percentage' | 'max_holding_period_days';

export interface StrategyAiSuggestion {
	id: string;
	suggestion_type?: StrategySuggestionType;
	field?: StrategySuggestionField | null;
	title: string;
	current_value?: number | null;
	suggested_value?: number | null;
	rule_action?: StrategyRuleAction | null;
	rule_payload?: StrategyRulePayload | null;
	reason: string;
}

export interface StrategyVariable {
	code: string;
	name: string;
	description: string;
	category: string;
	is_historical: boolean;
}

export interface StrategyOperator {
	value: string;
	symbol: string;
	label: string;
	display: string;
}

export interface StrategyMetadata {
	variables: StrategyVariable[];
	categories: string[];
	operators: StrategyOperator[];
}

export type StrategyVariableOption = StrategyVariable;
export type StrategyOperatorOption = StrategyOperator;
