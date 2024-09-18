#![allow(dead_code)]

use serde::Deserialize;
use std::fs;
use std::path::Path;

#[derive(Deserialize)]
pub struct Settings {
    company: Company,
    client: Client,
    unit: String,
    unit_cost: f32,
    kbm: u32,
    bill_no: String,
    date: String,
    deadline: String,
    account: Account,
    phone: Phone,
}

impl Settings {
    pub fn load() -> Settings {
        let path = Path::new("./settings.toml");
        let toml: String = fs::read_to_string(path).expect("Failed to read file");
        toml::from_str(&toml).expect("Failed to deserialize string")
    }

    pub fn get_company(&self) -> &Company {
        &self.company
    }

    pub fn get_client(&self) -> &Client {
        &self.client
    }

    pub fn get_unit(&self) -> &str {
        &self.unit
    }

    pub fn get_bill_no(&self) -> &str {
        &self.bill_no
    }

    pub fn get_date(&self) -> &str {
        &self.date
    }

    pub fn get_deadline(&self) -> &str {
        &self.deadline
    }

    pub fn get_unit_cost(&self) -> f32 {
        self.unit_cost
    }

    pub fn get_kbm(&self) -> u32 {
        self.kbm
    }

    pub fn get_account(&self) -> &Account {
        &self.account
    }

    pub fn get_phone(&self) -> &Phone {
        &self.phone
    }
}

#[derive(Deserialize)]
pub struct Company {
    name: String,
    email: String,
    registry_no: String,
    address: String,
    city: String,
    country: String,
    post_index: String,
    kmkr: String,
}

impl Company {
    pub fn get_name(&self) -> &str {
        &self.name
    }

    pub fn get_email(&self) -> &str {
        &self.email
    }

    pub fn get_registry_no(&self) -> &str {
        &self.registry_no
    }

    pub fn get_address(&self) -> &str {
        &self.address
    }

    pub fn get_city(&self) -> &str {
        &self.city
    }

    pub fn get_country(&self) -> &str {
        &self.country
    }

    pub fn get_post_index(&self) -> &str {
        &self.post_index
    }

    pub fn get_kmkr(&self) -> &str {
        &self.kmkr
    }
}

#[derive(Deserialize)]
pub struct Client {
    name: String,
    address: String,
    city: String,
    post_index: String,
    registry_no: String,
}

impl Client {
    pub fn get_name(&self) -> &str {
        &self.name
    }

    pub fn get_address(&self) -> &str {
        &self.address
    }

    pub fn get_city(&self) -> &str {
        &self.city
    }

    pub fn get_post_index(&self) -> &str {
        &self.post_index
    }

    pub fn get_registry_no(&self) -> &str {
        &self.registry_no
    }
}

#[derive(Deserialize)]
pub struct Account {
    bank_name: String,
    iban: String,
}

impl Account {
    pub fn get_bank_name(&self) -> &str {
        &self.bank_name
    }

    pub fn get_iban(&self) -> &str {
        &self.iban
    }
}

#[derive(Deserialize)]
pub struct Phone {
    phone_no: String,
    phone_type: String,
}

impl Phone {
    pub fn get_phone_no(&self) -> &str {
        &self.phone_no
    }

    pub fn get_phone_type(&self) -> &str {
        &self.phone_type
    }
}
