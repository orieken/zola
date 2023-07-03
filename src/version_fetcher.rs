use reqwest::header;
use serde_json::Value;

pub async fn get_version(url_base: &str, auth_token: Option<&str>, package_name: &str) -> Result<(String, String, String, String), Box<dyn std::error::Error>> {
    let url = format!("{}/{}", url_base, package_name);

    let client = reqwest::Client::new();
    let mut request_builder = client.get(&url);

    if let Some(token) = auth_token {
        request_builder = request_builder.header(header::AUTHORIZATION, format!("Bearer {}", token));
    }

    let res: Value = request_builder
        .send()
        .await?
        .json()
        .await?;

    let latest = res["dist-tags"]["latest"]
        .as_str()
        .unwrap_or("Not Found")
        .to_string();
    let next = res["dist-tags"]["next"]
        .as_str()
        .unwrap_or("Not Found")
        .to_string();
    let beta = res["dist-tags"]["beta"]
        .as_str()
        .unwrap_or("Not Found")
        .to_string();
    let rc = res["dist-tags"]["rc"]
        .as_str()
        .unwrap_or("Not Found")
        .to_string();

    Ok((latest, next, beta, rc))
}

#[cfg(test)]
mod tests {
    use super::*;
    use mockito;
    use serde_json::json;
    use tokio;

    #[tokio::test]
    async fn test_get_version() {

        let mut server = mockito::Server::new();
        let url = server.url();
        let mock = server.mock("GET", "/mypackage")
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(json!({
                "dist-tags": {
                    "latest": "1.0.0",
                    "next": "1.1.0",
                    "beta": "1.0.1-beta",
                    "rc": "1.1.0-rc"
                }
            }).to_string())
            .create();

        let (latest, next, beta, rc) = get_version(&url, None, "mypackage")
            .await
            .expect("get_version failed");

        mock.assert();
        assert_eq!(latest, "1.0.0");
        assert_eq!(next, "1.1.0");
        assert_eq!(beta, "1.0.1-beta");
        assert_eq!(rc, "1.1.0-rc");
    }

    #[tokio::test]
    async fn test_get_version_with_auth() {

        let mut server = mockito::Server::new();
        let url = server.url();
        let mock = server.mock("GET", "/mypackage")
            .with_status(200)
            .with_header("content-type", "application/json")
            .match_header("Authorization", "Bearer my_token")
            .with_body(json!({
                "dist-tags": {
                    "latest": "1.0.0",
                    "next": "1.1.0",
                    "beta": "1.0.1-beta",
                    "rc": "1.1.0-rc"
                }
            }).to_string())
            .create();

        let (latest, next, beta, rc) = get_version(&url, Some("my_token"), "mypackage")
            .await
            .expect("get_version failed");

        mock.assert();
        assert_eq!(latest, "1.0.0");
        assert_eq!(next, "1.1.0");
        assert_eq!(beta, "1.0.1-beta");
        assert_eq!(rc, "1.1.0-rc");
    }

    #[tokio::test]
    async fn test_get_version_http_failure() {
        let mut server = mockito::Server::new();
        let url = server.url();
        let mock = server.mock("GET", "/mypackage")
            .with_status(500)
            .create();

        match get_version(&url, None, "mypackage").await {
            Ok(_) => panic!("get_version should have returned an error"),
            Err(_) => {}
        }

        mock.assert();
    }
}
