use serde::Deserialize;
use std::fs;
use std::path::Path;

#[derive(Deserialize)]
pub struct Settings {
    company: Company,
    client: Client,
    unit: Option<String>,
    unit_cost: Option<f64>,
    accounts: Option<Vec<Account>>,
    phones: Option<Vec<Phone>>,
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
}

#[derive(Deserialize)]
pub struct Company {
    name: String,
    email: Option<String>,
    registry_no: Option<String>,
    adress: Option<String>,
    city: Option<String>,
    country: Option<String>,
    post_index: Option<String>,
    kmkr: Option<String>,
}

impl Company {
    pub fn get_name(&self) -> &str {
        &self.name
    }
}

#[derive(Deserialize)]
pub struct Client {
    name: String,
    adress: Option<String>,
    city: Option<String>,
    post_index: Option<String>,
    registry_no: Option<String>,
}

impl Client {
    pub fn get_name(&self) -> &str {
        &self.name
    }
}


#[derive(Deserialize)]
pub struct Account {
    bank_name: String,
    iban: String,
}

#[derive(Deserialize)]
pub struct Phone {
    phone_no: String,
    phone_type: String,
}
