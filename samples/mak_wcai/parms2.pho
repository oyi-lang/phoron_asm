.class public parms2
.super java/lang/Object

.field private static _runTimer LRunTimer;
.field private static _standardIn LPascalTextIn;

.field private static i I
.field private static j I

.method public <init>()V

	aload_0
	invokenonvirtual	java/lang/Object/<init>()V
	return

.limit locals 1
.limit stack 1
.end method

.method private static proc3(LIWrap;)V

.var 0 is p3 LIWrap;


.line 8
	getstatic	parms2/i I
	iconst_1
	iadd
	putstatic	parms2/i I
.line 9
	aload_0
	getstatic	parms2/i I
	putfield	IWrap/value I

	return

.limit locals 1
.limit stack 2
.end method

.method private static proc2(LIWrap;)V

.var 1 is j I
.var 0 is p2 LIWrap;


.line 16
	getstatic	parms2/i I
	iconst_1
	iadd
	putstatic	parms2/i I
.line 17
	getstatic	parms2/i I
	istore_1
.line 18
	new	IWrap
	dup
	iload_1
	invokenonvirtual	IWrap/<init>(I)V
	dup
	astore_2
	invokestatic	parms2/proc3(LIWrap;)V
	aload_2
	getfield	IWrap/value I
	istore_1
.line 19
	aload_0
	iload_1
	putfield	IWrap/value I

	return

.limit locals 3
.limit stack 3
.end method

.method private static proc1(LIWrap;)V

.var 0 is p1 LIWrap;


.line 24
	getstatic	parms2/i I
	iconst_1
	iadd
	putstatic	parms2/i I
.line 25
	aload_0
	getstatic	parms2/i I
	putfield	IWrap/value I
.line 26
	aload_0
	invokestatic	parms2/proc2(LIWrap;)V

	return

.limit locals 1
.limit stack 2
.end method

.method public static main([Ljava/lang/String;)V

	new	RunTimer
	dup
	invokenonvirtual	RunTimer/<init>()V
	putstatic	parms2/_runTimer LRunTimer;
	new	PascalTextIn
	dup
	invokenonvirtual	PascalTextIn/<init>()V
	putstatic	parms2/_standardIn LPascalTextIn;


.line 30
	iconst_0
	putstatic	parms2/i I
.line 31
	iconst_0
	putstatic	parms2/j I
.line 32
	getstatic	java/lang/System/out Ljava/io/PrintStream;
	ldc	"i = %d, j = %d\n"
	iconst_2
	anewarray	java/lang/Object
	dup
	iconst_0
	getstatic	parms2/i I
	invokestatic	java/lang/Integer.valueOf(I)Ljava/lang/Integer;
	aastore
	dup
	iconst_1
	getstatic	parms2/j I
	invokestatic	java/lang/Integer.valueOf(I)Ljava/lang/Integer;
	aastore
	invokestatic	java/lang/String/format(Ljava/lang/String;[Ljava/lang/Object;)Ljava/lang/String;
	invokevirtual	java/io/PrintStream.print(Ljava/lang/String;)V
.line 34
	new	IWrap
	dup
	getstatic	parms2/j I
	invokenonvirtual	IWrap/<init>(I)V
	dup
	astore_1
	invokestatic	parms2/proc1(LIWrap;)V
	aload_1
	getfield	IWrap/value I
	putstatic	parms2/j I
.line 36
	getstatic	java/lang/System/out Ljava/io/PrintStream;
	ldc	"i = %d, j = %d\n"
	iconst_2
	anewarray	java/lang/Object
	dup
	iconst_0
	getstatic	parms2/i I
	invokestatic	java/lang/Integer.valueOf(I)Ljava/lang/Integer;
	aastore
	dup
	iconst_1
	getstatic	parms2/j I
	invokestatic	java/lang/Integer.valueOf(I)Ljava/lang/Integer;
	aastore
	invokestatic	java/lang/String/format(Ljava/lang/String;[Ljava/lang/Object;)Ljava/lang/String;
	invokevirtual	java/io/PrintStream.print(Ljava/lang/String;)V

	getstatic	parms2/_runTimer LRunTimer;
	invokevirtual	RunTimer.printElapsedTime()V

	return

.limit locals 2
.limit stack 7
.end method
