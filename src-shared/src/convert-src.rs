use urlencoding::encode;

pub fn convert_src(src: &str) -> String {
    if src.starts_with("asset://localhost/") {
        src.to_string()
    } else {
        let url = encode(src);

        format!("asset://localhost/{}", url)
    }
}
