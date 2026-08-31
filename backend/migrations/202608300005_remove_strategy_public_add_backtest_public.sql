ALTER TABLE trading_strategies DROP COLUMN IF EXISTS is_public;

ALTER TABLE backtest_jobs ADD COLUMN is_public BOOLEAN NOT NULL DEFAULT false;
