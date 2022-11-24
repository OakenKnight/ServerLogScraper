use std::collections::HashMap;
use std::error::Error;
use std::sync::{Arc, mpsc, Mutex};
use std::{process, thread};
use std::thread::yield_now;
use std::time::Duration;
use log::error;
use tokio::signal;
use tokio::runtime::Runtime;
use tokio::sync::mpsc::{channel, Sender, unbounded_channel};


mod node;
mod scraper;
mod config;
mod config_watcher;


#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {


    env_logger::init();

    let global_config = Arc::new(config::new_config());
    let global_config_clone = Arc::clone(&global_config);
    let config = config_watcher::read_config(&global_config).unwrap();


    //
    // let config = match config_watcher::read_config(&global_config){
    //     Ok(config) => config,
    //     Err(_) => {
    //         error!("There has been an error reading config");
    //     }
    // };

    let ic_nodes = Arc::new(Mutex::new(config));
    let ic_nodes_for_watcher = Arc::clone(&ic_nodes);
    println!("{:?}", *ic_nodes.lock().unwrap());

    let mut coursor_map :HashMap<String, String> = HashMap::new();



    println!("MAIN CONFIG {:?}", *ic_nodes.lock().unwrap());


    let builder = thread::Builder::new();

    let join_handle: thread::JoinHandle<_> = builder.spawn(move || {
        config_watcher::watch_file(global_config_clone, ic_nodes_for_watcher);
    }).unwrap();
    // let thread = tokio::spawn(scraper::scraping(&global_config, &ic_nodes, send.clone()).await);

    //
    let (tx, rx) = mpsc::channel();
    let tx1 = tx.clone();
    thread::spawn(move || {
        let vals = vec![
            String::from("Cao"),
            String::from("Maune"),
            String::from("jean"),
        ];

        for val in vals{
            tx.send(val).unwrap();
            thread::sleep(Duration::from_secs(3));
        }
    });

    thread::spawn(move || {

        let vals = vec![String::from("123"), String::from("456"), String::from("789")];

        for v in vals {
            tx1.send(v).unwrap();
            thread::sleep(Duration::from_secs(2));
        }
    });

    thread::spawn(move || {
        for rec in rx{
            if rec == "STOP"{
                process:exit(1);
            }
        }
    })

    for rec in rx{
        println!("{:?}", rec);
    };

    // scraper::scraping(&global_config, &ic_nodes, &rx).await

    // tokio::spawn(async move {
    //     println!()
    //     thread::sleep(Duration::from_secs(10));
    //     tokio::signal::ctrl_c().await.unwrap();
    //     handler1.join();
    //     // process::exit(1);
    // });

    //moram dropovati sad al do ovoga ne dodje nikad jer ne ocita exit signal
    //drop(send);
    Ok(())
}
