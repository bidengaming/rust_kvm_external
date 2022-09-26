use memflow::prelude::v1::*;

mod rust_core;
use rust_core::*;

mod il2cpp;
extern crate lazy_static;
fn main() {
    env_logger::init();

    let inventory = Inventory::scan();
    let mut os = inventory
        .builder()
        .connector(il2cpp::CONN_NAME)
        .os(il2cpp::KRNL_NAME)
        .build()
        .expect("unable to instantiate connector / os");

    let mut keyboard = os
        .clone()
        .into_impl_oskeyboardinner()
        .expect("kernel plugin has not enabled the keyboard feature")
        .into_keyboard()
        .expect("unable to initialize keyboard");

    while !keyboard.is_down(0x2D) {
        println!("Press Insert to attach to Rust");
        std::thread::sleep(std::time::Duration::from_millis(1000));
    }

    println!("Attaching to Rust");

    let process = os.process_by_name("RustClient.exe").unwrap();
    println!("found process: {:?}", process.info());
    let mut rust = core::RustCore::new(process, keyboard);
    rust.update();
}
