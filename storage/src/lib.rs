use minio_rsc::client::{BucketArgs, KeyArgs};
use minio_rsc::provider::StaticProvider;
use minio_rsc::Minio;
use std::error::Error;


pub struct MinIoClient {
    pub ssh_only:   bool,
    pub access_key: String,
    pub secret_key: String,
    pub end_point:  String,
    pub agent: Option<String>
}

impl MinIoClient {
    pub fn create_minio(&self) -> Result<Minio, Box<dyn Error>> {
        let provider = StaticProvider::new(&self.access_key, &self.secret_key, None);
        let mut minio_builder = Minio::builder()           
            .endpoint(&self.end_point).provider(provider)
            .secure(self.ssh_only);
        if let Some(agent) = self.agent.as_ref() {
            minio_builder = minio_builder.agent(agent);
        }
        let minio = minio_builder.build();
        Ok(minio?)
    }
}

pub struct StoreObject {
    deletable: bool,
    name: String,
}

impl Default for StoreObject {
    fn default() -> Self {
        StoreObject {deletable: true, name: "my_bucket".to_string() }
    }
}


pub async fn make_storage(minio: Minio, s: StoreObject) -> Result<String, Box<dyn Error>> {
    let rslt = 
        minio.make_bucket(s.name, s.deletable).await;
    Ok(rslt?)
} 


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_create_minio_client() -> Result<(), Box<dyn Error>> {
        let client = MinIoClient {
            ssh_only: false, access_key: "".to_string(), secret_key: "".to_string(),
            end_point: "localhost:9002".to_string(), agent: None
        };
        let minio = client.create_minio();
        assert!(minio.is_ok(), " minio object created ");
        Ok(())
    }
}
