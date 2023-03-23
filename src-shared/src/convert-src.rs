use urlencoding::encode;

#[cfg(feature = "win")]
pub fn convert_src(src: &str) -> String {
    if src.starts_with("https://asset.localhost/") {
        src.to_string()
    } else {
        let url = encode(src);

        format!("https://asset.localhost/{}", url)
    }
}

#[cfg(not(feature = "win"))]
pub fn convert_src(src: &str) -> String {
    if src.starts_with("asset://localhost/") {
        src.to_string()
    } else {
        let url = encode(src);

        format!("asset://localhost/{}", url)
    }
}
