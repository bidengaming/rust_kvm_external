use hashbrown::HashMap;
use lazy_static::*;
use log::{debug, warn};
use memflow::prelude::v1::{Address, MemoryView, ModuleInfo, Pointer, Process};
use regex::Regex;

const ASSEMBLY_TABLE: u64 = 0x2F765E8;
const CLASS_TABLE: u64 = 0x2F760B8;

// ^(static)?(?:.*\.)?(\w+)\.(\w+)$

pub struct Class {
    pub instance: Address,
    pub index: u32,
    pub fields_size: u16,
    pub fields_table: u64,
    pub static_fields_table: u64,
    pub key_values: HashMap<String, i32>,
    pub key_value_types: HashMap<String, u64>,
    pub key_value_static: HashMap<String, bool>,
}

impl Class {
    pub fn populate<P: MemoryView + Process>(
        class_name: &str,
        process: &mut P,
        table: u64,
        size: u16,
        container: &mut HashMap<String, i32>,
        type_container: &mut HashMap<String, u64>,
        static_mapping: &mut HashMap<String, bool>,
    ) {
        let mut cursor = table;
        let end = table + size as u64 * 0x20;
        while cursor < end {
            let mut string_ptr: u64 = 0;
            let mut value: i32 = 0;
            let mut type_ptr: u64 = 0;
            match (
                process.read_ptr_into(Pointer::from(cursor), &mut string_ptr),
                process.read_ptr_into(Pointer::from(cursor + 0x8), &mut type_ptr),
                process.read_char_string(Address::from(string_ptr)),
                process.read_ptr_into(Pointer::from(cursor + 0x18), &mut value),
            ) {
                (Ok(_), Ok(_), Ok(name), Ok(_)) => {
                    let mut flags: u32 = 0;
                    match (
                        process.read_ptr_into(Pointer::from(type_ptr + 0x8), &mut flags),
                        process.read_ptr_into(Pointer::from(type_ptr), &mut type_ptr),
                    ) {
                        (Ok(_), Ok(_)) => {
                            debug!(
                                "{}::{} @ {:x} (tag {:x} = {:x}",
                                class_name, name, cursor, type_ptr, value
                            );
                            type_container.insert(name.clone(), type_ptr);
                            let is_static = (flags & 0x10) != 0;
                            static_mapping.insert(name.clone(), is_static);
                        }
                        _ => {
                            debug!(
                                "`ANONYMOUS` {}::{} @ {:x} = {:x}",
                                class_name, name, cursor, value
                            );
                        }
                    }
                    //if name.contains("cloneDestroyedMessage") {
                    //debug!("@ {:x}", cursor);
                    //loop {}
                    //}
                    container.insert(name, value);
                }
                _ => {}
            }
            cursor += 0x20;
        }
    }

    pub fn new<P: MemoryView + Process>(
        i: u32,
        name: &str,
        _base: u64,
        instance: u64,
        process: &mut P,
    ) -> Option<Self> {
        let mut field_size: u16 = 0;
        let mut field_table: u64 = 0;
        let mut static_field_table: u64 = 0;

        match (
            process.read_ptr_into(Pointer::from(instance + 0x11C), &mut field_size),
            process.read_ptr_into(Pointer::from(instance + 0x80), &mut field_table),
            process.read_ptr_into(Pointer::from(instance + 0xB8), &mut static_field_table),
        ) {
            (Ok(_), Ok(_), Ok(_)) => {
                let mut kv = HashMap::new(); // key values
                let mut kvt = HashMap::new(); // key value types
                let mut kvs = HashMap::new(); // key value types
                Self::populate(
                    name,
                    process,
                    field_table,
                    field_size,
                    &mut kv,
                    &mut kvt,
                    &mut kvs,
                );
                if field_size > 0 {
                    debug!("counting {} fields in class {}", field_size, name);
                }
                Some(Self {
                    instance: Address::from(instance),
                    index: i,
                    fields_size: field_size,
                    fields_table: field_table,
                    static_fields_table: static_field_table,
                    key_values: kv,
                    key_value_types: kvt,
                    key_value_static: kvs,
                })
            }
            _ => None,
        }
    }
    pub fn field(&self, key: &str) -> Option<u64> {
        match self.key_values.get(key) {
            Some(value) => Some(self.instance.to_umem() + *value as u64),
            None => None,
        }
    }

    pub fn static_field(&self, key: &str) -> Option<u64> {
        match self.key_values.get(key) {
            Some(value) => Some(self.static_fields_table + *value as u64),
            None => None,
        }
    }
}

pub struct Image {
    pub instance: Address,
    pub classes: HashMap<String, Class>,
    pub start_index: u32,
    pub end_index: u32,
}

impl Image {
    pub fn new<P: MemoryView + Process>(
        name: &str,
        base: u64,
        instance: u64,
        process: &mut P,
    ) -> Option<Self> {
        let mut class_table: u64 = 0;
        let mut class_size: u32 = 0;
        let mut class_start: u32 = 0;

        let mut classes = HashMap::new();

        match (
            process.read_ptr_into(Pointer::from(base + CLASS_TABLE), &mut class_table),
            process.read_ptr_into(Pointer::from(instance + 0x1C), &mut class_size),
            process.read_ptr_into(Pointer::from(instance + 0x18), &mut class_start),
        ) {
            (Ok(_), Ok(_), Ok(_)) => {
                for i in 0..class_size {
                    let index = i + class_start;
                    let mut cursor: u64 = 0;
                    let mut string_ptr: u64 = 0;
                    match (
                        process.read_ptr_into(
                            Pointer::from(class_table + index as u64 * 8),
                            &mut cursor,
                        ),
                        process.read_ptr_into(Pointer::from(cursor + 0x10), &mut string_ptr),
                        process.read_char_string(Address::from(string_ptr)),
                    ) {
                        (Ok(_), Ok(_), Ok(name)) => {
                            match Class::new(i, name.as_str(), base, cursor, process) {
                                Some(class) => {
                                    classes.insert(name, class);
                                }
                                None => {}
                            }
                        }
                        _ => {}
                    }
                }
                debug!("counting {} classes in {}", classes.len(), name);
                Some(Self {
                    instance: Address::from(instance),
                    classes,
                    start_index: class_start,
                    end_index: class_start + class_size,
                })
            }
            _ => None,
        }
    }

    pub fn class(&self, name: &str) -> Option<&Class> {
        self.classes.get(name)
    }
}

pub struct GameAssembly {
    pub module: ModuleInfo,
    pub images: HashMap<String, Image>,
}

impl GameAssembly {
    pub fn new<P: MemoryView + Process>(process: &mut P) -> Self {
        let module = process
            .module_by_name("GameAssembly.dll")
            .expect("GameAssembly.dll not found");
        println!("GameAssembly.dll @ {:x}", module.base);
        let mut images = HashMap::new();

        let mut begin: u64 = 0;
        let mut end: u64 = 0;
        process
            .read_ptr_into(Pointer::from(module.base + ASSEMBLY_TABLE), &mut begin)
            .expect("unable to read beginning of assembly table");
        process
            .read_ptr_into(Pointer::from(module.base + ASSEMBLY_TABLE + 8), &mut end)
            .expect("unable to read ending of assembly table");
        println!(
            "assemblies begin @ {:x} ( {:x} + {:x} )",
            begin, module.base, ASSEMBLY_TABLE
        );
        println!("assemblies end @ {:x}", end);

        let mut cursor = begin;
        while cursor < end {
            let mut string_ptr: u64 = 0;
            let mut instance_ptr: u64 = 0;

            match (
                process.read_ptr_into(Pointer::from(cursor), &mut instance_ptr),
                process.read_ptr_into(Pointer::from(instance_ptr + 0x18), &mut string_ptr),
                process.read_ptr_into(Pointer::from(instance_ptr), &mut instance_ptr),
            ) {
                (Ok(_), Ok(_), Ok(_)) => {
                    match (
                        process.read_char_string(Address::from(string_ptr)),
                        instance_ptr != 0,
                    ) {
                        (Ok(name), true) => {
                            match Image::new(
                                name.as_str(),
                                module.base.to_umem(),
                                instance_ptr,
                                process,
                            ) {
                                Some(image) => {
                                    warn!("found image {}", name);
                                    images.insert(name, image);
                                }
                                _ => {}
                            }
                        }
                        (Err(_), true) => {
                            let iname = format!("{}", instance_ptr);
                            warn!("found anonymous image {} @ {:x}", iname, cursor);
                            match Image::new(
                                iname.as_str(),
                                module.base.to_umem(),
                                instance_ptr,
                                process,
                            ) {
                                Some(image) => {
                                    images.insert(iname, image);
                                }
                                _ => {}
                            }
                        }

                        _ => {
                            warn!(
                                "unable to read instance of image {:x} @ {:x}",
                                instance_ptr, cursor
                            );
                        }
                    }
                }
                _ => {
                    warn!(
                        "unable to read instance of image {:x} @ {:x}",
                        instance_ptr, cursor
                    );
                }
            }

            cursor += 8;
        }

        debug!("counting {} images in GameAssembly.dll", images.len());

        Self { module, images }
    }

    pub fn image(&self, name: &str) -> Option<&Image> {
        self.images.get(name)
    }

    pub fn fast(&self, name: &str) -> Option<u64> {
        lazy_static! {
            static ref FIELD_QUERY: Regex =
                Regex::new(r"^([^:]+):\s*(static)?\s*(\w+).(\w+)$").unwrap();
        }
        match FIELD_QUERY.captures(name) {
            Some(captures) => {
                let (kw_assembly, kw_static, kw_class, kw_field) = (
                    captures.get(1).unwrap().as_str(),
                    captures.get(2).unwrap().as_str(),
                    captures.get(3).unwrap().as_str(),
                    captures.get(4).unwrap().as_str(),
                );
                match self.image(kw_assembly) {
                    Some(image) => match image.class(kw_class) {
                        Some(class) => {
                            if kw_static.eq("static") {
                                return class.static_field(kw_field);
                            } else {
                                return class.field(kw_field);
                            }
                        }
                        None => None,
                    },
                    None => None,
                }
            }
            None => {
                warn!("unknown specification \"{}\", follow those examples:", name);
                warn!("[IMAGE]:[CLASS].[FIELD]");
                warn!("[IMAGE]:static [CLASS].[FIELD]");
                warn!("where you replace the square brackets with their respective name");
                None
            }
        }
    }

    pub fn resolve(&self, name: &str) -> Option<u64> {
        lazy_static! {
            static ref FIELD_QUERY: Regex =
                Regex::new(r"^(static)?(?:.*\.)?(\w+)\.(\w+)$").unwrap();
        }
        match FIELD_QUERY.captures(name) {
            Some(captures) => {
                let (kw_static, kw_class, kw_field) = (
                    captures.get(1).unwrap().as_str(),
                    captures.get(2).unwrap().as_str(),
                    captures.get(3).unwrap().as_str(),
                );
                debug!("QUERYING CLASS {}::{}", kw_class, kw_field);
                for (image_name, content) in &self.images {
                    match content.class(kw_class) {
                        Some(class) => {
                            debug!(
                                "found class of \"{}\" in {} ({} fields)",
                                name,
                                image_name,
                                class.key_values.len()
                            );
                            for (name, off) in class.key_values.iter() {
                                debug!("{}::{} @ {:x}", kw_class, name, off);
                            }
                            if kw_static.eq("static") {
                                return class.static_field(kw_field);
                            } else {
                                return class.field(kw_field);
                            }
                        }
                        _ => {}
                    }
                }
                None
            }
            None => {
                warn!("unknown specification \"{}\", follow those examples:", name);
                warn!("[NAMESPACE].[CLASS].[FIELD]");
                warn!("static [NAMESPACE].[CLASS].[FIELD]");
                warn!("where you replace the square brackets with their respective name");
                None
            }
        }
        // ^(static)?(?:.*\.)?(\w+)\.(\w+)$
    }
}
