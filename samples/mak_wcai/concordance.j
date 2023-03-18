.class public concordance
.super java/lang/Object

.field private static _runTimer LRunTimer;
.field private static _standardIn LPascalTextIn;

.field private static entry I
.field private static nextentry I
.field private static table [Ljava/util/HashMap;
.field private static tablefull Z

.method public <init>()V

	aload_0
	invokenonvirtual	java/lang/Object/<init>()V
	return

.limit locals 1
.limit stack 1
.end method

.method private static isletter(C)Z

.var 0 is ch C
.var 1 is isletter Z


.line 26
	iload_0
	bipush	97
	if_icmpge	L001
	iconst_0
	goto	L002
L001:
	iconst_1
L002:
	iload_0
	bipush	122
	if_icmple	L003
	iconst_0
	goto	L004
L003:
	iconst_1
L004:
	iand
	iload_0
	bipush	65
	if_icmpge	L005
	iconst_0
	goto	L006
L005:
	iconst_1
L006:
	iload_0
	bipush	90
	if_icmple	L007
	iconst_0
	goto	L008
L007:
	iconst_1
L008:
	iand
	ior
	istore_1

	iload_1
	ireturn

.limit locals 2
.limit stack 4
.end method

.method private static readword(Ljava/lang/StringBuilder;)V

.var 0 is buffer Ljava/lang/StringBuilder;
.var 2 is ch C
.var 1 is charcount I


.line 40
	bipush	32
	istore_2
.line 42
	getstatic	concordance/_standardIn LPascalTextIn;
	invokevirtual	PascalTextIn.atEof()Z
	iconst_1
	ixor
	ifeq	L009
.line 43
L010:
.line 44
	getstatic	concordance/_standardIn LPascalTextIn;
	invokevirtual	PascalTextIn.readChar()C
	istore_2
	getstatic	concordance/_standardIn LPascalTextIn;
	invokevirtual	PascalTextIn.atEof()Z
	iload_2
	invokestatic	concordance/isletter(C)Z
	ior
	ifne	L011
	goto	L010
L011:
L009:
.line 47
	getstatic	concordance/_standardIn LPascalTextIn;
	invokevirtual	PascalTextIn.atEof()Z
	iconst_1
	ixor
	ifeq	L012
.line 48
	iconst_0
	istore_1
.line 49
L013:
	iload_2
	invokestatic	concordance/isletter(C)Z
	iconst_1
	ixor
	ifne	L014
.line 50
	iload_1
	bipush	20
	if_icmplt	L016
	iconst_0
	goto	L017
L016:
	iconst_1
L017:
	ifeq	L015
.line 51
	iload_1
	iconst_1
	iadd
	istore_1
.line 52
	aload_0
	iload_1
	iconst_1
	isub
	iload_2
	invokevirtual	java/lang/StringBuilder.setCharAt(IC)V
L015:
.line 54
	getstatic	concordance/_standardIn LPascalTextIn;
	invokevirtual	PascalTextIn.atEof()Z
	ifeq	L019
.line 54
	bipush	32
	istore_2
	goto	L018
L019:
.line 55
	getstatic	concordance/_standardIn LPascalTextIn;
	invokevirtual	PascalTextIn.readChar()C
	istore_2
L018:
	goto	L013
L014:
.line 57
	iload_1
	iconst_1
	iadd
	istore_1
.line 57
L020:
	iload_1
	bipush	20
	if_icmpgt	L022
	iconst_0
	goto	L023
L022:
	iconst_1
L023:
	ifne	L021
.line 58
	aload_0
	iload_1
	iconst_1
	isub
	bipush	32
	invokevirtual	java/lang/StringBuilder.setCharAt(IC)V
.line 57
	iload_1
	iconst_1
	iadd
	istore_1
	goto	L020
L021:
L012:

	return

.limit locals 3
.limit stack 3
.end method

.method private static printword(Ljava/lang/StringBuilder;)V

.var 0 is buffer Ljava/lang/StringBuilder;
.var 1 is charpos I


.line 72
	iconst_1
	istore_1
.line 72
L024:
	iload_1
	bipush	20
	if_icmpgt	L026
	iconst_0
	goto	L027
L026:
	iconst_1
L027:
	ifne	L025
.line 72
	getstatic	java/lang/System/out Ljava/io/PrintStream;
	ldc	"%c"
	iconst_1
	anewarray	java/lang/Object
	dup
	iconst_0
	aload_0
	iload_1
	iconst_1
	isub
	invokevirtual	java/lang/StringBuilder.charAt(I)C
	invokestatic	java/lang/Character.valueOf(C)Ljava/lang/Character;
	aastore
	invokestatic	java/lang/String/format(Ljava/lang/String;[Ljava/lang/Object;)Ljava/lang/String;
	invokevirtual	java/io/PrintStream.print(Ljava/lang/String;)V
.line 72
	iload_1
	iconst_1
	iadd
	istore_1
	goto	L024
L025:

	return

.limit locals 2
.limit stack 9
.end method

.method public static main([Ljava/lang/String;)V

	new	RunTimer
	dup
	invokenonvirtual	RunTimer/<init>()V
	putstatic	concordance/_runTimer LRunTimer;
	new	PascalTextIn
	dup
	invokenonvirtual	PascalTextIn/<init>()V
	putstatic	concordance/_standardIn LPascalTextIn;


	sipush	1000
	anewarray	java/util/HashMap

	iconst_0
	istore_1
L028:
	iload_1
	sipush	1000
	if_icmpge	L029

	dup
	iload_1

	new	java/util/HashMap
	dup
	invokenonvirtual	java/util/HashMap/<init>()V
	dup
	ldc	"count"
	iconst_0
	invokestatic	java/lang/Integer.valueOf(I)Ljava/lang/Integer;
	invokevirtual	java/util/HashMap.put(Ljava/lang/Object;Ljava/lang/Object;)Ljava/lang/Object;
	pop
	dup
	ldc	"word"
	bipush	20
	invokestatic	PaddedString.create(I)Ljava/lang/StringBuilder;
	invokevirtual	java/util/HashMap.put(Ljava/lang/Object;Ljava/lang/Object;)Ljava/lang/Object;
	pop
	aastore
	iinc	1 1
	goto	L028
L029:
	putstatic	concordance/table [Ljava/util/HashMap;

.line 76
	iconst_0
	putstatic	concordance/tablefull Z
.line 77
	iconst_1
	dup
	iconst_1
	sipush	1000
	invokestatic	RangeChecker/check(III)V
	putstatic	concordance/nextentry I
.line 78
L030:
	getstatic	concordance/_standardIn LPascalTextIn;
	invokevirtual	PascalTextIn.atEof()Z
	getstatic	concordance/tablefull Z
	ior
	iconst_1
	ixor
	iconst_1
	ixor
	ifne	L031
.line 79
	getstatic	concordance/table [Ljava/util/HashMap;
	getstatic	concordance/nextentry I
	iconst_1
	isub
	aaload
	ldc	"word"
	invokevirtual	java/util/HashMap.get(Ljava/lang/Object;)Ljava/lang/Object;
	checkcast	java/lang/StringBuilder
	invokestatic	concordance/readword(Ljava/lang/StringBuilder;)V
.line 80
	getstatic	concordance/_standardIn LPascalTextIn;
	invokevirtual	PascalTextIn.atEof()Z
	iconst_1
	ixor
	ifeq	L032
.line 81
	iconst_1
	dup
	iconst_1
	sipush	1000
	invokestatic	RangeChecker/check(III)V
	putstatic	concordance/entry I
.line 82
L033:
	getstatic	concordance/table [Ljava/util/HashMap;
	getstatic	concordance/entry I
	iconst_1
	isub
	aaload
	ldc	"word"
	invokevirtual	java/util/HashMap.get(Ljava/lang/Object;)Ljava/lang/Object;
	checkcast	java/lang/StringBuilder
	invokevirtual	java/lang/StringBuilder.toString()Ljava/lang/String;
	getstatic	concordance/table [Ljava/util/HashMap;
	getstatic	concordance/nextentry I
	iconst_1
	isub
	aaload
	ldc	"word"
	invokevirtual	java/util/HashMap.get(Ljava/lang/Object;)Ljava/lang/Object;
	checkcast	java/lang/StringBuilder
	invokevirtual	java/lang/StringBuilder.toString()Ljava/lang/String;
	invokevirtual	java/lang/String.compareTo(Ljava/lang/String;)I
	ifne	L035
	iconst_0
	goto	L036
L035:
	iconst_1
L036:
	iconst_1
	ixor
	ifne	L034
.line 83
	getstatic	concordance/entry I
	iconst_1
	iadd
	dup
	iconst_1
	sipush	1000
	invokestatic	RangeChecker/check(III)V
	putstatic	concordance/entry I
	goto	L033
L034:
.line 85
	getstatic	concordance/entry I
	getstatic	concordance/nextentry I
	if_icmplt	L038
	iconst_0
	goto	L039
L038:
	iconst_1
L039:
	ifeq	L040
.line 86
	getstatic	concordance/table [Ljava/util/HashMap;
	getstatic	concordance/entry I
	iconst_1
	isub
	aaload
	ldc	"count"
	getstatic	concordance/table [Ljava/util/HashMap;
	getstatic	concordance/entry I
	iconst_1
	isub
	aaload
	ldc	"count"
	invokevirtual	java/util/HashMap.get(Ljava/lang/Object;)Ljava/lang/Object;
	checkcast	java/lang/Integer
	invokevirtual	java/lang/Integer.intValue()I
	iconst_1
	iadd
	invokestatic	java/lang/Integer.valueOf(I)Ljava/lang/Integer;
	invokevirtual	java/util/HashMap.put(Ljava/lang/Object;Ljava/lang/Object;)Ljava/lang/Object;
	pop
	goto	L037
L040:
.line 88
	getstatic	concordance/nextentry I
	sipush	1000
	if_icmplt	L042
	iconst_0
	goto	L043
L042:
	iconst_1
L043:
	ifeq	L044
.line 89
	getstatic	concordance/nextentry I
	iconst_1
	iadd
	dup
	iconst_1
	sipush	1000
	invokestatic	RangeChecker/check(III)V
	putstatic	concordance/nextentry I
.line 90
	getstatic	concordance/table [Ljava/util/HashMap;
	getstatic	concordance/entry I
	iconst_1
	isub
	aaload
	ldc	"count"
	iconst_1
	invokestatic	java/lang/Integer.valueOf(I)Ljava/lang/Integer;
	invokevirtual	java/util/HashMap.put(Ljava/lang/Object;Ljava/lang/Object;)Ljava/lang/Object;
	pop
	goto	L041
L044:
.line 92
	iconst_1
	putstatic	concordance/tablefull Z
L041:
L037:
L032:
	goto	L030
L031:
.line 95
	getstatic	concordance/tablefull Z
	ifeq	L046
.line 96
	getstatic	java/lang/System/out Ljava/io/PrintStream;
	ldc	"The table is not large enough.\n"
	invokevirtual	java/io/PrintStream.print(Ljava/lang/String;)V
	goto	L045
L046:
.line 99
	iconst_1
	dup
	iconst_1
	sipush	1000
	invokestatic	RangeChecker/check(III)V
	putstatic	concordance/entry I
.line 99
L047:
	getstatic	concordance/entry I
	getstatic	concordance/nextentry I
	iconst_1
	isub
	if_icmpgt	L049
	iconst_0
	goto	L050
L049:
	iconst_1
L050:
	ifne	L048
.line 100
	getstatic	concordance/table [Ljava/util/HashMap;
	getstatic	concordance/entry I
	iconst_1
	isub
	aaload
	ldc	"word"
	invokevirtual	java/util/HashMap.get(Ljava/lang/Object;)Ljava/lang/Object;
	checkcast	java/lang/StringBuilder
	invokestatic	Cloner.deepClone(Ljava/lang/Object;)Ljava/lang/Object;
	checkcast	java/lang/StringBuilder
	invokestatic	concordance/printword(Ljava/lang/StringBuilder;)V
.line 101
	getstatic	java/lang/System/out Ljava/io/PrintStream;
	ldc	"%d\n"
	iconst_1
	anewarray	java/lang/Object
	dup
	iconst_0
	getstatic	concordance/table [Ljava/util/HashMap;
	getstatic	concordance/entry I
	iconst_1
	isub
	aaload
	ldc	"count"
	invokevirtual	java/util/HashMap.get(Ljava/lang/Object;)Ljava/lang/Object;
	checkcast	java/lang/Integer
	invokevirtual	java/lang/Integer.intValue()I
	invokestatic	java/lang/Integer.valueOf(I)Ljava/lang/Integer;
	aastore
	invokestatic	java/lang/String/format(Ljava/lang/String;[Ljava/lang/Object;)Ljava/lang/String;
	invokevirtual	java/io/PrintStream.print(Ljava/lang/String;)V
.line 99
	getstatic	concordance/entry I
	iconst_1
	iadd
	dup
	iconst_1
	sipush	1000
	invokestatic	RangeChecker/check(III)V
	putstatic	concordance/entry I
	goto	L047
L048:
L045:

	getstatic	concordance/_runTimer LRunTimer;
	invokevirtual	RunTimer.printElapsedTime()V

	return

.limit locals 2
.limit stack 9
.end method
