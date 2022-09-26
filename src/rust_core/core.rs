use memflow::prelude::v1::*;

use sdk::*;

use crate::il2cpp::Il2Cpp;

pub mod sdk;

pub struct RustCore<P, K> {
    process: P,
    keyboard: K,
}
impl<P: Process + MemoryView, K: Keyboard> RustCore<P, K> {
    pub fn find_code_cave(&mut self, size: i64) -> u64 {
        let mut last_region: u64 = 0_u64;
        let regions = self.process.mapped_mem_range_vec(
            size,
            Address::from(last_region),
            self.process.metadata().max_address,
        );
        for region in regions {
            if (region.2 & PageType::WRITEABLE) != PageType::WRITEABLE
                || (region.2 & PageType::NOEXEC) == PageType::NOEXEC
                || region.1 < size as u64
            {
                continue;
            }
            last_region = region.0.to_umem() + region.1;
        }
        0
    }

    pub fn new(process: P, keyboard: K) -> Self {
        Self { process, keyboard }
    }

    pub fn update(&mut self) {
        let il2cpp = Il2Cpp::new(&mut self.process);
        let assembly_csharp = il2cpp.images.get("Assembly-CSharp").unwrap();
        let local_player_class = assembly_csharp.classes.get("LocalPlayer").unwrap();

        let mut last_item_uid: u32 = 0;
        let offsets = Offsets::new(&mut self.process, &il2cpp);
        loop {
            std::thread::sleep(std::time::Duration::from_millis(1000));

            let local_player_static_fields = self
                .process
                .read::<u64>(Address::from(local_player_class.instance + 0xB8))
                .unwrap();
            if local_player_static_fields <= 0 as u64 {
                continue;
            }

            let local_player_instance = self
                .process
                .read::<u64>(Address::from(local_player_static_fields))
                .unwrap();

            if local_player_instance <= 0 as u64 {
                continue;
            }
            let mut local_player =
                BasePlayer::new(&mut self.process, local_player_instance, &offsets);
            if !local_player.is_player_valid() {
                continue;
            }
            let active_item = local_player.get_active_item(&mut self.process, &offsets);
            if active_item.uid > 0 as u32 && active_item.held_entity.instance > 0 as u64 {
                if active_item.is_weapon() {
                    active_item
                        .held_entity
                        .set_automatic(&mut self.process, 1, &offsets);
                    active_item
                        .held_entity
                        .set_no_sway(&mut self.process, 0.0, &offsets);
                    active_item
                        .held_entity
                        .set_spread(&mut self.process, 0.0, &offsets);
                    if last_item_uid != active_item.uid
                        && active_item.held_entity.recoil.instance > 0 as u64
                    {
                        last_item_uid = active_item.uid;
                        active_item
                            .held_entity
                            .recoil
                            .set_recoil(&mut self.process, 0.5);
                    }
                }
            }
        }
    }
}
