extern crate libl;

#[cfg(test)]
mod tests {
    use libl::curl::curl;
    use std::path::Path;

    #[test]
    fn test_os() {
        assert_eq!(1, 1);
    }

    #[test]
    fn curl_test() {
        curl(String::from("https://smoke-installer.github.io/2021_5_10_4_0.zip"));
        // assert_eq!(github_path, true)
    }
}
