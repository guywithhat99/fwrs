use imxrt_rt::{Family, RuntimeBuilder};

fn main() {
    // Teensy 4.1 ships with an 8 MiB QSPI flash device.
    RuntimeBuilder::from_flexspi(Family::Imxrt1060, 8 * 1024 * 1024)
        .build()
        .unwrap();
}
