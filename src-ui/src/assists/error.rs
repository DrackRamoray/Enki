use std::fmt::Debug;

pub fn beautify_error(err: impl Debug) -> String {
    let display = format!("{:?}", err);

    display
        .replace(r#"Binding("JsValue(\""#, "")
        .replace(r#"\")")"#, "")
}
