use std::collections::{hash_map::Iter, HashMap};

#[derive(Debug, PartialEq, Hash, Eq)]
pub enum PhoronConstantPoolKind {
    Class {
        name_index: u16,
    },
    Double([u8; 8]),
    Fieldref {
        class_index: u16,
        name_and_type_index: u16,
    },
    Float([u8; 4]),
    Integer([u8; 4]),
    InterfaceMethodref {
        class_index: u16,
        name_and_type_index: u16,
    },
    Long([u8; 8]),
    Methodref {
        class_index: u16,
        name_and_type_index: u16,
    },
    NameAndType {
        name_index: u16,
        descriptor_index: u16,
    },
    String {
        string_index: u16,
    },
    Utf8(String),
}

// Map from Constant Pool entries to indices
#[derive(Debug, PartialEq)]
pub struct PhoronConstantPool(pub(super) HashMap<PhoronConstantPoolKind, u16>);

impl PhoronConstantPool {
    pub fn new() -> Self {
        PhoronConstantPool(HashMap::new())
    }

    pub fn insert(&mut self, cp_key: PhoronConstantPoolKind, cp_val: u16) {
        self.0.insert(cp_key, cp_val);
    }

    /// Return the size of the Constant Pool
    pub fn len(&self) -> usize {
        let mut len = 0;

        for (cp_kind, _) in &self.0 {
            match cp_kind {
                PhoronConstantPoolKind::Long(_) | PhoronConstantPoolKind::Double(_) => len += 2,
                _ => len += 1,
            }
        }
        len
    }

    /// Return an iterator over the contents of the Constant Pool.
    pub fn iter(&self) -> Iter<'_, PhoronConstantPoolKind, u16> {
        self.0.iter()
    }

    /// Retrieve the index in the Constant Pool, if present, of the
    /// given double.
    pub fn get_double(&self, double: f64) -> Option<&u16> {
        self.0
            .get(&PhoronConstantPoolKind::Double(double.to_be_bytes()))
    }

    /// Retrieve the index in the Constant Pool, if present, of the
    /// given float.
    pub fn get_float(&self, float: f32) -> Option<&u16> {
        self.0
            .get(&PhoronConstantPoolKind::Float(float.to_be_bytes()))
    }

    /// Retrieve the index in the Constant Pool, if present, of the
    /// given long.
    pub fn get_long(&self, long: i64) -> Option<&u16> {
        self.0
            .get(&PhoronConstantPoolKind::Long(long.to_be_bytes()))
    }

    /// Retrieve the index in the Constant Pool, if present, of the
    /// given integer.
    pub fn get_integer(&self, int: i32) -> Option<&u16> {
        self.0
            .get(&PhoronConstantPoolKind::Integer(int.to_be_bytes()))
    }

    /// Retrueve the index in the Constant Pool, if present, of the
    /// given String.
    pub fn get_string(&self, string: &str) -> Option<&u16> {
        self.get_name(string).and_then(|string_index| {
            self.0.get(&PhoronConstantPoolKind::String {
                string_index: *string_index,
            })
        })
    }

    /// Retrieve the index in the Constant Pool, if present, of the
    /// given name.
    pub fn get_name(&self, name: &str) -> Option<&u16> {
        self.0.get(&PhoronConstantPoolKind::Utf8(name.to_string()))
    }

    /// Retrieve the index in the Constant Pool, if present,  of the given class.
    pub fn get_class(&self, class_name: &str) -> Option<&u16> {
        self.get_name(class_name).and_then(|name_index| {
            self.0.get(&PhoronConstantPoolKind::Class {
                name_index: *name_index,
            })
        })
    }

    /// Retrieve the index in the Constant Pool, if present, of the given NamdAndType.
    pub fn get_name_and_type(&self, name: &str, descriptor: &str) -> Option<&u16> {
        self.get_name(name).and_then(|name_index| {
            self.get_name(descriptor).and_then(|descriptor_index| {
                self.0.get(&PhoronConstantPoolKind::NameAndType {
                    name_index: *name_index,
                    descriptor_index: *descriptor_index,
                })
            })
        })
    }

    /// Retrieve the index in the Constant Pool, if present, of the given Fieldref.
    pub fn get_fieldref(
        &self,
        class_name: &str,
        field_name: &str,
        field_descriptor: &str,
    ) -> Option<&u16> {
        self.get_class(class_name).and_then(|class_index| {
            self.get_name_and_type(field_name, field_descriptor)
                .and_then(|name_and_type_index| {
                    self.0.get(&PhoronConstantPoolKind::Fieldref {
                        class_index: *class_index,
                        name_and_type_index: *name_and_type_index,
                    })
                })
        })
    }

    /// Retrieve the index in the Constant Pool, if present, of the given Methodref.
    pub fn get_methodref(
        &self,
        class_name: &str,
        method_name: &str,
        method_descriptor: &str,
    ) -> Option<&u16> {
        self.get_class(class_name).and_then(|class_index| {
            self.get_name_and_type(method_name, method_descriptor)
                .and_then(|name_and_type_index| {
                    self.0.get(&PhoronConstantPoolKind::Methodref {
                        class_index: *class_index,
                        name_and_type_index: *name_and_type_index,
                    })
                })
        })
    }
}
