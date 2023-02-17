The implementation of `phoron_asm` will proceed in three phases:

  Phase 1 - Compatibility with [Jasmin](https://jasmin.sourceforge.net/).

  Phase 2 - Support for the latest JVM features (JVM 19).

  Phase 3 - Suport for disassembly.

The core features supported by Phoron are those of Jasmin. On top of that, to support newer JVM features as well as in the shape of improvements, some
additional features have been/will be added.

## Grammar for Phoron syntax

```
    phoron_assembly_file ::=
        '.class' [ <access> ] <name> <break>
        '.super' <name> <break>
        [ <fields> ]
        [ <methods> ]


    <fields> ::= <field> [ <field> ... ] 

    <field> ::=
        '.field' <access> <name> <signature> [ = <default> ] <break>

    <default> ::=  <int> | <quoted_string> | <float> 

    <methods> ::= <method> [ <method> ... ] 

    <method> ::=
        '.method' <access> <name> <break>
            [ <statements> ]
        '.end' 'method' <break>

    <statements> ::= <statement> [ <statement> ... ] 

    <statement> ::=
        <directive> <break>
        |
        <instruction> <break>
        |
        <label> ':' <break>

    <directive> ::=
        '.limit' 'stack' <val>
        |
        '.limit' 'locals' <val>
        |
        '.throws' <classname>
        |
        '.catch' <classname> 'from' <label1> 'to' <label2> 'using' <label3>

    <instruction> ::= <simple_instruction> | <complex_instruction> 

    <simple_instruction> ::=
        <insn>
        |
        <insn> <int> <int>
        |
        <insn> <int>
        |
        <insn> <num>
        |
        <insn> <word>
        |
        <insn> <word> <int>
        |
        <insn> <word> <word>
        |
        <insn> <quoted_string>

    <complex_instruction> ::=
        <lookupswitch>
        |
        <tableswitch>

    <lookupswitch> ::=
        lookupswitch <nl>
            <int> : <label> <nl>
            <int> : <label> <nl>
            ...
            default : <label>

    <tableswitch> ::=
        tableswitch <low>  <nl>
            <label> <nl>
            <label> <nl>
            ...
            default : <label>

    <access> ::= <access_item> [ <access_item> ... ] 

    <access_item> ::=
        'public' | 'private' | 'protected' | 'static' | 'final' |
        'synchronized' | 'volatile' | 'transient' | 'native' |
        'interface' | 'abstract' 

```

Source: https://github.com/davidar/jasmin/blob/master/docs/syntax.bnf

For a testable grammar implementation (in PEG format), please refer to the [grammar document](../grammar/phoron.peg).


## Statements

Phoron source files consist of newline separated statements. There are three types of statements, each described in its own section next.

### Directives

Directive statements are for metadata used by Phoron during the process of assembly. The following directives are suported:

  File structure:

  `.source` - Optional directive, specifies the [SourceFile](https://docs.oracle.com/javase/specs/jvms/se19/html/jvms-4.html#jvms-4.7.10) attribute in the JVM spec. If no sourcefile 
  is provided, then the name of the Phoron assembly file itself is taken as the value for this attribute.

  `.class` - Grammar: `.class <access-spec> <classname>` specifies the fully qualified name of the class being defined. 
  Eg: ```.super public foo/bar/MyClass```

  `.super` - Grammar: `.super <access-spec> <classname>` specifies the fully qualified name of the superclass for the class being defined. 
  Eg: ```.super public java/lang/Object```

  `.interface` - Same as `.class` except that it implies that the entity being defined is an interface, not a class. 
  Eg: ```.interface public Foo```

  `.implements` - Grammar: `.implements <interface-name>` specifies the interfaces implemented by the class being defined.
  Eg:

      ```
      .class public foo/bar/Baz
      .super public quux/foobar/QuuxFoo
      .implements IOne
      .implements ITwo
      .implements IThree
      ```

  `.version` - Grammar: `.version <major>.<minor>` supplies a JVM bytecode version in the format `Major.minor`. For example: `45.2` or `65.0`. Default is `45.3`.

  `.end` - Grammar: `.end method` marks the end of a method.


  Field definitions:

  `.field` - Grammar: `.field <access-spec> <field-name> <descriptor> [=<value>]` defines a field using the grammar, where
    `access-spec` can be zero or more of - public, protected, private, static, final, transient, volatile. Eg:

    ```
      .field public foo I
      .field public static final PI = 3.14
      
    ```

  Method definitions:

  `.method` - Grammar: `.method <access-spec> <method-spec> <statements> .end method` defines a method for the class being defined, where
    `access-spec` is zero or more of the following - public, protected, private, abstract, static, final, synchronized, native. Eg:

    ```
      .method foo ()V
        return
      .end method
    ```

    Note: Method definitions cannot be nested.

  Method Directives:

  These are directives that can only be used within a method definition.

  `.limit` - set maximum values used by the stack and for locals. 

      Eg: .limit stack 1, .limit locals 10


  `.line` - Grammar: `.line <integer>` is used to tag the subsequent instructions with a line numeber. Uses the [LineNumberTableAttribute](https://docs.oracle.com/javase/specs/jvms/se19/html/jvms-4.html#jvms-4.7.12)
  attribute. Eg:

    ```
      .method foo() V
      .line 5
        bipush 10
        istore_2
      .line 6
    ```

  `.var` -  Grammar: `.var <var-number> is <name>  <descriptor> from <label1> to <label2>` defines the name, type descritor, and scope of a local variable.
  Uses the [LocalVariableTableAttribute](https://docs.oracle.com/javase/specs/jvms/se19/html/jvms-4.html#jvms-4.7.13) attribute.
  Eg:
    ```
      .method foo()V
        .limit locals 1
        .var 0 is Count I from Label1 to Label2

      Label1:
        bipush 10
        istore_0
      Label2:
        return
      .end method
    ```

  `.throws` - Grammar: `.throws <classname>` indicates that this method can throw an exception of type `<classname>`. Eg: `.throws java/io/IOException`.

  `.catch` - Grammar: `.catch <classname> from <label1> to <label2> using <label3>` appends an entry to the end of the exceptions table for this method. Eg:
    ```
      .catch java/io/IOException from L1 to L2 using IO_Handler
    ```

    Note: If the <classname> is "all", then all exceptions of any class are caught by the handler. 

    Abstract Methods:

    Eg:
      ```
        .method abstract myAbstract()V
        .end method
      ```

      ```
        .method abstract anotherAbstract()V
          .throws java/io/IOException
        .end method
      ```


#### Instructions

An instruction statement consists of an instruction name, zero or more arguments separated by spaces and a newline. These are the Phoron repsentations of the actual
JVM opcodes.

#### Labels

A Phoron label consists of a name followed by colon(`:`) and a newline. These are used for marking positions in the Phoron source file for use in conjunction with
different instructions such as jump instructions.

Labels cannot start with a numeric digit, contain any of the following special characters: `= : . " -`, or be any instruction name.

Labels can only be used within method definitions, and are local to the methods.

### Comments

Phoron comments are single-line comments that begin with a semi-colon (`;`) and mark the rest of the line as a comment.

### Numbers and Strings

Only simple decimal and integer numeric formats are supported. Floats in scientific or exponent format are not supported. Characters code and octals are not supported.

Basic quoted strings are supported. The full-range of escape sequences (apart from '\n' and '\t' are not supported).


## Class Names

Class names are written in Java class file convention. 

### Type Descriptors

Type information follows the Java class file convention.

## Methods

Method names are specified using a single token. For example:

```
  java/io/PrintStream/println(Ljava/lang/String;)V
```

### Fields

Field names are specified using two tokens - the name and class of the field, and the type descriptor. For example:

```
  getstatic mypackage/MyClass/my_field_name Ljava/lang/MyFieldType;
```

### File structure

Phoron assembly files start off with information on the class being defined in the file (using directives), like so:

```
  .source <source-file>
  .class <access-spec> <class-name>
  .super <class-name>
```

For example:

```
  .source MyClass.pho
  .class public MyClass
  .super java/lang/Object
```

Source:  http://web.mit.edu/javadev/packages/jasmin/doc/


## Versioning 

Versioning of features comparible with different JVM versions is the onus of the client using `phoron_asm`. As a general rules, versioning applies thusly for the
two main functionalities of `phoron_asm`:

  * Assembler - converting Phoron Assembly Format (`.pho`) files to JVM bytecode - if a version string is not provided, then the default version will be
                45.3 (Major 45, Minor 3). If a n explicit version is passed in, then that will be used as the JVM bytecode version instead.

  * Disassembler - converting JVM bytecode into Phoron Assembly Format (`.pho`) files. Versioning information will be picked up from the class file itself.
