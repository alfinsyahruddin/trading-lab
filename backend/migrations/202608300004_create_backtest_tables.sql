CREATE TYPE backtest_status AS ENUM ('PENDING', 'PROCESSING', 'DONE', 'FAILED');

CREATE TABLE backtest_jobs (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    user_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    strategy_id UUID NOT NULL REFERENCES trading_strategies(id) ON DELETE CASCADE,
    strategy_name VARCHAR(255) NOT NULL,
    name VARCHAR(255) NOT NULL,
    year INTEGER NOT NULL,
    initial_cash DOUBLE PRECISION NOT NULL,
    max_holding_stocks INTEGER NOT NULL,
    backtest_duration_months INTEGER NOT NULL,
    buy_fee_percentage DOUBLE PRECISION NOT NULL,
    sell_fee_percentage DOUBLE PRECISION NOT NULL,
    status backtest_status NOT NULL DEFAULT 'PENDING',
    error_message TEXT,
    created_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    CONSTRAINT uq_backtest_jobs_user_name UNIQUE (user_id, name)
);

CREATE INDEX idx_backtest_jobs_user_id ON backtest_jobs(user_id);
CREATE INDEX idx_backtest_jobs_strategy_id ON backtest_jobs(strategy_id);

CREATE TABLE backtest_results (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    backtest_job_id UUID NOT NULL UNIQUE REFERENCES backtest_jobs(id) ON DELETE CASCADE,
    available_cash DOUBLE PRECISION NOT NULL,
    trades_processed INTEGER NOT NULL,
    net_pnl DOUBLE PRECISION NOT NULL,
    net_pnl_percentage DOUBLE PRECISION NOT NULL,
    gross_pnl DOUBLE PRECISION NOT NULL,
    gross_pnl_percentage DOUBLE PRECISION NOT NULL,
    win_rate DOUBLE PRECISION NOT NULL,
    profit_factor DOUBLE PRECISION NOT NULL,
    wins INTEGER NOT NULL,
    losses INTEGER NOT NULL,
    sharpe_ratio DOUBLE PRECISION NOT NULL,
    max_profit DOUBLE PRECISION NOT NULL,
    max_profit_percentage DOUBLE PRECISION NOT NULL,
    max_loss DOUBLE PRECISION NOT NULL,
    max_loss_percentage DOUBLE PRECISION NOT NULL,
    avg_profit DOUBLE PRECISION NOT NULL,
    avg_profit_percentage DOUBLE PRECISION NOT NULL,
    avg_loss DOUBLE PRECISION NOT NULL,
    avg_loss_percentage DOUBLE PRECISION NOT NULL,
    avg_hold_time_days DOUBLE PRECISION NOT NULL,
    total_fees DOUBLE PRECISION NOT NULL,
    avg_win_hold_days DOUBLE PRECISION NOT NULL,
    avg_loss_hold_days DOUBLE PRECISION NOT NULL,
    portfolio_volatility DOUBLE PRECISION NOT NULL
);

CREATE TABLE backtest_portfolio_history (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    backtest_job_id UUID NOT NULL REFERENCES backtest_jobs(id) ON DELETE CASCADE,
    date DATE NOT NULL,
    net_value DOUBLE PRECISION NOT NULL,
    gross_value DOUBLE PRECISION NOT NULL
);

CREATE INDEX idx_bph_job ON backtest_portfolio_history(backtest_job_id);

CREATE TABLE backtest_trades (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    backtest_job_id UUID NOT NULL REFERENCES backtest_jobs(id) ON DELETE CASCADE,
    code VARCHAR(20) NOT NULL,
    pnl DOUBLE PRECISION NOT NULL,
    pnl_percentage DOUBLE PRECISION NOT NULL,
    exit_reason VARCHAR(20) NOT NULL,
    lot INTEGER NOT NULL,
    buy_price DOUBLE PRECISION NOT NULL,
    buy_value DOUBLE PRECISION NOT NULL,
    sell_price DOUBLE PRECISION NOT NULL,
    sell_value DOUBLE PRECISION NOT NULL,
    buy_fee DOUBLE PRECISION NOT NULL,
    sell_fee DOUBLE PRECISION NOT NULL,
    buy_date DATE NOT NULL,
    sell_date DATE NOT NULL
);

CREATE INDEX idx_bt_job ON backtest_trades(backtest_job_id);
