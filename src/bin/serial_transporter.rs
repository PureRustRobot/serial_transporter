use async_std;
use zenoh::Error;
use serial_transporter::serial_transporter;

#[async_std::main]
async fn main()->Result<(), Error>
{
    let task = async_std::task::spawn(serial_transporter(
        "wheel_serial", 
        "motor_command",
        "/dev/ttyACM0",
        115200)
    );

    task.await?;

    Ok(())
}