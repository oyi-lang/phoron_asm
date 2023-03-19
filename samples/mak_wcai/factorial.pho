.class public factorial
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

.method private static fact(I)I

.var 0 is n I
.var 1 is fact I


.line 9
	iload_0
	iconst_1
	if_icmple	L002
	iconst_0
	goto	L003
L002:
	iconst_1
L003:
	ifeq	L004
.line 9
	iconst_1
	istore_1
	goto	L001
L004:
.line 10
	iload_0
	iload_0
	iconst_1
	isub
	invokestatic	factorial/fact(I)I
	imul
	istore_1
L001:

	iload_1
	ireturn

.limit locals 2
.limit stack 3
.end method

.method public static main([Ljava/lang/String;)V

	new	RunTimer
	dup
	invokenonvirtual	RunTimer/<init>()V
	putstatic	factorial/_runTimer LRunTimer;
	new	PascalTextIn
	dup
	invokenonvirtual	PascalTextIn/<init>()V
	putstatic	factorial/_standardIn LPascalTextIn;


.line 14
	iconst_0
	putstatic	factorial/number I
.line 15
L005:
.line 16
	getstatic	java/lang/System/out Ljava/io/PrintStream;
	ldc	"number = %1d   number! = %5d\n"
	iconst_2
	anewarray	java/lang/Object
	dup
	iconst_0
	getstatic	factorial/number I
	invokestatic	java/lang/Integer.valueOf(I)Ljava/lang/Integer;
	aastore
	dup
	iconst_1
	getstatic	factorial/number I
	invokestatic	factorial/fact(I)I
	invokestatic	java/lang/Integer.valueOf(I)Ljava/lang/Integer;
	aastore
	invokestatic	java/lang/String/format(Ljava/lang/String;[Ljava/lang/Object;)Ljava/lang/String;
	invokevirtual	java/io/PrintStream.print(Ljava/lang/String;)V
.line 18
	getstatic	factorial/number I
	iconst_1
	iadd
	putstatic	factorial/number I
	getstatic	factorial/number I
	bipush	7
	if_icmpgt	L007
	iconst_0
	goto	L008
L007:
	iconst_1
L008:
	ifne	L006
	goto	L005
L006:

	getstatic	factorial/_runTimer LRunTimer;
	invokevirtual	RunTimer.printElapsedTime()V

	return

.limit locals 1
.limit stack 7
.end method
