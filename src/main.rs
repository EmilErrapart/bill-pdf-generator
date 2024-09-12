use lopdf::dictionary;
use lopdf::{Document, Object, Stream};
use lopdf::content::{Content, Operation};

fn main() {
    let mut doc = Document::with_version("1.7");

    let font_id = doc.add_object(dictionary! {
        "Type" => "Font",
        "Subtype" => "Type1",
        "BaseFont" => "DejaVuSansCondensed",
    });

    let font_bold_id = doc.add_object(dictionary! {
        "Type" => "Font",
        "Subtype" => "Type1",
        "BaseFont" => "DejaVuSansCondensed-Bold",
    });

    let resources_id = doc.add_object(dictionary! {
        "ProcSet" => vec!["PDF".into(), "Text".into(), "ImageB".into(), "ImageC".into(), "ImageI".into()],
        "Font" => dictionary! {
            "F1" => font_id,
            "F2" => font_bold_id,
        },
        "ExtGState" => dictionary! {
            "GS1" => dictionary! {
                "Type" => "ExtGState",
                "BM" => "Normal",
                "ca" => 1,
                "CA" => 1,
            },
        },
    });

    let content = Content {
        operations: vec![
            Operation::new("BT", vec![]),
            Operation::new("Tf", vec!["F1".into(), 48.into()]),
            Operation::new("Td", vec![100.into(), 600.into()]),
            Operation::new("Tj", vec![Object::string_literal("Hello World!")]),
            Operation::new("ET", vec![]),
        ],
    };

    let content_id = doc.add_object(Stream::new(dictionary! {}, content.encode().unwrap()));

    let pages_id = doc.new_object_id();

    let page_id = doc.add_object(dictionary! {
        "Type" => "Page",
        "Parent" => pages_id,
        "MediaBox" => vec![0.into(), 0.into(), 595.28.into(), 842.89.into()],
        "TrimBox" => vec![0.into(), 0.into(), 595.28.into(), 842.89.into()],
        "Resources" => resources_id,
        "Group" => dictionary! {
            "Type" => "Group",
            "S" => "Transparency",
            "CS" => "DeviceRGB",
        },
        "Contents" => content_id,
    });

    let pages = dictionary! {
        "Type" => "Pages",
        "Kids" => vec![page_id.into()],
        "Count" => 1,
        "MediaBox" => vec![0.into(), 0.into(), 595.28.into(), 842.89.into()],
    };

    doc.objects.insert(pages_id, Object::Dictionary(pages));

    let catalog_id = doc.add_object(dictionary! {
        "Type" => "Catalog",
        "Pages" => pages_id,
        "OpenAction" => vec![page_id.into(), "XYZ".into(), Object::Null, Object::Null, 1.into()],
        "PageLayout" => "OneColumn",
    });

    doc.trailer.set("Root", catalog_id);
    doc.compress();
    doc.save("example.pdf").unwrap();
}
