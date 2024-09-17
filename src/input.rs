use std::path::Path;
use serde::Deserialize;
use build_html::{HtmlElement, HtmlTag, Html, HtmlChild};

use crate::settings::Settings;

pub struct Input {
    items: Vec<Item>
}

impl Input {
    pub fn load(settings: &Settings) -> Self {
        let mut input = Self {items: Vec::new()};
        let path = Path::new("./input.csv");
        let rdr = csv::Reader::from_path(path);
        for result in rdr.expect("Failed to read csv").deserialize() {
            let row: CSVRow = result.expect("Failed to serialize input");
            input.items.push(Item::new(row, settings));
        }
        input
    }

    pub fn to_html(&self) -> String {
        self.items.iter()
            .map(|item| item.to_html())
            .collect::<Vec<String>>()
            .join("")
    }

    pub fn get_sum_total(&self) -> Cost {
        self.items.iter().fold(Cost::new(0.0), |acc, item| acc.add(item.get_sum()))
    }

    pub fn get_kbm_total(&self) -> Cost {
        self.items.iter().fold(Cost::new(0.0), |acc, item| acc.add(item.get_kbm()))
    }

    pub fn get_grand_total(&self) -> Cost {
        self.items.iter().fold(Cost::new(0.0), |acc, item| acc.add(item.get_total()))
    }
}

#[derive(Deserialize)]
struct CSVRow {
    description: String,
    amount: u32,
    discount: u32,
}

struct Item {
    item: String,
    cost: Cost,
    amount: String,
    sum: Cost,
    kbm: Cost,
    total: Cost,
}

impl Item {
    pub fn new(row: CSVRow, settings: &Settings) -> Self {
        let cost = settings.get_unit_cost() * (1.0 - row.discount as f32 / 100.0);
        let sum = cost * row.amount as f32;
        let kbm = sum * (settings.get_kbm() as f32 / 100.0);
        Self {
            item: row.description,
            cost: Cost::new(cost),
            amount: row.amount.to_string() + " " + settings.get_unit(),
            sum: Cost::new(sum),
            kbm: Cost::new(kbm),
            total: Cost::new(sum + kbm)
        }
    }

    pub fn to_html(&self) -> String {
        HtmlElement::new(HtmlTag::TableRow)
            .with_child(Self::create_cell(&self.item, false))
            .with_child(Self::create_cell(&self.cost.to_string(), false))
            .with_child(Self::create_cell(&self.amount, false))
            .with_child(Self::create_cell(&self.sum.to_string(), false))
            .with_child(Self::create_cell(&self.kbm.to_string(), false))
            .with_child(Self::create_cell(&self.total.to_string(), true))
        .to_html_string()
    }

    fn create_cell(value: &str, align_right: bool) -> HtmlChild {
        let mut html = HtmlElement::new(HtmlTag::TableCell).with_child(value.into());
        if align_right {
            html = html.with_attribute("class" ,"text-right")
        }
        html.into()
    }

    pub fn get_sum(&self) -> &Cost {
        &self.sum
    }

    pub fn get_kbm(&self) -> &Cost {
        &self.kbm
    }

    pub fn get_total(&self) -> &Cost {
        &self.total
    }
}

pub struct Cost {
    value: f32
}

impl Cost {
    pub fn new(value: f32) -> Self {
        Self{value}
    }

    pub fn to_string(&self) -> String {
        //todo proper formatting
        format!("{:.2} €", self.value).replace(".", ",")
    }

    pub fn add(mut self, other: &Self) -> Self {
        self.value += other.value;
        self
    }
}