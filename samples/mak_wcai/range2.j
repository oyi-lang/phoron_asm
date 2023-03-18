.class public range2
.super java/lang/Object

.field private static _runTimer LRunTimer;
.field private static _standardIn LPascalTextIn;

.field private static i I

.method public <init>()V

	aload_0
	invokenonvirtual	java/lang/Object/<init>()V
	return

.limit locals 1
.limit stack 1
.end method

.method private static bad(I)V

.var 0 is k I


	nop

	return

.limit locals 1
.limit stack 0
.end method

.method public static main([Ljava/lang/String;)V

	new	RunTimer
	dup
	invokenonvirtual	RunTimer/<init>()V
	putstatic	range2/_runTimer LRunTimer;
	new	PascalTextIn
	dup
	invokenonvirtual	PascalTextIn/<init>()V
	putstatic	range2/_standardIn LPascalTextIn;


.line 14
	bipush	12
	dup
	bipush	10
	bipush	20
	invokestatic	RangeChecker/check(III)V
	putstatic	range2/i I
.line 15
	getstatic	range2/i I
	bipush	10
	isub
	dup
	bipush	10
	bipush	20
	invokestatic	RangeChecker/check(III)V
	invokestatic	range2/bad(I)V

	getstatic	range2/_runTimer LRunTimer;
	invokevirtual	RunTimer.printElapsedTime()V

	return

.limit locals 1
.limit stack 4
.end method
