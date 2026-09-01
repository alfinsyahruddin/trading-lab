CREATE TABLE backtest_stars (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    user_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    backtest_job_id UUID NOT NULL REFERENCES backtest_jobs(id) ON DELETE CASCADE,
    created_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    CONSTRAINT uq_backtest_stars_user_job UNIQUE (user_id, backtest_job_id)
);

CREATE INDEX idx_backtest_stars_user ON backtest_stars(user_id);
CREATE INDEX idx_backtest_stars_job ON backtest_stars(backtest_job_id);
