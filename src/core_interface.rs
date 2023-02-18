use phoron_core::model::{
    attributes::AttributeInfo, constant_pool::types::CpInfo, ClassFile, FieldInfo, MethodInfo,
};

pub struct CoreInterface {
    class_file: ClassFile,
}

impl CoreInterface {
    pub fn new() -> Self {
        CoreInterface {
            class_file: ClassFile::default(),
        }
    }

    pub fn write_headers(&mut self, magic: u32, minor_version: u16, major_version: u16) {
        self.class_file.magic = magic;
        self.class_file.minor_version = minor_version;
        self.class_file.major_version = major_version;
    }

    pub fn write_constant_pool(
        &mut self,
        constant_pool_count: u16,
        constant_pool: Vec<Option<CpInfo>>,
    ) {
        self.class_file.constant_pool_count = constant_pool_count;
        self.class_file.constant_pool = constant_pool;
    }

    pub fn write_access_flags(&mut self, af: u16) {
        self.class_file.access_flags = af;
    }

    pub fn write_this_class(&mut self, this_class: u16) {
        self.class_file.this_class = this_class;
    }

    pub fn write_super_class(&mut self, super_class: u16) {
        self.class_file.super_class = super_class;
    }

    pub fn write_interfaces(&mut self, interfaces_count: u16, interfaces: Vec<u16>) {
        self.class_file.interfaces_count = interfaces_count;
        self.class_file.interfaces = interfaces;
    }

    pub fn write_fields(&mut self, fields_count: u16, fields: Vec<FieldInfo>) {
        self.class_file.fields_count = fields_count;
        self.class_file.fields = fields;
    }

    pub fn write_methods(&mut self, methods_count: u16, methods: Vec<MethodInfo>) {
        self.class_file.methods_count = methods_count;
        self.class_file.methods = methods;
    }

    pub fn write_attributes(&mut self, attributes_count: u16, attributes: Vec<AttributeInfo>) {
        self.class_file.attributes_count = attributes_count;
        self.class_file.attributes = attributes;
    }
}

#[cfg(test)]
mod tests {}
