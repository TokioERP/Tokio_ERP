use tokio_erp::erpnext::accounts::custom::address::{
    get_shipping_address, AddressLink, CustomerPrimaryAddressUpdate, ErpnextAddress,
    ShippingAddress,
};
use tokio_erp::erpnext::FieldSpec;

#[test]
fn custom_address_fields_match_erpnext_json_customization() {
    assert_eq!(
        ErpnextAddress::custom_fields(),
        vec![
            FieldSpec::link("tax_category", "Tax Category").options("Tax Category"),
            FieldSpec::check("is_your_company_address", "Is Your Company Address").default("0"),
        ]
    );
}

#[test]
fn validate_reference_requires_company_link_for_company_address() {
    let mut address = ErpnextAddress {
        name: "ADDR-0001".to_string(),
        is_your_company_address: true,
        links: vec![],
    };

    assert_eq!(
        address.validate(),
        Err("Company Not Linked: Address needs to be linked to a Company. Please add a row for Company in the Links table.".to_string())
    );

    address.links.push(AddressLink::new("Company", "Acme"));
    assert_eq!(address.validate(), Ok(()));
    assert!(address.is_your_company_address);
}

#[test]
fn update_company_address_marks_address_when_company_link_exists() {
    let mut address = ErpnextAddress {
        name: "ADDR-0002".to_string(),
        is_your_company_address: false,
        links: vec![
            AddressLink::new("Customer", "CUST-0001"),
            AddressLink::new("Company", "Acme"),
        ],
    };

    address.update_company_address();

    assert!(address.is_your_company_address);
}

#[test]
fn link_address_skips_owner_link_for_company_address() {
    let company_address = ErpnextAddress {
        name: "ADDR-0003".to_string(),
        is_your_company_address: true,
        links: vec![AddressLink::new("Company", "Acme")],
    };
    let customer_address = ErpnextAddress {
        name: "ADDR-0004".to_string(),
        is_your_company_address: false,
        links: vec![AddressLink::new("Customer", "CUST-0001")],
    };

    assert!(!company_address.should_link_address_by_owner());
    assert!(customer_address.should_link_address_by_owner());
}

#[test]
fn on_update_plans_customer_primary_address_updates() {
    let address = ErpnextAddress {
        name: "ADDR-0005".to_string(),
        is_your_company_address: false,
        links: vec![AddressLink::new("Customer", "CUST-0001")],
    };

    assert_eq!(
        address.on_update(
            "Rendered Address",
            &["CUST-0001".to_string(), "CUST-0002".to_string()]
        ),
        vec![
            CustomerPrimaryAddressUpdate {
                customer: "CUST-0001".to_string(),
                primary_address: "Rendered Address".to_string(),
            },
            CustomerPrimaryAddressUpdate {
                customer: "CUST-0002".to_string(),
                primary_address: "Rendered Address".to_string(),
            },
        ]
    );
}

#[test]
fn shipping_address_filters_match_erpnext_branching() {
    let filters_without_address = ErpnextAddress::shipping_address_filters("Acme", None, false);
    assert_eq!(
        filters_without_address,
        vec![
            ("Dynamic Link", "link_doctype", "=", "Company"),
            ("Dynamic Link", "link_name", "=", "Acme"),
            ("Address", "is_your_company_address", "=", "1"),
            ("Address", "is_shipping_address", "=", "1"),
        ]
    );

    let filters_with_valid_address =
        ErpnextAddress::shipping_address_filters("Acme", Some("ADDR-0006"), true);
    assert_eq!(
        filters_with_valid_address,
        vec![
            ("Dynamic Link", "link_doctype", "=", "Company"),
            ("Dynamic Link", "link_name", "=", "Acme"),
            ("Address", "is_your_company_address", "=", "1"),
            ("Address", "name", "=", "ADDR-0006"),
        ]
    );

    let filters_with_unlinked_address =
        ErpnextAddress::shipping_address_filters("Acme", Some("ADDR-0007"), false);
    assert_eq!(filters_with_unlinked_address, filters_without_address[..3]);
}

#[test]
fn get_shipping_address_returns_first_rendered_company_address() {
    let addresses = vec![
        ShippingAddress {
            name: "ADDR-0008".to_string(),
            company: "Acme".to_string(),
            is_your_company_address: true,
            is_shipping_address: true,
            display: "Rendered Shipping Address".to_string(),
        },
        ShippingAddress {
            name: "ADDR-0009".to_string(),
            company: "Acme".to_string(),
            is_your_company_address: true,
            is_shipping_address: false,
            display: "Rendered Billing Address".to_string(),
        },
    ];

    assert_eq!(
        get_shipping_address("Acme", None, false, &addresses),
        Some((
            "ADDR-0008".to_string(),
            "Rendered Shipping Address".to_string()
        ))
    );
    assert_eq!(get_shipping_address("Other", None, false, &addresses), None);
}

#[test]
fn get_shipping_address_matches_address_filter_fallbacks() {
    let addresses = vec![
        ShippingAddress {
            name: "ADDR-0010".to_string(),
            company: "Acme".to_string(),
            is_your_company_address: true,
            is_shipping_address: false,
            display: "Rendered Company Address".to_string(),
        },
        ShippingAddress {
            name: "ADDR-0011".to_string(),
            company: "Acme".to_string(),
            is_your_company_address: true,
            is_shipping_address: true,
            display: "Rendered Shipping Address".to_string(),
        },
    ];

    assert_eq!(
        get_shipping_address("Acme", Some("ADDR-0011"), true, &addresses),
        Some((
            "ADDR-0011".to_string(),
            "Rendered Shipping Address".to_string()
        ))
    );
    assert_eq!(
        get_shipping_address("Acme", Some("ADDR-MISSING"), false, &addresses),
        Some((
            "ADDR-0010".to_string(),
            "Rendered Company Address".to_string()
        ))
    );
}
