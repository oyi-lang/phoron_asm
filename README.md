# phoron_asm

This project defines `Phoron`, a [Jasmin](https://jasmin.sourceforge.net/)-compatible assembler for the [JVM Instruction Set.](https://docs.oracle.com/javase/specs/jvms/se19/html/jvms-6.html).

For the specification, please refer to the [specification document](doc/Specification.md).

For the testable grammar for `Phoron`, please refer to the [grammar](doc/grammar/README.md).

For the design, please refer to the [Design doc](doc/Design.md).

## Build


```
  $ cargo build --release 

```

## Sample Run

For the sample source file [HelloWorld.pho](samples/HelloWorld.pho):

```
.class public HelloWorld
.super java/lang/Object

.method public <init>()V
  aload_0 
  invokespecial java/lang/Object/<init>()V ; super ()
  return
.end method

.method public static main([Ljava/lang/String;)V
  .limit stack 2
  getstatic java/lang/System/out Ljava/io/PrintStream;
  ldc "Hello, world"
  invokevirtual java/io/PrintStream/println(Ljava/lang/String;)V
  return
.end method
```

We generate the `class` file:

```
  $ cargo run --release -- -f samples/HelloWorld.pho
```

Sanity-check to ensure that the generated `class` file is valid:

```
$ javap -v HelloWorld.class
Classfile /Users/z0ltan/dev/oyi-lang/phoron_asm/samples/HelloWorld.class
  Last modified 19-Mar-2023; size 389 bytes
  SHA-256 checksum 533a66c051831cba84a32b20d38c4bb20d68b78aabc137d7c7fb3cc864ff8bf9
  Compiled from "./samples/HelloWorld.pho"
public class HelloWorld
  minor version: 3
  major version: 45
  flags: (0x0021) ACC_PUBLIC, ACC_SUPER
  this_class: #4                          // HelloWorld
  super_class: #6                         // java/lang/Object
  interfaces: 0, fields: 0, methods: 2, attributes: 1
Constant pool:
   #1 = Utf8               SourceFile
   #2 = Utf8               ./samples/HelloWorld.pho
   #3 = Utf8               HelloWorld
   #4 = Class              #3             // HelloWorld
   #5 = Utf8               java/lang/Object
   #6 = Class              #5             // java/lang/Object
   #7 = Utf8               <init>
   #8 = Utf8               ()V
   #9 = Utf8               Code
  #10 = NameAndType        #7:#8          // "<init>":()V
  #11 = Methodref          #6.#10         // java/lang/Object."<init>":()V
  #12 = Utf8               main
  #13 = Utf8               ([Ljava/lang/String;)V
  #14 = Utf8               java/lang/System
  #15 = Class              #14            // java/lang/System
  #16 = Utf8               out
  #17 = Utf8               Ljava/io/PrintStream;
  #18 = NameAndType        #16:#17        // out:Ljava/io/PrintStream;
  #19 = Fieldref           #15.#18        // java/lang/System.out:Ljava/io/PrintStream;
  #20 = Utf8               Hello, world
  #21 = String             #20            // Hello, world
  #22 = Utf8               java/io/PrintStream
  #23 = Class              #22            // java/io/PrintStream
  #24 = Utf8               println
  #25 = Utf8               (Ljava/lang/String;)V
  #26 = NameAndType        #24:#25        // println:(Ljava/lang/String;)V
  #27 = Methodref          #23.#26        // java/io/PrintStream.println:(Ljava/lang/String;)V
{
  public HelloWorld();
    descriptor: ()V
    flags: (0x0001) ACC_PUBLIC
    Code:
      stack=1, locals=1, args_size=1
         0: aload_0
         1: invokespecial #11                 // Method java/lang/Object."<init>":()V
         4: return

  public static void main(java.lang.String[]);
    descriptor: ([Ljava/lang/String;)V
    flags: (0x0009) ACC_PUBLIC, ACC_STATIC
    Code:
      stack=2, locals=1, args_size=1
         0: getstatic     #19                 // Field java/lang/System.out:Ljava/io/PrintStream;
         3: ldc           #21                 // String Hello, world
         5: invokevirtual #27                 // Method java/io/PrintStream.println:(Ljava/lang/String;)V
         8: return
}
SourceFile: "./samples/HelloWorld.pho"
```

and then we can test it out by running the `class` file:

```
$ java -cp . HelloWorld
Hello, world

```

