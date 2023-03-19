.class public newton2
.super java/lang/Object

.field private static _runTimer LRunTimer;
.field private static _standardIn LPascalTextIn;

.field private static number I

.method public <init>()V

	aload_0
	invokenonvirtual	java/lang/Object/<init>()V
	return

.limit locals 1
.limit stack 1
.end method

.method private static root(F)F

.var 1 is r F
.var 0 is x F
.var 2 is root F


.line 14
	fconst_1
	fstore_1
.line 15
L001:
.line 16
	fload_0
	fload_1
	fdiv
	fload_1
	fadd
	iconst_2
	i2f
	fdiv
	fstore_1
	fload_0
	fload_1
	dup
	fmul
	fdiv
	iconst_1
	i2f
	fsub
	invokestatic	java/lang/Math/abs(F)F
	ldc	1.0E-6
	fcmpg
	iflt	L003
	iconst_0
	goto	L004
L003:
	iconst_1
L004:
	ifne	L002
	goto	L001
L002:
.line 18
	fload_1
	fstore_2

	fload_2
	freturn

.limit locals 3
.limit stack 3
.end method

.method private static print(IF)V

.var 0 is n I
.var 1 is root F


.line 23
	getstatic	java/lang/System/out Ljava/io/PrintStream;
	ldc	"The square root of %4d is %8.4f\n"
	iconst_2
	anewarray	java/lang/Object
	dup
	iconst_0
	iload_0
	invokestatic	java/lang/Integer.valueOf(I)Ljava/lang/Integer;
	aastore
	dup
	iconst_1
	fload_1
	invokestatic	java/lang/Float.valueOf(F)Ljava/lang/Float;
	aastore
	invokestatic	java/lang/String/format(Ljava/lang/String;[Ljava/lang/Object;)Ljava/lang/String;
	invokevirtual	java/io/PrintStream.print(Ljava/lang/String;)V

	return

.limit locals 2
.limit stack 7
.end method

.method public static main([Ljava/lang/String;)V

	new	RunTimer
	dup
	invokenonvirtual	RunTimer/<init>()V
	putstatic	newton2/_runTimer LRunTimer;
	new	PascalTextIn
	dup
	invokenonvirtual	PascalTextIn/<init>()V
	putstatic	newton2/_standardIn LPascalTextIn;


.line 27
L005:
.line 28
	getstatic	java/lang/System/out Ljava/io/PrintStream;
	invokevirtual	java/io/PrintStream.println()V
.line 29
	getstatic	java/lang/System/out Ljava/io/PrintStream;
	ldc	"Enter new number (0 to quit): "
	invokevirtual	java/io/PrintStream.print(Ljava/lang/String;)V
.line 30
	getstatic	newton2/_standardIn LPascalTextIn;
	invokevirtual	PascalTextIn.readInteger()I
	putstatic	newton2/number I
.line 32
	getstatic	newton2/number I
	iconst_0
	if_icmpeq	L008
	iconst_0
	goto	L009
L008:
	iconst_1
L009:
	ifeq	L010
.line 33
	getstatic	newton2/number I
	fconst_0
	invokestatic	newton2/print(IF)V
	goto	L007
L010:
.line 35
	getstatic	newton2/number I
	iconst_0
	if_icmplt	L012
	iconst_0
	goto	L013
L012:
	iconst_1
L013:
	ifeq	L014
.line 36
	getstatic	java/lang/System/out Ljava/io/PrintStream;
	ldc	"*** ERROR:  number < 0\n"
	invokevirtual	java/io/PrintStream.print(Ljava/lang/String;)V
	goto	L011
L014:
.line 39
	getstatic	newton2/number I
	getstatic	newton2/number I
	i2f
	invokestatic	newton2/root(F)F
	invokestatic	newton2/print(IF)V
L011:
L007:
	getstatic	newton2/number I
	iconst_0
	if_icmpeq	L015
	iconst_0
	goto	L016
L015:
	iconst_1
L016:
	ifne	L006
	goto	L005
L006:

	getstatic	newton2/_runTimer LRunTimer;
	invokevirtual	RunTimer.printElapsedTime()V

	return

.limit locals 1
.limit stack 3
.end method
