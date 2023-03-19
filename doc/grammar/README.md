## The Grammar

The grammar specification for `Phoron` is given in `phoron.peg`. This is the formal specification of the `Phoron` grammar, and is used to test, maintain, and update
`Phoron`'s grammar.


## How to run and test the grammar?

The grammar is written in [PEG](https://bford.info/pub/lang/peg/) format compatible with the [peg tool](https://www.piumarta.com/software/peg/peg.1.html) (which needs to be installed locally).

To test the grammar, run the following command at the command line:

```
  $ chmod a+x test_grammar.sh
  $ ./test_grammar.sh
```

or 

```
% sh test_grammar.sh
```

If everything passes, then the grammar is working as expected. You can add more test files (look at [AllInOne.pho](../../sanples/AllInOne.pho) for all the features supported by Phoron, and to the 
[specification](../Specification.md) for the full specification of Phoron.

