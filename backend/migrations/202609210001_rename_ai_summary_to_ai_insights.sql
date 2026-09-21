DO $$
BEGIN
    IF EXISTS (
        SELECT 1
        FROM information_schema.columns
        WHERE table_name = 'backtest_results' AND column_name = 'ai_summary'
    ) THEN
        ALTER TABLE backtest_results RENAME COLUMN ai_summary TO ai_insights;
    END IF;
END $$;
