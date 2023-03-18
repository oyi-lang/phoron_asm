.class public wolfisland
.super java/lang/Object

.field private static _runTimer LRunTimer;
.field private static _standardIn LPascalTextIn;

.field private static coloffset [I
.field private static foodunits [[I
.field private static island [[I
.field private static numprinttimes I
.field private static numrabbits I
.field private static numwolves I
.field private static rowoffset [I
.field private static seed I
.field private static t I

.method public <init>()V

	aload_0
	invokenonvirtual	java/lang/Object/<init>()V
	return

.limit locals 1
.limit stack 1
.end method

.method private static initialize()V

.var 2 is col I
.var 0 is i I
.var 1 is row I


.line 108
	iconst_0
	dup
	iconst_0
	sipush	32767
	invokestatic	RangeChecker/check(III)V
	istore_0
.line 108
L001:
	iload_0
	bipush	10
	if_icmpgt	L003
	iconst_0
	goto	L004
L003:
	iconst_1
L004:
	ifne	L002
.line 109
	getstatic	wolfisland/island [[I
	iconst_0
	aaload
	iload_0
	iconst_5
	iastore
.line 110
	getstatic	wolfisland/island [[I
	bipush	10
	aaload
	iload_0
	iconst_5
	iastore
.line 111
	getstatic	wolfisland/island [[I
	iload_0
	aaload
	iconst_0
	iconst_5
	iastore
.line 112
	getstatic	wolfisland/island [[I
	iload_0
	aaload
	bipush	10
	iconst_5
	iastore
.line 108
	iload_0
	iconst_1
	iadd
	dup
	iconst_0
	sipush	32767
	invokestatic	RangeChecker/check(III)V
	istore_0
	goto	L001
L002:
.line 114
	iconst_1
	dup
	iconst_0
	bipush	10
	invokestatic	RangeChecker/check(III)V
	istore_1
.line 114
L005:
	iload_1
	bipush	9
	if_icmpgt	L007
	iconst_0
	goto	L008
L007:
	iconst_1
L008:
	ifne	L006
.line 115
	iconst_1
	dup
	iconst_0
	bipush	10
	invokestatic	RangeChecker/check(III)V
	istore_2
.line 115
L009:
	iload_2
	bipush	9
	if_icmpgt	L011
	iconst_0
	goto	L012
L011:
	iconst_1
L012:
	ifne	L010
.line 116
	getstatic	wolfisland/island [[I
	iload_1
	aaload
	iload_2
	iconst_4
	iastore
.line 117
	getstatic	wolfisland/foodunits [[I
	iload_1
	iconst_1
	isub
	aaload
	iload_2
	iconst_1
	isub
	iconst_0
	iastore
.line 115
	iload_2
	iconst_1
	iadd
	dup
	iconst_0
	bipush	10
	invokestatic	RangeChecker/check(III)V
	istore_2
	goto	L009
L010:
.line 114
	iload_1
	iconst_1
	iadd
	dup
	iconst_0
	bipush	10
	invokestatic	RangeChecker/check(III)V
	istore_1
	goto	L005
L006:
.line 122
	getstatic	wolfisland/_standardIn LPascalTextIn;
	invokevirtual	PascalTextIn.readInteger()I
	dup
	iconst_0
	sipush	32767
	invokestatic	RangeChecker/check(III)V
	putstatic	wolfisland/numwolves I
.line 123
	iconst_1
	dup
	iconst_0
	sipush	32767
	invokestatic	RangeChecker/check(III)V
	istore_0
.line 123
L013:
	iload_0
	getstatic	wolfisland/numwolves I
	if_icmpgt	L015
	iconst_0
	goto	L016
L015:
	iconst_1
L016:
	ifne	L014
.line 124
	getstatic	wolfisland/_standardIn LPascalTextIn;
	invokevirtual	PascalTextIn.readInteger()I
	dup
	iconst_0
	bipush	10
	invokestatic	RangeChecker/check(III)V
	istore_1
	getstatic	wolfisland/_standardIn LPascalTextIn;
	invokevirtual	PascalTextIn.readInteger()I
	dup
	iconst_0
	bipush	10
	invokestatic	RangeChecker/check(III)V
	istore_2
.line 125
	getstatic	wolfisland/island [[I
	iload_1
	aaload
	iload_2
	iconst_0
	iastore
.line 126
	getstatic	wolfisland/foodunits [[I
	iload_1
	iconst_1
	isub
	aaload
	iload_2
	iconst_1
	isub
	bipush	6
	iastore
.line 123
	iload_0
	iconst_1
	iadd
	dup
	iconst_0
	sipush	32767
	invokestatic	RangeChecker/check(III)V
	istore_0
	goto	L013
L014:
.line 130
	getstatic	wolfisland/_standardIn LPascalTextIn;
	invokevirtual	PascalTextIn.readInteger()I
	dup
	iconst_0
	sipush	32767
	invokestatic	RangeChecker/check(III)V
	putstatic	wolfisland/numrabbits I
.line 131
	iconst_1
	dup
	iconst_0
	sipush	32767
	invokestatic	RangeChecker/check(III)V
	istore_0
.line 131
L017:
	iload_0
	getstatic	wolfisland/numrabbits I
	if_icmpgt	L019
	iconst_0
	goto	L020
L019:
	iconst_1
L020:
	ifne	L018
.line 132
	getstatic	wolfisland/_standardIn LPascalTextIn;
	invokevirtual	PascalTextIn.readInteger()I
	dup
	iconst_0
	bipush	10
	invokestatic	RangeChecker/check(III)V
	istore_1
	getstatic	wolfisland/_standardIn LPascalTextIn;
	invokevirtual	PascalTextIn.readInteger()I
	dup
	iconst_0
	bipush	10
	invokestatic	RangeChecker/check(III)V
	istore_2
.line 133
	getstatic	wolfisland/island [[I
	iload_1
	aaload
	iload_2
	iconst_1
	iastore
.line 131
	iload_0
	iconst_1
	iadd
	dup
	iconst_0
	sipush	32767
	invokestatic	RangeChecker/check(III)V
	istore_0
	goto	L017
L018:
.line 137
	getstatic	wolfisland/rowoffset [I
	iconst_0
	iconst_0
	iastore
.line 137
	getstatic	wolfisland/coloffset [I
	iconst_0
	iconst_0
	iastore
.line 138
	getstatic	wolfisland/rowoffset [I
	iconst_1
	iconst_1
	ineg
	iastore
.line 138
	getstatic	wolfisland/coloffset [I
	iconst_1
	iconst_0
	iastore
.line 139
	getstatic	wolfisland/rowoffset [I
	iconst_2
	iconst_0
	iastore
.line 139
	getstatic	wolfisland/coloffset [I
	iconst_2
	iconst_1
	ineg
	iastore
.line 140
	getstatic	wolfisland/rowoffset [I
	iconst_3
	iconst_0
	iastore
.line 140
	getstatic	wolfisland/coloffset [I
	iconst_3
	iconst_1
	iastore
.line 141
	getstatic	wolfisland/rowoffset [I
	iconst_4
	iconst_1
	iastore
.line 141
	getstatic	wolfisland/coloffset [I
	iconst_4
	iconst_0
	iastore
.line 144
	getstatic	wolfisland/_standardIn LPascalTextIn;
	invokevirtual	PascalTextIn.readInteger()I
	dup
	iconst_0
	sipush	32767
	invokestatic	RangeChecker/check(III)V
	putstatic	wolfisland/seed I

	return

.limit locals 3
.limit stack 4
.end method

.method private static random(I)I

.var 0 is limit I
.var 1 is random I


.line 157
	getstatic	wolfisland/seed I
	bipush	21
	imul
	bipush	77
	iadd
	sipush	1024
	irem
	dup
	iconst_0
	sipush	32767
	invokestatic	RangeChecker/check(III)V
	putstatic	wolfisland/seed I
.line 158
	getstatic	wolfisland/seed I
	iload_0
	imul
	sipush	1024
	idiv
	dup
	iconst_0
	sipush	32767
	invokestatic	RangeChecker/check(III)V
	istore_1

	iload_1
	ireturn

.limit locals 2
.limit stack 4
.end method

.method private static newlocation(IIILIWrap;LIWrap;)V

.var 5 is adj I
.var 0 is critter I
.var 7 is done Z
.var 4 is newcol LIWrap;
.var 3 is newrow LIWrap;
.var 2 is oldcol I
.var 1 is oldrow I
.var 6 is what I


.line 174
	iconst_0
	istore	7
.line 178
	iload_0
	iconst_0
	if_icmpeq	L022
	iconst_0
	goto	L023
L022:
	iconst_1
L023:
	ifeq	L021
.line 179
	iconst_0
	dup
	iconst_0
	iconst_4
	invokestatic	RangeChecker/check(III)V
	istore	5
.line 180
L024:
.line 181
	iload	5
	iconst_1
	iadd
	dup
	iconst_0
	iconst_4
	invokestatic	RangeChecker/check(III)V
	istore	5
.line 182
	aload_3
	iload_1
	getstatic	wolfisland/rowoffset [I
	iload	5
	iaload
	iadd
	dup
	iconst_0
	bipush	10
	invokestatic	RangeChecker/check(III)V
	putfield	IWrap/value I
.line 183
	aload	4
	iload_2
	getstatic	wolfisland/coloffset [I
	iload	5
	iaload
	iadd
	dup
	iconst_0
	bipush	10
	invokestatic	RangeChecker/check(III)V
	putfield	IWrap/value I
.line 184
	getstatic	wolfisland/island [[I
	aload_3
	getfield	IWrap/value I
	aaload
	aload	4
	getfield	IWrap/value I
	iaload
	istore	6
.line 185
	iload	6
	iconst_1
	if_icmpeq	L026
	iconst_0
	goto	L027
L026:
	iconst_1
L027:
	istore	7
	iload	7
	iload	5
	iconst_4
	if_icmpeq	L028
	iconst_0
	goto	L029
L028:
	iconst_1
L029:
	ior
	ifne	L025
	goto	L024
L025:
L021:
.line 190
	iload	7
	iconst_1
	ixor
	ifeq	L030
.line 191
L031:
.line 192
	iconst_5
	dup
	iconst_0
	sipush	32767
	invokestatic	RangeChecker/check(III)V
	invokestatic	wolfisland/random(I)I
	dup
	iconst_0
	iconst_4
	invokestatic	RangeChecker/check(III)V
	istore	5
.line 193
	aload_3
	iload_1
	getstatic	wolfisland/rowoffset [I
	iload	5
	iaload
	iadd
	dup
	iconst_0
	bipush	10
	invokestatic	RangeChecker/check(III)V
	putfield	IWrap/value I
.line 194
	aload	4
	iload_2
	getstatic	wolfisland/coloffset [I
	iload	5
	iaload
	iadd
	dup
	iconst_0
	bipush	10
	invokestatic	RangeChecker/check(III)V
	putfield	IWrap/value I
.line 195
	getstatic	wolfisland/island [[I
	aload_3
	getfield	IWrap/value I
	aaload
	aload	4
	getfield	IWrap/value I
	iaload
	istore	6
	iload	6
	iconst_4
	if_icmpeq	L033
	iconst_0
	goto	L034
L033:
	iconst_1
L034:
	aload_3
	getfield	IWrap/value I
	iload_1
	if_icmpeq	L035
	iconst_0
	goto	L036
L035:
	iconst_1
L036:
	aload	4
	getfield	IWrap/value I
	iload_2
	if_icmpeq	L037
	iconst_0
	goto	L038
L037:
	iconst_1
L038:
	iand
	ior
	ifne	L032
	goto	L031
L032:
L030:

	return

.limit locals 8
.limit stack 5
.end method

.method private static processwolf(II)V

.var 4 is moved Z
.var 3 is newcol I
.var 2 is newrow I
.var 1 is oldcol I
.var 0 is oldrow I


.line 212
	getstatic	wolfisland/foodunits [[I
	iload_0
	iconst_1
	isub
	aaload
	iload_1
	iconst_1
	isub
	getstatic	wolfisland/foodunits [[I
	iload_0
	iconst_1
	isub
	aaload
	iload_1
	iconst_1
	isub
	iaload
	iconst_1
	isub
	iastore
.line 214
	getstatic	wolfisland/foodunits [[I
	iload_0
	iconst_1
	isub
	aaload
	iload_1
	iconst_1
	isub
	iaload
	iconst_0
	if_icmpeq	L040
	iconst_0
	goto	L041
L040:
	iconst_1
L041:
	ifeq	L042
.line 217
	getstatic	wolfisland/island [[I
	iload_0
	aaload
	iload_1
	iconst_4
	iastore
.line 218
	getstatic	wolfisland/numwolves I
	iconst_1
	isub
	dup
	iconst_0
	sipush	32767
	invokestatic	RangeChecker/check(III)V
	putstatic	wolfisland/numwolves I
.line 219
	getstatic	java/lang/System/out Ljava/io/PrintStream;
	ldc	"t =%4d : Wolf died at [%1d, %1d]\n"
	iconst_3
	anewarray	java/lang/Object
	dup
	iconst_0
	getstatic	wolfisland/t I
	invokestatic	java/lang/Integer.valueOf(I)Ljava/lang/Integer;
	aastore
	dup
	iconst_1
	iload_0
	invokestatic	java/lang/Integer.valueOf(I)Ljava/lang/Integer;
	aastore
	dup
	iconst_2
	iload_1
	invokestatic	java/lang/Integer.valueOf(I)Ljava/lang/Integer;
	aastore
	invokestatic	java/lang/String/format(Ljava/lang/String;[Ljava/lang/Object;)Ljava/lang/String;
	invokevirtual	java/io/PrintStream.print(Ljava/lang/String;)V
	goto	L039
L042:
.line 223
	iconst_0
	dup
	iconst_0
	bipush	10
	invokestatic	RangeChecker/check(III)V
	istore_2
.line 224
	iconst_0
	dup
	iconst_0
	bipush	10
	invokestatic	RangeChecker/check(III)V
	istore_3
.line 227
	iconst_0
	iload_0
	dup
	iconst_0
	bipush	10
	invokestatic	RangeChecker/check(III)V
	iload_1
	dup
	iconst_0
	bipush	10
	invokestatic	RangeChecker/check(III)V
	new	IWrap
	dup
	iload_2
	invokenonvirtual	IWrap/<init>(I)V
	dup
	astore	5
	new	IWrap
	dup
	iload_3
	invokenonvirtual	IWrap/<init>(I)V
	dup
	astore	6
	invokestatic	wolfisland/newlocation(IIILIWrap;LIWrap;)V
	aload	5
	getfield	IWrap/value I
	istore_2
	aload	6
	getfield	IWrap/value I
	istore_3
.line 228
	iload_2
	iload_0
	if_icmpne	L043
	iconst_0
	goto	L044
L043:
	iconst_1
L044:
	iload_3
	iload_1
	if_icmpne	L045
	iconst_0
	goto	L046
L045:
	iconst_1
L046:
	ior
	istore	4
.line 230
	iload	4
	ifeq	L047
.line 233
	getstatic	wolfisland/island [[I
	iload_2
	aaload
	iload_3
	iaload
	iconst_1
	if_icmpeq	L049
	iconst_0
	goto	L050
L049:
	iconst_1
L050:
	ifeq	L048
.line 234
	getstatic	wolfisland/foodunits [[I
	iload_0
	iconst_1
	isub
	aaload
	iload_1
	iconst_1
	isub
	getstatic	wolfisland/foodunits [[I
	iload_0
	iconst_1
	isub
	aaload
	iload_1
	iconst_1
	isub
	iaload
	bipush	6
	iadd
	iastore
.line 236
	getstatic	wolfisland/numrabbits I
	iconst_1
	isub
	dup
	iconst_0
	sipush	32767
	invokestatic	RangeChecker/check(III)V
	putstatic	wolfisland/numrabbits I
.line 237
	getstatic	java/lang/System/out Ljava/io/PrintStream;
	ldc	"t =%4d : Rabbit eaten at [%1d, %1d]\n"
	iconst_3
	anewarray	java/lang/Object
	dup
	iconst_0
	getstatic	wolfisland/t I
	invokestatic	java/lang/Integer.valueOf(I)Ljava/lang/Integer;
	aastore
	dup
	iconst_1
	iload_2
	invokestatic	java/lang/Integer.valueOf(I)Ljava/lang/Integer;
	aastore
	dup
	iconst_2
	iload_3
	invokestatic	java/lang/Integer.valueOf(I)Ljava/lang/Integer;
	aastore
	invokestatic	java/lang/String/format(Ljava/lang/String;[Ljava/lang/Object;)Ljava/lang/String;
	invokevirtual	java/io/PrintStream.print(Ljava/lang/String;)V
L048:
.line 242
	getstatic	wolfisland/island [[I
	iload_2
	aaload
	iload_3
	iconst_2
	iastore
.line 243
	getstatic	wolfisland/island [[I
	iload_0
	aaload
	iload_1
	iconst_4
	iastore
.line 244
	getstatic	wolfisland/foodunits [[I
	iload_2
	iconst_1
	isub
	aaload
	iload_3
	iconst_1
	isub
	getstatic	wolfisland/foodunits [[I
	iload_0
	iconst_1
	isub
	aaload
	iload_1
	iconst_1
	isub
	iaload
	iastore
.line 245
	getstatic	wolfisland/foodunits [[I
	iload_0
	iconst_1
	isub
	aaload
	iload_1
	iconst_1
	isub
	iconst_0
	iastore
L047:
.line 249
	getstatic	wolfisland/t I
	bipush	12
	irem
	iconst_0
	if_icmpeq	L052
	iconst_0
	goto	L053
L052:
	iconst_1
L053:
	getstatic	wolfisland/foodunits [[I
	iload_2
	iconst_1
	isub
	aaload
	iload_3
	iconst_1
	isub
	iaload
	iconst_1
	if_icmpgt	L054
	iconst_0
	goto	L055
L054:
	iconst_1
L055:
	iand
	ifeq	L051
.line 251
	getstatic	wolfisland/foodunits [[I
	iload_2
	iconst_1
	isub
	aaload
	iload_3
	iconst_1
	isub
	getstatic	wolfisland/foodunits [[I
	iload_2
	iconst_1
	isub
	aaload
	iload_3
	iconst_1
	isub
	iaload
	iconst_2
	idiv
	iastore
.line 255
	iload	4
	ifeq	L056
.line 256
	getstatic	wolfisland/island [[I
	iload_0
	aaload
	iload_1
	iconst_2
	iastore
.line 257
	getstatic	wolfisland/foodunits [[I
	iload_0
	iconst_1
	isub
	aaload
	iload_1
	iconst_1
	isub
	getstatic	wolfisland/foodunits [[I
	iload_2
	iconst_1
	isub
	aaload
	iload_3
	iconst_1
	isub
	iaload
	iastore
.line 259
	getstatic	wolfisland/numwolves I
	iconst_1
	iadd
	dup
	iconst_0
	sipush	32767
	invokestatic	RangeChecker/check(III)V
	putstatic	wolfisland/numwolves I
.line 260
	getstatic	java/lang/System/out Ljava/io/PrintStream;
	ldc	"t =%4d : Wolf born at [%1d, %1d]\n"
	iconst_3
	anewarray	java/lang/Object
	dup
	iconst_0
	getstatic	wolfisland/t I
	invokestatic	java/lang/Integer.valueOf(I)Ljava/lang/Integer;
	aastore
	dup
	iconst_1
	iload_0
	invokestatic	java/lang/Integer.valueOf(I)Ljava/lang/Integer;
	aastore
	dup
	iconst_2
	iload_1
	invokestatic	java/lang/Integer.valueOf(I)Ljava/lang/Integer;
	aastore
	invokestatic	java/lang/String/format(Ljava/lang/String;[Ljava/lang/Object;)Ljava/lang/String;
	invokevirtual	java/io/PrintStream.print(Ljava/lang/String;)V
L056:
L051:
L039:

	return

.limit locals 7
.limit stack 8
.end method

.method private static processrabbit(II)V

.var 4 is moved Z
.var 3 is newcol I
.var 2 is newrow I
.var 1 is oldcol I
.var 0 is oldrow I


.line 276
	iconst_0
	dup
	iconst_0
	bipush	10
	invokestatic	RangeChecker/check(III)V
	istore_2
.line 277
	iconst_0
	dup
	iconst_0
	bipush	10
	invokestatic	RangeChecker/check(III)V
	istore_3
.line 280
	iconst_1
	iload_0
	dup
	iconst_0
	bipush	10
	invokestatic	RangeChecker/check(III)V
	iload_1
	dup
	iconst_0
	bipush	10
	invokestatic	RangeChecker/check(III)V
	new	IWrap
	dup
	iload_2
	invokenonvirtual	IWrap/<init>(I)V
	dup
	astore	5
	new	IWrap
	dup
	iload_3
	invokenonvirtual	IWrap/<init>(I)V
	dup
	astore	6
	invokestatic	wolfisland/newlocation(IIILIWrap;LIWrap;)V
	aload	5
	getfield	IWrap/value I
	istore_2
	aload	6
	getfield	IWrap/value I
	istore_3
.line 281
	iload_2
	iload_0
	if_icmpne	L057
	iconst_0
	goto	L058
L057:
	iconst_1
L058:
	iload_3
	iload_1
	if_icmpne	L059
	iconst_0
	goto	L060
L059:
	iconst_1
L060:
	ior
	istore	4
.line 282
	iload	4
	ifeq	L061
.line 283
	getstatic	wolfisland/island [[I
	iload_2
	aaload
	iload_3
	iconst_3
	iastore
.line 284
	getstatic	wolfisland/island [[I
	iload_0
	aaload
	iload_1
	iconst_4
	iastore
L061:
.line 288
	getstatic	wolfisland/t I
	iconst_5
	irem
	iconst_0
	if_icmpeq	L063
	iconst_0
	goto	L064
L063:
	iconst_1
L064:
	ifeq	L062
.line 291
	iload	4
	ifeq	L065
.line 292
	getstatic	wolfisland/island [[I
	iload_0
	aaload
	iload_1
	iconst_3
	iastore
.line 293
	getstatic	wolfisland/numrabbits I
	iconst_1
	iadd
	dup
	iconst_0
	sipush	32767
	invokestatic	RangeChecker/check(III)V
	putstatic	wolfisland/numrabbits I
.line 294
	getstatic	java/lang/System/out Ljava/io/PrintStream;
	ldc	"t =%4d : Rabbit born at [%1d, %1d]\n"
	iconst_3
	anewarray	java/lang/Object
	dup
	iconst_0
	getstatic	wolfisland/t I
	invokestatic	java/lang/Integer.valueOf(I)Ljava/lang/Integer;
	aastore
	dup
	iconst_1
	iload_0
	invokestatic	java/lang/Integer.valueOf(I)Ljava/lang/Integer;
	aastore
	dup
	iconst_2
	iload_1
	invokestatic	java/lang/Integer.valueOf(I)Ljava/lang/Integer;
	aastore
	invokestatic	java/lang/String/format(Ljava/lang/String;[Ljava/lang/Object;)Ljava/lang/String;
	invokevirtual	java/io/PrintStream.print(Ljava/lang/String;)V
L065:
L062:

	return

.limit locals 7
.limit stack 8
.end method

.method private static eventsoccur()V

.var 1 is col I
.var 0 is row I


.line 310
	iconst_1
	dup
	iconst_0
	bipush	10
	invokestatic	RangeChecker/check(III)V
	istore_0
.line 310
L066:
	iload_0
	bipush	9
	if_icmpgt	L068
	iconst_0
	goto	L069
L068:
	iconst_1
L069:
	ifne	L067
.line 311
	iconst_1
	dup
	iconst_0
	bipush	10
	invokestatic	RangeChecker/check(III)V
	istore_1
.line 311
L070:
	iload_1
	bipush	9
	if_icmpgt	L072
	iconst_0
	goto	L073
L072:
	iconst_1
L073:
	ifne	L071
.line 312
	getstatic	wolfisland/island [[I
	iload_0
	aaload
	iload_1
	iaload
	iconst_0
	if_icmpeq	L075
	iconst_0
	goto	L076
L075:
	iconst_1
L076:
	ifeq	L074
.line 313
	iload_0
	dup
	iconst_0
	bipush	10
	invokestatic	RangeChecker/check(III)V
	iload_1
	dup
	iconst_0
	bipush	10
	invokestatic	RangeChecker/check(III)V
	invokestatic	wolfisland/processwolf(II)V
L074:
.line 311
	iload_1
	iconst_1
	iadd
	dup
	iconst_0
	bipush	10
	invokestatic	RangeChecker/check(III)V
	istore_1
	goto	L070
L071:
.line 310
	iload_0
	iconst_1
	iadd
	dup
	iconst_0
	bipush	10
	invokestatic	RangeChecker/check(III)V
	istore_0
	goto	L066
L067:
.line 319
	iconst_1
	dup
	iconst_0
	bipush	10
	invokestatic	RangeChecker/check(III)V
	istore_0
.line 319
L077:
	iload_0
	bipush	9
	if_icmpgt	L079
	iconst_0
	goto	L080
L079:
	iconst_1
L080:
	ifne	L078
.line 320
	iconst_1
	dup
	iconst_0
	bipush	10
	invokestatic	RangeChecker/check(III)V
	istore_1
.line 320
L081:
	iload_1
	bipush	9
	if_icmpgt	L083
	iconst_0
	goto	L084
L083:
	iconst_1
L084:
	ifne	L082
.line 321
	getstatic	wolfisland/island [[I
	iload_0
	aaload
	iload_1
	iaload
	iconst_1
	if_icmpeq	L086
	iconst_0
	goto	L087
L086:
	iconst_1
L087:
	ifeq	L085
.line 322
	iload_0
	dup
	iconst_0
	bipush	10
	invokestatic	RangeChecker/check(III)V
	iload_1
	dup
	iconst_0
	bipush	10
	invokestatic	RangeChecker/check(III)V
	invokestatic	wolfisland/processrabbit(II)V
L085:
.line 320
	iload_1
	iconst_1
	iadd
	dup
	iconst_0
	bipush	10
	invokestatic	RangeChecker/check(III)V
	istore_1
	goto	L081
L082:
.line 319
	iload_0
	iconst_1
	iadd
	dup
	iconst_0
	bipush	10
	invokestatic	RangeChecker/check(III)V
	istore_0
	goto	L077
L078:

	return

.limit locals 2
.limit stack 5
.end method

.method private static printisland()V

.var 2 is cnts I
.var 1 is col I
.var 0 is row I


.line 337
	getstatic	java/lang/System/out Ljava/io/PrintStream;
	invokevirtual	java/io/PrintStream.println()V
.line 338
	getstatic	java/lang/System/out Ljava/io/PrintStream;
	ldc	"t =%4d : Wolf Island\n"
	iconst_1
	anewarray	java/lang/Object
	dup
	iconst_0
	getstatic	wolfisland/t I
	invokestatic	java/lang/Integer.valueOf(I)Ljava/lang/Integer;
	aastore
	invokestatic	java/lang/String/format(Ljava/lang/String;[Ljava/lang/Object;)Ljava/lang/String;
	invokevirtual	java/io/PrintStream.print(Ljava/lang/String;)V
.line 339
	getstatic	java/lang/System/out Ljava/io/PrintStream;
	invokevirtual	java/io/PrintStream.println()V
.line 341
	iconst_1
	dup
	iconst_0
	bipush	10
	invokestatic	RangeChecker/check(III)V
	istore_0
.line 341
L088:
	iload_0
	bipush	9
	if_icmpgt	L090
	iconst_0
	goto	L091
L090:
	iconst_1
L091:
	ifne	L089
.line 342
	getstatic	java/lang/System/out Ljava/io/PrintStream;
	ldc	" "
	invokevirtual	java/io/PrintStream.print(Ljava/lang/String;)V
.line 343
	iconst_1
	dup
	iconst_0
	bipush	10
	invokestatic	RangeChecker/check(III)V
	istore_1
.line 343
L092:
	iload_1
	bipush	9
	if_icmpgt	L094
	iconst_0
	goto	L095
L094:
	iconst_1
L095:
	ifne	L093
.line 344
	getstatic	wolfisland/island [[I
	iload_0
	aaload
	iload_1
	iaload
	istore_2
.line 345
	iload_2
	iconst_4
	if_icmpeq	L097
	iconst_0
	goto	L098
L097:
	iconst_1
L098:
	ifeq	L099
.line 345
	getstatic	java/lang/System/out Ljava/io/PrintStream;
	ldc	". "
	invokevirtual	java/io/PrintStream.print(Ljava/lang/String;)V
	goto	L096
L099:
.line 346
	iload_2
	iconst_0
	if_icmpeq	L101
	iconst_0
	goto	L102
L101:
	iconst_1
L102:
	ifeq	L103
.line 346
	getstatic	java/lang/System/out Ljava/io/PrintStream;
	ldc	"W "
	invokevirtual	java/io/PrintStream.print(Ljava/lang/String;)V
	goto	L100
L103:
.line 347
	iload_2
	iconst_1
	if_icmpeq	L105
	iconst_0
	goto	L106
L105:
	iconst_1
L106:
	ifeq	L104
.line 347
	getstatic	java/lang/System/out Ljava/io/PrintStream;
	ldc	"r "
	invokevirtual	java/io/PrintStream.print(Ljava/lang/String;)V
L104:
L100:
L096:
.line 343
	iload_1
	iconst_1
	iadd
	dup
	iconst_0
	bipush	10
	invokestatic	RangeChecker/check(III)V
	istore_1
	goto	L092
L093:
.line 349
	getstatic	java/lang/System/out Ljava/io/PrintStream;
	invokevirtual	java/io/PrintStream.println()V
.line 341
	iload_0
	iconst_1
	iadd
	dup
	iconst_0
	bipush	10
	invokestatic	RangeChecker/check(III)V
	istore_0
	goto	L088
L089:

	return

.limit locals 3
.limit stack 7
.end method

.method private static resetisland()V

.var 1 is col I
.var 0 is row I


.line 362
	iconst_1
	dup
	iconst_0
	bipush	10
	invokestatic	RangeChecker/check(III)V
	istore_0
.line 362
L107:
	iload_0
	bipush	9
	if_icmpgt	L109
	iconst_0
	goto	L110
L109:
	iconst_1
L110:
	ifne	L108
.line 363
	iconst_1
	dup
	iconst_0
	bipush	10
	invokestatic	RangeChecker/check(III)V
	istore_1
.line 363
L111:
	iload_1
	bipush	9
	if_icmpgt	L113
	iconst_0
	goto	L114
L113:
	iconst_1
L114:
	ifne	L112
.line 364
	getstatic	wolfisland/island [[I
	iload_0
	aaload
	iload_1
	iaload
	iconst_2
	if_icmpeq	L116
	iconst_0
	goto	L117
L116:
	iconst_1
L117:
	ifeq	L118
.line 365
	getstatic	wolfisland/island [[I
	iload_0
	aaload
	iload_1
	iconst_0
	iastore
	goto	L115
L118:
.line 367
	getstatic	wolfisland/island [[I
	iload_0
	aaload
	iload_1
	iaload
	iconst_3
	if_icmpeq	L120
	iconst_0
	goto	L121
L120:
	iconst_1
L121:
	ifeq	L119
.line 368
	getstatic	wolfisland/island [[I
	iload_0
	aaload
	iload_1
	iconst_1
	iastore
L119:
L115:
.line 363
	iload_1
	iconst_1
	iadd
	dup
	iconst_0
	bipush	10
	invokestatic	RangeChecker/check(III)V
	istore_1
	goto	L111
L112:
.line 362
	iload_0
	iconst_1
	iadd
	dup
	iconst_0
	bipush	10
	invokestatic	RangeChecker/check(III)V
	istore_0
	goto	L107
L108:

	return

.limit locals 2
.limit stack 4
.end method

.method public static main([Ljava/lang/String;)V

	new	RunTimer
	dup
	invokenonvirtual	RunTimer/<init>()V
	putstatic	wolfisland/_runTimer LRunTimer;
	new	PascalTextIn
	dup
	invokenonvirtual	PascalTextIn/<init>()V
	putstatic	wolfisland/_standardIn LPascalTextIn;


	iconst_5
	newarray	int
	putstatic	wolfisland/coloffset [I

	bipush	9
	bipush	9
	multianewarray	[[I 2
	putstatic	wolfisland/foodunits [[I

	bipush	11
	bipush	11
	multianewarray	[[I 2
	putstatic	wolfisland/island [[I

	iconst_5
	newarray	int
	putstatic	wolfisland/rowoffset [I

.line 375
	invokestatic	wolfisland/initialize()V
.line 377
	iconst_0
	dup
	iconst_0
	sipush	32767
	invokestatic	RangeChecker/check(III)V
	putstatic	wolfisland/t I
.line 378
	invokestatic	wolfisland/printisland()V
.line 381
L122:
.line 382
	getstatic	java/lang/System/out Ljava/io/PrintStream;
	invokevirtual	java/io/PrintStream.println()V
.line 383
	getstatic	wolfisland/t I
	iconst_1
	iadd
	dup
	iconst_0
	sipush	32767
	invokestatic	RangeChecker/check(III)V
	putstatic	wolfisland/t I
.line 385
	invokestatic	wolfisland/eventsoccur()V
.line 386
	invokestatic	wolfisland/resetisland()V
.line 387
	invokestatic	wolfisland/printisland()V
	getstatic	wolfisland/numwolves I
	iconst_0
	if_icmpeq	L124
	iconst_0
	goto	L125
L124:
	iconst_1
L125:
	getstatic	wolfisland/numrabbits I
	iconst_0
	if_icmpeq	L126
	iconst_0
	goto	L127
L126:
	iconst_1
L127:
	ior
	ifne	L123
	goto	L122
L123:

	getstatic	wolfisland/_runTimer LRunTimer;
	invokevirtual	RunTimer.printElapsedTime()V

	return

.limit locals 1
.limit stack 4
.end method
