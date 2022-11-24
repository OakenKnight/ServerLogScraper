use log::{info, error};
use std::{
    error::Error,
    fs
};
use std::sync::{Arc, Mutex};
use std::sync::mpsc::{channel};
use std::time::Duration;
use notify::{RecommendedWatcher, RecursiveMode, Watcher};
use notify::event::DataChange;
use notify::event::ModifyKind::Data;
use notify::EventKind::Modify;


use crate::{config::Config, node::IcNode};

pub fn read_config(config : &Config) -> Result<Vec<IcNode>, Box<dyn Error>>{
    info!("Reading config at {}", config.config_path);
    let config_file = match fs::read_to_string(&config.config_path){
        Ok(config_file) => config_file,
        Err(e) => {
            error!("Error reading config file {}", e);
            error!("Current file standing at {}", config.config_path);
            std::process::exit(1);
        }
    };

    let ic_nodes : Vec<IcNode> = match serde_json::from_str(&config_file){
        Ok(ic_nodes) => ic_nodes,
        Err(e) => {
            error!("Error {}",e);

            return Err(Box::new(e));
        }
    };

    Ok(ic_nodes)

}

pub fn watch_file(config: Arc<Config>, ic_nodes : Arc<Mutex<Vec<IcNode>>>) {

    let (tx, rx) = channel();

    //kreirati watcher

    let mut watcher = match RecommendedWatcher::new(tx,
                                                    notify::Config::default()
                                                        .with_poll_interval(Duration::from_secs(1))) {
        Ok(watcher) => watcher,
        Err(e) => {
            error!("Error creating watcher: {}", e);
            std::process::exit(1);
        }
    };


    //subscribeovati watcher
    match watcher.watch(&config.config_path.as_ref(), RecursiveMode::Recursive) {
        Ok(_)=>{},
        Err(_e)=>{ error!("There has been an error watching file"); }
    }


    // u beskonacno vrteti primajuci kanal i ako se menjaju podaci u config fajlu ucitati ih opet
    // zato je otkljucan ic_nodes
    loop{
        match rx.recv() {
            Ok(event) => {
                if event.unwrap().kind == Modify(Data(DataChange::Any)) {
                    info!("Reloading config");
                    let mut ref_to_ic_nodes = match ic_nodes.lock() {
                        Ok(nodes) => nodes,
                        Err(e) => {error!("There has been an error getting locked nodes, {:?}", e); continue;}
                    };
                    match read_config(&config){
                        Ok(nodes) => {
                            *ref_to_ic_nodes = nodes;
                            println!("new nodes {:?}", ref_to_ic_nodes);
                        },Err(_e) => {
                            error!("There has been an error reading config an reloading ic nodes");
                        }
                    }
                }
            },
                Err(e) => println!("watch error: {:?}", e)
            }
        }
    }


