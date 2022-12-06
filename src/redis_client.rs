use redis::{Commands, RedisError};
use std::env;

const MASTER_ADMIN: &str = "224597366324461568";
pub fn connect() -> redis::Connection {
    dotenv::dotenv().ok();
    let redis_host_name =
        env::var("REDIS_HOSTNAME").expect("Missing environment variable REDIS_HOSTNAME");

    let redis_password = env::var("REDIS_PASSWORD").unwrap_or_default();

    let is_tls: bool = env::var("TLS")
        .expect("Missing environment variable TLS")
        .parse()
        .expect("Environment variable TLS must be true or false");

    let uri_scheme = if is_tls { "rediss" } else { "redis" };

    let redis_conn_url = format!("{}://:{}@{}", uri_scheme, redis_password, redis_host_name);
    redis::Client::open(redis_conn_url)
        .expect("Invalid connection URL")
        .get_connection()
        .expect("Failed to connect to Redis")
}

pub fn check_master_admin(conn: &mut redis::Connection) -> redis::RedisResult<()> {
    let value: Result<bool, RedisError> = conn.sismember("admins", MASTER_ADMIN);
    match value {
        Ok(master_admin_found) => {
            if !master_admin_found {
                conn.sadd("admins", MASTER_ADMIN)?;
            }
            Ok(())
        }
        Err(e) => Err(e),
    }
}

pub fn add_admin(conn: &mut redis::Connection, admin_id: String) -> redis::RedisResult<()> {
    conn.sadd("admins", admin_id)?;
    Ok(())
}

pub fn remove_admin(conn: &mut redis::Connection, admin_id: String) -> redis::RedisResult<()> {
    conn.srem("admins", admin_id)?;
    Ok(())
}

pub fn check_admin(conn: &mut redis::Connection, admin_id: String) -> Result<bool, RedisError> {
    let value: Result<bool, RedisError> = conn.sismember("admins", admin_id);
    value
}

pub fn get_bot_role(conn: &mut redis::Connection) -> Result<Option<String>, RedisError> {
    let value: Option<String> = conn.get("bot admin role")?;
    Ok(value)
}

pub fn set_bot_role(conn: &mut redis::Connection, role_id: String) -> redis::RedisResult<()> {
    conn.set("bot admin role", role_id)?;
    Ok(())
}

pub fn get_follower_role(conn: &mut redis::Connection) -> Result<Option<String>, RedisError> {
    let value: Option<String> = conn.get("follower role")?;
    Ok(value)
}

pub fn set_guild_id(conn: &mut redis::Connection, guild_id: String) -> redis::RedisResult<()> {
    let _: () = conn.set("guild id", guild_id)?;
    Ok(())
}

pub fn get_guild_id(conn: &mut redis::Connection) -> Result<Option<String>, RedisError> {
    let value: Option<String> = conn.get("guild id")?;
    Ok(value)
}

pub fn set_follower_role(conn: &mut redis::Connection, role_id: String) -> redis::RedisResult<()> {
    let _: () = conn.set("follower role", role_id)?;
    Ok(())
}

pub fn get_major_log_channel(conn: &mut redis::Connection) -> Result<Option<String>, RedisError> {
    let value: Option<String> = conn.get("major log channel")?;
    Ok(value)
}

pub fn set_major_log_channel(
    conn: &mut redis::Connection,
    channel_id: String,
) -> redis::RedisResult<()> {
    let _: () = conn.set("major log channel", channel_id)?;
    Ok(())
}

pub fn get_minor_log_channel(conn: &mut redis::Connection) -> Result<Option<String>, RedisError> {
    let value: Option<String> = conn.get("minor log channel")?;
    Ok(value)
}

pub fn set_minor_log_channel(
    conn: &mut redis::Connection,
    channel_id: String,
) -> redis::RedisResult<()> {
    let _: () = conn.set("minor log channel", channel_id)?;
    Ok(())
}
