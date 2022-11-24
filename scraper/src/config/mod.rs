use std::env;

pub struct Config{
    pub config_path: String,
    pub journal_port: String,
    pub scrape_interval: String,
    pub loki_url: String,
    pub use_ipv6: bool,
    pub log_path: String
}

pub fn new_config() -> Config {
    let config_path = env::var("CONFIG_PATH").unwrap_or(String::from("./config.json"));
    let journal_port = env::var("JOURNAL_PORT").unwrap_or(String::from("8000"));
    let scrape_interval = env::var("INTERVAL").unwrap_or(String::from("10"));
    let loki_url = env::var("LOKI_URL").unwrap_or(String::from("http://localhost"));
    let use_ipv6 = env::var("IPV6").unwrap_or(String::from("false"))=="true";
    let log_path = env::var("LOG_PATH").unwrap_or(String::from("get-logs"));

    Config{
        config_path,
        journal_port,
        scrape_interval,
        loki_url,
        use_ipv6,
        log_path
    }
}
