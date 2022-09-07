use memflow::prelude::v1::*;

use sdk::*;

use crate::il2cpp::Il2Cpp;

pub mod sdk;

pub struct RustCore<P, K> {
    process: P,
    keyboard: K,
}
impl<P: Process + MemoryView, K: Keyboard> RustCore<P, K> {
    pub fn new(process: P, keyboard: K) -> Self {
        Self { process, keyboard }
    }

    pub fn update(&mut self) {
        let il2cpp = Il2Cpp::new(&mut self.process);
        let assembly_csharp = il2cpp.images.get("Assembly-CSharp").unwrap();
        let local_player_class = assembly_csharp.classes.get("LocalPlayer").unwrap();
        let offsets = Offsets::new(&mut self.process, &il2cpp);

        loop {
            let local_player_static_fields = self
                .process
                .read::<u64>(Address::from(local_player_class.instance + 0xB8))
                .unwrap();
            if local_player_static_fields < 0 as u64 {
                continue;
            }

            let local_player_instance = self
                .process
                .read::<u64>(Address::from(local_player_static_fields))
                .unwrap();

            if local_player_instance < 0 as u64 {
                continue;
            }
            let local_player = BasePlayer::new(&mut self.process, local_player_instance, &offsets);
            if !local_player.is_player_valid() {
                continue;
            }
            println!("displayname: {}", local_player.display_name.inner.as_str());
            println!("health: {}", local_player.base_combat_entity._health);
            std::thread::sleep(std::time::Duration::from_millis(1000));
        }
    }
}
