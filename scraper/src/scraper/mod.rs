use std::error::Error;
use std::ops::{Deref};
use std::rc::Rc;
use std::sync::Arc;
// use std::thread;
use std::time::Duration;
use std::sync::Mutex;
use serde_json::Value;
use serde_json::Value::Array;
use crate::config::Config;
use crate::node::IcNode;
use tokio::sync::mpsc::{channel, Sender, UnboundedReceiver};
use tokio::time::sleep;

pub async fn send_req(url : &String, node : &IcNode) -> Result<Vec<String>, Box<dyn Error>> {
    let response = reqwest::get(url).await.unwrap();
    let resp = response.text().await?;
    let v : Value = serde_json::from_str(&resp)?;
    let mut scraped_logs : Vec<String> = Vec::new();
    match v{
        Array(array) => {
            for log in array{
                let mut annotated_log = log.clone();
                annotated_log["dc"] = Value::String(String::from((&node.ic_node).deref()));
                annotated_log["ipv6"] =  Value::String(String::from((&node.ipv6).deref()));
                annotated_log["ic_subnet"] =  Value::String(String::from((&node.ic_subnet).deref()));
                scraped_logs.push(serde_json::to_string(&annotated_log).unwrap());
            }
        },
        _ => ()
    };
    Ok(scraped_logs)
}
pub async fn scraping(config : &Config, ic_nodes : &Arc<Mutex<Vec<IcNode>>>, rx : &Rc<&str>) -> Result<(), Box<dyn Error>> {
    // for r in rx.deref(){
    //     print!("{:?}",r);
    //
    // }
        print!("{:?}",rx.deref());

    loop{
        let mut scraped_logs : Vec<String> = Vec::new();
        for node in ic_nodes.lock().unwrap().iter(){

            let url = get_url(&config, &node.ipv6, &node.port);
            println!("{:?}", url);
            let new_logs = send_req(&url, &node);
            println!("{:?}", scraped_logs);
            scraped_logs.extend(new_logs.await.unwrap());

            // let response = reqwest::get("http://localhost:5000/logs").await.unwrap();
            // let resp = response.text().await?;
            // let v : Value = serde_json::from_str(&resp)?;
            // match v{
            //     Array(array) => {
            //         for log in array{
            //             let mut annotated_log = log.clone();
            //             annotated_log["dc"] = Value::String(String::from((&node.ic_node).deref()));
            //             annotated_log["ipv6"] =  Value::String(String::from((&node.ipv6).deref()));
            //             annotated_log["ic_subnet"] =  Value::String(String::from((&node.ic_subnet).deref()));
            //             scraped_logs.push(serde_json::to_string(&annotated_log).unwrap());
            //         }
            //     },
            //     _ => ()
            //
            // }

        }
        println!("{:?}", scraped_logs);


        let interval = config.scrape_interval.parse::<u64>().unwrap();
        sleep(Duration::from_secs(interval)).await;
    }
}
fn get_url(config : &Config, node_ip : &String, port : &String) -> String{
    match config.use_ipv6 {
        true => return format!("http://[{}]:{}/{}", node_ip, port ,config.log_path),
        false => return format!("http://localhost:{}/{}/{}", port, config.log_path, port)
    }

}