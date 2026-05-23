use std::collections::BTreeMap;

use tokio_erp::erpnext::accounts::doctype::bank_statement_import::bank_statement_import::{
    add_bank_account, bank_statement_import_template_options, get_import_status, is_mt940_format,
    parse_data_from_template, preprocess_mt940_content, BankStatementImport,
    BankStatementImportClientConfig, DataImportLog, ImportStatus,
};
use tokio_erp::erpnext::{DocumentController, FieldSpec};

#[test]
fn bank_statement_import_matches_erpnext_metadata_fields_and_client_config() {
    assert_eq!(BankStatementImport::DOCTYPE, "Bank Statement Import");
    assert_eq!(BankStatementImport::MODULE, "Accounts");
    assert_eq!(
        BankStatementImport::AUTONAME,
        "format:Bank Statement Import on {creation}"
    );
    assert_eq!(
        BankStatementImport::FIELD_ORDER,
        [
            "company",
            "bank_account",
            "bank",
            "column_break_4",
            "import_mt940_fromat",
            "custom_delimiters",
            "delimiter_options",
            "google_sheets_url",
            "refresh_google_sheet",
            "html_5",
            "import_file",
            "download_template",
            "status",
            "template_options",
            "use_csv_sniffer",
            "import_warnings_section",
            "template_warnings",
            "import_warnings",
            "section_import_preview",
            "import_preview",
            "import_log_section",
            "show_failed_logs",
            "import_log_preview",
            "reference_doctype",
            "import_type",
            "submit_after_import",
            "mute_emails",
        ]
    );
    assert!(BankStatementImport::BETA);
    assert!(BankStatementImport::EDITABLE_GRID);
    assert!(BankStatementImport::HIDE_TOOLBAR);
    assert!(BankStatementImport::TRACK_CHANGES);
    assert_eq!(BankStatementImport::SORT_FIELD, "creation");
    assert_eq!(BankStatementImport::SORT_ORDER, "DESC");

    let fields = BankStatementImport::fields();
    assert!(fields.contains(
        &FieldSpec::link("company", "Company")
            .options("Company")
            .required()
    ));
    assert!(fields.contains(
        &FieldSpec::link("bank_account", "Bank Account")
            .options("Bank Account")
            .required()
            .in_list_view()
    ));
    assert!(fields.contains(
        &FieldSpec::link("bank", "Bank")
            .options("Bank")
            .depends_on("eval:doc.bank_account")
            .fetch_from("bank_account.bank")
            .read_only()
    ));
    assert!(fields.contains(
        &FieldSpec::attach("import_file", "Import File")
            .depends_on("eval:!doc.__islocal")
            .in_list_view()
    ));
    assert!(fields.contains(
        &FieldSpec::select("status", "Status")
            .options("Pending\nSuccess\nPartial Success\nError")
            .default("Pending")
            .hidden()
            .read_only()
    ));
    assert!(fields.contains(
        &FieldSpec::data("delimiter_options", "Delimiter options")
            .default(",;\\t|")
            .depends_on("custom_delimiters")
            .description("If your CSV uses a different delimiter, add that character here, ensuring no spaces or additional characters are included.")
    ));
    assert!(fields
        .contains(&FieldSpec::check("import_mt940_fromat", "Import MT940 Fromat").default("0")));

    let client = BankStatementImportClientConfig::from_erpnext_js();
    assert_eq!(client.bank_account_query_company_filter, "company");
    assert_eq!(
        client.allowed_file_types,
        [".csv", ".xls", ".xlsx", ".TXT", ".txt"]
    );
    assert_eq!(client.import_refresh_event, "data_import_refresh");
    assert_eq!(client.import_progress_event, "data_import_progress");
    assert_eq!(client.mt940_parse_method, "convert_mt940_to_csv");
}

#[test]
fn mt940_preprocessor_matches_erpnext_truncation_rules() {
    assert_eq!(preprocess_mt940_content(""), "");
    assert_eq!(preprocess_mt940_content(":28C:12345/1"), ":28C:12345/1");
    assert_eq!(preprocess_mt940_content(":28C:1234/1"), ":28C:1234/1");
    assert_eq!(preprocess_mt940_content(":28C:167619/1"), ":28C:67619/1");
    assert_eq!(preprocess_mt940_content(":28C:987654321"), ":28C:54321");
    assert_eq!(
        preprocess_mt940_content(":28C:167619/1   \n"),
        ":28C:67619/1   \n"
    );
    assert_eq!(
        preprocess_mt940_content(":28C:167619/1\r\n"),
        ":28C:67619/1\r\n"
    );
    assert_eq!(
        preprocess_mt940_content("   :28C:167619/1\n"),
        "   :28C:167619/1\n"
    );
    assert_eq!(
        preprocess_mt940_content(":28C:167619/1\n:28C:987654/2"),
        ":28C:67619/1\n:28C:87654/2"
    );
}

#[test]
fn mt940_format_detection_matches_required_tag_check() {
    let valid_mt940 = ":20:STARTUMSE\n:25:1234567890\n:28C:167619/1\n:61:0310021002DR123,45";
    assert!(is_mt940_format(valid_mt940));

    assert!(!is_mt940_format(
        "Date,Description,Amount\n2023-01-01,Test Transaction,100.00"
    ));
    assert!(!is_mt940_format(
        ":20:STARTUMSE\n:25:1234567890\n:60F:C031002EUR0,00"
    ));
    assert!(!is_mt940_format(""));
}

#[test]
fn template_data_and_bank_account_helpers_match_erpnext_behavior() {
    let raw_data = vec![
        vec!["Date".to_string(), "Amount".to_string()],
        vec!["".to_string(), "".to_string()],
        vec!["2026-05-01".to_string(), "100".to_string()],
    ];
    assert_eq!(
        parse_data_from_template(raw_data),
        vec![
            vec!["Date".to_string(), "Amount".to_string()],
            vec!["2026-05-01".to_string(), "100".to_string()],
        ]
    );

    let mut data = vec![
        vec!["Date".to_string(), "Amount".to_string()],
        vec!["2026-05-01".to_string(), "100".to_string()],
    ];
    add_bank_account(&mut data, "BA-0001");
    assert_eq!(
        data,
        vec![
            vec![
                "Date".to_string(),
                "Amount".to_string(),
                "Bank Account".to_string()
            ],
            vec![
                "2026-05-01".to_string(),
                "100".to_string(),
                "BA-0001".to_string()
            ],
        ]
    );

    let mut existing = vec![
        vec![
            "Date".to_string(),
            "Bank Account".to_string(),
            "Amount".to_string(),
        ],
        vec![
            "2026-05-01".to_string(),
            "OLD-BA".to_string(),
            "100".to_string(),
        ],
    ];
    add_bank_account(&mut existing, "BA-0002");
    assert_eq!(existing[1][1], "BA-0002");

    let mut first_column = vec![
        vec![
            "Bank Account".to_string(),
            "Date".to_string(),
            "Amount".to_string(),
        ],
        vec![
            "OLD-BA".to_string(),
            "2026-05-01".to_string(),
            "100".to_string(),
        ],
    ];
    add_bank_account(&mut first_column, "BA-0003");
    assert_eq!(
        first_column[1],
        vec![
            "OLD-BA".to_string(),
            "2026-05-01".to_string(),
            "100".to_string(),
            "BA-0003".to_string(),
        ]
    );
}

#[test]
fn mapping_and_status_plans_match_erpnext_shapes() {
    let options = bank_statement_import_template_options(&[
        ("Date", "date"),
        ("Withdrawal", "withdrawal"),
        ("Deposit", "deposit"),
    ]);
    assert_eq!(
        options,
        BTreeMap::from([(
            "column_to_field_map".to_string(),
            BTreeMap::from([
                ("Date".to_string(), "date".to_string()),
                ("Deposit".to_string(), "deposit".to_string()),
                ("Withdrawal".to_string(), "withdrawal".to_string()),
            ])
        )])
    );

    let status = get_import_status(
        "Partial Success",
        &[
            DataImportLog {
                success: true,
                count: 3,
            },
            DataImportLog {
                success: false,
                count: 2,
            },
        ],
    );
    assert_eq!(
        status,
        ImportStatus {
            status: "Partial Success".to_string(),
            success: Some(3),
            failed: Some(2),
            total_records: 5,
        }
    );
}

#[test]
fn bank_statement_import_controller_matches_erpnext_hooks() {
    let doc = BankStatementImport::default();

    assert_eq!(doc.doctype(), "Bank Statement Import");
    assert_eq!(doc.module(), "Accounts");
    assert_eq!(doc.custom_hooks(), ["validate", "start_import"]);
}
