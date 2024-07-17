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

    fn connect_redis_cloud() -> RedisResult<Connection> {
        let endpoint = "redis-11516.c295.ap-southeast-1-1.ec2.redns.redis-cloud.com";
        let mut client = create_client(endpoint, Some("jiQ2LPgrn9ZIKQjmqm1o6DbF1bSHKO5w"), Some(11516));
        assert!(client.check_connection());
        return client.get_connection()
    }
    fn fetch_an_integer(key:&str, value:isize) -> redis::RedisResult<isize> {
        // connect to redis

        let mut con = connect_redis_cloud()?;
        // throw away the result, just make sure it does not fail
        let _: () = con.set(key, value)?;
        // read back the key and return it.
        //  Because the return value from the function is a result for integer this will automatically convert into one.
        con.get(key)
    }

    #[test]
    fn redis_rs_example() {
        let r = fetch_an_integer("my_key", 42);
        assert_eq!(42,r.unwrap());

    }
}
