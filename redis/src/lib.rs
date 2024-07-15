use redis::*;
pub fn create_client(host: &str, password: Option<&str>, port: Option<i32>) -> Client {
    let _port = port.unwrap_or(6379);
    let redis_url;
    if password.is_none() {
        redis_url = format!("redis://{}:{}", host, _port);
    } else {
        redis_url = format!("redis://:{}@{}:{}", password.unwrap(), host, _port);
    };


    return Client::open(redis_url).unwrap();
}

#[cfg(test)]
mod tests {
    use super::*;


    #[test]
    fn connect_redis_cloud() {
        let endpoint = "redis-11516.c295.ap-southeast-1-1.ec2.redns.redis-cloud.com";
        let mut client = create_client(endpoint, Some("jiQ2LPgrn9ZIKQjmqm1o6DbF1bSHKO5w"), Some(11516));
        assert!(client.check_connection());
    }
}
