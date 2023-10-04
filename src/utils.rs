use reqwest::Url;

pub fn valid_domain(url: &str) -> bool {
    // check if there is a scheme, if not append http://
    match Url::parse(url) {
        Ok(_) => true,
        Err(_) => Url::parse(&format!("http://{}", url)).is_ok(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_valid_domain() {
        assert!(valid_domain("https://google.com"));
        assert!(valid_domain("http://google.com"));
        assert!(valid_domain("google.com"));
        assert!(valid_domain("google"));
    }
}
