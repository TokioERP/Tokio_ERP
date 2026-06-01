use tokio_erp::erpnext::accounts::doctype::account::chart_of_accounts::verified::syscohada_chart_of_accounts::{
    generate_syscohada_country_chart_plans, syscohada_countries, SyscohadaChartPlan,
};

#[test]
fn syscohada_countries_match_erpnext_source_order() {
    assert_eq!(
        syscohada_countries(),
        [
            "bj", "bf", "cm", "cf", "ci", "cg", "km", "ga", "gn", "gw", "gq", "ml", "ne", "cd",
            "sn", "td", "tg",
        ]
    );
}

#[test]
fn syscohada_generation_plan_matches_python_script() {
    let source = r#"{"name":"SYSCOHADA","country_code":"syscohada","accounts":[]}"#;
    let plans = generate_syscohada_country_chart_plans("syscohada.json", source);
    assert_eq!(plans.len(), 17);
    assert_eq!(
        plans[0],
        SyscohadaChartPlan {
            country_code: "bj".to_string(),
            output_file: "bj.json".to_string(),
            json:
                "{\n  \"name\": \"SYSCOHADA\",\n  \"country_code\": \"bj\",\n  \"accounts\": []\n}"
                    .to_string(),
        }
    );
    assert_eq!(plans[13].country_code, "cd");
    assert_eq!(plans[13].output_file, "cd.json");
    assert!(plans[13].json.contains("\"country_code\": \"cd\""));

    let prefixed = generate_syscohada_country_chart_plans("syscohada_chart.json", source);
    assert_eq!(prefixed[0].output_file, "bj_chart.json");
}
