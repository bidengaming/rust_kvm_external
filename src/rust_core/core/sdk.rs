use memflow::prelude::v1::*;

use crate::il2cpp::Il2Cpp;

pub struct Offsets {
    pub flag_offset: u64,
    pub model_offset: u64,
    pub net_offset: u64,
    pub net_id_offset: u64,
    pub health_offset: u64,
    pub playermodel_offset: u64,
    pub displayname_offset: u64,
    pub cl_active_item_offset: u64,
    pub inventory_offset: u64,
    pub item_list_offset: u64,
    pub container_belt_offset: u64,
    pub item_uid_offset: u64,
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
        let net_offset: u64 = il2cpp
            .images
            .get("Assembly-CSharp")
            .unwrap()
            .classes
            .get("BaseNetworkable")
            .unwrap()
            .get_field_offset(process, String::from("net")) as u64;
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
        Self {
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
    fn size() -> usize {
        0
    }
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
}

impl BaseCombatEntity {
    pub fn new<P: MemoryView>(process: &mut P, instance: u64, offsets: &Offsets) -> Self {
        let health = process
            .read::<f32>(Address::from(instance + offsets.health_offset))
            .unwrap();
        Self {
            base_entity: BaseEntity::new(process, instance, offsets),
            _health: health,
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

#[derive(Clone)]
pub struct Item {
    pub instance: u64,
    pub uid: u32,
}

impl RemoteObject for Item {
    fn size() -> usize {
        std::mem::size_of::<Item>()
    }

    fn new() -> Self {
        Self {
            instance: 0,
            uid: 0,
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
        Some(())
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
            println!("No active item");
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
            && self.base_combat_entity.base_entity.flags & 0x100 == 0
    }
}
