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
    pub static ref FLAG_OFFSET: u64 = (*GAME_ASSEMBLY)
        .fast("Assembly-CSharp:static BaseEntity.flags")
        .unwrap();
    pub static ref NET_OFFSET: u64 = (*GAME_ASSEMBLY)
        .fast("Assembly-CSharp:static BaseNetworkable.net")
        .unwrap();
    pub static ref NET_ID_OFFSET: u64 = (*GAME_ASSEMBLY)
        .fast("Facepunch.Network:static Networkable.ID")
        .unwrap();
}

pub struct Networkable {
    pub ID: u32,
}

unsafe impl super::Pod for Networkable {}

impl Networkable {
    pub fn new<P: MemoryView + Process>(process: &mut P, instance: u64) -> Self {
        let id = process
            .read::<u32>(Address::from(instance + *NET_ID_OFFSET))
            .unwrap();
        Self { ID: id }
    }
}

pub struct BaseNetworkable {
    instance: u64,
    client_entities: u64,
    net: Networkable,
}

impl BaseNetworkable {
    pub fn new<P: MemoryView + Process>(process: &mut P, instance: u64) -> Self {
        let mut net = Networkable { ID: 0 };
        process
            .read_ptr_into(Pointer::from(instance + *NET_OFFSET), &mut net)
            .unwrap();

        Self {
            client_entities: 0,
            instance: instance,
            net: net,
        }
    }
}
pub struct BaseEntity {
    base_networkable: BaseNetworkable,
    flags: i32,
}

impl BaseEntity {
    pub fn new<P: MemoryView + Process>(process: &mut P, instance: u64) -> Self {
        let flags = process
            .read::<i32>(Address::from(instance + *FLAG_OFFSET))
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
