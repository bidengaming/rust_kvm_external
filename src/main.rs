use memflow::prelude::v1::*;

mod rust_core;
use rust_core::*;

mod il2cpp;

const CONN_NAME: &str = "qemu";
const KRNL_NAME: &str = "win32";

fn main() {
    env_logger::init();

    let inventory = Inventory::scan();
    let mut os = inventory
        .builder()
        .connector(CONN_NAME)
        .os(KRNL_NAME)
        .build()
        .expect("unable to instantiate connector / os");

    let keyboard = os
        .clone()
        .into_impl_oskeyboardinner()
        .expect("kernel plugin has not enabled the keyboard feature")
        .into_keyboard()
        .expect("unable to initialize keyboard");

    let process = os.process_by_name("RustClient.exe").unwrap();
    println!("found process: {:?}", process.info());
    let mut rust = core::RustCore::new(process, keyboard);
    rust.update();
}
