.class public parms1
.super java/lang/Object

.field private static _runTimer LRunTimer;
.field private static _standardIn LPascalTextIn;

.field private static i I
.field private static j I
.field private static x F
.field private static y F

.method public <init>()V

	aload_0
	invokenonvirtual	java/lang/Object/<init>()V
	return

.limit locals 1
.limit stack 1
.end method

.method private static goodswap(LIWrap;LIWrap;)V

.var 2 is t I
.var 0 is v1 LIWrap;
.var 1 is v2 LIWrap;


.line 12
	aload_0
	getfield	IWrap/value I
	istore_2
.line 13
	aload_0
	aload_1
	getfield	IWrap/value I
	putfield	IWrap/value I
.line 14
	aload_1
	iload_2
	putfield	IWrap/value I

	return

.limit locals 3
.limit stack 2
.end method

.method private static badswap(FF)V

.var 2 is t F
.var 0 is v1 F
.var 1 is v2 F


.line 22
	fload_0
	fstore_2
.line 23
	fload_1
	fstore_0
.line 24
	fload_2
	fstore_1

	return

.limit locals 3
.limit stack 1
.end method

.method public static main([Ljava/lang/String;)V

	new	RunTimer
	dup
	invokenonvirtual	RunTimer/<init>()V
	putstatic	parms1/_runTimer LRunTimer;
	new	PascalTextIn
	dup
	invokenonvirtual	PascalTextIn/<init>()V
	putstatic	parms1/_standardIn LPascalTextIn;


.line 28
	bipush	10
	putstatic	parms1/i I
.line 29
	bipush	20
	putstatic	parms1/j I
.line 30
	getstatic	java/lang/System/out Ljava/io/PrintStream;
	ldc	"Calling goodSwap: i = %1d, j = %1d\n"
	iconst_2
	anewarray	java/lang/Object
	dup
	iconst_0
	getstatic	parms1/i I
	invokestatic	java/lang/Integer.valueOf(I)Ljava/lang/Integer;
	aastore
	dup
	iconst_1
	getstatic	parms1/j I
	invokestatic	java/lang/Integer.valueOf(I)Ljava/lang/Integer;
	aastore
	invokestatic	java/lang/String/format(Ljava/lang/String;[Ljava/lang/Object;)Ljava/lang/String;
	invokevirtual	java/io/PrintStream.print(Ljava/lang/String;)V
.line 31
	new	IWrap
	dup
	getstatic	parms1/i I
	invokenonvirtual	IWrap/<init>(I)V
	dup
	astore_1
	new	IWrap
	dup
	getstatic	parms1/j I
	invokenonvirtual	IWrap/<init>(I)V
	dup
	astore_2
	invokestatic	parms1/goodswap(LIWrap;LIWrap;)V
	aload_1
	getfield	IWrap/value I
	putstatic	parms1/i I
	aload_2
	getfield	IWrap/value I
	putstatic	parms1/j I
.line 32
	getstatic	java/lang/System/out Ljava/io/PrintStream;
	ldc	"          Result: i = %1d, j = %1d\n"
	iconst_2
	anewarray	java/lang/Object
	dup
	iconst_0
	getstatic	parms1/i I
	invokestatic	java/lang/Integer.valueOf(I)Ljava/lang/Integer;
	aastore
	dup
	iconst_1
	getstatic	parms1/j I
	invokestatic	java/lang/Integer.valueOf(I)Ljava/lang/Integer;
	aastore
	invokestatic	java/lang/String/format(Ljava/lang/String;[Ljava/lang/Object;)Ljava/lang/String;
	invokevirtual	java/io/PrintStream.print(Ljava/lang/String;)V
.line 34
	ldc	10.0
	putstatic	parms1/x F
.line 35
	ldc	20.0
	putstatic	parms1/y F
.line 36
	getstatic	java/lang/System/out Ljava/io/PrintStream;
	ldc	"Calling  badSwap: x = %1.1f, y = %1.1f\n"
	iconst_2
	anewarray	java/lang/Object
	dup
	iconst_0
	getstatic	parms1/x F
	invokestatic	java/lang/Float.valueOf(F)Ljava/lang/Float;
	aastore
	dup
	iconst_1
	getstatic	parms1/y F
	invokestatic	java/lang/Float.valueOf(F)Ljava/lang/Float;
	aastore
	invokestatic	java/lang/String/format(Ljava/lang/String;[Ljava/lang/Object;)Ljava/lang/String;
	invokevirtual	java/io/PrintStream.print(Ljava/lang/String;)V
.line 37
	getstatic	parms1/x F
	getstatic	parms1/y F
	invokestatic	parms1/badswap(FF)V
.line 38
	getstatic	java/lang/System/out Ljava/io/PrintStream;
	ldc	"          Result: x = %1.1f, y = %1.1f\n"
	iconst_2
	anewarray	java/lang/Object
	dup
	iconst_0
	getstatic	parms1/x F
	invokestatic	java/lang/Float.valueOf(F)Ljava/lang/Float;
	aastore
	dup
	iconst_1
	getstatic	parms1/y F
	invokestatic	java/lang/Float.valueOf(F)Ljava/lang/Float;
	aastore
	invokestatic	java/lang/String/format(Ljava/lang/String;[Ljava/lang/Object;)Ljava/lang/String;
	invokevirtual	java/io/PrintStream.print(Ljava/lang/String;)V

	getstatic	parms1/_runTimer LRunTimer;
	invokevirtual	RunTimer.printElapsedTime()V

	return

.limit locals 3
.limit stack 7
.end method
