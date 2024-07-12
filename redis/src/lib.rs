pub fn create_client(url: String) -> redis::Client {
    return redis::Client::open(url).unwrap();
}

#[cfg(test)]
mod tests {
    use super::*;


    fn create_client2(url: &str) -> redis::Client {
        let client = redis::Client::open(url).unwrap();
        client
    }

    #[test]
    fn connectRedisCloud() {
        ;// TODO
    }
}
