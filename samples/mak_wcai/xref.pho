.class public xref
.super java/lang/Object

.field private static _runTimer LRunTimer;
.field private static _standardIn LPascalTextIn;

.field private static linenumber I
.field private static newline Z
.field private static nextnumberindex I
.field private static nextwordindex I
.field private static numbertable [Ljava/util/HashMap;
.field private static numbertablefull Z
.field private static wordtable [Ljava/util/HashMap;
.field private static wordtablefull Z

.method public <init>()V

	aload_0
	invokenonvirtual	java/lang/Object/<init>()V
	return

.limit locals 1
.limit stack 1
.end method

.method private static nextchar()C

.var 0 is ch C
.var 1 is nextchar C


.line 57
	getstatic	xref/_standardIn LPascalTextIn;
	invokevirtual	PascalTextIn.atEoln()Z
	putstatic	xref/newline Z
.line 58
	getstatic	xref/newline Z
	ifeq	L001
.line 59
	getstatic	xref/_standardIn LPascalTextIn;
	invokevirtual	PascalTextIn.nextLine()V
.line 60
	getstatic	java/lang/System/out Ljava/io/PrintStream;
	invokevirtual	java/io/PrintStream.println()V
.line 61
	getstatic	xref/linenumber I
	iconst_1
	iadd
	dup
	iconst_1
	sipush	999
	invokestatic	RangeChecker/check(III)V
	putstatic	xref/linenumber I
.line 62
	getstatic	java/lang/System/out Ljava/io/PrintStream;
	ldc	"%5d : "
	iconst_1
	anewarray	java/lang/Object
	dup
	iconst_0
	getstatic	xref/linenumber I
	invokestatic	java/lang/Integer.valueOf(I)Ljava/lang/Integer;
	aastore
	invokestatic	java/lang/String/format(Ljava/lang/String;[Ljava/lang/Object;)Ljava/lang/String;
	invokevirtual	java/io/PrintStream.print(Ljava/lang/String;)V
L001:
.line 64
	getstatic	xref/newline Z
	getstatic	xref/_standardIn LPascalTextIn;
	invokevirtual	PascalTextIn.atEof()Z
	ior
	ifeq	L003
.line 65
	bipush	32
	istore_0
	goto	L002
L003:
.line 68
	getstatic	xref/_standardIn LPascalTextIn;
	invokevirtual	PascalTextIn.readChar()C
	istore_0
.line 69
	getstatic	java/lang/System/out Ljava/io/PrintStream;
	ldc	"%c"
	iconst_1
	anewarray	java/lang/Object
	dup
	iconst_0
	iload_0
	invokestatic	java/lang/Character.valueOf(C)Ljava/lang/Character;
	aastore
	invokestatic	java/lang/String/format(Ljava/lang/String;[Ljava/lang/Object;)Ljava/lang/String;
	invokevirtual	java/io/PrintStream.print(Ljava/lang/String;)V
L002:
.line 71
	iload_0
	istore_1

	iload_1
	ireturn

.limit locals 2
.limit stack 7
.end method

.method private static isletter(C)Z

.var 0 is ch C
.var 1 is isletter Z


.line 80
	iload_0
	bipush	97
	if_icmpge	L004
	iconst_0
	goto	L005
L004:
	iconst_1
L005:
	iload_0
	bipush	122
	if_icmple	L006
	iconst_0
	goto	L007
L006:
	iconst_1
L007:
	iand
	iload_0
	bipush	65
	if_icmpge	L008
	iconst_0
	goto	L009
L008:
	iconst_1
L009:
	iload_0
	bipush	90
	if_icmple	L010
	iconst_0
	goto	L011
L010:
	iconst_1
L011:
	iand
	ior
	istore_1

	iload_1
	ireturn

.limit locals 2
.limit stack 4
.end method

.method private static readword(Ljava/lang/StringBuilder;)Z

.var 0 is buffer Ljava/lang/StringBuilder;
.var 2 is ch C
.var 1 is charcount I
.var 3 is readword Z


.line 97
	iconst_0
	istore_3
.line 98
	bipush	32
	istore_2
.line 101
	getstatic	xref/_standardIn LPascalTextIn;
	invokevirtual	PascalTextIn.atEof()Z
	iconst_1
	ixor
	ifeq	L012
.line 102
L013:
.line 103
	invokestatic	xref/nextchar()C
	istore_2
	getstatic	xref/_standardIn LPascalTextIn;
	invokevirtual	PascalTextIn.atEof()Z
	iload_2
	invokestatic	xref/isletter(C)Z
	ior
	ifne	L014
	goto	L013
L014:
L012:
.line 108
	getstatic	xref/_standardIn LPascalTextIn;
	invokevirtual	PascalTextIn.atEof()Z
	iconst_1
	ixor
	ifeq	L015
.line 109
	iconst_0
	istore_1
.line 113
L016:
	iload_2
	invokestatic	xref/isletter(C)Z
	iconst_1
	ixor
	ifne	L017
.line 114
	iload_1
	bipush	20
	if_icmplt	L019
	iconst_0
	goto	L020
L019:
	iconst_1
L020:
	ifeq	L018
.line 115
	iload_2
	bipush	65
	if_icmpge	L022
	iconst_0
	goto	L023
L022:
	iconst_1
L023:
	iload_2
	bipush	90
	if_icmple	L024
	iconst_0
	goto	L025
L024:
	iconst_1
L025:
	iand
	ifeq	L021
.line 116
	iload_2
	bipush	97
	bipush	65
	isub
	iadd
	i2c
	istore_2
L021:
.line 118
	iload_1
	iconst_1
	iadd
	istore_1
.line 119
	aload_0
	iload_1
	iconst_1
	isub
	iload_2
	invokevirtual	java/lang/StringBuilder.setCharAt(IC)V
L018:
.line 121
	invokestatic	xref/nextchar()C
	istore_2
	goto	L016
L017:
.line 125
	iload_1
	iconst_1
	iadd
	istore_1
.line 125
L026:
	iload_1
	bipush	20
	if_icmpgt	L028
	iconst_0
	goto	L029
L028:
	iconst_1
L029:
	ifne	L027
.line 126
	aload_0
	iload_1
	iconst_1
	isub
	bipush	32
	invokevirtual	java/lang/StringBuilder.setCharAt(IC)V
.line 125
	iload_1
	iconst_1
	iadd
	istore_1
	goto	L026
L027:
.line 129
	iconst_1
	istore_3
L015:

	iload_3
	ireturn

.limit locals 4
.limit stack 3
.end method

.method private static appendlinenumber(Ljava/util/HashMap;)V

.var 0 is entry Ljava/util/HashMap;


.line 140
	getstatic	xref/nextnumberindex I
	sipush	2000
	if_icmplt	L031
	iconst_0
	goto	L032
L031:
	iconst_1
L032:
	ifeq	L033
.line 146
	aload_0
	ldc	"lastnumberindex"
	invokevirtual	java/util/HashMap.get(Ljava/lang/Object;)Ljava/lang/Object;
	checkcast	java/lang/Integer
	invokevirtual	java/lang/Integer.intValue()I
	iconst_0
	if_icmpeq	L035
	iconst_0
	goto	L036
L035:
	iconst_1
L036:
	ifeq	L037
.line 147
	aload_0
	ldc	"firstnumberindex"
	getstatic	xref/nextnumberindex I
	invokestatic	java/lang/Integer.valueOf(I)Ljava/lang/Integer;
	invokevirtual	java/util/HashMap.put(Ljava/lang/Object;Ljava/lang/Object;)Ljava/lang/Object;
	pop
	goto	L034
L037:
.line 150
	getstatic	xref/numbertable [Ljava/util/HashMap;
	aload_0
	ldc	"lastnumberindex"
	invokevirtual	java/util/HashMap.get(Ljava/lang/Object;)Ljava/lang/Object;
	checkcast	java/lang/Integer
	invokevirtual	java/lang/Integer.intValue()I
	aaload
	ldc	"nextindex"
	getstatic	xref/nextnumberindex I
	invokestatic	java/lang/Integer.valueOf(I)Ljava/lang/Integer;
	invokevirtual	java/util/HashMap.put(Ljava/lang/Object;Ljava/lang/Object;)Ljava/lang/Object;
	pop
L034:
.line 155
	getstatic	xref/numbertable [Ljava/util/HashMap;
	getstatic	xref/nextnumberindex I
	aaload
	ldc	"number"
	getstatic	xref/linenumber I
	invokestatic	java/lang/Integer.valueOf(I)Ljava/lang/Integer;
	invokevirtual	java/util/HashMap.put(Ljava/lang/Object;Ljava/lang/Object;)Ljava/lang/Object;
	pop
.line 156
	getstatic	xref/numbertable [Ljava/util/HashMap;
	getstatic	xref/nextnumberindex I
	aaload
	ldc	"nextindex"
	iconst_0
	invokestatic	java/lang/Integer.valueOf(I)Ljava/lang/Integer;
	invokevirtual	java/util/HashMap.put(Ljava/lang/Object;Ljava/lang/Object;)Ljava/lang/Object;
	pop
.line 157
	aload_0
	ldc	"lastnumberindex"
	getstatic	xref/nextnumberindex I
	invokestatic	java/lang/Integer.valueOf(I)Ljava/lang/Integer;
	invokevirtual	java/util/HashMap.put(Ljava/lang/Object;Ljava/lang/Object;)Ljava/lang/Object;
	pop
.line 158
	getstatic	xref/nextnumberindex I
	iconst_1
	iadd
	dup
	iconst_0
	sipush	2000
	invokestatic	RangeChecker/check(III)V
	putstatic	xref/nextnumberindex I
	goto	L030
L033:
.line 161
	iconst_1
	putstatic	xref/numbertablefull Z
L030:

	return

.limit locals 1
.limit stack 4
.end method

.method private static enterword()V

.var 0 is i I


.line 178
	getstatic	xref/newline Z
	ifeq	L038
.line 178
	getstatic	xref/linenumber I
	iconst_1
	isub
	dup
	iconst_1
	sipush	999
	invokestatic	RangeChecker/check(III)V
	putstatic	xref/linenumber I
L038:
.line 183
	iconst_1
	dup
	iconst_1
	sipush	500
	invokestatic	RangeChecker/check(III)V
	istore_0
.line 184
L039:
	getstatic	xref/wordtable [Ljava/util/HashMap;
	iload_0
	iconst_1
	isub
	aaload
	ldc	"word"
	invokevirtual	java/util/HashMap.get(Ljava/lang/Object;)Ljava/lang/Object;
	checkcast	java/lang/StringBuilder
	invokevirtual	java/lang/StringBuilder.toString()Ljava/lang/String;
	getstatic	xref/wordtable [Ljava/util/HashMap;
	getstatic	xref/nextwordindex I
	iconst_1
	isub
	aaload
	ldc	"word"
	invokevirtual	java/util/HashMap.get(Ljava/lang/Object;)Ljava/lang/Object;
	checkcast	java/lang/StringBuilder
	invokevirtual	java/lang/StringBuilder.toString()Ljava/lang/String;
	invokevirtual	java/lang/String.compareTo(Ljava/lang/String;)I
	ifne	L041
	iconst_0
	goto	L042
L041:
	iconst_1
L042:
	iconst_1
	ixor
	ifne	L040
.line 186
	iload_0
	iconst_1
	iadd
	dup
	iconst_1
	sipush	500
	invokestatic	RangeChecker/check(III)V
	istore_0
	goto	L039
L040:
.line 190
	iload_0
	getstatic	xref/nextwordindex I
	if_icmplt	L044
	iconst_0
	goto	L045
L044:
	iconst_1
L045:
	ifeq	L046
.line 191
	getstatic	xref/wordtable [Ljava/util/HashMap;
	iload_0
	iconst_1
	isub
	aaload
	invokestatic	xref/appendlinenumber(Ljava/util/HashMap;)V
	goto	L043
L046:
.line 195
	getstatic	xref/nextwordindex I
	sipush	500
	if_icmplt	L048
	iconst_0
	goto	L049
L048:
	iconst_1
L049:
	ifeq	L050
.line 196
	getstatic	xref/wordtable [Ljava/util/HashMap;
	iload_0
	iconst_1
	isub
	aaload
	ldc	"lastnumberindex"
	iconst_0
	invokestatic	java/lang/Integer.valueOf(I)Ljava/lang/Integer;
	invokevirtual	java/util/HashMap.put(Ljava/lang/Object;Ljava/lang/Object;)Ljava/lang/Object;
	pop
.line 197
	getstatic	xref/wordtable [Ljava/util/HashMap;
	iload_0
	iconst_1
	isub
	aaload
	invokestatic	xref/appendlinenumber(Ljava/util/HashMap;)V
.line 198
	getstatic	xref/nextwordindex I
	iconst_1
	iadd
	dup
	iconst_1
	sipush	500
	invokestatic	RangeChecker/check(III)V
	putstatic	xref/nextwordindex I
	goto	L047
L050:
.line 202
	iconst_1
	putstatic	xref/wordtablefull Z
L047:
L043:
.line 204
	getstatic	xref/newline Z
	ifeq	L051
.line 204
	getstatic	xref/linenumber I
	iconst_1
	iadd
	dup
	iconst_1
	sipush	999
	invokestatic	RangeChecker/check(III)V
	putstatic	xref/linenumber I
L051:

	return

.limit locals 1
.limit stack 4
.end method

.method private static sortwords()V

.var 0 is i I
.var 1 is j I
.var 2 is temp Ljava/util/HashMap;


	new	java/util/HashMap
	dup
	invokenonvirtual	java/util/HashMap/<init>()V
	dup
	ldc	"firstnumberindex"
	iconst_0
	invokestatic	java/lang/Integer.valueOf(I)Ljava/lang/Integer;
	invokevirtual	java/util/HashMap.put(Ljava/lang/Object;Ljava/lang/Object;)Ljava/lang/Object;
	pop
	dup
	ldc	"lastnumberindex"
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
	astore_2

.line 217
	iconst_1
	dup
	iconst_1
	sipush	500
	invokestatic	RangeChecker/check(III)V
	istore_0
.line 217
L052:
	iload_0
	getstatic	xref/nextwordindex I
	iconst_2
	isub
	if_icmpgt	L054
	iconst_0
	goto	L055
L054:
	iconst_1
L055:
	ifne	L053
.line 218
	iload_0
	iconst_1
	iadd
	dup
	iconst_1
	sipush	500
	invokestatic	RangeChecker/check(III)V
	istore_1
.line 218
L056:
	iload_1
	getstatic	xref/nextwordindex I
	iconst_1
	isub
	if_icmpgt	L058
	iconst_0
	goto	L059
L058:
	iconst_1
L059:
	ifne	L057
.line 219
	getstatic	xref/wordtable [Ljava/util/HashMap;
	iload_0
	iconst_1
	isub
	aaload
	ldc	"word"
	invokevirtual	java/util/HashMap.get(Ljava/lang/Object;)Ljava/lang/Object;
	checkcast	java/lang/StringBuilder
	invokevirtual	java/lang/StringBuilder.toString()Ljava/lang/String;
	getstatic	xref/wordtable [Ljava/util/HashMap;
	iload_1
	iconst_1
	isub
	aaload
	ldc	"word"
	invokevirtual	java/util/HashMap.get(Ljava/lang/Object;)Ljava/lang/Object;
	checkcast	java/lang/StringBuilder
	invokevirtual	java/lang/StringBuilder.toString()Ljava/lang/String;
	invokevirtual	java/lang/String.compareTo(Ljava/lang/String;)I
	ifgt	L061
	iconst_0
	goto	L062
L061:
	iconst_1
L062:
	ifeq	L060
.line 220
	getstatic	xref/wordtable [Ljava/util/HashMap;
	iload_0
	iconst_1
	isub
	aaload
	astore_2
.line 221
	getstatic	xref/wordtable [Ljava/util/HashMap;
	iload_0
	iconst_1
	isub
	getstatic	xref/wordtable [Ljava/util/HashMap;
	iload_1
	iconst_1
	isub
	aaload
	aastore
.line 222
	getstatic	xref/wordtable [Ljava/util/HashMap;
	iload_1
	iconst_1
	isub
	aload_2
	aastore
L060:
.line 218
	iload_1
	iconst_1
	iadd
	dup
	iconst_1
	sipush	500
	invokestatic	RangeChecker/check(III)V
	istore_1
	goto	L056
L057:
.line 217
	iload_0
	iconst_1
	iadd
	dup
	iconst_1
	sipush	500
	invokestatic	RangeChecker/check(III)V
	istore_0
	goto	L052
L053:

	return

.limit locals 3
.limit stack 5
.end method

.method private static printnumbers(I)V

.var 0 is i I


.line 234
L063:
.line 235
	getstatic	java/lang/System/out Ljava/io/PrintStream;
	ldc	"%4d"
	iconst_1
	anewarray	java/lang/Object
	dup
	iconst_0
	getstatic	xref/numbertable [Ljava/util/HashMap;
	iload_0
	aaload
	ldc	"number"
	invokevirtual	java/util/HashMap.get(Ljava/lang/Object;)Ljava/lang/Object;
	checkcast	java/lang/Integer
	invokevirtual	java/lang/Integer.intValue()I
	invokestatic	java/lang/Integer.valueOf(I)Ljava/lang/Integer;
	aastore
	invokestatic	java/lang/String/format(Ljava/lang/String;[Ljava/lang/Object;)Ljava/lang/String;
	invokevirtual	java/io/PrintStream.print(Ljava/lang/String;)V
.line 236
	getstatic	xref/numbertable [Ljava/util/HashMap;
	iload_0
	aaload
	ldc	"nextindex"
	invokevirtual	java/util/HashMap.get(Ljava/lang/Object;)Ljava/lang/Object;
	checkcast	java/lang/Integer
	invokevirtual	java/lang/Integer.intValue()I
	dup
	iconst_0
	sipush	2000
	invokestatic	RangeChecker/check(III)V
	istore_0
	iload_0
	iconst_0
	if_icmpeq	L065
	iconst_0
	goto	L066
L065:
	iconst_1
L066:
	ifne	L064
	goto	L063
L064:
.line 238
	getstatic	java/lang/System/out Ljava/io/PrintStream;
	invokevirtual	java/io/PrintStream.println()V

	return

.limit locals 1
.limit stack 8
.end method

.method private static printxref()V

.var 0 is i I


.line 250
	getstatic	java/lang/System/out Ljava/io/PrintStream;
	invokevirtual	java/io/PrintStream.println()V
.line 251
	getstatic	java/lang/System/out Ljava/io/PrintStream;
	invokevirtual	java/io/PrintStream.println()V
.line 252
	getstatic	java/lang/System/out Ljava/io/PrintStream;
	ldc	"Cross-reference\n"
	invokevirtual	java/io/PrintStream.print(Ljava/lang/String;)V
.line 253
	getstatic	java/lang/System/out Ljava/io/PrintStream;
	ldc	"---------------\n"
	invokevirtual	java/io/PrintStream.print(Ljava/lang/String;)V
.line 254
	getstatic	java/lang/System/out Ljava/io/PrintStream;
	invokevirtual	java/io/PrintStream.println()V
.line 255
	invokestatic	xref/sortwords()V
.line 256
	iconst_1
	dup
	iconst_1
	sipush	500
	invokestatic	RangeChecker/check(III)V
	istore_0
.line 256
L067:
	iload_0
	getstatic	xref/nextwordindex I
	iconst_1
	isub
	if_icmpgt	L069
	iconst_0
	goto	L070
L069:
	iconst_1
L070:
	ifne	L068
.line 257
	getstatic	java/lang/System/out Ljava/io/PrintStream;
	ldc	"%s"
	iconst_1
	anewarray	java/lang/Object
	dup
	iconst_0
	getstatic	xref/wordtable [Ljava/util/HashMap;
	iload_0
	iconst_1
	isub
	aaload
	ldc	"word"
	invokevirtual	java/util/HashMap.get(Ljava/lang/Object;)Ljava/lang/Object;
	checkcast	java/lang/StringBuilder
	aastore
	invokestatic	java/lang/String/format(Ljava/lang/String;[Ljava/lang/Object;)Ljava/lang/String;
	invokevirtual	java/io/PrintStream.print(Ljava/lang/String;)V
.line 258
	getstatic	xref/wordtable [Ljava/util/HashMap;
	iload_0
	iconst_1
	isub
	aaload
	ldc	"firstnumberindex"
	invokevirtual	java/util/HashMap.get(Ljava/lang/Object;)Ljava/lang/Object;
	checkcast	java/lang/Integer
	invokevirtual	java/lang/Integer.intValue()I
	dup
	iconst_0
	sipush	2000
	invokestatic	RangeChecker/check(III)V
	invokestatic	xref/printnumbers(I)V
.line 256
	iload_0
	iconst_1
	iadd
	dup
	iconst_1
	sipush	500
	invokestatic	RangeChecker/check(III)V
	istore_0
	goto	L067
L068:

	return

.limit locals 1
.limit stack 9
.end method

.method public static main([Ljava/lang/String;)V

	new	RunTimer
	dup
	invokenonvirtual	RunTimer/<init>()V
	putstatic	xref/_runTimer LRunTimer;
	new	PascalTextIn
	dup
	invokenonvirtual	PascalTextIn/<init>()V
	putstatic	xref/_standardIn LPascalTextIn;


	sipush	2001
	anewarray	java/util/HashMap

	iconst_0
	istore_1
L071:
	iload_1
	sipush	2001
	if_icmpge	L072

	dup
	iload_1

	new	java/util/HashMap
	dup
	invokenonvirtual	java/util/HashMap/<init>()V
	dup
	ldc	"nextindex"
	iconst_0
	invokestatic	java/lang/Integer.valueOf(I)Ljava/lang/Integer;
	invokevirtual	java/util/HashMap.put(Ljava/lang/Object;Ljava/lang/Object;)Ljava/lang/Object;
	pop
	dup
	ldc	"number"
	iconst_0
	invokestatic	java/lang/Integer.valueOf(I)Ljava/lang/Integer;
	invokevirtual	java/util/HashMap.put(Ljava/lang/Object;Ljava/lang/Object;)Ljava/lang/Object;
	pop
	aastore
	iinc	1 1
	goto	L071
L072:
	putstatic	xref/numbertable [Ljava/util/HashMap;

	sipush	500
	anewarray	java/util/HashMap

	iconst_0
	istore_1
L073:
	iload_1
	sipush	500
	if_icmpge	L074

	dup
	iload_1

	new	java/util/HashMap
	dup
	invokenonvirtual	java/util/HashMap/<init>()V
	dup
	ldc	"firstnumberindex"
	iconst_0
	invokestatic	java/lang/Integer.valueOf(I)Ljava/lang/Integer;
	invokevirtual	java/util/HashMap.put(Ljava/lang/Object;Ljava/lang/Object;)Ljava/lang/Object;
	pop
	dup
	ldc	"lastnumberindex"
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
	goto	L073
L074:
	putstatic	xref/wordtable [Ljava/util/HashMap;

.line 264
	iconst_0
	putstatic	xref/wordtablefull Z
.line 265
	iconst_0
	putstatic	xref/numbertablefull Z
.line 266
	iconst_1
	dup
	iconst_1
	sipush	500
	invokestatic	RangeChecker/check(III)V
	putstatic	xref/nextwordindex I
.line 267
	iconst_1
	dup
	iconst_0
	sipush	2000
	invokestatic	RangeChecker/check(III)V
	putstatic	xref/nextnumberindex I
.line 268
	iconst_1
	dup
	iconst_1
	sipush	999
	invokestatic	RangeChecker/check(III)V
	putstatic	xref/linenumber I
.line 269
	getstatic	java/lang/System/out Ljava/io/PrintStream;
	ldc	"    1 : "
	invokevirtual	java/io/PrintStream.print(Ljava/lang/String;)V
.line 272
L075:
	getstatic	xref/_standardIn LPascalTextIn;
	invokevirtual	PascalTextIn.atEof()Z
	getstatic	xref/wordtablefull Z
	ior
	getstatic	xref/numbertablefull Z
	ior
	iconst_1
	ixor
	iconst_1
	ixor
	ifne	L076
.line 276
	getstatic	xref/wordtable [Ljava/util/HashMap;
	getstatic	xref/nextwordindex I
	iconst_1
	isub
	aaload
	ldc	"word"
	invokevirtual	java/util/HashMap.get(Ljava/lang/Object;)Ljava/lang/Object;
	checkcast	java/lang/StringBuilder
	invokestatic	xref/readword(Ljava/lang/StringBuilder;)Z
	ifeq	L077
.line 277
	invokestatic	xref/enterword()V
L077:
	goto	L075
L076:
.line 282
	getstatic	xref/wordtablefull Z
	ifeq	L079
.line 283
	getstatic	java/lang/System/out Ljava/io/PrintStream;
	invokevirtual	java/io/PrintStream.println()V
.line 284
	getstatic	java/lang/System/out Ljava/io/PrintStream;
	ldc	"*** The word table is not large enough. ***\n"
	invokevirtual	java/io/PrintStream.print(Ljava/lang/String;)V
	goto	L078
L079:
.line 286
	getstatic	xref/numbertablefull Z
	ifeq	L081
.line 287
	getstatic	java/lang/System/out Ljava/io/PrintStream;
	invokevirtual	java/io/PrintStream.println()V
.line 288
	getstatic	java/lang/System/out Ljava/io/PrintStream;
	ldc	"*** The number table is not large enough. ***\n"
	invokevirtual	java/io/PrintStream.print(Ljava/lang/String;)V
	goto	L080
L081:
.line 291
	invokestatic	xref/printxref()V
L080:
L078:
.line 295
	getstatic	java/lang/System/out Ljava/io/PrintStream;
	invokevirtual	java/io/PrintStream.println()V
.line 296
	getstatic	java/lang/System/out Ljava/io/PrintStream;
	ldc	"%5d word entries.\n"
	iconst_1
	anewarray	java/lang/Object
	dup
	iconst_0
	getstatic	xref/nextwordindex I
	iconst_1
	isub
	invokestatic	java/lang/Integer.valueOf(I)Ljava/lang/Integer;
	aastore
	invokestatic	java/lang/String/format(Ljava/lang/String;[Ljava/lang/Object;)Ljava/lang/String;
	invokevirtual	java/io/PrintStream.print(Ljava/lang/String;)V
.line 297
	getstatic	java/lang/System/out Ljava/io/PrintStream;
	ldc	"%5d line number entries.\n"
	iconst_1
	anewarray	java/lang/Object
	dup
	iconst_0
	getstatic	xref/nextnumberindex I
	iconst_1
	isub
	invokestatic	java/lang/Integer.valueOf(I)Ljava/lang/Integer;
	aastore
	invokestatic	java/lang/String/format(Ljava/lang/String;[Ljava/lang/Object;)Ljava/lang/String;
	invokevirtual	java/io/PrintStream.print(Ljava/lang/String;)V

	getstatic	xref/_runTimer LRunTimer;
	invokevirtual	RunTimer.printElapsedTime()V

	return

.limit locals 2
.limit stack 8
.end method
