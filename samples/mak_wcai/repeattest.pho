.class public repeattest
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

.method private static sqroot(I)F

.var 0 is n I
.var 1 is r F
.var 2 is sqroot F


.line 15
	iload_0
	i2f
	fstore_1
.line 17
L001:
.line 18
	iload_0
	i2f
	fload_1
	fdiv
	fload_1
	fadd
	iconst_2
	i2f
	fdiv
	fstore_1
	fload_1
	fload_1
	fmul
	iload_0
	i2f
	fsub
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
.line 21
	fload_1
	fstore_2

	fload_2
	freturn

.limit locals 3
.limit stack 2
.end method

.method public static main([Ljava/lang/String;)V

	new	RunTimer
	dup
	invokenonvirtual	RunTimer/<init>()V
	putstatic	repeattest/_runTimer LRunTimer;
	new	PascalTextIn
	dup
	invokenonvirtual	PascalTextIn/<init>()V
	putstatic	repeattest/_standardIn LPascalTextIn;


.line 25
	sipush	1024
	putstatic	repeattest/number I
.line 27
	getstatic	java/lang/System/out Ljava/io/PrintStream;
	ldc	"The square root of %4d\n"
	iconst_1
	anewarray	java/lang/Object
	dup
	iconst_0
	getstatic	repeattest/number I
	invokestatic	java/lang/Integer.valueOf(I)Ljava/lang/Integer;
	aastore
	invokestatic	java/lang/String/format(Ljava/lang/String;[Ljava/lang/Object;)Ljava/lang/String;
	invokevirtual	java/io/PrintStream.print(Ljava/lang/String;)V
.line 28
	getstatic	java/lang/System/out Ljava/io/PrintStream;
	ldc	"    by standard sqrt() function: %9.6f\n"
	iconst_1
	anewarray	java/lang/Object
	dup
	iconst_0
	getstatic	repeattest/number I
	i2d
	invokestatic	java/lang/Math/sqrt(D)D
	d2f
	invokestatic	java/lang/Float.valueOf(F)Ljava/lang/Float;
	aastore
	invokestatic	java/lang/String/format(Ljava/lang/String;[Ljava/lang/Object;)Ljava/lang/String;
	invokevirtual	java/io/PrintStream.print(Ljava/lang/String;)V
.line 29
	getstatic	java/lang/System/out Ljava/io/PrintStream;
	ldc	"  by declared sqroot() function: %9.6f\n"
	iconst_1
	anewarray	java/lang/Object
	dup
	iconst_0
	getstatic	repeattest/number I
	invokestatic	repeattest/sqroot(I)F
	invokestatic	java/lang/Float.valueOf(F)Ljava/lang/Float;
	aastore
	invokestatic	java/lang/String/format(Ljava/lang/String;[Ljava/lang/Object;)Ljava/lang/String;
	invokevirtual	java/io/PrintStream.print(Ljava/lang/String;)V

	getstatic	repeattest/_runTimer LRunTimer;
	invokevirtual	RunTimer.printElapsedTime()V

	return

.limit locals 1
.limit stack 8
.end method
