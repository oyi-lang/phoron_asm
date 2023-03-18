.class public parms3
.super java/lang/Object

.field private static _runTimer LRunTimer;
.field private static _standardIn LPascalTextIn;

.field private static vvv [[[I

.method public <init>()V

	aload_0
	invokenonvirtual	java/lang/Object/<init>()V
	return

.limit locals 1
.limit stack 1
.end method

.method private static docube1([[[I)V

.var 0 is c [[[I
.var 1 is i I
.var 2 is j I
.var 3 is k I


.line 14
	iconst_0
	istore_1
.line 14
L001:
	iload_1
	iconst_1
	if_icmpgt	L003
	iconst_0
	goto	L004
L003:
	iconst_1
L004:
	ifne	L002
.line 15
	iconst_0
	istore_2
.line 15
L005:
	iload_2
	iconst_2
	if_icmpgt	L007
	iconst_0
	goto	L008
L007:
	iconst_1
L008:
	ifne	L006
.line 16
	iconst_0
	istore_3
.line 16
L009:
	iload_3
	iconst_3
	if_icmpgt	L011
	iconst_0
	goto	L012
L011:
	iconst_1
L012:
	ifne	L010
.line 17
	aload_0
	iload_1
	aaload
	iload_2
	aaload
	iload_3
	bipush	100
	iload_1
	imul
	bipush	10
	iload_2
	imul
	iadd
	iload_3
	iadd
	iastore
.line 16
	iload_3
	iconst_1
	iadd
	istore_3
	goto	L009
L010:
.line 15
	iload_2
	iconst_1
	iadd
	istore_2
	goto	L005
L006:
.line 14
	iload_1
	iconst_1
	iadd
	istore_1
	goto	L001
L002:

	return

.limit locals 4
.limit stack 5
.end method

.method private static docube2([[[I)V

.var 0 is c [[[I
.var 1 is i I
.var 2 is j I
.var 3 is k I


.line 28
	iconst_0
	istore_1
.line 28
L013:
	iload_1
	iconst_1
	if_icmpgt	L015
	iconst_0
	goto	L016
L015:
	iconst_1
L016:
	ifne	L014
.line 29
	iconst_0
	istore_2
.line 29
L017:
	iload_2
	iconst_2
	if_icmpgt	L019
	iconst_0
	goto	L020
L019:
	iconst_1
L020:
	ifne	L018
.line 30
	iconst_0
	istore_3
.line 30
L021:
	iload_3
	iconst_3
	if_icmpgt	L023
	iconst_0
	goto	L024
L023:
	iconst_1
L024:
	ifne	L022
.line 31
	aload_0
	iload_1
	aaload
	iload_2
	aaload
	iload_3
	sipush	200
	iload_1
	imul
	bipush	10
	iload_2
	imul
	iadd
	iload_3
	iadd
	iastore
.line 30
	iload_3
	iconst_1
	iadd
	istore_3
	goto	L021
L022:
.line 29
	iload_2
	iconst_1
	iadd
	istore_2
	goto	L017
L018:
.line 28
	iload_1
	iconst_1
	iadd
	istore_1
	goto	L013
L014:

	return

.limit locals 4
.limit stack 5
.end method

.method private static docube3([[[I)V

.var 0 is c [[[I
.var 1 is i I
.var 2 is j I
.var 3 is k I


.line 42
	iconst_0
	istore_1
.line 42
L025:
	iload_1
	iconst_1
	if_icmpgt	L027
	iconst_0
	goto	L028
L027:
	iconst_1
L028:
	ifne	L026
.line 43
	iconst_0
	istore_2
.line 43
L029:
	iload_2
	iconst_2
	if_icmpgt	L031
	iconst_0
	goto	L032
L031:
	iconst_1
L032:
	ifne	L030
.line 44
	iconst_0
	istore_3
.line 44
L033:
	iload_3
	iconst_3
	if_icmpgt	L035
	iconst_0
	goto	L036
L035:
	iconst_1
L036:
	ifne	L034
.line 45
	aload_0
	iload_1
	aaload
	iload_2
	aaload
	iload_3
	sipush	200
	iload_1
	imul
	bipush	10
	iload_2
	imul
	iadd
	iload_3
	iadd
	iastore
.line 44
	iload_3
	iconst_1
	iadd
	istore_3
	goto	L033
L034:
.line 43
	iload_2
	iconst_1
	iadd
	istore_2
	goto	L029
L030:
.line 42
	iload_1
	iconst_1
	iadd
	istore_1
	goto	L025
L026:

	return

.limit locals 4
.limit stack 5
.end method

.method private static printcube([[[I)V

.var 0 is c [[[I
.var 1 is i I
.var 2 is j I
.var 3 is k I


.line 56
	getstatic	java/lang/System/out Ljava/io/PrintStream;
	invokevirtual	java/io/PrintStream.println()V
.line 57
	iconst_0
	istore_2
.line 57
L037:
	iload_2
	iconst_2
	if_icmpgt	L039
	iconst_0
	goto	L040
L039:
	iconst_1
L040:
	ifne	L038
.line 58
	iconst_0
	istore_1
.line 58
L041:
	iload_1
	iconst_1
	if_icmpgt	L043
	iconst_0
	goto	L044
L043:
	iconst_1
L044:
	ifne	L042
.line 59
	iconst_0
	istore_3
.line 59
L045:
	iload_3
	iconst_3
	if_icmpgt	L047
	iconst_0
	goto	L048
L047:
	iconst_1
L048:
	ifne	L046
.line 60
	getstatic	java/lang/System/out Ljava/io/PrintStream;
	ldc	"%4d"
	iconst_1
	anewarray	java/lang/Object
	dup
	iconst_0
	aload_0
	iload_1
	aaload
	iload_2
	aaload
	iload_3
	iaload
	invokestatic	java/lang/Integer.valueOf(I)Ljava/lang/Integer;
	aastore
	invokestatic	java/lang/String/format(Ljava/lang/String;[Ljava/lang/Object;)Ljava/lang/String;
	invokevirtual	java/io/PrintStream.print(Ljava/lang/String;)V
.line 59
	iload_3
	iconst_1
	iadd
	istore_3
	goto	L045
L046:
.line 62
	getstatic	java/lang/System/out Ljava/io/PrintStream;
	ldc	"     "
	invokevirtual	java/io/PrintStream.print(Ljava/lang/String;)V
.line 58
	iload_1
	iconst_1
	iadd
	istore_1
	goto	L041
L042:
.line 64
	getstatic	java/lang/System/out Ljava/io/PrintStream;
	invokevirtual	java/io/PrintStream.println()V
.line 57
	iload_2
	iconst_1
	iadd
	istore_2
	goto	L037
L038:

	return

.limit locals 4
.limit stack 8
.end method

.method public static main([Ljava/lang/String;)V

	new	RunTimer
	dup
	invokenonvirtual	RunTimer/<init>()V
	putstatic	parms3/_runTimer LRunTimer;
	new	PascalTextIn
	dup
	invokenonvirtual	PascalTextIn/<init>()V
	putstatic	parms3/_standardIn LPascalTextIn;


	iconst_2
	iconst_3
	iconst_4
	multianewarray	[[[I 3
	putstatic	parms3/vvv [[[I

.line 69
	getstatic	parms3/vvv [[[I
	invokestatic	parms3/docube1([[[I)V
.line 70
	getstatic	parms3/vvv [[[I
	invokestatic	parms3/printcube([[[I)V
.line 72
	getstatic	parms3/vvv [[[I
	invokestatic	Cloner.deepClone(Ljava/lang/Object;)Ljava/lang/Object;
	checkcast	[[[I
	invokestatic	parms3/docube2([[[I)V
.line 73
	getstatic	parms3/vvv [[[I
	invokestatic	parms3/printcube([[[I)V
.line 75
	getstatic	parms3/vvv [[[I
	invokestatic	parms3/docube3([[[I)V
.line 76
	getstatic	parms3/vvv [[[I
	invokestatic	parms3/printcube([[[I)V

	getstatic	parms3/_runTimer LRunTimer;
	invokevirtual	RunTimer.printElapsedTime()V

	return

.limit locals 1
.limit stack 4
.end method
