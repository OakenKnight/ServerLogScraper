#[macro_use] extern crate rocket;
use std::thread;
use std::time::{Duration, SystemTime, UNIX_EPOCH};
use once_cell::sync::Lazy;

use lipsum::lipsum;
use rand::Rng;
use rocket::serde::json::{Json};
use rocket::serde::{Serialize, Deserialize};
use rocket_prometheus::{
    prometheus::{opts, IntCounterVec},
    PrometheusMetrics,
};


#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
enum LogSeverity {
    Emergency,
    Alert,
    Critical,
    Error,
    Warning,
    Notice,
    Informational,
    Debug
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(crate = "rocket::serde")]
struct LogFromServer {
    date : u128,
    text : String,
    severity : LogSeverity,

}

fn match_rand_to_severity(rand : &i32) -> LogSeverity {
    match rand {
        0 => LogSeverity::Debug,
        1 => LogSeverity::Informational,
        2 => LogSeverity::Notice,
        3 => LogSeverity::Warning,
        4 => LogSeverity::Error,
        5 => LogSeverity::Critical,
        6 => LogSeverity::Alert,
        _ => LogSeverity::Emergency,
    }
}

#[get("/ping")]
fn health_check() -> &'static str {
    "pong\n"
}

#[get("/get-logs/<port>")]
fn get_logs(port : &str) -> Json<Vec<LogFromServer>>  {

    REQ_COUNTER.with_label_values(&[port]);

    let random_sleep :u64 = rand::thread_rng().gen_range(0..3);

    thread::sleep(Duration::from_secs(random_sleep));
    let sev :i32 = rand::thread_rng().gen_range(0..7);
    let log_sev = match_rand_to_severity(&sev);
    let mut list_logs : Vec<LogFromServer> = Vec::new();

    let rand_len_of_list = rand::thread_rng().gen_range(5..10);

    for i in 0..rand_len_of_list as usize {
        let new_sev :i32 = rand::thread_rng().gen_range(0..7);
        let new_log_sev = match_rand_to_severity(&new_sev);
        let time = SystemTime::now();
        let since_the_epoch = time
            .duration_since(UNIX_EPOCH)
            .expect("Time went backwards");

        let new_log = LogFromServer{
            date : since_the_epoch.as_millis() + new_sev as u128,
            text : lipsum((new_sev + 1 )as usize),
            severity :new_log_sev
        };
        list_logs.push(new_log);
    }
    Json(list_logs)
}

static REQ_COUNTER: Lazy<IntCounterVec> = Lazy::new(|| {
    IntCounterVec::new(opts!("server", "Server that has received"), &["port"])
        .expect("Could not create REQ_COUNTER")
});

#[launch]
fn rocket() -> _ {

    let prometheus = PrometheusMetrics::new();
    prometheus
        .registry()
        .register(Box::new(REQ_COUNTER.clone()))
        .unwrap();

    rocket::build().attach(prometheus.clone())
        .mount("/", routes![health_check, get_logs])
        .mount("/metrics", prometheus)

}
