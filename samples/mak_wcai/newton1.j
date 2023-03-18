.class public newton1
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


.line 17
	fconst_1
	fstore_1
.line 19
	fload_0
	fload_1
	fdiv
	fload_1
	fadd
	iconst_2
	i2f
	fdiv
	fstore_1
.line 20
	fload_0
	fload_1
	fdiv
	fload_1
	fadd
	iconst_2
	i2f
	fdiv
	fstore_1
.line 21
	fload_0
	fload_1
	fdiv
	fload_1
	fadd
	iconst_2
	i2f
	fdiv
	fstore_1
.line 22
	fload_0
	fload_1
	fdiv
	fload_1
	fadd
	iconst_2
	i2f
	fdiv
	fstore_1
.line 23
	fload_0
	fload_1
	fdiv
	fload_1
	fadd
	iconst_2
	i2f
	fdiv
	fstore_1
.line 24
	fload_0
	fload_1
	fdiv
	fload_1
	fadd
	iconst_2
	i2f
	fdiv
	fstore_1
.line 25
	fload_0
	fload_1
	fdiv
	fload_1
	fadd
	iconst_2
	i2f
	fdiv
	fstore_1
.line 26
	fload_0
	fload_1
	fdiv
	fload_1
	fadd
	iconst_2
	i2f
	fdiv
	fstore_1
.line 28
	fload_1
	fstore_2

	fload_2
	freturn

.limit locals 3
.limit stack 2
.end method

.method private static print(IF)V

.var 0 is n I
.var 1 is root F


.line 33
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
	putstatic	newton1/_runTimer LRunTimer;
	new	PascalTextIn
	dup
	invokenonvirtual	PascalTextIn/<init>()V
	putstatic	newton1/_standardIn LPascalTextIn;


.line 37
	getstatic	java/lang/System/out Ljava/io/PrintStream;
	invokevirtual	java/io/PrintStream.println()V
.line 38
	getstatic	java/lang/System/out Ljava/io/PrintStream;
	ldc	"Enter a new number: "
	invokevirtual	java/io/PrintStream.print(Ljava/lang/String;)V
.line 39
	getstatic	newton1/_standardIn LPascalTextIn;
	invokevirtual	PascalTextIn.readInteger()I
	dup
	iconst_0
	sipush	32767
	invokestatic	RangeChecker/check(III)V
	putstatic	newton1/number I
.line 40
	getstatic	newton1/number I
	getstatic	newton1/number I
	i2f
	invokestatic	newton1/root(F)F
	invokestatic	newton1/print(IF)V
.line 42
	getstatic	java/lang/System/out Ljava/io/PrintStream;
	invokevirtual	java/io/PrintStream.println()V
.line 43
	getstatic	java/lang/System/out Ljava/io/PrintStream;
	ldc	"Enter a new number: "
	invokevirtual	java/io/PrintStream.print(Ljava/lang/String;)V
.line 44
	getstatic	newton1/_standardIn LPascalTextIn;
	invokevirtual	PascalTextIn.readInteger()I
	dup
	iconst_0
	sipush	32767
	invokestatic	RangeChecker/check(III)V
	putstatic	newton1/number I
.line 45
	getstatic	newton1/number I
	getstatic	newton1/number I
	i2f
	invokestatic	newton1/root(F)F
	invokestatic	newton1/print(IF)V

	getstatic	newton1/_runTimer LRunTimer;
	invokevirtual	RunTimer.printElapsedTime()V

	return

.limit locals 1
.limit stack 4
.end method
