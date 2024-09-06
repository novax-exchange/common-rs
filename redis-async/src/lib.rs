use std::collections::HashMap;
use redis::streams::{StreamClaimOptions,StreamClaimReply};
use redis::{Connection, Cmd, Commands,RedisResult, Pipeline};
use redis::Value;
use redis::aio::ConnectionManager;

type RedisErr = redis::RedisError;

pub struct RedisConn {
    port:   i16,
    user:   String,
    pwd:    String,
    host:   String
}

pub async fn redis(cn: RedisConn) -> Result<ConnectionManager, RedisErr> {
    let redis_client = redis::Client::open(
        &format!("redis://{}:{}@{}:{}/", 
            cn.user, cn.pwd,
            cn.host, cn.port)[..]
    )?;
    let cn_mgr = ConnectionManager::new(redis_client).await?;
    Ok(cn_mgr)
}


#[cfg(test)]
mod tests {
    use super::*;

   
}
