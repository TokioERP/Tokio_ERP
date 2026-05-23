use std::collections::BTreeMap;

use crate::erpnext::{DocumentController, FieldSpec};

#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct BankStatementImport {
    pub bank: Option<String>,
    pub bank_account: Option<String>,
    pub company: Option<String>,
    pub custom_delimiters: bool,
    pub delimiter_options: Option<String>,
    pub google_sheets_url: Option<String>,
    pub import_file: Option<String>,
    pub import_mt940_fromat: bool,
    pub import_type: Option<String>,
    pub mute_emails: bool,
    pub reference_doctype: Option<String>,
    pub show_failed_logs: bool,
    pub status: Option<String>,
    pub submit_after_import: bool,
    pub template_options: Option<String>,
    pub template_warnings: Option<String>,
    pub use_csv_sniffer: bool,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct BankStatementImportClientConfig {
    pub bank_account_query_company_filter: &'static str,
    pub allowed_file_types: [&'static str; 5],
    pub import_refresh_event: &'static str,
    pub import_progress_event: &'static str,
    pub mt940_parse_method: &'static str,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct DataImportLog {
    pub success: bool,
    pub count: usize,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ImportStatus {
    pub status: String,
    pub success: Option<usize>,
    pub failed: Option<usize>,
    pub total_records: usize,
}

impl BankStatementImport {
    pub const DOCTYPE: &'static str = "Bank Statement Import";
    pub const MODULE: &'static str = "Accounts";
    pub const AUTONAME: &'static str = "format:Bank Statement Import on {creation}";
    pub const FIELD_ORDER: [&'static str; 27] = [
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
    ];
    pub const BETA: bool = true;
    pub const EDITABLE_GRID: bool = true;
    pub const HIDE_TOOLBAR: bool = true;
    pub const TRACK_CHANGES: bool = true;
    pub const SORT_FIELD: &'static str = "creation";
    pub const SORT_ORDER: &'static str = "DESC";

    pub fn fields() -> Vec<FieldSpec> {
        vec![
            FieldSpec::link("company", "Company")
                .options("Company")
                .required(),
            FieldSpec::link("bank_account", "Bank Account")
                .options("Bank Account")
                .required()
                .in_list_view(),
            FieldSpec::link("bank", "Bank")
                .options("Bank")
                .depends_on("eval:doc.bank_account")
                .fetch_from("bank_account.bank")
                .read_only(),
            FieldSpec::column_break("column_break_4"),
            FieldSpec::check("import_mt940_fromat", "Import MT940 Fromat").default("0"),
            FieldSpec::check("custom_delimiters", "Custom delimiters").default("0"),
            FieldSpec::data("delimiter_options", "Delimiter options")
                .default(",;\\t|")
                .depends_on("custom_delimiters")
                .description("If your CSV uses a different delimiter, add that character here, ensuring no spaces or additional characters are included."),
            FieldSpec::data("google_sheets_url", "Import from Google Sheets")
                .depends_on("eval:!doc.__islocal && !doc.import_file\n")
                .description("Must be a publicly accessible Google Sheets URL and adding Bank Account column is necessary for importing via Google Sheets"),
            FieldSpec::button("refresh_google_sheet", "Refresh Google Sheet")
                .depends_on("eval:doc.google_sheets_url && !doc.__unsaved"),
            FieldSpec::html("html_5", "")
                .depends_on("eval:!doc.__islocal && !doc.import_file")
                .options("<h5 class=\"text-muted uppercase\">Or</h5>"),
            FieldSpec::attach("import_file", "Import File")
                .depends_on("eval:!doc.__islocal")
                .in_list_view(),
            FieldSpec::button("download_template", "Download Template")
                .depends_on("eval:!doc.__islocal"),
            FieldSpec::select("status", "Status")
                .options("Pending\nSuccess\nPartial Success\nError")
                .default("Pending")
                .hidden()
                .read_only(),
            FieldSpec::code("template_options", "Template Options")
                .options("JSON")
                .hidden()
                .read_only(),
            FieldSpec::check("use_csv_sniffer", "Use CSV Sniffer")
                .default("0")
                .hidden(),
            FieldSpec::section_break("import_warnings_section")
                .label("Import File Errors and Warnings"),
            FieldSpec::code("template_warnings", "Template Warnings")
                .options("JSON")
                .hidden(),
            FieldSpec::html("import_warnings", "Import Warnings"),
            FieldSpec::section_break("section_import_preview").label("Preview"),
            FieldSpec::html("import_preview", "Import Preview"),
            FieldSpec::section_break("import_log_section").label("Import Log"),
            FieldSpec::check("show_failed_logs", "Show Failed Logs").default("0"),
            FieldSpec::html("import_log_preview", "Import Log Preview"),
            FieldSpec::link("reference_doctype", "Document Type")
                .options("DocType")
                .default("Bank Transaction")
                .required()
                .hidden()
                .in_list_view(),
            FieldSpec::select("import_type", "Import Type")
                .options("\nInsert New Records\nUpdate Existing Records")
                .default("Insert New Records")
                .required()
                .hidden()
                .in_list_view(),
            FieldSpec::check("submit_after_import", "Submit After Import")
                .default("1")
                .hidden(),
            FieldSpec::check("mute_emails", "Don't Send Emails")
                .default("1")
                .hidden(),
        ]
    }
}

impl BankStatementImportClientConfig {
    pub const fn from_erpnext_js() -> Self {
        Self {
            bank_account_query_company_filter: "company",
            allowed_file_types: [".csv", ".xls", ".xlsx", ".TXT", ".txt"],
            import_refresh_event: "data_import_refresh",
            import_progress_event: "data_import_progress",
            mt940_parse_method: "convert_mt940_to_csv",
        }
    }
}

impl DocumentController for BankStatementImport {
    fn doctype(&self) -> &'static str {
        Self::DOCTYPE
    }

    fn module(&self) -> &'static str {
        Self::MODULE
    }

    fn custom_hooks(&self) -> &'static [&'static str] {
        &["validate", "start_import"]
    }
}

pub fn preprocess_mt940_content(content: &str) -> String {
    if !content.contains(":28C:") {
        return content.to_string();
    }

    let mut processed = String::with_capacity(content.len());
    let mut remaining = content;

    while let Some(newline_idx) = remaining.find('\n') {
        let (line_with_newline, rest) = remaining.split_at(newline_idx + 1);
        processed.push_str(&preprocess_mt940_line(line_with_newline));
        remaining = rest;
    }

    if !remaining.is_empty() {
        processed.push_str(&preprocess_mt940_line(remaining));
    }

    processed
}

fn preprocess_mt940_line(line: &str) -> String {
    let (line_body, newline) = line
        .strip_suffix('\n')
        .map_or((line, ""), |body| (body, "\n"));

    if !line_body.starts_with(":28C:") {
        return line.to_string();
    }

    let after_tag = &line_body[5..];
    let digit_len = after_tag
        .as_bytes()
        .iter()
        .take_while(|byte| byte.is_ascii_digit())
        .count();

    if digit_len < 6 {
        return line.to_string();
    }

    let statement_number = &after_tag[..digit_len];
    let mut cursor = digit_len;
    let mut sequence_end = cursor;

    if after_tag[cursor..].starts_with('/') {
        let seq_digits = after_tag[cursor + 1..]
            .as_bytes()
            .iter()
            .take_while(|byte| byte.is_ascii_digit())
            .count();

        if seq_digits == 0 {
            return line.to_string();
        }

        sequence_end = cursor + 1 + seq_digits;
        cursor = sequence_end;
    }

    let trailing = &after_tag[cursor..];
    if !trailing.chars().all(char::is_whitespace) {
        return line.to_string();
    }

    let sequence = &after_tag[digit_len..sequence_end];
    format!(
        ":28C:{}{}{}{}",
        &statement_number[statement_number.len() - 5..],
        sequence,
        trailing,
        newline
    )
}

pub fn is_mt940_format(content: &str) -> bool {
    [":20:", ":25:", ":28C:", ":61:"]
        .iter()
        .all(|tag| content.contains(tag))
}

pub fn parse_data_from_template(raw_data: Vec<Vec<String>>) -> Vec<Vec<String>> {
    raw_data
        .into_iter()
        .filter(|row| !row.iter().all(|value| value.is_empty()))
        .collect()
}

pub fn add_bank_account(data: &mut Vec<Vec<String>>, bank_account: &str) {
    let mut bank_account_loc = None;
    if !data
        .first()
        .is_some_and(|header| header.iter().any(|value| value == "Bank Account"))
    {
        if let Some(header) = data.first_mut() {
            header.push("Bank Account".to_string());
        }
    } else if let Some(header) = data.first() {
        for (loc, value) in header.iter().enumerate() {
            if value == "Bank Account" {
                bank_account_loc = Some(loc);
            }
        }
    }

    for row in data.iter_mut().skip(1) {
        if let Some(loc) = bank_account_loc.filter(|loc| *loc != 0) {
            if loc < row.len() {
                row[loc] = bank_account.to_string();
            }
        } else {
            row.push(bank_account.to_string());
        }
    }
}

pub fn bank_statement_import_template_options(
    mappings: &[(&str, &str)],
) -> BTreeMap<String, BTreeMap<String, String>> {
    BTreeMap::from([(
        "column_to_field_map".to_string(),
        mappings
            .iter()
            .map(|(file_field, bank_transaction_field)| {
                (
                    (*file_field).to_string(),
                    (*bank_transaction_field).to_string(),
                )
            })
            .collect(),
    )])
}

pub fn get_import_status(status: &str, logs: &[DataImportLog]) -> ImportStatus {
    let mut import_status = ImportStatus {
        status: status.to_string(),
        success: None,
        failed: None,
        total_records: 0,
    };

    for log in logs {
        import_status.total_records += log.count;
        if log.success {
            import_status.success = Some(log.count);
        } else {
            import_status.failed = Some(log.count);
        }
    }

    import_status
}
