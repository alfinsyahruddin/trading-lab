-- Alter default value of max_stocks from 12 to 24
ALTER TABLE backtest_jobs ALTER COLUMN max_stocks SET DEFAULT 24;
