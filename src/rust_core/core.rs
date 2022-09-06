use memflow::prelude::v1::*;

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
        let health_offset = (*sdk::GAME_ASSEMBLY)
            .fast("Assembly-CSharp:static BaseCombatEntity._health")
            .expect("Failed to find BasePlayer");
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

            let local_player = self
                .process
                .read::<u64>(Address::from(local_player_static_fields))
                .unwrap();
            let health = self
                .process
                .read::<f32>(Address::from(local_player + health_offset))
                .unwrap();
            std::thread::sleep(std::time::Duration::from_millis(1000));
        }
    }
}
