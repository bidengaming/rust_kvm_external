use memflow::prelude::v1::*;

use sdk::*;

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
        let assembly_csharp = (*sdk::GAME_ASSEMBLY)
            .image("Assembly-CSharp")
            .expect("Failed to find Assembly-CSharp");

        let local_player_class = assembly_csharp
            .class("LocalPlayer")
            .expect("Failed to find BasePlayer");

        loop {
            let local_player_static_fields = self
                .process
                .read::<u64>(Address::from(local_player_class.instance.to_umem() + 0xB8))
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
            let local_player = BasePlayer::new(&mut self.process, local_player_instance);
            if !local_player.is_player_valid() {
                continue;
            }

            std::thread::sleep(std::time::Duration::from_millis(1000));
        }
    }
}
