use scylla::{Session, SessionBuilder};

mod error;

pub use error::ScyllaError as ScyllaError;

pub async fn scylla_session(nodes: Vec<String>) -> Result<Session, ScyllaError> {
    let mut sess_build = SessionBuilder::new();
        // .known_node("127.0.0.1:9042")
        // .known_node("1.2.3.4:9876")
        // .build()
        // .await.map_err(ScyllaError::NewSessionErr)?;
    for n in nodes.iter() {
        sess_build = sess_build.known_node(&n);
    }
    let session: Session = sess_build.build().await.map_err(ScyllaError::NewSessionErr)?;
    Ok(session)
}


#[cfg(test)]
mod tests {
    use super::*;

}
