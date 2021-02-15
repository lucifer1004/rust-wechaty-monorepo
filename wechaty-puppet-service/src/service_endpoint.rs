use serde::Deserialize;

use crate::error::EndpointError;

#[derive(Debug, Deserialize)]
struct Endpoint {
    ip: String,
    port: usize,
}

const WECHATY_ENDPOINT_RESOLUTION_SERVICE_URI: &str = "https://api.chatie.io/v0/hosties/";

pub async fn discover(token: &str) -> Result<Endpoint, EndpointError> {
    match reqwest::get(&format!("{}{}", WECHATY_ENDPOINT_RESOLUTION_SERVICE_URI, token)).await {
        Ok(res) => match res.json::<Endpoint>().await {
            Ok(endpoint) => {
                if endpoint.port == 0 {
                    Err(EndpointError::InvalidToken)
                } else {
                    Ok(endpoint)
                }
            }
            Err(err) => Err(EndpointError::from(err)),
        },
        Err(err) => Err(EndpointError::from(err)),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[actix_rt::test]
    async fn it_works() {
        println!("{:?}", discover("123").await);
    }
}
