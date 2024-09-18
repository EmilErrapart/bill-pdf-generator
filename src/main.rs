mod settings;
mod input;

use std::path::PathBuf;
use std::env;
use std::fs::File;
use std::io::prelude::*;
use settings::Settings;
use input::Input;
use tera::Tera;
use headless_chrome::{types::PrintToPdfOptions, LaunchOptions};

fn main() {
    let filename = "MR0001".to_string();

    let settings = Settings::load();

    let mut body_context = tera::Context::new();
    body_context.insert("bill_no", &filename);
    body_context.insert("kbm", &settings.get_kbm());

    let input = Input::load(&settings);
    body_context.insert("table_values", &input.to_html());
    body_context.insert("sum_total", &input.get_sum_total().to_string());
    body_context.insert("kbm_total", &input.get_kbm_total().to_string());
    body_context.insert("grand_total", &input.get_grand_total().to_string());

    let company = settings.get_company();
    body_context.insert("company_name", company.get_name());

    let client = settings.get_client();
    body_context.insert("client_name", client.get_name());
    body_context.insert("client_address", client.get_address());
    body_context.insert("client_registry_no", client.get_registry_no());
    body_context.insert("client_city", client.get_city());
    body_context.insert("client_post_index", client.get_post_index());

    let body_html = Tera::one_off(include_str!("templates/body.html"), &body_context, false).expect("Failed to render template");
    let mut body_file = File::create("temp.html").expect("Failed to create temp body.html file");
    body_file.write_all(body_html.as_bytes()).expect("");
    let body_dir = env::current_dir().expect("Failed to get current working directory").join("temp.html");

    let mut footer_context = tera::Context::new();
    footer_context.insert("company_name", company.get_name());
    footer_context.insert("company_registry_no", company.get_registry_no());
    footer_context.insert("company_address", company.get_address());
    footer_context.insert("company_post_index", company.get_post_index());
    footer_context.insert("company_city", company.get_city());
    footer_context.insert("company_country", company.get_country());
    footer_context.insert("company_kmkr", company.get_kmkr());
    footer_context.insert("company_email", company.get_email());

    let account = settings.get_account();
    footer_context.insert("bank_name", account.get_bank_name());
    footer_context.insert("iban", account.get_iban());

    let phone = settings.get_phone();
    footer_context.insert("phone_type", phone.get_phone_type());
    footer_context.insert("phone_no", phone.get_phone_no());

    let footer_html = Tera::one_off(include_str!("templates/footer.html"), &footer_context, false).expect("Failed to render template");

    let pdf_options = PrintToPdfOptions {
        landscape: None,
        display_header_footer: Some(true),
        print_background: Some(true),
        scale: None,
        paper_width: Some(8.26772),
        paper_height: Some(11.7),
        margin_top: None,
        margin_bottom: None,
        margin_left: None,
        margin_right: None,
        page_ranges: None,
        ignore_invalid_page_ranges: None,
        header_template: None,
        footer_template: Some(footer_html),
        prefer_css_page_size: None,
        transfer_mode: None,
    };

    let launch_options = LaunchOptions::default();

    let output = PathBuf::from(filename + ".pdf");
    html2pdf::html_to_pdf(body_dir, output, pdf_options, launch_options, None).expect("Failed to print pdf");
}