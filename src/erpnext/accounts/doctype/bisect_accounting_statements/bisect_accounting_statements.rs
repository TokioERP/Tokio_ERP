use std::collections::{BTreeMap, VecDeque};

use crate::erpnext::{DocumentController, FieldSpec};

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum BisectAlgorithm {
    Bfs,
    Dfs,
}

impl Default for BisectAlgorithm {
    fn default() -> Self {
        Self::Bfs
    }
}

#[derive(Clone, Debug, Default, PartialEq)]
pub struct BisectAccountingStatements {
    pub company: Option<String>,
    pub from_date: Option<String>,
    pub to_date: Option<String>,
    pub algorithm: BisectAlgorithm,
    pub current_node: Option<String>,
    pub current_from_date: Option<String>,
    pub current_to_date: Option<String>,
    pub p_l_summary: f64,
    pub b_s_summary: f64,
    pub difference: f64,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum BisectAccountingStatementsError {
    FromDateAfterToDate { from_date: String, to_date: String },
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum BisectMessage {
    NoMoreChildrenLeft,
    NoMoreChildrenRight,
    ReachedRoot,
}

#[derive(Clone, Debug, PartialEq)]
pub struct BisectNode {
    pub name: String,
    pub root: Option<String>,
    pub period_from_date: String,
    pub period_to_date: String,
    pub left_child: Option<String>,
    pub right_child: Option<String>,
    pub generated: bool,
    pub balance_sheet_summary: f64,
    pub profit_loss_summary: f64,
    pub difference: f64,
}

impl BisectNode {
    pub fn new(
        name: &str,
        root: Option<&str>,
        period_from_date: &str,
        period_to_date: &str,
    ) -> Self {
        Self {
            name: name.to_string(),
            root: root.map(str::to_string),
            period_from_date: period_from_date.to_string(),
            period_to_date: period_to_date.to_string(),
            left_child: None,
            right_child: None,
            generated: false,
            balance_sheet_summary: 0.0,
            profit_loss_summary: 0.0,
            difference: 0.0,
        }
    }

    pub fn children(mut self, left_child: &str, right_child: &str) -> Self {
        self.left_child = Some(left_child.to_string());
        self.right_child = Some(right_child.to_string());
        self
    }

    pub fn summary(mut self, balance_sheet_summary: f64, profit_loss_summary: f64) -> Self {
        self.balance_sheet_summary = balance_sheet_summary;
        self.profit_loss_summary = profit_loss_summary;
        self.difference = (profit_loss_summary - balance_sheet_summary).abs();
        self.generated = true;
        self
    }
}

impl BisectAccountingStatements {
    pub const DOCTYPE: &'static str = "Bisect Accounting Statements";
    pub const MODULE: &'static str = "Accounts";
    pub const FIELD_ORDER: [&'static str; 27] = [
        "section_break_cvfg",
        "company",
        "column_break_hcam",
        "from_date",
        "column_break_qxbi",
        "to_date",
        "column_break_iwny",
        "algorithm",
        "section_break_8ph9",
        "current_node",
        "section_break_ngid",
        "bisect_heatmap",
        "section_break_hmsy",
        "bisecting_from",
        "current_from_date",
        "column_break_uqyd",
        "bisecting_to",
        "current_to_date",
        "section_break_hbyo",
        "heading_cppb",
        "p_l_summary",
        "column_break_aivo",
        "balance_sheet_summary",
        "b_s_summary",
        "column_break_gvwx",
        "difference_heading",
        "difference",
    ];

    pub fn fields() -> Vec<FieldSpec> {
        vec![
            FieldSpec::section_break("section_break_cvfg"),
            FieldSpec::link("company", "Company").options("Company"),
            FieldSpec::column_break("column_break_hcam"),
            FieldSpec::datetime("from_date", "From Date"),
            FieldSpec::column_break("column_break_qxbi"),
            FieldSpec::datetime("to_date", "To Date"),
            FieldSpec::column_break("column_break_iwny"),
            FieldSpec::select("algorithm", "Algorithm")
                .options("BFS\nDFS")
                .default("BFS"),
            FieldSpec::section_break("section_break_8ph9").hidden(),
            FieldSpec::link("current_node", "Current Node").options("Bisect Nodes"),
            FieldSpec::section_break("section_break_ngid"),
            FieldSpec::html("bisect_heatmap", "Heatmap"),
            FieldSpec::section_break("section_break_hmsy"),
            FieldSpec::heading("bisecting_from", "Bisecting From"),
            FieldSpec::datetime("current_from_date", "").read_only(),
            FieldSpec::column_break("column_break_uqyd"),
            FieldSpec::heading("bisecting_to", "Bisecting To"),
            FieldSpec::datetime("current_to_date", "").read_only(),
            FieldSpec::section_break("section_break_hbyo"),
            FieldSpec::heading("heading_cppb", "Profit and Loss Summary"),
            FieldSpec::float("p_l_summary", "").read_only(),
            FieldSpec::column_break("column_break_aivo"),
            FieldSpec::heading("balance_sheet_summary", "Balance Sheet Summary"),
            FieldSpec::float("b_s_summary", "").read_only(),
            FieldSpec::column_break("column_break_gvwx"),
            FieldSpec::heading("difference_heading", "Difference"),
            FieldSpec::float("difference", "").read_only(),
        ]
    }

    pub fn validate(&self) -> Result<(), BisectAccountingStatementsError> {
        self.validate_dates()
    }

    pub fn validate_dates(&self) -> Result<(), BisectAccountingStatementsError> {
        let (Some(from_date), Some(to_date)) = (self.from_date.as_deref(), self.to_date.as_deref())
        else {
            return Ok(());
        };

        if parse_date(from_date) > parse_date(to_date) {
            return Err(BisectAccountingStatementsError::FromDateAfterToDate {
                from_date: from_date.to_string(),
                to_date: to_date.to_string(),
            });
        }

        Ok(())
    }

    pub fn build_tree(&mut self, p_l_summary: f64, b_s_summary: f64) -> Vec<BisectNode> {
        let nodes = self.generate_nodes();
        if let Some(root) = nodes.iter().find(|node| node.root.is_none()) {
            self.current_node = Some(root.name.clone());
            self.current_from_date = self.from_date.clone();
            self.current_to_date = self.to_date.clone();
        }
        self.get_report_summary(p_l_summary, b_s_summary);
        nodes
    }

    pub fn generate_nodes(&self) -> Vec<BisectNode> {
        let from_date = self.from_date.as_deref().unwrap_or_default();
        let to_date = self.to_date.as_deref().unwrap_or_default();
        match self.algorithm {
            BisectAlgorithm::Bfs => generate_bfs_nodes(from_date, to_date),
            BisectAlgorithm::Dfs => generate_dfs_nodes(from_date, to_date),
        }
    }

    pub fn get_report_summary(&mut self, p_l_summary: f64, b_s_summary: f64) {
        self.p_l_summary = p_l_summary;
        self.b_s_summary = b_s_summary;
        self.difference = (self.p_l_summary - self.b_s_summary).abs();
    }

    pub fn update_node(&self, nodes: &mut BTreeMap<String, BisectNode>) {
        if let Some(current_node) = self
            .current_node
            .as_deref()
            .and_then(|name| nodes.get_mut(name))
        {
            current_node.balance_sheet_summary = self.b_s_summary;
            current_node.profit_loss_summary = self.p_l_summary;
            current_node.difference = self.difference;
            current_node.generated = true;
        }
    }

    pub fn current_node_has_summary_info(&self, nodes: &BTreeMap<String, BisectNode>) -> bool {
        self.current_node
            .as_deref()
            .and_then(|name| nodes.get(name))
            .is_some_and(|node| node.generated)
    }

    pub fn fetch_summary_info_from_current_node(&mut self, nodes: &BTreeMap<String, BisectNode>) {
        if let Some(current_node) = self
            .current_node
            .as_deref()
            .and_then(|name| nodes.get(name))
        {
            self.p_l_summary = current_node.balance_sheet_summary;
            self.b_s_summary = current_node.profit_loss_summary;
            self.difference = (self.p_l_summary - self.b_s_summary).abs();
        }
    }

    pub fn fetch_or_calculate(
        &mut self,
        nodes: &mut BTreeMap<String, BisectNode>,
        p_l_summary: f64,
        b_s_summary: f64,
    ) {
        if self.current_node_has_summary_info(nodes) {
            self.fetch_summary_info_from_current_node(nodes);
        } else {
            self.get_report_summary(p_l_summary, b_s_summary);
            self.update_node(nodes);
        }
    }

    pub fn bisect_left(
        &mut self,
        nodes: &mut BTreeMap<String, BisectNode>,
        p_l_summary: f64,
        b_s_summary: f64,
    ) -> Option<BisectMessage> {
        let current = self
            .current_node
            .as_deref()
            .and_then(|name| nodes.get(name))?;
        let Some(left_child) = current.left_child.clone() else {
            return Some(BisectMessage::NoMoreChildrenLeft);
        };
        self.move_to_node(nodes, &left_child, p_l_summary, b_s_summary);
        None
    }

    pub fn bisect_right(
        &mut self,
        nodes: &mut BTreeMap<String, BisectNode>,
        p_l_summary: f64,
        b_s_summary: f64,
    ) -> Option<BisectMessage> {
        let current = self
            .current_node
            .as_deref()
            .and_then(|name| nodes.get(name))?;
        let Some(right_child) = current.right_child.clone() else {
            return Some(BisectMessage::NoMoreChildrenRight);
        };
        self.move_to_node(nodes, &right_child, p_l_summary, b_s_summary);
        None
    }

    pub fn move_up(
        &mut self,
        nodes: &mut BTreeMap<String, BisectNode>,
        p_l_summary: f64,
        b_s_summary: f64,
    ) -> Option<BisectMessage> {
        let current = self
            .current_node
            .as_deref()
            .and_then(|name| nodes.get(name))?;
        let Some(root) = current.root.clone() else {
            return Some(BisectMessage::ReachedRoot);
        };
        self.move_to_node(nodes, &root, p_l_summary, b_s_summary);
        None
    }

    fn move_to_node(
        &mut self,
        nodes: &mut BTreeMap<String, BisectNode>,
        node_name: &str,
        p_l_summary: f64,
        b_s_summary: f64,
    ) {
        if let Some(node) = nodes.get(node_name) {
            self.current_node = Some(node_name.to_string());
            self.current_from_date = Some(node.period_from_date.clone());
            self.current_to_date = Some(node.period_to_date.clone());
        }
        self.fetch_or_calculate(nodes, p_l_summary, b_s_summary);
    }
}

impl DocumentController for BisectAccountingStatements {
    fn doctype(&self) -> &'static str {
        Self::DOCTYPE
    }

    fn module(&self) -> &'static str {
        Self::MODULE
    }

    fn custom_hooks(&self) -> &'static [&'static str] {
        &[
            "validate",
            "build_tree",
            "bisect_left",
            "bisect_right",
            "move_up",
        ]
    }
}

fn generate_bfs_nodes(from_date: &str, to_date: &str) -> Vec<BisectNode> {
    let mut nodes = Vec::new();
    let root = next_node(None, from_date, to_date, nodes.len() + 1);
    nodes.push(root.clone());

    let mut queue = VecDeque::from([root.name]);
    while let Some(current_name) = queue.pop_front() {
        let current = nodes
            .iter()
            .find(|node| node.name == current_name)
            .cloned()
            .expect("node exists");
        if date_diff(&current.period_to_date, &current.period_from_date) == 0 {
            continue;
        }

        let (left, right) = split_node(&current, nodes.len() + 1);
        set_children(&mut nodes, &current.name, &left.name, &right.name);
        queue.push_back(left.name.clone());
        queue.push_back(right.name.clone());
        nodes.push(left);
        nodes.push(right);
    }

    nodes
}

fn generate_dfs_nodes(from_date: &str, to_date: &str) -> Vec<BisectNode> {
    let mut nodes = Vec::new();
    let root = next_node(None, from_date, to_date, nodes.len() + 1);
    nodes.push(root.clone());

    let mut stack = vec![root.name];
    while let Some(current_name) = stack.pop() {
        let current = nodes
            .iter()
            .find(|node| node.name == current_name)
            .cloned()
            .expect("node exists");
        if date_diff(&current.period_to_date, &current.period_from_date) == 0 {
            continue;
        }

        let (left, right) = split_node(&current, nodes.len() + 1);
        set_children(&mut nodes, &current.name, &left.name, &right.name);
        stack.push(left.name.clone());
        stack.push(right.name.clone());
        nodes.push(left);
        nodes.push(right);
    }

    nodes
}

fn split_node(current: &BisectNode, next_index: usize) -> (BisectNode, BisectNode) {
    let delta_days = date_diff(&current.period_to_date, &current.period_from_date);
    let cur_floor = delta_days / 2;
    let next_to_date = add_days(&current.period_from_date, cur_floor);
    let next_from_date = add_days(&current.period_from_date, cur_floor + 1);
    (
        next_node(
            Some(&current.name),
            &current.period_from_date,
            &next_to_date,
            next_index,
        ),
        next_node(
            Some(&current.name),
            &next_from_date,
            &current.period_to_date,
            next_index + 1,
        ),
    )
}

fn next_node(
    root: Option<&str>,
    period_from_date: &str,
    period_to_date: &str,
    index: usize,
) -> BisectNode {
    BisectNode::new(
        &format!("BIS-NODE-{index:04}"),
        root,
        period_from_date,
        period_to_date,
    )
}

fn set_children(nodes: &mut [BisectNode], current_name: &str, left_name: &str, right_name: &str) {
    if let Some(node) = nodes.iter_mut().find(|node| node.name == current_name) {
        node.left_child = Some(left_name.to_string());
        node.right_child = Some(right_name.to_string());
    }
}

fn add_days(date: &str, days: i32) -> String {
    SimpleDate::from_ordinal(parse_date(date).ordinal() + days).to_string()
}

fn date_diff(left: &str, right: &str) -> i32 {
    parse_date(left).ordinal() - parse_date(right).ordinal()
}

fn parse_date(value: &str) -> SimpleDate {
    SimpleDate::parse(value)
}

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
struct SimpleDate {
    year: i32,
    month: u8,
    day: u8,
}

impl SimpleDate {
    fn parse(value: &str) -> Self {
        let date = value.split_whitespace().next().unwrap_or(value);
        let mut parts = date.split('-');
        let year = parts
            .next()
            .and_then(|part| part.parse::<i32>().ok())
            .expect("invalid date year");
        let month = parts
            .next()
            .and_then(|part| part.parse::<u8>().ok())
            .expect("invalid date month");
        let day = parts
            .next()
            .and_then(|part| part.parse::<u8>().ok())
            .expect("invalid date day");
        Self { year, month, day }
    }

    fn ordinal(self) -> i32 {
        days_from_civil(self.year, self.month, self.day)
    }

    fn from_ordinal(days: i32) -> Self {
        let z = days + 719_468;
        let era = if z >= 0 { z } else { z - 146_096 } / 146_097;
        let doe = z - era * 146_097;
        let yoe = (doe - doe / 1460 + doe / 36524 - doe / 146096) / 365;
        let y = yoe + era * 400;
        let doy = doe - (365 * yoe + yoe / 4 - yoe / 100);
        let mp = (5 * doy + 2) / 153;
        let day = doy - (153 * mp + 2) / 5 + 1;
        let month = mp + if mp < 10 { 3 } else { -9 };
        let year = y + if month <= 2 { 1 } else { 0 };
        Self {
            year,
            month: month as u8,
            day: day as u8,
        }
    }
}

impl std::fmt::Display for SimpleDate {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            formatter,
            "{:04}-{:02}-{:02}",
            self.year, self.month, self.day
        )
    }
}

fn days_from_civil(year: i32, month: u8, day: u8) -> i32 {
    let year = year - if month <= 2 { 1 } else { 0 };
    let era = if year >= 0 { year } else { year - 399 } / 400;
    let yoe = year - era * 400;
    let month = i32::from(month);
    let day = i32::from(day);
    let doy = (153 * (month + if month > 2 { -3 } else { 9 }) + 2) / 5 + day - 1;
    let doe = yoe * 365 + yoe / 4 - yoe / 100 + doy;
    era * 146_097 + doe - 719_468
}
