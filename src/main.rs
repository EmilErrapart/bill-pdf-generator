use wkhtmltopdf::{PdfApplication, Size};

fn main() {
    let html = include_str!("templates/template.html");
    let pdf_app = PdfApplication::new().expect("Failed to init PDF application");
    let mut pdfout = pdf_app.builder()
        .margin(Size::Millimeters(0))
        .build_from_html(&html)
        .expect("Failed to build pdf");

    pdfout.save("arve.pdf").expect("Failed to save pdf");
}