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
    pub static ref MODEL_OFFSET: u64 = (*GAME_ASSEMBLY)
        .fast("Assembly-CSharp:static BaseEntity.model")
        .unwrap();
    pub static ref NET_OFFSET: u64 = (*GAME_ASSEMBLY)
        .fast("Assembly-CSharp:BaseNetworkable.net")
        .unwrap();
    pub static ref NET_ID_OFFSET: u64 = (*GAME_ASSEMBLY)
        .fast("Facepunch.Network:static Networkable.ID")
        .unwrap();
    pub static ref HEALTH_OFFSET: u64 = (*GAME_ASSEMBLY)
        .fast("Assembly-CSharp:static BaseCombatEntity._health")
        .unwrap();
    pub static ref PLAYERMODEL_OFFSET: u64 = (*GAME_ASSEMBLY)
        .fast("Assembly-CSharp:static BasePlayer.playerModel")
        .unwrap();
    pub static ref PLAYEREYES_OFFSET: u64 = (*GAME_ASSEMBLY)
        .fast("Assembly-CSharp:static BaseCombatEntity.eyes")
        .unwrap();
}

pub struct Networkable {
    pub id: u32,
}

impl Networkable {
    pub fn new<P: MemoryView + Process>(process: &mut P, instance: u64) -> Self {
        let id = process
            .read::<u32>(Address::from(instance + *NET_ID_OFFSET))
            .unwrap();
        Self { id: id }
    }
}

pub struct BaseNetworkable {
    pub client_entities: u64,
    pub net: Networkable,
}

impl BaseNetworkable {
    pub fn new<P: MemoryView + Process>(process: &mut P, instance: u64) -> Self {
        let net_instance = process.read::<u64>(Address::from(instance + 0x58)).unwrap();
        let net = Networkable::new(process, net_instance);

        Self {
            client_entities: 0,
            net: net,
        }
    }
}

pub struct Model {}

impl Model {
    pub fn new<P: MemoryView + Process>(process: &mut P, instance: u64) -> Self {
        Self {}
    }
}

pub struct BaseEntity {
    pub base_networkable: BaseNetworkable,
    pub flags: i32,
    pub model: Model,
}

impl BaseEntity {
    pub fn new<P: MemoryView + Process>(process: &mut P, instance: u64) -> Self {
        let flags = process
            .read::<i32>(Address::from(instance + *FLAG_OFFSET))
            .unwrap();
        let model = process
            .read::<u64>(Address::from(instance + *MODEL_OFFSET))
            .unwrap();

        Self {
            base_networkable: BaseNetworkable::new(process, instance),
            flags: flags,
            model: Model::new(process, model),
        }
    }
}
pub struct BaseCombatEntity {
    pub base_entity: BaseEntity,
    pub _health: f32,
}

impl BaseCombatEntity {
    pub fn new<P: MemoryView + Process>(process: &mut P, instance: u64) -> Self {
        let health = process
            .read::<f32>(Address::from(instance + *HEALTH_OFFSET))
            .unwrap();
        Self {
            base_entity: BaseEntity::new(process, instance),
            _health: health,
        }
    }
}
pub struct PlayerModel {}

impl PlayerModel {
    pub fn new<P: MemoryView + Process>(process: &mut P, instance: u64) -> Self {
        Self {}
    }
}
pub struct PlayerEyes {}

impl PlayerEyes {
    pub fn new<P: MemoryView + Process>(process: &mut P, instance: u64) -> Self {
        Self {}
    }
}

pub struct BasePlayer {
    pub base_combat_entity: BaseCombatEntity,
    pub player_model: PlayerModel,
    pub eyes: PlayerEyes,
}

impl BasePlayer {
    pub fn new<P: MemoryView + Process>(process: &mut P, instance: u64) -> Self {
        let player_model = process
            .read::<u64>(Address::from(instance + *PLAYERMODEL_OFFSET))
            .unwrap();
        let eyes = process
            .read::<u64>(Address::from(instance + *PLAYEREYES_OFFSET))
            .unwrap();

        Self {
            base_combat_entity: BaseCombatEntity::new(process, instance),
            player_model: PlayerModel::new(process, player_model),
            eyes: PlayerEyes::new(process, eyes),
        }
    }

    pub fn is_player_valid(&self) -> bool {
        self.base_combat_entity.base_entity.base_networkable.net.id > 0
            && self.base_combat_entity._health > 0.0
            && self.base_combat_entity.base_entity.flags & 0x100 == 0
    }
}
