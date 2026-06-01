use std::collections::BTreeMap;

use tokio_erp::erpnext::accounts::doctype::bisect_accounting_statements::bisect_accounting_statements::{
    BisectAccountingStatements, BisectAccountingStatementsError, BisectAlgorithm, BisectMessage,
    BisectNode,
};
use tokio_erp::erpnext::{DocumentController, FieldSpec};

#[test]
fn bisect_accounting_statements_matches_erpnext_metadata() {
    assert_eq!(
        BisectAccountingStatements::DOCTYPE,
        "Bisect Accounting Statements"
    );
    assert_eq!(BisectAccountingStatements::MODULE, "Accounts");
    assert_eq!(BisectAccountingStatements::FIELD_ORDER.len(), 27);
    assert_eq!(
        &BisectAccountingStatements::FIELD_ORDER[..8],
        [
            "section_break_cvfg",
            "company",
            "column_break_hcam",
            "from_date",
            "column_break_qxbi",
            "to_date",
            "column_break_iwny",
            "algorithm",
        ]
    );
    assert_eq!(
        &BisectAccountingStatements::FIELD_ORDER
            [BisectAccountingStatements::FIELD_ORDER.len() - 4..],
        [
            "b_s_summary",
            "column_break_gvwx",
            "difference_heading",
            "difference",
        ]
    );

    let fields = BisectAccountingStatements::fields();
    assert!(fields.contains(&FieldSpec::link("company", "Company").options("Company")));
    assert!(fields.contains(&FieldSpec::datetime("from_date", "From Date")));
    assert!(fields.contains(&FieldSpec::datetime("to_date", "To Date")));
    assert!(fields.contains(
        &FieldSpec::select("algorithm", "Algorithm")
            .options("BFS\nDFS")
            .default("BFS")
    ));
    assert!(fields.contains(&FieldSpec::html("bisect_heatmap", "Heatmap")));
}

#[test]
fn bisect_accounting_statements_validates_date_order() {
    let doc = BisectAccountingStatements {
        from_date: Some("2026-05-02".to_string()),
        to_date: Some("2026-05-01".to_string()),
        ..Default::default()
    };

    assert_eq!(
        doc.validate(),
        Err(BisectAccountingStatementsError::FromDateAfterToDate {
            from_date: "2026-05-02".to_string(),
            to_date: "2026-05-01".to_string(),
        })
    );
}

#[test]
fn bisect_accounting_statements_builds_bfs_tree_like_erpnext_queue() {
    let mut doc = BisectAccountingStatements {
        from_date: Some("2026-05-01".to_string()),
        to_date: Some("2026-05-04".to_string()),
        algorithm: BisectAlgorithm::Bfs,
        ..Default::default()
    };

    let nodes = doc.build_tree(30.0, 12.5);

    assert_eq!(doc.current_node.as_deref(), Some("BIS-NODE-0001"));
    assert_eq!(doc.current_from_date.as_deref(), Some("2026-05-01"));
    assert_eq!(doc.current_to_date.as_deref(), Some("2026-05-04"));
    assert_eq!(doc.p_l_summary, 30.0);
    assert_eq!(doc.b_s_summary, 12.5);
    assert_eq!(doc.difference, 17.5);
    assert_eq!(
        nodes,
        vec![
            BisectNode::new("BIS-NODE-0001", None, "2026-05-01", "2026-05-04")
                .children("BIS-NODE-0002", "BIS-NODE-0003"),
            BisectNode::new(
                "BIS-NODE-0002",
                Some("BIS-NODE-0001"),
                "2026-05-01",
                "2026-05-02",
            )
            .children("BIS-NODE-0004", "BIS-NODE-0005"),
            BisectNode::new(
                "BIS-NODE-0003",
                Some("BIS-NODE-0001"),
                "2026-05-03",
                "2026-05-04",
            )
            .children("BIS-NODE-0006", "BIS-NODE-0007"),
            BisectNode::new(
                "BIS-NODE-0004",
                Some("BIS-NODE-0002"),
                "2026-05-01",
                "2026-05-01",
            ),
            BisectNode::new(
                "BIS-NODE-0005",
                Some("BIS-NODE-0002"),
                "2026-05-02",
                "2026-05-02",
            ),
            BisectNode::new(
                "BIS-NODE-0006",
                Some("BIS-NODE-0003"),
                "2026-05-03",
                "2026-05-03",
            ),
            BisectNode::new(
                "BIS-NODE-0007",
                Some("BIS-NODE-0003"),
                "2026-05-04",
                "2026-05-04",
            ),
        ]
    );
}

#[test]
fn bisect_accounting_statements_builds_dfs_tree_like_erpnext_stack() {
    let doc = BisectAccountingStatements {
        from_date: Some("2026-05-01".to_string()),
        to_date: Some("2026-05-04".to_string()),
        algorithm: BisectAlgorithm::Dfs,
        ..Default::default()
    };

    let nodes = doc.generate_nodes();
    let names_and_periods = nodes
        .iter()
        .map(|node| {
            (
                node.name.as_str(),
                node.root.as_deref(),
                node.period_from_date.as_str(),
                node.period_to_date.as_str(),
            )
        })
        .collect::<Vec<_>>();

    assert_eq!(
        names_and_periods,
        vec![
            ("BIS-NODE-0001", None, "2026-05-01", "2026-05-04"),
            (
                "BIS-NODE-0002",
                Some("BIS-NODE-0001"),
                "2026-05-01",
                "2026-05-02",
            ),
            (
                "BIS-NODE-0003",
                Some("BIS-NODE-0001"),
                "2026-05-03",
                "2026-05-04",
            ),
            (
                "BIS-NODE-0004",
                Some("BIS-NODE-0003"),
                "2026-05-03",
                "2026-05-03",
            ),
            (
                "BIS-NODE-0005",
                Some("BIS-NODE-0003"),
                "2026-05-04",
                "2026-05-04",
            ),
            (
                "BIS-NODE-0006",
                Some("BIS-NODE-0002"),
                "2026-05-01",
                "2026-05-01",
            ),
            (
                "BIS-NODE-0007",
                Some("BIS-NODE-0002"),
                "2026-05-02",
                "2026-05-02",
            ),
        ]
    );
}

#[test]
fn bisect_accounting_statements_navigation_and_cached_summary_match_erpnext() {
    let mut doc = BisectAccountingStatements {
        current_node: Some("BIS-NODE-0001".to_string()),
        ..Default::default()
    };
    let mut nodes = BTreeMap::from([
        (
            "BIS-NODE-0001".to_string(),
            BisectNode::new("BIS-NODE-0001", None, "2026-05-01", "2026-05-04")
                .children("BIS-NODE-0002", "BIS-NODE-0003"),
        ),
        (
            "BIS-NODE-0002".to_string(),
            BisectNode::new(
                "BIS-NODE-0002",
                Some("BIS-NODE-0001"),
                "2026-05-01",
                "2026-05-02",
            )
            .summary(70.0, 50.0),
        ),
        (
            "BIS-NODE-0003".to_string(),
            BisectNode::new(
                "BIS-NODE-0003",
                Some("BIS-NODE-0001"),
                "2026-05-03",
                "2026-05-04",
            ),
        ),
    ]);

    assert_eq!(doc.bisect_left(&mut nodes, 1.0, 2.0), None);
    assert_eq!(doc.current_node.as_deref(), Some("BIS-NODE-0002"));
    assert_eq!(doc.current_from_date.as_deref(), Some("2026-05-01"));
    assert_eq!(doc.current_to_date.as_deref(), Some("2026-05-02"));
    assert_eq!(doc.p_l_summary, 70.0);
    assert_eq!(doc.b_s_summary, 50.0);
    assert_eq!(doc.difference, 20.0);

    assert_eq!(doc.move_up(&mut nodes, 30.0, 12.0), None);
    assert_eq!(doc.current_node.as_deref(), Some("BIS-NODE-0001"));
    assert!(nodes["BIS-NODE-0001"].generated);
    assert_eq!(nodes["BIS-NODE-0001"].profit_loss_summary, 30.0);
    assert_eq!(nodes["BIS-NODE-0001"].balance_sheet_summary, 12.0);

    assert_eq!(
        doc.move_up(&mut nodes, 0.0, 0.0),
        Some(BisectMessage::ReachedRoot)
    );
    assert_eq!(doc.bisect_right(&mut nodes, 5.0, 1.0), None);
    assert_eq!(doc.current_node.as_deref(), Some("BIS-NODE-0003"));
    assert_eq!(doc.p_l_summary, 5.0);
    assert_eq!(doc.b_s_summary, 1.0);
    assert_eq!(doc.difference, 4.0);
    assert_eq!(
        doc.bisect_right(&mut nodes, 0.0, 0.0),
        Some(BisectMessage::NoMoreChildrenRight)
    );
}

#[test]
fn bisect_accounting_statements_preserves_controller_hooks() {
    let doc = BisectAccountingStatements::default();

    assert_eq!(doc.doctype(), "Bisect Accounting Statements");
    assert_eq!(doc.module(), "Accounts");
    assert_eq!(
        doc.custom_hooks(),
        [
            "validate",
            "build_tree",
            "bisect_left",
            "bisect_right",
            "move_up",
        ]
    );
}
