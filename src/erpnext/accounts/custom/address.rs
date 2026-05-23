#[derive(Clone, Debug, Eq, PartialEq)]
pub struct AddressLink {
    pub link_doctype: String,
    pub link_name: String,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ErpnextAddress {
    pub name: String,
    pub is_your_company_address: bool,
    pub links: Vec<AddressLink>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CustomerPrimaryAddressUpdate {
    pub customer: String,
    pub primary_address: String,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ShippingAddress {
    pub name: String,
    pub company: String,
    pub is_your_company_address: bool,
    pub is_shipping_address: bool,
    pub display: String,
}

impl AddressLink {
    pub fn new(link_doctype: impl Into<String>, link_name: impl Into<String>) -> Self {
        Self {
            link_doctype: link_doctype.into(),
            link_name: link_name.into(),
        }
    }
}

impl ErpnextAddress {
    pub fn validate(&mut self) -> Result<(), String> {
        self.validate_reference()?;
        self.update_company_address();
        Ok(())
    }

    pub fn should_link_address_by_owner(&self) -> bool {
        !self.is_your_company_address
    }

    pub fn update_company_address(&mut self) {
        if self.links.iter().any(|link| link.link_doctype == "Company") {
            self.is_your_company_address = true;
        }
    }

    pub fn validate_reference(&self) -> Result<(), String> {
        if self.is_your_company_address
            && !self.links.iter().any(|link| link.link_doctype == "Company")
        {
            return Err("Company Not Linked: Address needs to be linked to a Company. Please add a row for Company in the Links table.".to_string());
        }

        Ok(())
    }

    pub fn on_update(
        &self,
        address_display: &str,
        customers_with_primary_address: &[String],
    ) -> Vec<CustomerPrimaryAddressUpdate> {
        customers_with_primary_address
            .iter()
            .map(|customer| CustomerPrimaryAddressUpdate {
                customer: customer.clone(),
                primary_address: address_display.to_string(),
            })
            .collect()
    }

    pub fn shipping_address_filters<'a>(
        company: &'a str,
        address: Option<&'a str>,
        address_linked_to_company: bool,
    ) -> Vec<(&'static str, &'static str, &'static str, &'a str)> {
        let mut filters = vec![
            ("Dynamic Link", "link_doctype", "=", "Company"),
            ("Dynamic Link", "link_name", "=", company),
            ("Address", "is_your_company_address", "=", "1"),
        ];

        if let Some(address) = address {
            if address_linked_to_company {
                filters.push(("Address", "name", "=", address));
            }
        } else {
            filters.push(("Address", "is_shipping_address", "=", "1"));
        }

        filters
    }
}

pub fn get_shipping_address(
    company: &str,
    address: Option<&str>,
    address_linked_to_company: bool,
    addresses: &[ShippingAddress],
) -> Option<(String, String)> {
    let _filters =
        ErpnextAddress::shipping_address_filters(company, address, address_linked_to_company);

    addresses
        .iter()
        .find(|candidate| {
            candidate.company == company
                && candidate.is_your_company_address
                && match (address, address_linked_to_company) {
                    (Some(address), true) => candidate.name == address,
                    (Some(_), false) => true,
                    (None, _) => candidate.is_shipping_address,
                }
        })
        .map(|candidate| (candidate.name.clone(), candidate.display.clone()))
}
