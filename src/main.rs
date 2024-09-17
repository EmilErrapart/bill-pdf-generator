mod settings;
mod input;

use settings::Settings;
use input::Input;
use tera::Tera;
use wkhtmltopdf::{PdfApplication, Size};

fn main() {
    let filename = "MR0001".to_string();

    let settings = Settings::load();

    let mut context = tera::Context::new();
    context.insert("bill_no", &filename);
    context.insert("kbm", &settings.get_kbm());

    let company = settings.get_company();
    context.insert("company_name", company.get_name());

    let client = settings.get_client();
    context.insert("client_name", client.get_name());
    context.insert("client_address", client.get_address());
    context.insert("client_registry_no", client.get_registry_no());
    context.insert("client_city", client.get_city());
    context.insert("client_post_index", client.get_post_index());

    let html = Tera::one_off(include_str!("templates/template.html"), &context, true).expect("Failed to render template");

    let pdf_app = PdfApplication::new().expect("Failed to init PDF application");
    let mut pdfout = pdf_app.builder()
        .margin(Size::Millimeters(0))
        .build_from_html(&html)
        .expect("Failed to build pdf");
    pdfout.save(filename + ".pdf").expect("Failed to save pdf");
}