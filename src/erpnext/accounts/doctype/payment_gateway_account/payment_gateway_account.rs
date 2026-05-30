use crate::erpnext::{DocumentController, FieldSpec};

#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct PaymentGatewayAccount {
    pub name: Option<String>,
    pub payment_gateway: Option<String>,
    pub payment_channel: Option<String>,
    pub company: Option<String>,
    pub is_default: bool,
    pub payment_account: Option<String>,
    pub currency: Option<String>,
    pub message: Option<String>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PaymentGatewayDefaultUpdate {
    pub doctype: &'static str,
    pub company: Option<String>,
    pub current_name: Option<String>,
    pub fieldname: &'static str,
    pub value: bool,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PaymentGatewayValidatePlan {
    pub unset_other_default: Option<PaymentGatewayDefaultUpdate>,
}

impl PaymentGatewayAccount {
    pub const DOCTYPE: &'static str = "Payment Gateway Account";
    pub const MODULE: &'static str = "Accounts";
    pub const FIELD_ORDER: [&'static str; 10] = [
        "payment_gateway",
        "payment_channel",
        "company",
        "is_default",
        "column_break_4",
        "payment_account",
        "currency",
        "payment_request_message",
        "message",
        "message_examples",
    ];
    pub const EDITABLE_GRID: bool = true;
    pub const INDEX_WEB_PAGES_FOR_SEARCH: bool = true;
    pub const ROW_FORMAT: &'static str = "Dynamic";
    pub const SORT_FIELD: &'static str = "creation";
    pub const SORT_ORDER: &'static str = "DESC";
    pub const MESSAGE_EXAMPLES: &'static str = r#"<pre><h5>Message Example</h5>

&lt;p&gt; Thank You for being a part of {{ doc.company }}! We hope you are enjoying the service.&lt;/p&gt;

&lt;p&gt; Please find enclosed the E Bill statement. The outstanding amount is {{ doc.grand_total }}.&lt;/p&gt;

&lt;p&gt; We don't want you to be spending time running around in order to pay for your Bill.<br>After all, life is beautiful and the time you have in hand should be spent to enjoy it!<br>So here are our little ways to help you get more time for life! &lt;/p&gt;

&lt;a href="{{ payment_url }}"&gt; click here to pay &lt;/a&gt;

</pre>
"#;

    pub fn fields() -> Vec<FieldSpec> {
        vec![
            FieldSpec::link("payment_gateway", "Payment Gateway")
                .options("Payment Gateway")
                .in_list_view()
                .required(),
            FieldSpec::select("payment_channel", "Payment Channel")
                .options("\nEmail\nPhone\nOther")
                .default("Email"),
            FieldSpec::link("company", "Company")
                .options("Company")
                .in_list_view()
                .print_hide()
                .required(),
            FieldSpec::check("is_default", "Is Default").default("0"),
            FieldSpec::column_break("column_break_4"),
            FieldSpec::link("payment_account", "Payment Account")
                .options("Account")
                .in_list_view()
                .required(),
            FieldSpec::read_only_field("currency", "Currency")
                .fetch_from("payment_account.account_currency"),
            FieldSpec::section_break("payment_request_message")
                .depends_on("eval: doc.payment_channel == 'Email' || (!doc.payment_channel)"),
            FieldSpec::small_text("message", "Default Payment Request Message")
                .default("Please click on the link below to make your payment"),
            FieldSpec::html("message_examples", "Message Examples").options(Self::MESSAGE_EXAMPLES),
        ]
    }

    pub fn autoname_with_company_abbr(&mut self, company_abbr: &str) -> String {
        let payment_gateway = self.payment_gateway.as_deref().unwrap_or_default();
        let currency = self.currency.as_deref().unwrap_or_default();
        let name = format!("{payment_gateway} - {currency} - {company_abbr}");
        self.name = Some(name.clone());
        name
    }

    pub fn validate_with_account_currency(
        &mut self,
        account_currency: impl Into<String>,
        default_exists_excluding_self: bool,
    ) -> PaymentGatewayValidatePlan {
        self.currency = Some(account_currency.into());
        let unset_other_default = self.update_default_payment_gateway_plan();
        self.set_as_default_if_not_set(default_exists_excluding_self);
        PaymentGatewayValidatePlan {
            unset_other_default,
        }
    }

    pub fn update_default_payment_gateway_plan(&self) -> Option<PaymentGatewayDefaultUpdate> {
        self.is_default.then(|| PaymentGatewayDefaultUpdate {
            doctype: Self::DOCTYPE,
            company: self.company.clone(),
            current_name: self.name.clone(),
            fieldname: "is_default",
            value: false,
        })
    }

    pub fn set_as_default_if_not_set(&mut self, default_exists_excluding_self: bool) {
        if !default_exists_excluding_self {
            self.is_default = true;
        }
    }
}

impl DocumentController for PaymentGatewayAccount {
    fn doctype(&self) -> &'static str {
        Self::DOCTYPE
    }

    fn module(&self) -> &'static str {
        Self::MODULE
    }

    fn custom_hooks(&self) -> &'static [&'static str] {
        &["autoname", "validate"]
    }
}
