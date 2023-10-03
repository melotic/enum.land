use reqwest::Url;

pub fn valid_domain(url: &str) -> bool {
    // check if there is a scheme, if not append http://
    match Url::parse(url) {
        Ok(_) => return true,
        Err(_) => match Url::parse(&format!("http://{}", url)) {
            Ok(_) => return true,
            Err(_) => return false,
        },
    }
}
