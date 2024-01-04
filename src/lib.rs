use zenoh::{
    config::Config,
    prelude::r#async::*,
    Error
};

use std::{time::Duration, io::Write};

use serialport;
use zenoh_manage_utils::{param, logger};

pub async fn serial_transporter(node_name:&str, yaml_path:&str)->Result<(), Error>
{
    let session = zenoh::open(Config::default()).res().await.unwrap();

    let sub_topic = param::get_str_param(yaml_path, node_name, "sub_topic", "motor_command".to_string());
    let port_name = param::get_str_param(yaml_path, node_name, "port_name", "/dev/ttyACM0".to_string());
    let baud_rate = param::get_i64_param(yaml_path, node_name, "baud_rate", 115200) as u32;

    let subscriber = session.declare_subscriber(sub_topic).res().await.unwrap();

    logger::log_info(node_name, "Open serial port...".to_string());
    let mut serialport = serialport::new(port_name, baud_rate)
        .timeout(Duration::from_millis(100))
        .open()?;

    let msg = format!("Start sub:{}", subscriber.key_expr().to_string());
    logger::log_info(node_name, msg);

    loop {
        let sample = subscriber.recv_async().await?;

        match serialport.write(sample.value.to_string().as_bytes()) {
            Ok(_)=>{
                if let Err(e) = std::io::stdout().flush(){
                    let msg = format!("Failed to flush stdout: {:?}", e);
                    logger::log_error(node_name, msg);
                }
                logger::log_info(node_name, sample.value.to_string());
            },
            Err(e)=>logger::log_error(node_name, e.to_string()),
        }
    }
}