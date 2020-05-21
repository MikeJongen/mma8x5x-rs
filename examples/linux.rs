use linux_embedded_hal::I2cdev;
use mma8x5x::Mma8x5x;

fn main() {
    let dev = I2cdev::new("/dev/i2c-1").unwrap();
    let mut sensor = Mma8x5x::new_mma8653(dev);
    loop {
        let id = sensor.device_id().unwrap_or(0);
        println!("Device ID: 0x{:x}", id);
    }
}
