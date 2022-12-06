use std::{
    thread::sleep,
    time::Duration
}; // Since they needed in one place so we will put them together, its not cool but better nothing
mod collector;
mod exporter;
use clap::Parser;

#[derive(Parser)]
struct Config{
    container_id : u32,
    user_name : String,
    server_id : u16,
    #[clap(default_value_t=60,short, long)]
    timeout : u64,
    #[clap(default_value="eth0",short, long)]
    device_name : String,
    ch_host : String,
    ch_schema: String,
    ch_user : String,
    #[clap(default_value="",short, long)]
    ch_password : String,
    ch_table: String
}

//use std::env::{self, args};
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    
    let args = Config::parse();
    let ch_exporter = exporter::Exporter::new(args.ch_host,args.ch_user,args.ch_password, args.ch_schema);
    //cargo run -- container_id, 
    let container_id = args.container_id;
    let user_name = &args.user_name;
    let server_id = args.server_id;
    let timeout = Duration::from_secs(args.timeout);
    let device_name = args.device_name;
    let stat = collector::Collector::new(device_name.clone());
    println!(
        "Starting parsing data : \r\nContainerID :\t{}\r\nUserName:\t{}\r\nServerID:\t{}\r\nDevice:\t{}\r\nWith timeout:\t{} secs\r\n",
        container_id, user_name, server_id, &device_name, timeout.as_secs()
    );
    loop{
        if let (Some(tx), Some(rx)) = (stat.get_tx(), stat.get_rx()){
            match ch_exporter.insert(tx, rx, container_id, user_name.to_string(), server_id).await{
                Ok(_) => {}
                Err(e) => {
                    println!("Unable to export data : {}" ,e.to_string());
                }
            }
        }else{
            println!("Unabled to parse stats");
        
        }
        sleep(timeout);
    }
    
    //Ok(())
}

