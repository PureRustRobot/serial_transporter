use zenoh::{
    config::Config,
    prelude::r#async::*,
    Error
};

use std::{time::Duration, io::Write};

use serialport;
use prr_utils::logger;

use prr_msgs::msg::*;

pub async fn serial_transporter(
    node_name:&str, 
    sub_topic:&str,
    port_name:&str,
    baud_rate:u32,
    enable_debug:bool
)->Result<(), Error>
{
    let session = zenoh::open(Config::default()).res().await.unwrap();

    let subscriber = session.declare_subscriber(sub_topic).res().await.unwrap();

    logger::log_info(node_name, "Open serial port...".to_string());
    let mut serialport = serialport::new(port_name, baud_rate)
        .timeout(Duration::from_millis(100))
        .open()?;

    let msg = format!("Start sub:{}", subscriber.key_expr().to_string());
    logger::log_info(node_name, msg);

    loop {
        let sample = subscriber.recv_async().await?;

        let get_data = Wheel::deserialize(sample.value.to_string());

        let write_data = Wheel{
            front_left:get_data.front_left,
            front_right:get_data.front_right,
            rear_left:get_data.rear_left,
            rear_right:get_data.rear_right
        };

        

        match serialport.write(Wheel::serialize(&write_data).as_bytes()) {
            Ok(_)=>{
                if let Err(e) = std::io::stdout().flush(){
                    let msg = format!("Failed to flush stdout: {:?}", e);
                    logger::log_error(node_name, msg);
                }
                if enable_debug
                {
                    logger::log_info(node_name, format!("{:?}", Wheel::serialize(&write_data)));
                }
            },
            Err(e)=>logger::log_error(node_name, e.to_string()),
        }
    }
}