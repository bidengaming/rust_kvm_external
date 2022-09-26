use memflow::prelude::v1::*;

use crate::il2cpp::Il2Cpp;

pub struct Offsets {
    pub flag_offset: u64,
    pub model_offset: u64,
    pub net_offset: u64,
    pub net_id_offset: u64,
    pub health_offset: u64,
    pub base_combat_entity_lifestate: u64,
    pub playermodel_offset: u64,
    pub displayname_offset: u64,
    pub cl_active_item_offset: u64,
    pub inventory_offset: u64,
    pub item_list_offset: u64,
    pub container_belt_offset: u64,
    pub item_uid_offset: u64,
    pub item_info_offset: u64,
    pub item_definition_category_offset: u64,
    pub item_held_entity_offset: u64,
    pub base_projectile_automatic_offset: u64,
    pub base_projectile_aim_sway_offset: u64,
    pub base_projectile_recoil_offset: u64,
    pub base_projectile_aimcone_offset: u64,
    pub base_projectile_hip_aim_cone_offset: u64,
    pub base_projectile_created_projectile_offset: u64,
    pub recoil_properties_new_recoil_override_offset: u64,
}

impl Offsets {
    pub fn new<P: Process + MemoryView>(process: &mut P, il2cpp: &Il2Cpp) -> Self {
        let flag_offset: u64 = il2cpp
            .images
            .get("Assembly-CSharp")
            .unwrap()
            .classes
            .get("BaseEntity")
            .unwrap()
            .get_field_offset(process, String::from("flags")) as u64;
        let displayname_offset: u64 = il2cpp
            .images
            .get("Assembly-CSharp")
            .unwrap()
            .classes
            .get("BasePlayer")
            .unwrap()
            .get_field_offset(process, String::from("_displayName"))
            as u64;

        let model_offset: u64 = il2cpp
            .images
            .get("Assembly-CSharp")
            .unwrap()
            .classes
            .get("BaseEntity")
            .unwrap()
            .get_field_offset(process, String::from("model"))
            as u64;
        let net_offset: u64 = 0x58 as u64;
        // due to basenet being encrypted cant get class instance
        // il2cpp
        //     .images
        //     .get("Assembly-CSharp")
        //     .unwrap()
        //     .classes
        //     .get("BaseNetworkable")
        //     .unwrap()
        //     .get_field_offset(process, String::from("net")) as u64;
        let net_id_offset: u64 = il2cpp
            .images
            .get("Facepunch.Network")
            .unwrap()
            .classes
            .get("Networkable")
            .unwrap()
            .get_field_offset(process, String::from("ID")) as u64;
        let health_offset: u64 = il2cpp
            .images
            .get("Assembly-CSharp")
            .unwrap()
            .classes
            .get("BaseCombatEntity")
            .unwrap()
            .get_field_offset(process, String::from("_health"))
            as u64;
        let playermodel_offset: u64 = il2cpp
            .images
            .get("Assembly-CSharp")
            .unwrap()
            .classes
            .get("BasePlayer")
            .unwrap()
            .get_field_offset(process, String::from("playerModel"))
            as u64;
        let cl_active_item_offset: u64 = il2cpp
            .images
            .get("Assembly-CSharp")
            .unwrap()
            .classes
            .get("BasePlayer")
            .unwrap()
            .get_field_offset(process, String::from("clActiveItem"))
            as u64;
        let inventory_offset: u64 = il2cpp
            .images
            .get("Assembly-CSharp")
            .unwrap()
            .classes
            .get("BasePlayer")
            .unwrap()
            .get_field_offset(process, String::from("inventory"))
            as u64;

        let container_belt_offset: u64 = il2cpp
            .images
            .get("Assembly-CSharp")
            .unwrap()
            .classes
            .get("PlayerInventory")
            .unwrap()
            .get_field_offset(process, String::from("containerBelt"))
            as u64;
        let item_list_offset: u64 = il2cpp
            .images
            .get("Assembly-CSharp")
            .unwrap()
            .classes
            .get("ItemContainer")
            .unwrap()
            .get_field_offset(process, String::from("itemList"))
            as u64;

        let item_uid_offset: u64 = il2cpp
            .images
            .get("Assembly-CSharp")
            .unwrap()
            .classes
            .get("Item")
            .unwrap()
            .get_field_offset(process, String::from("uid"))
            as u64;
        let item_info_offset: u64 = il2cpp
            .images
            .get("Assembly-CSharp")
            .unwrap()
            .classes
            .get("Item")
            .unwrap()
            .get_field_offset(process, String::from("info"))
            as u64;

        let item_definition_category_offset: u64 = il2cpp
            .images
            .get("Assembly-CSharp")
            .unwrap()
            .classes
            .get("ItemDefinition")
            .unwrap()
            .get_field_offset(process, String::from("category"))
            as u64;

        let item_held_entity_offset: u64 = il2cpp
            .images
            .get("Assembly-CSharp")
            .unwrap()
            .classes
            .get("Item")
            .unwrap()
            .get_field_offset(process, String::from("heldEntity"))
            as u64;
        let base_projectile_automatic_offset: u64 = il2cpp
            .images
            .get("Assembly-CSharp")
            .unwrap()
            .classes
            .get("BaseProjectile")
            .unwrap()
            .get_field_offset(process, String::from("automatic"))
            as u64;
        let base_projectile_aim_sway_offset: u64 = il2cpp
            .images
            .get("Assembly-CSharp")
            .unwrap()
            .classes
            .get("BaseProjectile")
            .unwrap()
            .get_field_offset(process, String::from("aimSway"))
            as u64;
        let base_projectile_recoil_offset: u64 = il2cpp
            .images
            .get("Assembly-CSharp")
            .unwrap()
            .classes
            .get("BaseProjectile")
            .unwrap()
            .get_field_offset(process, String::from("recoil"))
            as u64;
        let base_projectile_aimcone_offset: u64 = il2cpp
            .images
            .get("Assembly-CSharp")
            .unwrap()
            .classes
            .get("BaseProjectile")
            .unwrap()
            .get_field_offset(process, String::from("aimCone"))
            as u64;
        let base_projectile_created_projectile_offset: u64 = il2cpp
            .images
            .get("Assembly-CSharp")
            .unwrap()
            .classes
            .get("BaseProjectile")
            .unwrap()
            .get_field_offset(process, String::from("createdProjectiles"))
            as u64;
        let base_projectile_hip_aim_cone_offset: u64 = il2cpp
            .images
            .get("Assembly-CSharp")
            .unwrap()
            .classes
            .get("BaseProjectile")
            .unwrap()
            .get_field_offset(process, String::from("hipAimCone"))
            as u64;
        let base_combat_entity_lifestate = il2cpp
            .images
            .get("Assembly-CSharp")
            .unwrap()
            .classes
            .get("BaseCombatEntity")
            .unwrap()
            .get_field_offset(process, String::from("lifestate"))
            as u64;
        let recoil_properties_new_recoil_override_offset = il2cpp
            .images
            .get("Assembly-CSharp")
            .unwrap()
            .classes
            .get("RecoilProperties")
            .unwrap()
            .get_field_offset(process, String::from("newRecoilOverride"))
            as u64;
        Self {
            recoil_properties_new_recoil_override_offset,
            base_combat_entity_lifestate,
            base_projectile_hip_aim_cone_offset,
            base_projectile_aimcone_offset,
            base_projectile_created_projectile_offset,
            base_projectile_recoil_offset,
            base_projectile_aim_sway_offset,
            base_projectile_automatic_offset,
            item_held_entity_offset,
            item_definition_category_offset,
            item_info_offset,
            flag_offset,
            model_offset,
            net_offset,
            net_id_offset,
            health_offset,
            playermodel_offset,
            displayname_offset,
            cl_active_item_offset,
            inventory_offset,
            container_belt_offset,
            item_list_offset,
            item_uid_offset,
        }
    }
}

pub trait RemoteObject {
    fn new() -> Self;
    fn update<P: MemoryView + Process>(
        &mut self,
        ptr: Address,
        process: &mut P,
        offsets: &Offsets,
    ) -> Option<()>;
}

pub struct MonoArray<T: RemoteObject> {
    pub address: Address,
    pub size: i32,
    pub elements: Vec<T>,
}

impl<T: Clone + RemoteObject> MonoArray<T> {
    pub fn new<P: Process + MemoryView>(
        process: &mut P,
        address: Address,
        offsets: &Offsets,
    ) -> Self {
        let count = process.read::<i32>(address + 0x18).unwrap();
        if count > 0x7fff {
            println!("Array count is too large");
            return Self {
                address: Address::from(0),
                size: 0 as i32,
                elements: Vec::new(),
            };
        }
        let mut elements = Vec::new();
        elements.resize(count as usize, T::new());
        let mut counter = 0;
        for entry in elements.iter_mut() {
            match entry.update(address + 0x20 + counter as usize * 0x8, process, offsets) {
                Some(()) => {
                    counter += 1;
                }
                None => {
                    println!("Failed to update array element");
                    break;
                }
            }
        }

        Self {
            address,
            size: count,
            elements: elements,
        }
    }
}

pub struct MonoString {
    pub address: Address,
    pub content: Vec<u16>,
    pub inner: String,
}

impl MonoString {
    pub fn new<P: MemoryView>(process: &mut P, instance: u64) -> Self {
        let mut _self = Self {
            address: Address::from(instance),
            content: Vec::default(),
            inner: "".to_string(),
        };
        let mut count: i32 = 0;
        process
            .read_ptr_into(Pointer::from(instance + 0x10), &mut count)
            .unwrap();
        if count > 0x7fff {
            return _self;
        }
        _self.content.resize(count as usize, 0);
        for i in 0..count {
            process
                .read_ptr_into(
                    Pointer::from(instance + 0x14 + i as u64 * 2),
                    _self.content.get_mut(i as usize).expect(""),
                )
                .expect("unable to read string element");
        }
        _self.inner = String::from_utf16(_self.content.iter().as_slice()).unwrap();
        _self
    }
}

pub struct Object {}
impl Object {
    pub fn new<P: MemoryView>(process: &mut P, instance: u64, offsets: &Offsets) -> Self {
        Self {}
    }
}

pub struct Component {
    object: Object,
}
impl Component {
    pub fn new<P: MemoryView>(process: &mut P, instance: u64, offsets: &Offsets) -> Self {
        Self {
            object: Object::new(process, instance, offsets),
        }
    }
}

pub struct Behaviour {
    component: Component,
}
impl Behaviour {
    pub fn new<P: MemoryView>(process: &mut P, instance: u64, offsets: &Offsets) -> Self {
        let component = Component::new(process, instance, offsets);
        Self { component }
    }
}

pub struct MonoBehaviour {
    behaviour: Behaviour,
}
impl MonoBehaviour {
    pub fn new<P: MemoryView>(process: &mut P, instance: u64, offsets: &Offsets) -> Self {
        let behaviour = Behaviour::new(process, instance, offsets);
        Self { behaviour }
    }
}

pub struct FacepunchBehaviour {
    mono_behaviour: MonoBehaviour,
}
impl FacepunchBehaviour {
    pub fn new<P: MemoryView>(process: &mut P, instance: u64, offsets: &Offsets) -> Self {
        let mono_behaviour = MonoBehaviour::new(process, instance, offsets);
        Self { mono_behaviour }
    }
}

pub struct BaseMonoBehaviour {
    facepunch_behaviour: FacepunchBehaviour,
}

impl BaseMonoBehaviour {
    pub fn new<P: MemoryView>(process: &mut P, instance: u64, offsets: &Offsets) -> Self {
        let facepunch_behaviour = FacepunchBehaviour::new(process, instance, offsets);
        Self {
            facepunch_behaviour,
        }
    }
}

pub struct Networkable {
    pub id: u32,
}

impl Networkable {
    pub fn new<P: MemoryView>(process: &mut P, instance: u64, offsets: &Offsets) -> Self {
        let id = process
            .read::<u32>(Address::from(instance + offsets.net_id_offset))
            .unwrap();
        Self { id }
    }
}

pub struct BaseNetworkable {
    pub base_mono_behaviour: BaseMonoBehaviour,
    pub client_entities: u64,
    pub net: Networkable,
}

impl BaseNetworkable {
    pub fn new<P: MemoryView>(process: &mut P, instance: u64, offsets: &Offsets) -> Self {
        let net_instance = process.read::<u64>(Address::from(instance + 0x58)).unwrap();
        let net = Networkable::new(process, net_instance, offsets);
        let base_mono_behaviour = BaseMonoBehaviour::new(process, instance, offsets);
        Self {
            client_entities: 0,
            net,
            base_mono_behaviour,
        }
    }
}

pub struct Model {}

impl Model {
    pub fn new<P: MemoryView>(process: &mut P, instance: u64, offsets: &Offsets) -> Self {
        Self {}
    }
}

pub struct BaseEntity {
    pub base_networkable: BaseNetworkable,
    pub flags: i32,
    pub model: Model,
}

impl BaseEntity {
    pub fn new<P: MemoryView>(process: &mut P, instance: u64, offsets: &Offsets) -> Self {
        let flags = process
            .read::<i32>(Address::from(instance + offsets.flag_offset))
            .unwrap();
        let model = process
            .read::<u64>(Address::from(instance + offsets.model_offset))
            .unwrap();

        Self {
            base_networkable: BaseNetworkable::new(process, instance, offsets),
            flags,
            model: Model::new(process, model, offsets),
        }
    }
}
pub struct BaseCombatEntity {
    pub base_entity: BaseEntity,
    pub _health: f32,
    pub lifestate: i32,
}

impl BaseCombatEntity {
    pub fn new<P: MemoryView>(process: &mut P, instance: u64, offsets: &Offsets) -> Self {
        let _health = process
            .read::<f32>(Address::from(instance + offsets.health_offset))
            .unwrap();
        let lifestate = process
            .read::<i32>(Address::from(
                instance + offsets.base_combat_entity_lifestate,
            ))
            .unwrap();
        Self {
            base_entity: BaseEntity::new(process, instance, offsets),
            _health,
            lifestate,
        }
    }
}
pub struct PlayerModel {}

impl PlayerModel {
    pub fn new<P: MemoryView>(process: &mut P, instance: u64, offsets: &Offsets) -> Self {
        Self {}
    }
}
pub struct PlayerEyes {}

impl PlayerEyes {
    pub fn new<P: MemoryView>(process: &mut P, instance: u64, offsets: &Offsets) -> Self {
        Self {}
    }
}

pub struct BasePlayer {
    pub instance: u64,
    pub base_combat_entity: BaseCombatEntity,
    pub player_model: PlayerModel,
    pub eyes: PlayerEyes,
    pub display_name: MonoString,
    pub cl_active_item: u32,
    pub inventory: PlayerInventory,
}

pub enum ItemCategory {
    Weapon = 0,
    Construction = 1,
    Items = 2,
    Resources = 3,
    Attire = 4,
    Tool = 5,
    Medical = 6,
    Food = 7,
    Ammunition = 8,
    Traps = 9,
    Misc = 10,
    All = 11,
    Common = 12,
    Component = 13,
    Search = 14,
    Favourite = 15,
    Electrical = 16,
    Fun = 17,
}

#[derive(Clone)]
pub struct ItemDefinition {
    pub instance: u64,
    pub category: u32,
}

impl ItemDefinition {
    pub fn new<P: MemoryView>(process: &mut P, instance: u64, offsets: &Offsets) -> Self {
        let category = process
            .read::<u32>(Address::from(
                instance + offsets.item_definition_category_offset,
            ))
            .unwrap();
        Self { instance, category }
    }
}

#[derive(Clone)]
pub struct Projectile {}

impl RemoteObject for Projectile {
    fn new() -> Self {
        Self {}
    }
    fn update<P: MemoryView + Process>(
        &mut self,
        ptr: Address,
        process: &mut P,
        offsets: &Offsets,
    ) -> Option<()> {
        Some(())
    }
}
#[derive(Clone)]
pub struct RecoilProperties {
    pub instance: u64,
    pub recoil_yaw_min: f32,
    pub recoil_yaw_max: f32,
    pub recoil_pitch_min: f32,
    pub recoil_pitch_max: f32,
}

impl RecoilProperties {
    pub fn new<P: MemoryView>(process: &mut P, instance: u64, offsets: &Offsets) -> Self {
        let recoil_yaw_min = process.read::<f32>(Address::from(instance + 0x18)).unwrap();
        let recoil_yaw_max = process.read::<f32>(Address::from(instance + 0x1C)).unwrap();
        let recoil_pitch_min = process.read::<f32>(Address::from(instance + 0x20)).unwrap();
        let recoil_pitch_max = process.read::<f32>(Address::from(instance + 0x24)).unwrap();
        Self {
            instance,
            recoil_yaw_min,
            recoil_yaw_max,
            recoil_pitch_min,
            recoil_pitch_max,
        }
    }

    pub fn set_recoil<P: MemoryView>(&self, process: &mut P, recoil: f32) {
        let recoil_yaw_min = recoil * self.recoil_yaw_min;
        process
            .write::<f32>(Address::from(self.instance + 0x18), &recoil_yaw_min)
            .unwrap();
        let recoil_yaw_max = recoil * self.recoil_yaw_max;
        process
            .write::<f32>(Address::from(self.instance + 0x1C), &recoil_yaw_max)
            .unwrap();
        let recoil_pitch_min = recoil * self.recoil_pitch_min;
        process
            .write::<f32>(Address::from(self.instance + 0x20), &recoil_pitch_min)
            .unwrap();
        let recoil_pitch_max = recoil * self.recoil_pitch_max;
        process
            .write::<f32>(Address::from(self.instance + 0x24), &recoil_pitch_max)
            .unwrap();
    }
}

#[derive(Clone)]
pub struct BaseProjectile {
    pub instance: u64,
    pub created_projectiles: Vec<Projectile>,
    pub recoil: RecoilProperties,
}

impl BaseProjectile {
    pub fn new<P: Process + MemoryView>(process: &mut P, instance: u64, offsets: &Offsets) -> Self {
        let created_projectiles = process
            .read::<u64>(Address::from(
                instance + offsets.base_projectile_created_projectile_offset,
            ))
            .unwrap();
        let created_projectiles = process
            .read::<u64>(Address::from(created_projectiles + 0x10))
            .unwrap();
        let created_projectiles =
            MonoArray::<Projectile>::new(process, Address::from(created_projectiles), offsets);

        let recoil = process
            .read::<u64>(Address::from(
                instance + offsets.base_projectile_recoil_offset,
            ))
            .unwrap();
        let recoil = process
            .read::<u64>(Address::from(
                recoil + offsets.recoil_properties_new_recoil_override_offset,
            ))
            .unwrap();
        Self {
            instance,
            created_projectiles: created_projectiles.elements,
            recoil: RecoilProperties::new(process, recoil, offsets),
        }
    }

    pub fn set_automatic<P: MemoryView>(&self, process: &mut P, automatic: i32, offsets: &Offsets) {
        process
            .write::<i32>(
                Address::from(self.instance + offsets.base_projectile_automatic_offset),
                &automatic,
            )
            .unwrap();
    }

    pub fn set_no_sway<P: MemoryView>(&self, process: &mut P, sway: f32, offsets: &Offsets) {
        process
            .write::<f32>(
                Address::from(self.instance + offsets.base_projectile_aim_sway_offset),
                &sway,
            )
            .unwrap();
    }

    pub fn set_spread<P: MemoryView>(&self, process: &mut P, aim_cone: f32, offsets: &Offsets) {
        process
            .write::<f32>(
                Address::from(self.instance + offsets.base_projectile_aimcone_offset),
                &aim_cone,
            )
            .unwrap();
        process
            .write::<f32>(
                Address::from(self.instance + offsets.base_projectile_hip_aim_cone_offset),
                &aim_cone,
            )
            .unwrap();
    }
}

#[derive(Clone)]
pub struct Item {
    pub instance: u64,
    pub uid: u32,
    pub item_definition: ItemDefinition,
    pub held_entity: BaseProjectile,
}

impl RemoteObject for Item {
    fn new() -> Self {
        Self {
            instance: 0,
            uid: 0,
            item_definition: ItemDefinition {
                instance: 0,
                category: 0,
            },
            held_entity: BaseProjectile {
                instance: 0,
                created_projectiles: Vec::new(),
                recoil: RecoilProperties {
                    instance: 0,
                    recoil_yaw_min: 0.0,
                    recoil_yaw_max: 0.0,
                    recoil_pitch_min: 0.0,
                    recoil_pitch_max: 0.0,
                },
            },
        }
    }
    fn update<P: MemoryView + Process>(
        &mut self,
        ptr: Address,
        process: &mut P,
        offsets: &Offsets,
    ) -> Option<()> {
        self.instance = process.read::<u64>(ptr).unwrap();
        self.uid = process
            .read::<u32>(Address::from(self.instance + offsets.item_uid_offset))
            .unwrap();

        let item_definition = process
            .read::<u64>(Address::from(self.instance + offsets.item_info_offset))
            .unwrap();
        self.item_definition = ItemDefinition::new(process, item_definition, offsets);

        if self.is_weapon() {
            let held_entity = process
                .read::<u64>(Address::from(
                    self.instance + offsets.item_held_entity_offset,
                ))
                .unwrap();
            self.held_entity = BaseProjectile::new(process, held_entity, offsets);
        }
        Some(())
    }
}

impl Item {
    pub fn is_weapon(&self) -> bool {
        if self.item_definition.category == ItemCategory::Weapon as u32
            && self.item_definition.instance > 0 as u64
        {
            return true;
        }
        return false;
    }
}

pub struct ItemContainer {
    pub item_list: Vec<Item>,
}
impl ItemContainer {
    pub fn new<P: MemoryView + Process>(process: &mut P, instance: u64, offsets: &Offsets) -> Self {
        let item_list = process
            .read::<u64>(Address::from(instance + offsets.item_list_offset))
            .unwrap();
        let internal_list = process
            .read::<u64>(Address::from(item_list + 0x10))
            .unwrap();
        let mono_array = MonoArray::<Item>::new(process, Address::from(internal_list), offsets);
        Self {
            item_list: mono_array.elements,
        }
    }
}

pub struct PlayerInventory {
    pub instance: u64,
    pub container_belt: ItemContainer,
}

impl PlayerInventory {
    pub fn new<P: MemoryView + Process>(process: &mut P, instance: u64, offsets: &Offsets) -> Self {
        let container_belt = process
            .read::<u64>(Address::from(instance + offsets.container_belt_offset))
            .unwrap();
        let container_belt = ItemContainer::new(process, container_belt, offsets);
        Self {
            instance,
            container_belt,
        }
    }
}

impl BasePlayer {
    pub fn get_active_item<P: MemoryView + Process>(
        &mut self,
        process: &mut P,
        offsets: &Offsets,
    ) -> Item {
        if self.cl_active_item <= 0 as u32 {
            return Item::new();
        }

        if self.inventory.instance <= 0 as u64 {
            println!("No inventory");
            return Item::new();
        }

        if self.inventory.container_belt.item_list.len() <= 0 {
            println!("No belt items");
            return Item::new();
        }

        //loop through container belt item list
        for item in self.inventory.container_belt.item_list.iter() {
            //if item id matches active item id
            if item.uid == self.cl_active_item {
                //return item
                return item.clone();
            }
        }

        return Item::new();
    }

    pub fn new<P: MemoryView + Process>(process: &mut P, instance: u64, offsets: &Offsets) -> Self {
        let display_name_instance = process
            .read::<u64>(Address::from(instance + offsets.displayname_offset))
            .unwrap();
        let display_name = MonoString::new(process, display_name_instance);
        let cl_active_item = process
            .read::<u32>(Address::from(instance + offsets.cl_active_item_offset))
            .unwrap();
        let inventory = process
            .read::<u64>(Address::from(instance + offsets.inventory_offset))
            .unwrap();

        let inventory = PlayerInventory::new(process, inventory, offsets);
        Self {
            instance,
            base_combat_entity: BaseCombatEntity::new(process, instance, offsets),
            player_model: PlayerModel::new(process, display_name_instance, offsets),
            eyes: PlayerEyes::new(process, display_name_instance, offsets),
            display_name,
            cl_active_item,
            inventory,
        }
    }

    pub fn is_player_valid(&self) -> bool {
        self.base_combat_entity.base_entity.base_networkable.net.id > 0
            && self.base_combat_entity._health > 0.0
            && self.base_combat_entity.lifestate == 0
    }
}
