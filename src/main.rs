use anyhow::{anyhow, Result};
use reqwest::{blocking::Response, StatusCode};
use serde::Deserialize;

struct Turso {
    token: String,
    org: String,
}

#[derive(Debug, Clone, Deserialize)]
struct Usage {
    pub rows_read: u64,
    pub rows_written: u64,
    pub databases: u16,
    pub locations: u8,
    pub storage_bytes: u64,
    pub groups: u8,
    pub bytes_synced: u64,
}

#[derive(Debug, Clone, Deserialize)]
struct Organisation {
    pub usage: Usage,
}

#[derive(Debug, Clone, Deserialize)]
struct TursoUsage {
    pub organization: Organisation,
}

impl Turso {
    pub fn new() -> Self {
        let token = std::env::var("TURSO_API_TOKEN").expect("Missing TURSO_API_TOKEN");
        let org = "arinono".to_string();

        Self { token, org }
    }

    pub fn get_usage(&self) -> Result<TursoUsage> {
        let url = format!("https://api.turso.tech/v1/organizations/{}/usage", self.org);
        let client = reqwest::blocking::Client::new();
        let res = client
            .get(url)
            .bearer_auth(&self.token)
            .send()?
            .json::<TursoUsage>()?;

        Ok(res)
    }
}

struct Influx {
    token: String,
    url: String,
}

type InfluxPoint = String;

impl From<TursoUsage> for InfluxPoint {
    fn from(value: TursoUsage) -> Self {
        let now = std::time::SystemTime::now();
        let ts = now
            .duration_since(std::time::UNIX_EPOCH)
            .expect("Failed to get current timestamp ms")
            .as_millis();

        format!("usage rows_read={},rows_written={},databases={},locations={},storage_bytes={},groups={},bytes_synced={} {}",
            value.organization.usage.rows_read,
            value.organization.usage.rows_written,
            value.organization.usage.databases,
            value.organization.usage.locations,
            value.organization.usage.storage_bytes,
            value.organization.usage.groups,
            value.organization.usage.bytes_synced,
            ts,
        ).to_string()
    }
}

impl Influx {
    pub fn new() -> Self {
        let token = std::env::var("INFLUX_DB_TOKEN").expect("Missing INFLUX_DB_TOKEN");
        let url = std::env::var("INFLUX_DB_URL").expect("Missing INFLUX_DB_URL");

        Self { token, url }
    }

    pub fn send(&self, usage: TursoUsage) -> Result<()> {
        let body: InfluxPoint = usage.into();

        let client = reqwest::blocking::Client::new();
        let res = client
            .post(&self.url)
            .header("Authorization", format!("Token {}", &self.token))
            .header("Content-Type", "text/plain; charset=utf-8")
            .header("Accept", "application/json")
            .body(body)
            .send();

        match res {
            Err(_) => {
                println!("Failed to send");
                return Err(anyhow!("Failed to send"));
            }
            Ok(ok) => {
                if ok.status() != StatusCode::NO_CONTENT {
                    println!("Failed to send");
                    return Err(anyhow!("Failed to send"));
                }
            }
        }

        Ok(())
    }
}

fn main() -> Result<()> {
    dotenvy::dotenv().ok();

    let every = std::env::var("EVERY_SEC").unwrap_or("300".to_string());
    let every = every.parse::<u64>().unwrap_or(300);

    let turso = Turso::new();
    let influx = Influx::new();

    println!("Started");

    loop {
        println!("Sending");
        match turso.get_usage() {
            Ok(usage) => {
                let _ = influx.send(usage);
            }
            Err(_) => println!("Failed to get usage"),
        };

        std::thread::sleep(std::time::Duration::from_secs(every));
    }
}
