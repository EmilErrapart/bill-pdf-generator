mod settings;

use settings::Settings;
use tera::Tera;
use wkhtmltopdf::{PdfApplication, Size};

fn main() {
    let filename = "MR0001";

    let settings = Settings::load();

    let mut context = tera::Context::new();
    context.insert("company_name", settings.get_company().get_name());
    context.insert("client_name", settings.get_client().get_name());
    context.insert("bill_no", filename);

    let html = Tera::one_off(include_str!("templates/template.html"), &context, true).expect("Failed to render template");

    let pdf_app = PdfApplication::new().expect("Failed to init PDF application");
    let mut pdfout = pdf_app.builder()
        .margin(Size::Millimeters(0))
        .build_from_html(&html)
        .expect("Failed to build pdf");
    pdfout.save(filename).expect("Failed to save pdf");
}