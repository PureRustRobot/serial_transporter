use zenoh::{
    config::Config,
    prelude::r#async::*,
    Error
};

use std::{time::Duration, io::Write};

use serialport;
use zenoh_manage_utils::logger;

use prr_msgs::msg::*;

pub async fn serial_transporter(
    node_name:&str, 
    sub_topic:&str,
    port_name:&str,
    baud_rate:u32
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

        let get_data = deserialize_wheel(sample.value.to_string());

        let write_data = MotorControl{
            id:1,
            motor_1:get_data.front_left,
            motor_2:get_data.front_right,
            motor_3:get_data.rear_left,
            motor_4:get_data.rear_right,
        };

        let mut buf = [0_u8; std::mem::size_of::<MotorControl>()];

        serialize(&write_data, &mut buf);

        let mut vec = buf.to_vec();
        vec.insert(0, b't');
        vec.insert(0, b's');

        vec.push(b'e');
        vec.push(b'n');

        match serialport.write(vec.as_slice()) {
            Ok(_)=>{
                if let Err(e) = std::io::stdout().flush(){
                    let msg = format!("Failed to flush stdout: {:?}", e);
                    logger::log_error(node_name, msg);
                }
                logger::log_info(node_name, format!("{:?}", buf));
            },
            Err(e)=>logger::log_error(node_name, e.to_string()),
        }
    }
}

pub fn serialize<T: Sized>(obj: &T, buf: &mut [u8]) {
    let size = std::mem::size_of::<T>();

    let obj_ptr = obj as *const T as *const u8;
    let obj_slice = unsafe { std::slice::from_raw_parts(obj_ptr, size) };
    buf[..size].copy_from_slice(obj_slice);
}