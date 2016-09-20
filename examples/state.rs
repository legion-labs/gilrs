extern crate gilrs;
extern crate env_logger;

use gilrs::Gilrs;

#[cfg(not(target_os = "windows"))]
fn main() {
    env_logger::init().unwrap();
    let mut gilrs = Gilrs::new();

    loop {
        for _ in gilrs.poll_events() {}
        // Clear
        print!("{}[2J", 0o33 as char);
        println!("{:#?}", gilrs.gamepad(0).state());
        std::thread::sleep(std::time::Duration::from_millis(100));
    }
}

#[cfg(target_os = "windows")]
fn main() {
    panic!("This example doesn't work on Windows");
}
