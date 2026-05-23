use crate::erpnext::{DocumentController, FieldSpec};

#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct ProcessSubscription {
    pub posting_date: String,
    pub subscription: Option<String>,
    pub amended_from: Option<String>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SubscriptionJob {
    pub method: String,
    pub queue: String,
    pub subscriptions: Vec<String>,
    pub posting_date: String,
}

impl ProcessSubscription {
    pub const DOCTYPE: &'static str = "Process Subscription";
    pub const MODULE: &'static str = "Accounts";
    pub const FIELD_ORDER: [&'static str; 3] = ["posting_date", "subscription", "amended_from"];
    pub const ALLOW_RENAME: bool = true;
    pub const EDITABLE_GRID: bool = true;
    pub const INDEX_WEB_PAGES_FOR_SEARCH: bool = true;
    pub const IS_SUBMITTABLE: bool = true;

    pub fn new(posting_date: impl Into<String>) -> Self {
        Self {
            posting_date: posting_date.into(),
            subscription: None,
            amended_from: None,
        }
    }

    pub fn fields() -> Vec<FieldSpec> {
        vec![
            FieldSpec::date("posting_date", "Posting Date")
                .required()
                .in_list_view(),
            FieldSpec::link("subscription", "Subscription").options("Subscription"),
            FieldSpec::link("amended_from", "Amended From")
                .options("Process Subscription")
                .read_only()
                .print_hide(),
        ]
    }

    pub fn on_submit(&self, active_subscriptions: &[String]) -> Vec<SubscriptionJob> {
        self.process_all_subscription(active_subscriptions)
    }

    pub fn process_all_subscription(
        &self,
        active_subscriptions: &[String],
    ) -> Vec<SubscriptionJob> {
        let subscriptions = active_subscriptions
            .iter()
            .filter(|subscription| {
                self.subscription
                    .as_ref()
                    .map(|selected| selected == *subscription)
                    .unwrap_or(true)
            })
            .cloned()
            .collect::<Vec<_>>();

        subscriptions
            .chunks(500)
            .map(|subscription| SubscriptionJob {
                method: "erpnext.accounts.doctype.subscription.subscription.process_all"
                    .to_string(),
                queue: "long".to_string(),
                subscriptions: subscription.to_vec(),
                posting_date: self.posting_date.clone(),
            })
            .collect()
    }
}

impl DocumentController for ProcessSubscription {
    fn doctype(&self) -> &'static str {
        Self::DOCTYPE
    }

    fn module(&self) -> &'static str {
        Self::MODULE
    }

    fn custom_hooks(&self) -> &'static [&'static str] {
        &["on_submit"]
    }
}

pub fn create_subscription_process(
    subscription: Option<&str>,
    posting_date: Option<&str>,
) -> ProcessSubscription {
    let mut doc = ProcessSubscription::new(posting_date.unwrap_or_default());
    doc.subscription = subscription.map(str::to_string);
    doc
}
