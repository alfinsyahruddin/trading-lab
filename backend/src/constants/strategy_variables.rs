use crate::entities::strategy_variable::{
    StrategyMetadataResponse, StrategyOperator, StrategyOperatorDef, StrategyVariable,
    StrategyVariableDef,
};

pub const VARIABLE_CATEGORIES: &[&str] = &[
    "Price & Market",
    "Valuation Ratios",
    "Income Statement",
    "Profitability & Returns",
    "Dividends & Per Share",
    "Cash Flow",
    "Balance Sheet & Assets",
    "Liabilities & Solvency",
    "Banking & Regulatory",
    "Efficiency & Forecasts",
];

pub const STRATEGY_OPERATORS: &[StrategyOperatorDef] = &[
    StrategyOperatorDef {
        value: "=",
        symbol: "[=]",
        label: "equals",
        display: "[=] equals",
    },
    StrategyOperatorDef {
        value: "!=",
        symbol: "[!=]",
        label: "not equals",
        display: "[!=] not equals",
    },
    StrategyOperatorDef {
        value: ">",
        symbol: "[>]",
        label: "greater than",
        display: "[>] greater than",
    },
    StrategyOperatorDef {
        value: "<",
        symbol: "[<]",
        label: "less than",
        display: "[<] less than",
    },
    StrategyOperatorDef {
        value: ">=",
        symbol: "[>=]",
        label: "greater than or equals",
        display: "[>=] greater than or equals",
    },
    StrategyOperatorDef {
        value: "<=",
        symbol: "[<=]",
        label: "less than or equals",
        display: "[<=] less than or equals",
    },
    StrategyOperatorDef {
        value: "~~",
        symbol: "[~~]",
        label: "like operator",
        display: "[~~] like operator",
    },
    StrategyOperatorDef {
        value: "in",
        symbol: "[in]",
        label: "in operator",
        display: "[in] in operator",
    },
    StrategyOperatorDef {
        value: "is",
        symbol: "[is]",
        label: "null / true / false",
        display: "[is] null / true / false",
    },
];

pub const STRATEGY_VARIABLES: &[StrategyVariableDef] = &[
    StrategyVariableDef {
        code: "price",
        name: "Price",
        description: "Current or closing market price.",
        category: "Price & Market",
        is_historical: false,
    },
    StrategyVariableDef {
        code: "volume",
        name: "Volume",
        description: "Trading volume in shares.",
        category: "Price & Market",
        is_historical: false,
    },
    StrategyVariableDef {
        code: "value",
        name: "Value",
        description: "Daily transaction value in IDR.",
        category: "Price & Market",
        is_historical: false,
    },
    StrategyVariableDef {
        code: "market_cap",
        name: "Market Cap",
        description: "Total market capitalization in IDR.",
        category: "Price & Market",
        is_historical: false,
    },
    StrategyVariableDef {
        code: "last_1_week_foreign_flow",
        name: "Last 1 Week Foreign Flow",
        description: "Total net foreign inflow in the 7 days (IDR).",
        category: "Price & Market",
        is_historical: false,
    },
    StrategyVariableDef {
        code: "last_1_month_foreign_flow",
        name: "Last 1 Month Foreign Flow",
        description: "Total net foreign inflow in the 30 days (IDR).",
        category: "Price & Market",
        is_historical: false,
    },
    StrategyVariableDef {
        code: "last_3_months_foreign_flow",
        name: "Last 3 Months Foreign Flow",
        description: "Total net foreign inflow in the 90 days (IDR).",
        category: "Price & Market",
        is_historical: false,
    },
    StrategyVariableDef {
        code: "pe",
        name: "P/E Ratio",
        description: "Price-to-earnings ratio.",
        category: "Valuation Ratios",
        is_historical: true,
    },
    StrategyVariableDef {
        code: "pb",
        name: "P/B Ratio",
        description: "Price-to-book ratio.",
        category: "Valuation Ratios",
        is_historical: true,
    },
    StrategyVariableDef {
        code: "ps",
        name: "P/S Ratio",
        description: "Price-to-sales ratio.",
        category: "Valuation Ratios",
        is_historical: true,
    },
    StrategyVariableDef {
        code: "pcf",
        name: "P/CF Ratio",
        description: "Price-to-cash-flow ratio.",
        category: "Valuation Ratios",
        is_historical: true,
    },
    StrategyVariableDef {
        code: "peg",
        name: "PEG Ratio",
        description: "Price/earnings-to-growth ratio.",
        category: "Valuation Ratios",
        is_historical: true,
    },
    StrategyVariableDef {
        code: "enterprise_to_ebitda",
        name: "EV / EBITDA",
        description: "Enterprise value to EBITDA.",
        category: "Valuation Ratios",
        is_historical: true,
    },
    StrategyVariableDef {
        code: "enterprise_to_revenue",
        name: "EV / Revenue",
        description: "Enterprise value to revenue.",
        category: "Valuation Ratios",
        is_historical: true,
    },
    StrategyVariableDef {
        code: "pb_peer_avg",
        name: "P/B Peer Average",
        description: "Peer average price-to-book ratio.",
        category: "Valuation Ratios",
        is_historical: true,
    },
    StrategyVariableDef {
        code: "pe_peer_avg",
        name: "P/E Peer Average",
        description: "Peer average price-to-earnings ratio.",
        category: "Valuation Ratios",
        is_historical: true,
    },
    StrategyVariableDef {
        code: "ps_peer_avg",
        name: "P/S Peer Average",
        description: "Peer average price-to-sales ratio.",
        category: "Valuation Ratios",
        is_historical: true,
    },
    StrategyVariableDef {
        code: "revenue",
        name: "Revenue",
        description: "Annual total revenue in IDR.",
        category: "Income Statement",
        is_historical: true,
    },
    StrategyVariableDef {
        code: "cost_of_revenue",
        name: "Cost of Revenue",
        description: "Cost of goods sold / cost of revenue in IDR.",
        category: "Income Statement",
        is_historical: true,
    },
    StrategyVariableDef {
        code: "gross_profit",
        name: "Gross Profit",
        description: "Revenue minus cost of revenue in IDR.",
        category: "Income Statement",
        is_historical: true,
    },
    StrategyVariableDef {
        code: "operating_expense",
        name: "Operating Expenses",
        description: "Total operating expenses in IDR.",
        category: "Income Statement",
        is_historical: true,
    },
    StrategyVariableDef {
        code: "operating_pnl",
        name: "Operating PnL",
        description: "Operating profit/loss (revenue minus operating expenses) in IDR.",
        category: "Income Statement",
        is_historical: true,
    },
    StrategyVariableDef {
        code: "ebit",
        name: "EBIT",
        description: "Earnings before interest and tax in IDR.",
        category: "Income Statement",
        is_historical: true,
    },
    StrategyVariableDef {
        code: "ebitda",
        name: "EBITDA",
        description: "Earnings before interest, tax, depreciation and amortisation in IDR.",
        category: "Income Statement",
        is_historical: true,
    },
    StrategyVariableDef {
        code: "earnings_before_tax",
        name: "Earnings Before Tax",
        description: "Earnings before income tax in IDR.",
        category: "Income Statement",
        is_historical: true,
    },
    StrategyVariableDef {
        code: "tax",
        name: "Income Tax",
        description: "Income tax expense in IDR.",
        category: "Income Statement",
        is_historical: true,
    },
    StrategyVariableDef {
        code: "earnings",
        name: "Net Profit / Earnings",
        description: "Annual net profit/loss in IDR.",
        category: "Income Statement",
        is_historical: true,
    },
    StrategyVariableDef {
        code: "non_operating_income_or_loss",
        name: "Non-Operating Income / Loss",
        description: "Income or losses outside core operations in IDR.",
        category: "Income Statement",
        is_historical: true,
    },
    StrategyVariableDef {
        code: "premium_income",
        name: "Premium Income",
        description: "Gross insurance premium income in IDR.",
        category: "Income Statement",
        is_historical: true,
    },
    StrategyVariableDef {
        code: "net_premium_income",
        name: "Net Premium Income",
        description: "Net insurance premium income in IDR.",
        category: "Income Statement",
        is_historical: true,
    },
    StrategyVariableDef {
        code: "premium_expense",
        name: "Premium Expense",
        description: "Insurance premium expenses in IDR.",
        category: "Income Statement",
        is_historical: true,
    },
    StrategyVariableDef {
        code: "net_profit_margin",
        name: "Net Profit Margin",
        description: "Net profit as a percentage of revenue.",
        category: "Profitability & Returns",
        is_historical: true,
    },
    StrategyVariableDef {
        code: "gross_profit_margin",
        name: "Gross Profit Margin",
        description: "Gross profit as a percentage of revenue.",
        category: "Profitability & Returns",
        is_historical: true,
    },
    StrategyVariableDef {
        code: "operating_profit_margin",
        name: "Operating Profit Margin",
        description: "Operating profit as a percentage of revenue.",
        category: "Profitability & Returns",
        is_historical: true,
    },
    StrategyVariableDef {
        code: "roa",
        name: "ROA",
        description: "Return on assets.",
        category: "Profitability & Returns",
        is_historical: true,
    },
    StrategyVariableDef {
        code: "roe",
        name: "ROE",
        description: "Return on equity.",
        category: "Profitability & Returns",
        is_historical: true,
    },
    StrategyVariableDef {
        code: "operating_cash_flow_margin",
        name: "Operating CF Margin",
        description: "Operating cash flow as a percentage of revenue.",
        category: "Profitability & Returns",
        is_historical: true,
    },
    StrategyVariableDef {
        code: "eps",
        name: "EPS",
        description: "Earnings per share.",
        category: "Dividends & Per Share",
        is_historical: true,
    },
    StrategyVariableDef {
        code: "eps_growth",
        name: "EPS Growth",
        description: "Year-over-year EPS growth rate.",
        category: "Dividends & Per Share",
        is_historical: true,
    },
    StrategyVariableDef {
        code: "total_dividend",
        name: "Total Dividend",
        description: "Total dividends paid per share.",
        category: "Dividends & Per Share",
        is_historical: true,
    },
    StrategyVariableDef {
        code: "total_yield",
        name: "Dividend Yield",
        description: "Total dividend yield.",
        category: "Dividends & Per Share",
        is_historical: true,
    },
    StrategyVariableDef {
        code: "outstanding_shares",
        name: "Outstanding Shares",
        description: "Total shares outstanding.",
        category: "Dividends & Per Share",
        is_historical: true,
    },
    StrategyVariableDef {
        code: "operating_cash_flow",
        name: "Operating Cash Flow",
        description: "Net cash generated from core operations in IDR.",
        category: "Cash Flow",
        is_historical: true,
    },
    StrategyVariableDef {
        code: "investing_cash_flow",
        name: "Investing Cash Flow",
        description: "Net cash from investing activities in IDR.",
        category: "Cash Flow",
        is_historical: true,
    },
    StrategyVariableDef {
        code: "financing_cash_flow",
        name: "Financing Cash Flow",
        description: "Net cash from financing activities in IDR.",
        category: "Cash Flow",
        is_historical: true,
    },
    StrategyVariableDef {
        code: "net_cash_flow",
        name: "Net Cash Flow",
        description: "Net change in cash for the period in IDR.",
        category: "Cash Flow",
        is_historical: true,
    },
    StrategyVariableDef {
        code: "free_cash_flow",
        name: "Free Cash Flow",
        description: "Operating cash flow minus capex in IDR.",
        category: "Cash Flow",
        is_historical: true,
    },
    StrategyVariableDef {
        code: "capital_expenditure",
        name: "Capital Expenditure",
        description: "Capital expenditure in IDR.",
        category: "Cash Flow",
        is_historical: true,
    },
    StrategyVariableDef {
        code: "cash_inflow",
        name: "Cash Inflow",
        description: "Total cash inflow in IDR.",
        category: "Cash Flow",
        is_historical: true,
    },
    StrategyVariableDef {
        code: "cash_outflow",
        name: "Cash Outflow",
        description: "Total cash outflow in IDR.",
        category: "Cash Flow",
        is_historical: true,
    },
    StrategyVariableDef {
        code: "end_cash_position",
        name: "End Cash Position",
        description: "Ending cash position from the cash flow statement in IDR.",
        category: "Cash Flow",
        is_historical: true,
    },
    StrategyVariableDef {
        code: "cash_and_equivalents",
        name: "Cash & Equivalents",
        description: "Cash and cash equivalents in IDR.",
        category: "Cash Flow",
        is_historical: true,
    },
    StrategyVariableDef {
        code: "cash_only",
        name: "Cash Only",
        description: "Cash excluding equivalents in IDR.",
        category: "Cash Flow",
        is_historical: true,
    },
    StrategyVariableDef {
        code: "total_cash_and_due_from_banks",
        name: "Cash & Due From Banks",
        description: "Cash and amounts due from other banks in IDR.",
        category: "Cash Flow",
        is_historical: true,
    },
    StrategyVariableDef {
        code: "total_assets",
        name: "Total Assets",
        description: "Total assets on the balance sheet in IDR.",
        category: "Balance Sheet & Assets",
        is_historical: true,
    },
    StrategyVariableDef {
        code: "current_assets",
        name: "Current Assets",
        description: "Total current assets in IDR.",
        category: "Balance Sheet & Assets",
        is_historical: true,
    },
    StrategyVariableDef {
        code: "fixed_assets",
        name: "Fixed Assets",
        description: "Net property, plant and equipment in IDR.",
        category: "Balance Sheet & Assets",
        is_historical: true,
    },
    StrategyVariableDef {
        code: "inventories",
        name: "Inventories",
        description: "Inventories on the balance sheet in IDR.",
        category: "Balance Sheet & Assets",
        is_historical: true,
    },
    StrategyVariableDef {
        code: "prepaid_assets",
        name: "Prepaid Assets",
        description: "Prepaid expenses and other current assets in IDR.",
        category: "Balance Sheet & Assets",
        is_historical: true,
    },
    StrategyVariableDef {
        code: "non_loan_assets",
        name: "Non-Loan Assets",
        description: "Total assets excluding loans in IDR.",
        category: "Balance Sheet & Assets",
        is_historical: true,
    },
    StrategyVariableDef {
        code: "total_equity",
        name: "Total Equity",
        description: "Total shareholders equity in IDR.",
        category: "Balance Sheet & Assets",
        is_historical: true,
    },
    StrategyVariableDef {
        code: "retained_earnings",
        name: "Retained Earnings",
        description: "Cumulative retained earnings on balance sheet in IDR.",
        category: "Balance Sheet & Assets",
        is_historical: true,
    },
    StrategyVariableDef {
        code: "realized_capital_goods_investment",
        name: "Capital Goods Investment",
        description: "Realised investment in capital goods in IDR.",
        category: "Balance Sheet & Assets",
        is_historical: true,
    },
    StrategyVariableDef {
        code: "total_liabilities",
        name: "Total Liabilities",
        description: "Total liabilities on the balance sheet in IDR.",
        category: "Liabilities & Solvency",
        is_historical: true,
    },
    StrategyVariableDef {
        code: "current_liabilities",
        name: "Current Liabilities",
        description: "Total current liabilities in IDR.",
        category: "Liabilities & Solvency",
        is_historical: true,
    },
    StrategyVariableDef {
        code: "non_current_liabilities",
        name: "Non-Current Liabilities",
        description: "Long-term liabilities in IDR.",
        category: "Liabilities & Solvency",
        is_historical: true,
    },
    StrategyVariableDef {
        code: "total_debt",
        name: "Total Debt",
        description: "Total interest-bearing debt in IDR.",
        category: "Liabilities & Solvency",
        is_historical: true,
    },
    StrategyVariableDef {
        code: "non_interest_bearing_liabilities",
        name: "Non-Interest Liabilities",
        description: "Liabilities that do not accrue interest in IDR.",
        category: "Liabilities & Solvency",
        is_historical: true,
    },
    StrategyVariableDef {
        code: "other_interest_bearing_liabilities",
        name: "Other Interest Liabilities",
        description: "Other interest-bearing liabilities excluding deposits in IDR.",
        category: "Liabilities & Solvency",
        is_historical: true,
    },
    StrategyVariableDef {
        code: "debt_to_asset_ratio",
        name: "Debt to Asset Ratio",
        description: "Total debt divided by total assets.",
        category: "Liabilities & Solvency",
        is_historical: true,
    },
    StrategyVariableDef {
        code: "debt_to_equity_ratio",
        name: "Debt to Equity Ratio",
        description: "Total debt divided by shareholders equity.",
        category: "Liabilities & Solvency",
        is_historical: true,
    },
    StrategyVariableDef {
        code: "cash_flow_to_debt_ratio",
        name: "Cash Flow to Debt",
        description: "Operating cash flow divided by total debt.",
        category: "Liabilities & Solvency",
        is_historical: true,
    },
    StrategyVariableDef {
        code: "interest_coverage_ratio",
        name: "Interest Coverage Ratio",
        description: "EBIT divided by interest expense.",
        category: "Liabilities & Solvency",
        is_historical: true,
    },
    StrategyVariableDef {
        code: "current_ratio",
        name: "Current Ratio",
        description: "Current assets divided by current liabilities.",
        category: "Liabilities & Solvency",
        is_historical: true,
    },
    StrategyVariableDef {
        code: "total_deposit",
        name: "Total Deposits",
        description: "Total customer deposits in IDR.",
        category: "Banking & Regulatory",
        is_historical: true,
    },
    StrategyVariableDef {
        code: "current_account",
        name: "Current Account Deposits",
        description: "Current account deposits in IDR.",
        category: "Banking & Regulatory",
        is_historical: true,
    },
    StrategyVariableDef {
        code: "savings_account",
        name: "Savings Account Deposits",
        description: "Savings account deposits in IDR.",
        category: "Banking & Regulatory",
        is_historical: true,
    },
    StrategyVariableDef {
        code: "time_deposit",
        name: "Time Deposits",
        description: "Time deposit liabilities in IDR.",
        category: "Banking & Regulatory",
        is_historical: true,
    },
    StrategyVariableDef {
        code: "gross_loan",
        name: "Gross Loans",
        description: "Gross loan portfolio before allowances in IDR.",
        category: "Banking & Regulatory",
        is_historical: true,
    },
    StrategyVariableDef {
        code: "net_loan",
        name: "Net Loans",
        description: "Net loans after allowances in IDR.",
        category: "Banking & Regulatory",
        is_historical: true,
    },
    StrategyVariableDef {
        code: "non_loan_earning_assets",
        name: "Non-Loan Earning Assets",
        description: "Interest-earning assets excluding loans in IDR.",
        category: "Banking & Regulatory",
        is_historical: true,
    },
    StrategyVariableDef {
        code: "non_loan_non_earning_assets",
        name: "Non-Loan Non-Earning Assets",
        description: "Non-earning assets excluding loans in IDR.",
        category: "Banking & Regulatory",
        is_historical: true,
    },
    StrategyVariableDef {
        code: "allowance_for_loans",
        name: "Allowance for Loans",
        description: "Allowance for loan losses in IDR.",
        category: "Banking & Regulatory",
        is_historical: true,
    },
    StrategyVariableDef {
        code: "provision",
        name: "Loan Loss Provision",
        description: "Provision for loan losses or liabilities in IDR.",
        category: "Banking & Regulatory",
        is_historical: true,
    },
    StrategyVariableDef {
        code: "special_mention_loan",
        name: "Special Mention Loans",
        description: "Special mention (watch-list) loans in IDR.",
        category: "Banking & Regulatory",
        is_historical: true,
    },
    StrategyVariableDef {
        code: "non_performing_loan",
        name: "NPL (Non-Performing)",
        description: "Non-performing loans (NPL) in IDR.",
        category: "Banking & Regulatory",
        is_historical: true,
    },
    StrategyVariableDef {
        code: "restructured_loan_current",
        name: "Restructured Loans (Current)",
        description: "Restructured loans currently performing in IDR.",
        category: "Banking & Regulatory",
        is_historical: true,
    },
    StrategyVariableDef {
        code: "interest_income",
        name: "Interest Income",
        description: "Total interest income in IDR.",
        category: "Banking & Regulatory",
        is_historical: true,
    },
    StrategyVariableDef {
        code: "interest_expense",
        name: "Interest Expense",
        description: "Total interest expense in IDR.",
        category: "Banking & Regulatory",
        is_historical: true,
    },
    StrategyVariableDef {
        code: "interest_expense_non_operating",
        name: "Non-Operating Interest Expense",
        description: "Non-operating interest expense in IDR.",
        category: "Banking & Regulatory",
        is_historical: true,
    },
    StrategyVariableDef {
        code: "net_interest_income",
        name: "Net Interest Income",
        description: "Interest income minus interest expense in IDR.",
        category: "Banking & Regulatory",
        is_historical: true,
    },
    StrategyVariableDef {
        code: "non_interest_income",
        name: "Non-Interest Income",
        description: "Fee and commission income outside of interest in IDR.",
        category: "Banking & Regulatory",
        is_historical: true,
    },
    StrategyVariableDef {
        code: "net_interest_margin",
        name: "Net Interest Margin (NIM)",
        description: "Net interest income as a percentage of earning assets.",
        category: "Banking & Regulatory",
        is_historical: true,
    },
    StrategyVariableDef {
        code: "cost_to_income_ratio",
        name: "Cost to Income Ratio",
        description: "Operating costs divided by operating income.",
        category: "Banking & Regulatory",
        is_historical: true,
    },
    StrategyVariableDef {
        code: "efficiency_ratio",
        name: "Efficiency Ratio",
        description: "Operating expenses divided by net revenue.",
        category: "Banking & Regulatory",
        is_historical: true,
    },
    StrategyVariableDef {
        code: "loan_to_deposit_ratio",
        name: "Loan to Deposit Ratio (LDR)",
        description: "Net loans divided by total deposits.",
        category: "Banking & Regulatory",
        is_historical: true,
    },
    StrategyVariableDef {
        code: "casa_ratio",
        name: "CASA Ratio",
        description: "Current and savings account deposits as a share of total deposits.",
        category: "Banking & Regulatory",
        is_historical: true,
    },
    StrategyVariableDef {
        code: "high_quality_liquid_asset",
        name: "HQLA",
        description: "High-quality liquid assets (HQLA) held in IDR.",
        category: "Banking & Regulatory",
        is_historical: true,
    },
    StrategyVariableDef {
        code: "liquidity_coverage_ratio",
        name: "Liquidity Coverage Ratio (LCR)",
        description: "HQLA divided by net cash outflows over 30 days.",
        category: "Banking & Regulatory",
        is_historical: true,
    },
    StrategyVariableDef {
        code: "core_capital_tier1",
        name: "Core Capital (Tier 1)",
        description: "Tier 1 core capital in IDR.",
        category: "Banking & Regulatory",
        is_historical: true,
    },
    StrategyVariableDef {
        code: "supplementary_capital_tier2",
        name: "Supplementary Capital (Tier 2)",
        description: "Tier 2 supplementary capital in IDR.",
        category: "Banking & Regulatory",
        is_historical: true,
    },
    StrategyVariableDef {
        code: "total_capital",
        name: "Total Capital",
        description: "Total regulatory capital in IDR.",
        category: "Banking & Regulatory",
        is_historical: true,
    },
    StrategyVariableDef {
        code: "credit_rwa",
        name: "Credit RWA",
        description: "Credit risk-weighted assets in IDR.",
        category: "Banking & Regulatory",
        is_historical: true,
    },
    StrategyVariableDef {
        code: "market_rwa",
        name: "Market RWA",
        description: "Market risk-weighted assets in IDR.",
        category: "Banking & Regulatory",
        is_historical: true,
    },
    StrategyVariableDef {
        code: "operational_rwa",
        name: "Operational RWA",
        description: "Operational risk-weighted assets in IDR.",
        category: "Banking & Regulatory",
        is_historical: true,
    },
    StrategyVariableDef {
        code: "total_risk_weighted_asset",
        name: "Total RWA",
        description: "Total risk-weighted assets in IDR.",
        category: "Banking & Regulatory",
        is_historical: true,
    },
    StrategyVariableDef {
        code: "capital_adequacy_ratio",
        name: "Capital Adequacy Ratio (CAR)",
        description: "Regulatory capital as a percentage of risk-weighted assets.",
        category: "Banking & Regulatory",
        is_historical: true,
    },
    StrategyVariableDef {
        code: "leverage_ratio",
        name: "Leverage Ratio",
        description: "Tier 1 capital divided by total exposure.",
        category: "Banking & Regulatory",
        is_historical: true,
    },
    StrategyVariableDef {
        code: "fixed_asset_turnover",
        name: "Fixed Asset Turnover",
        description: "Revenue divided by net fixed assets.",
        category: "Efficiency & Forecasts",
        is_historical: true,
    },
    StrategyVariableDef {
        code: "total_asset_turnover",
        name: "Total Asset Turnover",
        description: "Revenue divided by total assets.",
        category: "Efficiency & Forecasts",
        is_historical: true,
    },
    StrategyVariableDef {
        code: "forecast_eps_growth",
        name: "Forecast EPS Growth",
        description: "Analyst consensus EPS growth forecast.",
        category: "Efficiency & Forecasts",
        is_historical: true,
    },
    StrategyVariableDef {
        code: "forecast_revenue_growth",
        name: "Forecast Revenue Growth",
        description: "Analyst consensus revenue growth forecast.",
        category: "Efficiency & Forecasts",
        is_historical: true,
    },
    StrategyVariableDef {
        code: "forecast_eps_estimate",
        name: "Forecast EPS Estimate",
        description: "Analyst consensus EPS estimate in IDR.",
        category: "Efficiency & Forecasts",
        is_historical: true,
    },
    StrategyVariableDef {
        code: "forecast_revenue_estimate",
        name: "Forecast Revenue Estimate",
        description: "Analyst consensus revenue estimate in IDR.",
        category: "Efficiency & Forecasts",
        is_historical: true,
    },
];

pub fn get_strategy_metadata() -> StrategyMetadataResponse {
    StrategyMetadataResponse {
        variables: STRATEGY_VARIABLES
            .iter()
            .map(StrategyVariable::from)
            .collect(),
        categories: VARIABLE_CATEGORIES.iter().map(|s| s.to_string()).collect(),
        operators: STRATEGY_OPERATORS
            .iter()
            .map(StrategyOperator::from)
            .collect(),
    }
}

pub fn format_strategy_variables_for_ai_prompt() -> String {
    let mut lines = Vec::new();
    for category in VARIABLE_CATEGORIES {
        let vars: Vec<String> = STRATEGY_VARIABLES
            .iter()
            .filter(|v| v.category == *category)
            .map(|v| format!("{} ({})", v.code, v.name))
            .collect();
        if !vars.is_empty() {
            lines.push(format!("- {}: {}", category, vars.join(", ")));
        }
    }
    lines.join("\n")
}

pub fn format_strategy_operators_for_ai_prompt() -> String {
    let ops: Vec<String> = STRATEGY_OPERATORS
        .iter()
        .map(|o| format!("\"{}\"", o.value))
        .collect();
    ops.join(", ")
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashSet;

    #[test]
    fn should_contain_exact_variable_and_operator_counts() {
        assert_eq!(STRATEGY_VARIABLES.len(), 114);
        assert_eq!(VARIABLE_CATEGORIES.len(), 10);
        assert_eq!(STRATEGY_OPERATORS.len(), 9);
    }

    #[test]
    fn should_have_unique_variable_codes() {
        let mut seen = HashSet::new();
        for v in STRATEGY_VARIABLES {
            assert!(seen.insert(v.code), "Duplicate code: {}", v.code);
        }
    }

    #[test]
    fn should_return_strategy_metadata() {
        let meta = get_strategy_metadata();
        assert_eq!(meta.variables.len(), 114);
        assert_eq!(meta.categories.len(), 10);
        assert_eq!(meta.operators.len(), 9);
    }

    #[test]
    fn should_format_strategy_variables_for_ai_prompt() {
        let formatted = format_strategy_variables_for_ai_prompt();
        assert!(formatted.contains("Price & Market: price (Price)"));
        assert!(formatted.contains("Valuation Ratios: pe (P/E Ratio)"));
        assert!(formatted.contains("Efficiency & Forecasts:"));
    }

    #[test]
    fn should_format_strategy_operators_for_ai_prompt() {
        let formatted = format_strategy_operators_for_ai_prompt();
        assert_eq!(
            formatted,
            "\"=\", \"!=\", \">\", \"<\", \'>=\'".replace("'", "\"")
                + ", \"<=\", \"~~\", \"in\", \"is\""
        );
    }
}
