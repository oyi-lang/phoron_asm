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


## Error Reporting

Since `Phoron` is mroe strongly (read strictly) typed than Jasmin, it behooves us to provide better error reporting at all stages than what is currently available in Jasmin.  To that end, 
this section is divided into three sub-sections - the first describes the actual implementation details for the error-reporting mechanism, the second describes how the error recovery mechanism 
which is used, *mutatis mutandis*,  at each stage of processing to provide decent error messages.  and the third specifies the template for diagnostic reports.  


### Implementation Details

When a source file is read in, it is first converted into a `SourceFile` entity:

```
pub struct SourceFile {
  name: String, // the name of the source file
  src: String, // the actual raw sourc code
  beginnings: Vec<Po>> // absolute byte offsets for the beginning of each line
}

```

This is then fed into the lexer, which produces tokens with spans:

```
pub struct Spanned<T> {
  node: T,
  span: Span
}
```

```
pub struct Span {
  low: Pos,
  high: Pos
}
```

```
pub struct Pos(u32);
```

where each `u32` value is an absolute byte offset from the beginning of the source code.

```
pub struct Token {
  kind: TokenKind,
}
```

```
pub fn lex(&self) -> LexerResult<Spanned<Token>> 
```

During parsing, each AST node is likewise decorated with a span for the entire non-terminal AST:


```
pub fn parse(&mut self) -> ParserResult<Spanned<PhoronProgram>> 
```

```
pub struct PhoronProgram {
  pub header: Spanned<PhoronHeader>,
  pub body: Spanned<PhoronBody>,
}
```

and so on.


### Error Recovery 

If we are in a function, `parse_x`, then we skip tokens until an element in `FIRST(x)` (meaning a token that can start the phrase `x`) is encountered. If encountered, parsing continues from this stage.
If not encountered, then we look for a token in `FOLLOW(x)` (meaning a token that marks the end of the phrase `x`). If encountered, then we skip parsing this phrase, report the error, and continue parsing the
next phrase. If not encountered, then we report the error, and stop parsing since error recovery is not meaningful at this stage.


### Diagnostic Template

`Phoron` is line-based, and so the greatest possible span will correspond to a single line. However, this template is more general purpose that could potentially work with multiple-lines (by merging spans), and the template shown below
describes how a single line might be reported along with the diagnostic information, but, again, a similar mechanism could be used to reporting multiple lines.

  ```
    <File:Line:Col>: <Error Text>
              |
   <LINE NUM> | <SOURCE CODE LINE> generated from the span
              |          ^^^^   

  ```

  So we need the followoing functionality in order to generate and report as per this template:

    - `merge_span` - this will take another span, and "merger" that with the current span. This is used to convert the token spans into spans for the AST nodes. The algorithm is simple:

          new_span.low = min(curr_span.low, other_span.low), and
          new_span.high = max(curr_span.higg, other_span.high)

      So, essentially merging intervals.

    - `span_to_location` : this will take a `Span` and generate the <File, Line number, Column number> information. This will make use of the `beginnings` field of the `SourceFile` for faster lookups using binary search.

      So, for instance, we have the following situation:

      ```
        Beginnings         Line Numbers
          0                  0
         13                  1
         21                  2
         31                  3
         34                  4

      ```

      Then, given a span like so:

      ```
      Span {
        low: Pos(15),
        high: Pos(18),
      }
      ```

      Then `File` will be the source file name, `Line ` will be 2 (1 + pos using Binary Search), and `Column` will be 3 (low - beginning + 1).

    - `span_to_source` - this will take a `Span` and generate the region of source code as a string (a single line in the case of `Phoron`) in the format specified above.

    - `emit_diagnostic` - this will generate the pretty-printed (and colour-coded) diagnostic report in the form of the template above.
