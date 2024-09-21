use reqwest::{Client, header, tls::Identity};
use std::error::Error;
use std::fs;
use std::io::Result as IoResult;

pub struct ClientConfig {
    pub ca_crt:         Option<String>,
    pub client_pem:     Option<String>,
    pub authorisatin:   Option<String>
}

fn file_buf(f: &'_ str) -> IoResult<Vec::<u8>> {
    let buf = fs::read(f)?;
    Ok(buf)
}


pub async fn http_client(cfg: ClientConfig) -> Result<Client, Box::<dyn Error>> {
    let mut client_builder = if let (Some(ca_crt), Some(client_pem)) = (cfg.ca_crt, cfg.client_pem) {
        let ca_buf = file_buf(&ca_crt)?;
        let client_buf = file_buf(&client_pem)?;
        let cert = reqwest::Certificate::from_pem(&ca_buf)?;
        let identity = Identity::from_pem(&client_buf)?;
        let client = reqwest::Client::builder().use_rustls_tls();
        client
            .tls_built_in_root_certs(false)
            .add_root_certificate(cert)
            .identity(identity)
            .https_only(true)
    } else {
        Client::builder()
    };
    if let Some(auth) = cfg.authorisatin {
        let mut headers = header::HeaderMap::new();
        headers.insert(header::AUTHORIZATION, header::HeaderValue::from_str(&auth)?);
        client_builder = client_builder.default_headers(headers);
    }
    Ok( client_builder.build()? )
}


#[cfg(test)]
mod tests {
    use novax_tokio::tokio as tokio;

    #[tokio::test]
    async fn should_connect_mtls_service() -> Result<(), Box<dyn super::Error>> {
        let r_client = super::http_client(
            super::ClientConfig {
                ca_crt: Some("key/ca.crt".to_string()),
                client_pem: Some("key/client.pem".to_string()), 
                authorisatin: None
            }
        ).await?;
        let resp = r_client.get("https://localhost:9092/").send().await?;
        println!("{:?}.... ", resp);
        Ok(())
    }
 
}
