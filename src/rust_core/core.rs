use memflow::prelude::v1::*;

pub struct RustCore<P, K>
{
    process: P,
    keyboard: K,  
}

impl<P: Process + MemoryView, K: Keyboard> RustCore<P, K>
{
    pub fn new( process: P, keyboard: K) -> Self
    {
        Self{process, keyboard}
    }

    pub fn update(&mut self)
    {
        let game_assembly = self.process.module_by_name("GameAssembly.dll").expect("Unable to find GameAssembly.dll");
        loop {
            let local_player = self.process.read::<u64>(game_assembly.base + 0x1F4F6F8).expect("Unable to read game state");
            std::thread::sleep(std::time::Duration::from_millis(10));
        }
    }
}