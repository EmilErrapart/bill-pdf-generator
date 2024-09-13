use wkhtmltopdf::{PdfApplication, Orientation, Size};

fn main() {
    let html = r#"<html><body><div>Hello World!</div></body></html>"#;
    let pdf_app = PdfApplication::new().expect("Failed to init PDF application");
    let mut pdfout = pdf_app.builder()
        .orientation(Orientation::Portrait)
        .margin(Size::Millimeters(17))
        .build_from_html(&html)
        .expect("failed to build pdf");

    pdfout.save("arve.pdf").expect("failed to save foo.pdf");
}