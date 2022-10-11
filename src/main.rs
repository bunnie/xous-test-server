use std::{thread, time};

// build with:
// cargo build --target riscv32imac-unknown-xous-elf --release

fn main() -> ! {
    xous_api_log::init_wait().unwrap();
    log::set_max_level(log::LevelFilter::Info);
    log::info!("my PID is {}", xous::process::id());

    let timeout = time::Duration::from_millis(1000);
    let mut count = 0;
    loop {
        log::info!("test loop {}", count);
        count += 1;
        thread::sleep(timeout);
    }
}
