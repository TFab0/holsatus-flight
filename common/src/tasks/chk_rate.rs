use crate::*;

use embassy_time::{Duration, Ticker};

use crate::signals as s;

/// Routine to calibrate gyroscopes, by calculating their bias.
#[embassy_executor::task]
pub async fn main() -> ! {
    const ID: &str = "acc_calib";

    // Input channels
    let mut rcv_raw_imu = s::RAW_MULTI_IMU_DATA
        .get(0 as usize)
        .expect("Invalid sensor ID")
        .receiver();

    info!(
        "{}: Starting accelerometer stream check on sensor {}",
        ID, 0
    );

    let mut ticker = Ticker::every(Duration::from_hz(25));
    let mut elapsed = embassy_time::Instant::now();
    // Mark any stale data as seen
    _ = rcv_raw_imu.try_get();

    let mut ct = 0;
    let mut fails = 0;
    // Calibration loop
    loop {
        ticker.next().await;

        match rcv_raw_imu.try_changed() {
            Some(_) => {
                ct += 1;
                // debug!("{}: Received new IMU data for sensor {}", ID, sensor_id);
                // break 'calibration;
            }
            None => {
                fails += 1;
                // debug!("{}: No new IMU data for sensor {}, waiting...", ID, 0);
            }
        }
        if ct >= 4{
            let got4time = embassy_time::Instant::now() - elapsed;
            elapsed = embassy_time::Instant::now();
            info!("{}: Received {} IMU data points for sensor {}, in {} seconds | CUM FAILED: {}", ID, ct, 0, got4time.as_millis() as f32/1000.0, fails);
            ct = 0;
            // fails = 0;
        }

    };
}
