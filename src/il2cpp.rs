use memflow::prelude::v1::*;
use std::collections::HashMap;

const ASSEMBLY_TABLE: u64 = 0x2F8E628;
const ASSEMBLY_TABLE_END: u64 = ASSEMBLY_TABLE + 8;
const CLASS_TABLE: u64 = 0x2F8E128;

pub const CONN_NAME: &str = "qemu";
pub const KRNL_NAME: &str = "win32";

pub struct Il2CppClass {
    fields_size: u32,
    fields_table: u64,
    static_fields_table: u64,
}

impl Il2CppClass {
    pub fn get_field_offset<P: MemoryView + Process>(
        &self,
        process: &mut P,
        field_to_find: String,
    ) -> u32 {
        let mut current_field = self.fields_table;
        while current_field < self.fields_table + (self.fields_size * 0x20) as u64 {
            let unk = process.read::<u64>(Address::from(current_field)).unwrap();

            let field_name = process.read_char_string(Address::from(unk)).unwrap();
            if field_name == field_to_find {
                let offset = process
                    .read::<u32>(Address::from(current_field + 0x18))
                    .unwrap();
                return offset;
            }

            current_field += 0x20;
        }

        0
    }

    pub fn get_static_field_value<P: MemoryView + Process>(
        &self,
        process: &mut P,
        field_to_find: String,
    ) -> u64 {
        let current_field_key = self.get_field_offset(process, field_to_find);
        if current_field_key < 0 {
            return 0 as u64;
        }

        self.static_fields_table + current_field_key as u64
    }

    pub fn new<P: MemoryView + Process>(process: &mut P, instance: u64) -> Self {
        let fields_size = process
            .read::<u32>(Address::from(instance + 0x11C))
            .unwrap();
        let fields_table = process.read::<u64>(Address::from(instance + 0x80)).unwrap();
        let static_fields_table = process.read::<u64>(Address::from(instance + 0xB8)).unwrap();

        Self {
            fields_table: fields_table,
            static_fields_table: static_fields_table,
            fields_size: fields_size,
        }
    }
}
pub struct Il2CppImage {
    pub classes: HashMap<String, Il2CppClass>,
}

impl Il2CppImage {
    pub fn new<P: MemoryView + Process>(process: &mut P, instance: u64, class_table: &u64) -> Self {
        let mut classes = HashMap::new();

        let classes_size = process.read::<u32>(Address::from(instance + 0x1C)).unwrap();
        let class_table_idx_start = process.read::<u32>(Address::from(instance + 0x18)).unwrap();
        for i in 0..classes_size {
            let current_class = process
                .read::<u64>(Address::from(
                    class_table + ((i + class_table_idx_start) * 8) as u64,
                ))
                .unwrap();

            let unk = process
                .read::<u64>(Address::from(current_class + 0x10))
                .unwrap();
            let class_name = process.read_char_string(Address::from(unk)).unwrap();
            classes.insert(class_name, Il2CppClass::new(process, current_class));
        }
        Self { classes }
    }
}

pub struct Il2Cpp {
    pub assemblies_end: u64,
    pub images: HashMap<String, Il2CppImage>,
}

impl Il2Cpp {
    pub fn new<P: MemoryView + Process>(process: &mut P) -> Self {
        let mut images = HashMap::new();
        let game_assembly = process.module_by_name("GameAssembly.dll").unwrap();

        let assemblies_end = process
            .read::<u64>(game_assembly.base + ASSEMBLY_TABLE_END)
            .unwrap();

        let mut current_assembly: u64 = process
            .read::<u64>(game_assembly.base + ASSEMBLY_TABLE)
            .unwrap();

        let class_table: u64 = process
            .read::<u64>(game_assembly.base + CLASS_TABLE)
            .unwrap();

        let inventory = Inventory::scan();
        let mut os = inventory
            .builder()
            .connector(CONN_NAME)
            .os(KRNL_NAME)
            .build()
            .expect("unable to instantiate connector / os");

        let mut _process = os.process_by_name("RustClient.exe").unwrap();

        while current_assembly < assemblies_end {
            let unk = process
                .read::<u64>(Address::from(current_assembly))
                .unwrap();

            let unk1 = process.read::<u64>(Address::from(unk + 0x18)).unwrap();

            let unk2 = process.read::<u64>(Address::from(unk)).unwrap();

            let image_name = process.read_char_string(Address::from(unk1)).unwrap();

            images.insert(
                image_name,
                Il2CppImage::new(&mut _process, unk2, &class_table),
            );
            current_assembly += 8;
        }

        Self {
            assemblies_end: assemblies_end,
            images: images,
        }
    }
}
