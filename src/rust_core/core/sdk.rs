use crate::il2cpp::GameAssembly;
use lazy_static::*;
use memflow::prelude::v1::*;

pub const CONN_NAME: &str = "qemu";
pub const KRNL_NAME: &str = "win32";

lazy_static! {
    pub static ref GAME_ASSEMBLY: GameAssembly = {
        let inventory = Inventory::scan();
        let mut os = inventory
            .builder()
            .connector(CONN_NAME)
            .os(KRNL_NAME)
            .build()
            .expect("unable to instantiate connector / os");
        let mut process = os.process_by_name("RustClient.exe").unwrap();
        GameAssembly::new(&mut process)
    };
}

pub struct Networkable {
    pub ID: u32,
}

impl Networkable {
    pub fn new<P: MemoryView + Process>(process: &mut P, instance: u64) -> Self {
        let ID = process
            .read::<u32>(Address::from(
                instance
                    + (*GAME_ASSEMBLY)
                        .fast("Facepunch.Network:static Networkable.ID")
                        .unwrap(),
            ))
            .unwrap();
        Self { ID: ID }
    }
}

pub struct BaseNetworkable {
    instance: u64,
    client_entities: u64,
}

impl BaseNetworkable {
    pub fn new<P: MemoryView + Process>(process: &mut P, instance: u64) -> Self {
        Self {
            client_entities: 0,
            instance: instance,
        }
    }
}

pub struct BaseEntity {
    base_networkable: BaseNetworkable,
    flags: i32,
    net: Networkable,
}

impl BaseEntity {
    pub fn new<P: MemoryView + Process>(process: &mut P, instance: u64) -> Self {
        let flags = process
            .read::<i32>(Address::from(
                instance
                    + (*GAME_ASSEMBLY)
                        .fast("Assembly-CSharp:static BaseEntity.flags")
                        .unwrap(),
            ))
            .unwrap();
        Self {
            base_networkable: BaseNetworkable::new(process, instance),
            flags: flags,
        }
    }
}

pub struct BaseCombatEntity {
    base_entity: BaseEntity,
    _health: f32,
}

impl BaseCombatEntity {
    pub fn new<P: MemoryView + Process>(process: &mut P, instance: u64) -> Self {
        let health = process
            .read::<f32>(Address::from(
                instance
                    + (*GAME_ASSEMBLY)
                        .fast("Assembly-CSharp:static BaseCombatEntity._health")
                        .unwrap(),
            ))
            .unwrap();
        Self {
            base_entity: BaseEntity::new(process, instance),
            _health: health,
        }
    }
}

pub struct BasePlayer {
    base_combat_entity: BaseCombatEntity,
}
