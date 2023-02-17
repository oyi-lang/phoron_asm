#[cfg(test)]
mod test {
    use phoron_core::{
        model::{
            attributes::AttributeInfo::{self, *},
            constant_pool::types::CpInfo::{self, *},
            *,
        },
        rw::writer::Writer,
        serializer::Serializer,
    };

    fn write_headers(cf: &mut ClassFile, magic: u32, minor_version: u16, major_version: u16) {
        cf.magic = 0xcafebabe;
        cf.minor_version = 3;
        cf.major_version = 45;
    }

    fn write_constant_pool(
        cf: &mut ClassFile,
        constant_pool_count: u16,
        constant_pool: Vec<Option<CpInfo>>,
    ) {
        cf.constant_pool_count = constant_pool_count;
        cf.constant_pool = constant_pool;
    }

    fn write_access_flags(cf: &mut ClassFile, af: u16) {
        cf.access_flags = af;
    }

    fn write_this_class(cf: &mut ClassFile, this_class: u16) {
        cf.this_class = this_class;
    }

    fn write_super_class(cf: &mut ClassFile, super_class: u16) {
        cf.super_class = super_class;
    }

    fn write_interfaces(cf: &mut ClassFile, interfaces_count: u16, interfaces: Vec<u16>) {
        cf.interfaces_count = interfaces_count;
        cf.interfaces = interfaces;
    }

    fn write_fields(cf: &mut ClassFile, fields_count: u16, fields: Vec<FieldInfo>) {
        cf.fields_count = fields_count;
        cf.fields = fields;
    }

    fn write_methods(cf: &mut ClassFile, methods_count: u16, methods: Vec<MethodInfo>) {
        cf.methods_count = methods_count;
        cf.methods = methods;
    }

    fn write_attributes(cf: &mut ClassFile, attributes_count: u16, attributes: Vec<AttributeInfo>) {
        cf.attributes_count = attributes_count;
        cf.attributes = attributes;
    }

    #[test]
    fn test_serialize_minimal() {
        let expected_bytes = [
            0xca, 0xfe, 0xba, 0xbe, 0x00, 0x03, 0x00, 0x2d, 0x00, 0x0e, 0x0a, 0x00, 0x0d, 0x00,
            0x07, 0x01, 0x00, 0x10, 0x6a, 0x61, 0x76, 0x61, 0x2f, 0x6c, 0x61, 0x6e, 0x67, 0x2f,
            0x4f, 0x62, 0x6a, 0x65, 0x63, 0x74, 0x01, 0x00, 0x0a, 0x53, 0x6f, 0x75, 0x72, 0x63,
            0x65, 0x46, 0x69, 0x6c, 0x65, 0x01, 0x00, 0x06, 0x3c, 0x69, 0x6e, 0x69, 0x74, 0x3e,
            0x01, 0x00, 0x04, 0x6d, 0x61, 0x69, 0x6e, 0x01, 0x00, 0x07, 0x4d, 0x69, 0x6e, 0x69,
            0x6d, 0x61, 0x6c, 0x0c, 0x00, 0x04, 0x00, 0x0b, 0x01, 0x00, 0x04, 0x43, 0x6f, 0x64,
            0x65, 0x01, 0x00, 0x0c, 0x4d, 0x69, 0x6e, 0x69, 0x6d, 0x61, 0x6c, 0x2e, 0x6a, 0x61,
            0x76, 0x61, 0x01, 0x00, 0x16, 0x28, 0x5b, 0x4c, 0x6a, 0x61, 0x76, 0x61, 0x2f, 0x6c,
            0x61, 0x6e, 0x67, 0x2f, 0x53, 0x74, 0x72, 0x69, 0x6e, 0x67, 0x3b, 0x29, 0x56, 0x01,
            0x00, 0x03, 0x28, 0x29, 0x56, 0x07, 0x00, 0x06, 0x07, 0x00, 0x02, 0x00, 0x21, 0x00,
            0x0c, 0x00, 0x0d, 0x00, 0x00, 0x00, 0x00, 0x00, 0x02, 0x00, 0x01, 0x00, 0x04, 0x00,
            0x0b, 0x00, 0x01, 0x00, 0x08, 0x00, 0x00, 0x00, 0x11, 0x00, 0x01, 0x00, 0x01, 0x00,
            0x00, 0x00, 0x05, 0x2a, 0xb7, 0x00, 0x01, 0xb1, 0x00, 0x00, 0x00, 0x00, 0x00, 0x09,
            0x00, 0x05, 0x00, 0x0a, 0x00, 0x01, 0x00, 0x08, 0x00, 0x00, 0x00, 0x0d, 0x00, 0x01,
            0x00, 0x01, 0x00, 0x00, 0x00, 0x01, 0xb1, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00,
            0x03, 0x00, 0x00, 0x00, 0x02, 0x00, 0x09,
        ];

        let mut classfile = ClassFile::default();
        write_headers(&mut classfile, 0xcafebabe, 3, 45);

        write_constant_pool(
            &mut classfile,
            14,
            vec![
                None,
                Some(ConstantMethodrefInfo {
                    tag: 10,
                    class_index: 13,
                    name_and_type_index: 7,
                }),
                Some(ConstantUtf8Info {
                    tag: 1,
                    length: 16,
                    bytes: vec![
                        106, 97, 118, 97, 47, 108, 97, 110, 103, 47, 79, 98, 106, 101, 99, 116,
                    ],
                }),
                Some(ConstantUtf8Info {
                    tag: 1,
                    length: 10,
                    bytes: vec![83, 111, 117, 114, 99, 101, 70, 105, 108, 101],
                }),
                Some(ConstantUtf8Info {
                    tag: 1,
                    length: 6,
                    bytes: vec![60, 105, 110, 105, 116, 62],
                }),
                Some(ConstantUtf8Info {
                    tag: 1,
                    length: 4,
                    bytes: vec![109, 97, 105, 110],
                }),
                Some(ConstantUtf8Info {
                    tag: 1,
                    length: 7,
                    bytes: vec![77, 105, 110, 105, 109, 97, 108],
                }),
                Some(ConstantNameAndTypeInfo {
                    tag: 12,
                    name_index: 4,
                    descriptor_index: 11,
                }),
                Some(ConstantUtf8Info {
                    tag: 1,
                    length: 4,
                    bytes: vec![67, 111, 100, 101],
                }),
                Some(ConstantUtf8Info {
                    tag: 1,
                    length: 12,
                    bytes: vec![77, 105, 110, 105, 109, 97, 108, 46, 106, 97, 118, 97],
                }),
                Some(ConstantUtf8Info {
                    tag: 1,
                    length: 22,
                    bytes: vec![
                        40, 91, 76, 106, 97, 118, 97, 47, 108, 97, 110, 103, 47, 83, 116, 114, 105,
                        110, 103, 59, 41, 86,
                    ],
                }),
                Some(ConstantUtf8Info {
                    tag: 1,
                    length: 3,
                    bytes: vec![40, 41, 86],
                }),
                Some(ConstantClassInfo {
                    tag: 7,
                    name_index: 6,
                }),
                Some(ConstantClassInfo {
                    tag: 7,
                    name_index: 2,
                }),
            ],
        );

        write_access_flags(&mut classfile, 33);
        write_this_class(&mut classfile, 12);
        write_super_class(&mut classfile, 12);
        write_interfaces(&mut classfile, 0, Vec::new());
        write_fields(&mut classfile, 0, Vec::new());
        write_methods(
            &mut classfile,
            2,
            vec![
                MethodInfo {
                    access_flags: 1,
                    name_index: 4,
                    descriptor_index: 11,
                    attributes_count: 1,
                    attributes: vec![Code {
                        attribute_name_index: 8,
                        attribute_length: 17,
                        max_stack: 1,
                        max_locals: 1,
                        code_length: 5,
                        code: vec![42, 183, 0, 1, 177],
                        exception_table_length: 0,
                        exception_table: vec![],
                        code_attributes_count: 0,
                        code_attributes: vec![],
                    }],
                },
                MethodInfo {
                    access_flags: 9,
                    name_index: 5,
                    descriptor_index: 10,
                    attributes_count: 1,
                    attributes: vec![Code {
                        attribute_name_index: 8,
                        attribute_length: 13,
                        max_stack: 1,
                        max_locals: 1,
                        code_length: 1,
                        code: vec![177],
                        exception_table_length: 0,
                        exception_table: vec![],
                        code_attributes_count: 0,
                        code_attributes: vec![],
                    }],
                },
            ],
        );

        write_attributes(
            &mut classfile,
            1,
            vec![SourceFile {
                attribute_name_index: 3,
                attribute_length: 2,
                sourcefile_index: 9,
            }],
        );

        let mut bytes: Vec<u8> = Vec::new();
        let mut serializer = Serializer::new(Writer::new(&mut bytes));
        serializer.serialize(&classfile).unwrap();
        assert_eq!(expected_bytes, &bytes[..]);
    }
}
