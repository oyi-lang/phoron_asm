The Phoron source files here were generated from the Pascal source files [here](https://github.com/GunterMueller/Mak_Writing_Compilers_and_Interpreters/tree/master/Mak_Writing_Compiler_3rd_Ed/Chapter18.), 
which is the source code repository for the book, ["Writing compilers and Interpreters" by Ronald Mak.](https://www.wiley.com/en-us/Writing+Compilers+and+Interpreters%3A+A+Software+Engineering+Approach%2C+3rd+Edition-p-9780470177075). 
The Jasmin files were simply renamed to the `.pho` extension. 

(If interested in generating them yourself, follow the instructons over at http://web.archive.org/web/20180825124453/http://www.apropos-logic.com/wci/).
The source files are provided here to act as a handy reference for `Phoron` (Jasmin) syntax and semantics. The secondary goal was to act as testing materials for `Phoron`.


## Build and Run

```
  $ chmod a+x run-samples.sh
  $ ./run-samples.sh

```

or

```
  # sh run-samples.sh
```

Note: This script simply ensures that the given Phoron (Jasmin) source file is parsed correctly, and that the `class` file is generated correctly, The generated `class` file depends on runtime elements outside this project's scope (these can be found in the link given above).
