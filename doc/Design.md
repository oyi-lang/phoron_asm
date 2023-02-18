# Phoron Design and Implementation

## Overall Design

The Phoron (or Jasmin) syntax is very much line-oriented in the spirit of most assemblers. This makes parsing and code generation relatively straightforward.
The complexity lies in the way the [Constant Pool]() is constructed, how the [Attributes]() are generated, as well as handling the offsets within the Constant Pool
as well as lables within specific [Instructions]().

This project uses the [phoron_core](https://github.com/oyi-lang/phoron_core) crate to perform the low-level serialisation of the type-checked and decorated AST into JVM bytecode.

References:
  1. [The JVMww 19 Reference](https://docs.oracle.com/javase/specs/jvms/se19/html/jvms-4.html)
  2. [The Jasmin User Guide](https://jasmin.sourceforge.net/guide.html).

### Algorithm

  - Read the Phoron source file, lex it, and then parse it into an AST.
  - Type-check the AST and generate a decorated AST.
  - Use the type information in the decorated AST to generate the Constant Pool (CP) (as described in the next section).
  - Fix all the indices used in instructions and labels, into the CP, using the generated CP representation in the previous step.
  - Construct the `ClassFile` structure as requited by [phoron_core](https://github.com/oyi-lang/phoron_core).
  - Use `phoron_core` to serialise to JVM bytecode.`

## Constant Pool

The CP is represented in Phoron as:
  - A hashmap of key-value pairs where each key is a String, and each value is a (usize, CpInfo) pair, where
    `CpInfo` is the `phoron_core` type representing CP entries.
  - The String key represents the name of the CP entity.
  - The `usize` component of the value represents the CP index proper, and the `CpInfo` represents the 
  - This hashmap is then used to construct a vector of `CpInfo` objects according to the scheme accepted by `phoron_core`.

CP entries are generated from the following sources:
  - Instructions such as `ldc`, `ldc_w`, `ldc2_w` et al, which explicitly index into literal constants stored in the CP.
  - Class, Fieldref, Methodref, InterfaceMethoref, and NameAndType definitions in the Phoron file.

## Attributes

How Phoron directives map to the JVM model:

  .version populates the `major_version` and `minor_version`fields in the `ClassFile` object.

  .source generates a `SourceFile` attribute.  If this attribute is not explicitly specified, the name of the Phoron file is taken as the value for this attribute.

  .class is simply associated with the `ClassFile` object being generated during the current run of Phoron. Its index in 
  the CP is also used for the `this_class` field in `ClassFile`. The <access-spec> is used to set various [access flags](https://docs.oracle.com/javase/specs/jvms/se19/html/jvms-4.html#jvms-4.1-200-E.1)
  as valid for classes.

  .super is used to populate the `super_class` attribute (as an index into the CP) of the `ClassFile`. The <access-spec> is 
  used to set various [access flags](https://docs.oracle.com/javase/specs/jvms/se19/html/jvms-4.html#jvms-4.1-200-E.1) as valid for classes.

  .interface sets the [ACC_INTERFACE}() access flag in the `ClassFile`.

  .implements popiulates the `interfaces` field of `ClassFile`.

  .field sets the `fields` fields of `ClassFile`. Each entry in this vector is a `FieldInfo` object.

  .method populates the `methods` field of `ClassFile`. Each entry in this vector is a `MethodInfo` object.

  .limit `.limit stack N` and `CodeAttribute` populate the `max_stack` and `max_locals` fields of the `CodeAttribute` of the relevant method.

  .var generates a `LocalVariable` entry in the `LocalVariableTable` attribute.

  .line generates a `LineNumber` entry in the `LineNumberTable` attribute.

  .throws populates the `Exceptions` attribute of the relevant method.

  .catch creates entries in the `exception_table` of the `CodeAttribute` of the relevant method.


## Instructions


## Phoron Modules

### Lexer

### Parser

### Type-Checker

### Codegen

