use async_std;
use zenoh::Error;
use serial_transporter::serial_transporter;

#[async_std::main]
async fn main()->Result<(), Error>
{
    let task = async_std::task::spawn(serial_transporter("wheel_serial", "./param/serial_transporter.yaml"));

    task.await?;

    Ok(())
}