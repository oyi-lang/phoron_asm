.class public arrays
.super java/lang/Object

.field private static _runTimer LRunTimer;
.field private static _standardIn LPascalTextIn;

.field private static i I
.field private static j I
.field private static k I
.field private static v [I
.field private static vv [[I
.field private static vvv [[[I

.method public <init>()V

	aload_0
	invokenonvirtual	java/lang/Object/<init>()V
	return

.limit locals 1
.limit stack 1
.end method

.method private static domatrix([[I)V

.var 0 is m [[I


.line 16
	iconst_0
	putstatic	arrays/i I
.line 16
L001:
	getstatic	arrays/i I
	iconst_4
	if_icmpgt	L003
	iconst_0
	goto	L004
L003:
	iconst_1
L004:
	ifne	L002
.line 17
	iconst_0
	putstatic	arrays/j I
.line 17
L005:
	getstatic	arrays/j I
	iconst_4
	if_icmpgt	L007
	iconst_0
	goto	L008
L007:
	iconst_1
L008:
	ifne	L006
.line 18
	aload_0
	getstatic	arrays/i I
	aaload
	getstatic	arrays/j I
	getstatic	arrays/i I
	getstatic	arrays/j I
	imul
	iastore
.line 17
	getstatic	arrays/j I
	iconst_1
	iadd
	putstatic	arrays/j I
	goto	L005
L006:
.line 16
	getstatic	arrays/i I
	iconst_1
	iadd
	putstatic	arrays/i I
	goto	L001
L002:

	return

.limit locals 1
.limit stack 4
.end method

.method private static printcube([[[I)V

.var 0 is c [[[I


.line 25
	getstatic	java/lang/System/out Ljava/io/PrintStream;
	invokevirtual	java/io/PrintStream.println()V
.line 26
	iconst_0
	putstatic	arrays/j I
.line 26
L009:
	getstatic	arrays/j I
	iconst_2
	if_icmpgt	L011
	iconst_0
	goto	L012
L011:
	iconst_1
L012:
	ifne	L010
.line 27
	iconst_0
	putstatic	arrays/i I
.line 27
L013:
	getstatic	arrays/i I
	iconst_1
	if_icmpgt	L015
	iconst_0
	goto	L016
L015:
	iconst_1
L016:
	ifne	L014
.line 28
	iconst_0
	putstatic	arrays/k I
.line 28
L017:
	getstatic	arrays/k I
	iconst_3
	if_icmpgt	L019
	iconst_0
	goto	L020
L019:
	iconst_1
L020:
	ifne	L018
.line 29
	getstatic	java/lang/System/out Ljava/io/PrintStream;
	ldc	"%4d"
	iconst_1
	anewarray	java/lang/Object
	dup
	iconst_0
	aload_0
	getstatic	arrays/i I
	aaload
	getstatic	arrays/j I
	aaload
	getstatic	arrays/k I
	iaload
	invokestatic	java/lang/Integer.valueOf(I)Ljava/lang/Integer;
	aastore
	invokestatic	java/lang/String/format(Ljava/lang/String;[Ljava/lang/Object;)Ljava/lang/String;
	invokevirtual	java/io/PrintStream.print(Ljava/lang/String;)V
.line 28
	getstatic	arrays/k I
	iconst_1
	iadd
	putstatic	arrays/k I
	goto	L017
L018:
.line 31
	getstatic	java/lang/System/out Ljava/io/PrintStream;
	ldc	"     "
	invokevirtual	java/io/PrintStream.print(Ljava/lang/String;)V
.line 27
	getstatic	arrays/i I
	iconst_1
	iadd
	putstatic	arrays/i I
	goto	L013
L014:
.line 33
	getstatic	java/lang/System/out Ljava/io/PrintStream;
	invokevirtual	java/io/PrintStream.println()V
.line 26
	getstatic	arrays/j I
	iconst_1
	iadd
	putstatic	arrays/j I
	goto	L009
L010:

	return

.limit locals 1
.limit stack 8
.end method

.method public static main([Ljava/lang/String;)V

	new	RunTimer
	dup
	invokenonvirtual	RunTimer/<init>()V
	putstatic	arrays/_runTimer LRunTimer;
	new	PascalTextIn
	dup
	invokenonvirtual	PascalTextIn/<init>()V
	putstatic	arrays/_standardIn LPascalTextIn;


	bipush	10
	newarray	int
	putstatic	arrays/v [I

	iconst_5
	iconst_5
	multianewarray	[[I 2
	putstatic	arrays/vv [[I

	iconst_2
	iconst_3
	iconst_4
	multianewarray	[[[I 3
	putstatic	arrays/vvv [[[I

.line 38
	getstatic	arrays/v [I
	iconst_0
	iconst_1
	iastore
.line 39
	getstatic	java/lang/System/out Ljava/io/PrintStream;
	ldc	"%d\n"
	iconst_1
	anewarray	java/lang/Object
	dup
	iconst_0
	getstatic	arrays/v [I
	iconst_0
	iaload
	invokestatic	java/lang/Integer.valueOf(I)Ljava/lang/Integer;
	aastore
	invokestatic	java/lang/String/format(Ljava/lang/String;[Ljava/lang/Object;)Ljava/lang/String;
	invokevirtual	java/io/PrintStream.print(Ljava/lang/String;)V
.line 41
	getstatic	arrays/vv [[I
	iconst_3
	aaload
	iconst_4
	bipush	34
	iastore
.line 42
	getstatic	java/lang/System/out Ljava/io/PrintStream;
	ldc	"%d\n"
	iconst_1
	anewarray	java/lang/Object
	dup
	iconst_0
	getstatic	arrays/vv [[I
	iconst_3
	aaload
	iconst_4
	iaload
	invokestatic	java/lang/Integer.valueOf(I)Ljava/lang/Integer;
	aastore
	invokestatic	java/lang/String/format(Ljava/lang/String;[Ljava/lang/Object;)Ljava/lang/String;
	invokevirtual	java/io/PrintStream.print(Ljava/lang/String;)V
.line 44
	getstatic	arrays/vvv [[[I
	iconst_1
	aaload
	iconst_2
	aaload
	iconst_3
	bipush	123
	iastore
.line 45
	getstatic	java/lang/System/out Ljava/io/PrintStream;
	ldc	"%d\n"
	iconst_1
	anewarray	java/lang/Object
	dup
	iconst_0
	getstatic	arrays/vvv [[[I
	iconst_1
	aaload
	iconst_2
	aaload
	iconst_3
	iaload
	invokestatic	java/lang/Integer.valueOf(I)Ljava/lang/Integer;
	aastore
	invokestatic	java/lang/String/format(Ljava/lang/String;[Ljava/lang/Object;)Ljava/lang/String;
	invokevirtual	java/io/PrintStream.print(Ljava/lang/String;)V
.line 47
	iconst_1
	putstatic	arrays/i I
.line 47
L021:
	getstatic	arrays/i I
	bipush	9
	if_icmpgt	L023
	iconst_0
	goto	L024
L023:
	iconst_1
L024:
	ifne	L022
.line 48
	getstatic	arrays/v [I
	getstatic	arrays/i I
	iconst_2
	getstatic	arrays/v [I
	getstatic	arrays/i I
	iconst_1
	isub
	iaload
	imul
	iastore
.line 47
	getstatic	arrays/i I
	iconst_1
	iadd
	putstatic	arrays/i I
	goto	L021
L022:
.line 51
	getstatic	java/lang/System/out Ljava/io/PrintStream;
	invokevirtual	java/io/PrintStream.println()V
.line 52
	iconst_0
	putstatic	arrays/i I
.line 52
L025:
	getstatic	arrays/i I
	bipush	9
	if_icmpgt	L027
	iconst_0
	goto	L028
L027:
	iconst_1
L028:
	ifne	L026
.line 53
	getstatic	java/lang/System/out Ljava/io/PrintStream;
	ldc	"%4d"
	iconst_1
	anewarray	java/lang/Object
	dup
	iconst_0
	getstatic	arrays/v [I
	getstatic	arrays/i I
	iaload
	invokestatic	java/lang/Integer.valueOf(I)Ljava/lang/Integer;
	aastore
	invokestatic	java/lang/String/format(Ljava/lang/String;[Ljava/lang/Object;)Ljava/lang/String;
	invokevirtual	java/io/PrintStream.print(Ljava/lang/String;)V
.line 52
	getstatic	arrays/i I
	iconst_1
	iadd
	putstatic	arrays/i I
	goto	L025
L026:
.line 55
	getstatic	java/lang/System/out Ljava/io/PrintStream;
	invokevirtual	java/io/PrintStream.println()V
.line 57
	getstatic	arrays/vv [[I
	invokestatic	arrays/domatrix([[I)V
.line 59
	getstatic	java/lang/System/out Ljava/io/PrintStream;
	invokevirtual	java/io/PrintStream.println()V
.line 60
	iconst_0
	putstatic	arrays/i I
.line 60
L029:
	getstatic	arrays/i I
	iconst_4
	if_icmpgt	L031
	iconst_0
	goto	L032
L031:
	iconst_1
L032:
	ifne	L030
.line 61
	iconst_0
	putstatic	arrays/j I
.line 61
L033:
	getstatic	arrays/j I
	iconst_4
	if_icmpgt	L035
	iconst_0
	goto	L036
L035:
	iconst_1
L036:
	ifne	L034
.line 62
	getstatic	java/lang/System/out Ljava/io/PrintStream;
	ldc	"%3d"
	iconst_1
	anewarray	java/lang/Object
	dup
	iconst_0
	getstatic	arrays/vv [[I
	getstatic	arrays/i I
	aaload
	getstatic	arrays/j I
	iaload
	invokestatic	java/lang/Integer.valueOf(I)Ljava/lang/Integer;
	aastore
	invokestatic	java/lang/String/format(Ljava/lang/String;[Ljava/lang/Object;)Ljava/lang/String;
	invokevirtual	java/io/PrintStream.print(Ljava/lang/String;)V
.line 61
	getstatic	arrays/j I
	iconst_1
	iadd
	putstatic	arrays/j I
	goto	L033
L034:
.line 64
	getstatic	java/lang/System/out Ljava/io/PrintStream;
	invokevirtual	java/io/PrintStream.println()V
.line 60
	getstatic	arrays/i I
	iconst_1
	iadd
	putstatic	arrays/i I
	goto	L029
L030:
.line 67
	iconst_0
	putstatic	arrays/i I
.line 67
L037:
	getstatic	arrays/i I
	iconst_1
	if_icmpgt	L039
	iconst_0
	goto	L040
L039:
	iconst_1
L040:
	ifne	L038
.line 68
	iconst_0
	putstatic	arrays/j I
.line 68
L041:
	getstatic	arrays/j I
	iconst_2
	if_icmpgt	L043
	iconst_0
	goto	L044
L043:
	iconst_1
L044:
	ifne	L042
.line 69
	iconst_0
	putstatic	arrays/k I
.line 69
L045:
	getstatic	arrays/k I
	iconst_3
	if_icmpgt	L047
	iconst_0
	goto	L048
L047:
	iconst_1
L048:
	ifne	L046
.line 70
	getstatic	arrays/vvv [[[I
	getstatic	arrays/i I
	aaload
	getstatic	arrays/j I
	aaload
	getstatic	arrays/k I
	bipush	100
	getstatic	arrays/i I
	imul
	bipush	10
	getstatic	arrays/j I
	imul
	iadd
	getstatic	arrays/k I
	iadd
	iastore
.line 69
	getstatic	arrays/k I
	iconst_1
	iadd
	putstatic	arrays/k I
	goto	L045
L046:
.line 68
	getstatic	arrays/j I
	iconst_1
	iadd
	putstatic	arrays/j I
	goto	L041
L042:
.line 67
	getstatic	arrays/i I
	iconst_1
	iadd
	putstatic	arrays/i I
	goto	L037
L038:
.line 75
	getstatic	arrays/vvv [[[I
	invokestatic	arrays/printcube([[[I)V

	getstatic	arrays/_runTimer LRunTimer;
	invokevirtual	RunTimer.printElapsedTime()V

	return

.limit locals 1
.limit stack 8
.end method
