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
