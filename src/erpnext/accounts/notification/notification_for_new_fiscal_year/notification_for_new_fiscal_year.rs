#[derive(Clone, Debug, Eq, PartialEq)]
pub struct FiscalYearNotificationContext {
    pub name: String,
    pub year_start_date: String,
    pub year_end_date: String,
    pub companies: Vec<String>,
    pub disabled: bool,
    pub auto_created: bool,
}

pub struct NotificationForNewFiscalYear;

impl NotificationForNewFiscalYear {
    pub const NAME: &'static str = "Notification for new fiscal year";
    pub const MODULE: &'static str = "Accounts";
    pub const SUBJECT: &'static str = "New Fiscal Year {{ doc.name }} - Review Required";
    pub const DOCUMENT_TYPE: &'static str = "Fiscal Year";
    pub const EVENT: &'static str = "New";
    pub const CHANNEL: &'static str = "Email";
    pub const ENABLED: bool = true;
    pub const CONDITION: &'static str = "doc.auto_created == 1";
    pub const ATTACH_PRINT: bool = false;
    pub const RECIPIENT_ROLES: [&'static str; 2] = ["Accounts User", "Accounts Manager"];
    pub const MESSAGE: &'static str = r#"<h4>{{ _("New Fiscal Year - {0}").format(doc.name) }}</h4>

<p>{{ _("A new fiscal year has been automatically created.") }}</p>

<p>{{ _("Fiscal Year Details") }}</p>

<table style="margin-bottom: 1rem; width: 70%">
    <tr>
        <td style="font-weight:bold; width: 40%">{{ _("Year Name") }}</td>
        <td>{{ doc.name }}</td>
    </tr>
    <tr>
        <td style="font-weight:bold; width: 40%">{{ _("Start Date") }}</td>
        <td>{{ frappe.format_value(doc.year_start_date) }}</td>
    </tr>
    <tr>
        <td style="font-weight:bold; width: 40%">{{ _("End Date") }}</td>
        <td>{{ frappe.format_value(doc.year_end_date) }}</td>
    </tr>
    {% if doc.companies|length > 0 %}
    <tr>
        <td style="vertical-align: top; font-weight: bold; width: 40%" rowspan="{{ doc.companies|length }}">
        {% if doc.companies|length < 2 %}
            {{ _("Company") }}
        {% else %}
            {{ _("Companies") }}
        {% endif %}
        </td>
        <td>{{ doc.companies[0].company }}</td>
    </tr>
    {% for idx in range(1, doc.companies|length) %}
    <tr>
        <td>{{ doc.companies[idx].company }}</td>
    </tr>
    {% endfor %}
    {% endif %}
</table>

{% if doc.disabled %}
<p>{{ _("The fiscal year has been automatically created in a Disabled state to maintain consistency with the previous fiscal year's status.") }}</p>
{% endif %}

<p>{{ _("Please review the {0} configuration and complete any required financial setup activities.").format(frappe.utils.get_link_to_form("Fiscal Year", doc.name, frappe.bold("Fiscal Year"))) }}</p>"#;
}

pub fn get_context(context: FiscalYearNotificationContext) -> FiscalYearNotificationContext {
    context
}
