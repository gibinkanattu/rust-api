#[warn(dead_code)]
#[allow(unused_imports)]
use std::env;
use serde_json::Value;
use std::fs::File;


pub fn read_config()-> serde_json::Value
{
    let file = match File::open("conf/conf.json"){
        Ok(file) => file,
        Err(e) => {
            println!("Error while reading config file:  {:?}", e);
            std::process::exit(0);
        }
    };

    // Reading the config file path from the environment variable
    // let args: Vec<String> = env::args().collect();
    // let file = File::open(&args[1]).expect("config file not found!!");
    // let file = File::open("server_config.json").expect("config file not found!!");
    let config: Value = serde_json::from_reader(file).expect("error while reading config file!!");
    // print!("{:?}",config);
    return config
}