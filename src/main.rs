use memflow::prelude::v1::*;

mod rust_core;
use rust_core::*;

use crate::rust_core::core::sdk;
mod il2cpp;
extern crate lazy_static;
fn main() {
    env_logger::init();

    let inventory = Inventory::scan();
    let mut os = inventory
        .builder()
        .connector(sdk::CONN_NAME)
        .os(sdk::KRNL_NAME)
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
