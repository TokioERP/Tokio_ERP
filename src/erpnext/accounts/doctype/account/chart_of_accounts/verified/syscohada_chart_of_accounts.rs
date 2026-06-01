#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SyscohadaChartPlan {
    pub country_code: String,
    pub output_file: String,
    pub json: String,
}

pub fn syscohada_countries() -> [&'static str; 17] {
    [
        "bj", "bf", "cm", "cf", "ci", "cg", "km", "ga", "gn", "gw", "gq", "ml", "ne", "cd", "sn",
        "td", "tg",
    ]
}

pub fn generate_syscohada_country_chart_plans(
    file_name: &str,
    chart_json: &str,
) -> Vec<SyscohadaChartPlan> {
    syscohada_countries()
        .into_iter()
        .map(|country_code| SyscohadaChartPlan {
            country_code: country_code.to_string(),
            output_file: file_name.replace("syscohada", country_code),
            json: set_country_code(chart_json, country_code),
        })
        .collect()
}

fn set_country_code(chart_json: &str, country_code: &str) -> String {
    let mut pairs = Vec::new();
    let content = chart_json
        .trim()
        .trim_start_matches('{')
        .trim_end_matches('}')
        .trim();

    for entry in split_top_level_entries(content) {
        let Some((key, value)) = entry.split_once(':') else {
            continue;
        };
        let key = key.trim().trim_matches('"').to_string();
        let value = if key == "country_code" {
            format!("\"{country_code}\"")
        } else {
            value.trim().to_string()
        };
        pairs.push((key, value));
    }

    let body = pairs
        .into_iter()
        .map(|(key, value)| format!("  \"{key}\": {value}"))
        .collect::<Vec<_>>()
        .join(",\n");
    format!("{{\n{body}\n}}")
}

fn split_top_level_entries(content: &str) -> Vec<String> {
    let mut entries = Vec::new();
    let mut current = String::new();
    let mut string_open = false;
    let mut escape = false;
    let mut depth = 0_i32;

    for ch in content.chars() {
        match ch {
            '\\' if string_open && !escape => {
                escape = true;
                current.push(ch);
            }
            '"' if !escape => {
                string_open = !string_open;
                current.push(ch);
            }
            '[' | '{' if !string_open => {
                depth += 1;
                current.push(ch);
            }
            ']' | '}' if !string_open => {
                depth -= 1;
                current.push(ch);
            }
            ',' if !string_open && depth == 0 => {
                entries.push(current.trim().to_string());
                current.clear();
            }
            _ => {
                escape = false;
                current.push(ch);
            }
        }
    }

    if !current.trim().is_empty() {
        entries.push(current.trim().to_string());
    }
    entries
}
