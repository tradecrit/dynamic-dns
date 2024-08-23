use reqwest::RequestBuilder;

pub(crate) async fn request(built_request: RequestBuilder) -> Result<reqwest::Response, reqwest::Error> {
    let mut backoff = 1;

    loop {
        let request_clone = built_request.try_clone().expect("Failed to clone request");
        let response = request_clone.send().await;

        match response {
            Ok(response) => {
                return Ok(response);
            },
            Err(error) => {
                tracing::error!("Failed to send request {:?}", error);
                tokio::time::sleep(std::time::Duration::from_secs(backoff)).await;
                backoff *= 2;
            }
        }
    }
}