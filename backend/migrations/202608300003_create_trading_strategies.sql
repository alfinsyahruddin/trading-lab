CREATE TABLE trading_strategies (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    user_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    name VARCHAR(255) NOT NULL,
    description TEXT,
    is_public BOOLEAN NOT NULL DEFAULT false,
    tp_percentage DOUBLE PRECISION NOT NULL,
    sl_percentage DOUBLE PRECISION NOT NULL,
    max_holding_period_days INTEGER NOT NULL,
    rules JSONB NOT NULL DEFAULT '[]'::jsonb,
    created_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    CONSTRAINT uq_trading_strategies_user_name UNIQUE (user_id, name)
);

CREATE INDEX idx_trading_strategies_user_id ON trading_strategies(user_id);
