.class public allocarraytest1
.super java/lang/Object

.field private static _runTimer LRunTimer;
.field private static _standardIn LPascalTextIn;

.field private static a1 [I
.field private static a2 [[I
.field private static a3 [[[I

.method public <init>()V

	aload_0
	invokenonvirtual	java/lang/Object/<init>()V
	return

.limit locals 1
.limit stack 1
.end method

.method public static main([Ljava/lang/String;)V

	new	RunTimer
	dup
	invokenonvirtual	RunTimer/<init>()V
	putstatic	allocarraytest1/_runTimer LRunTimer;
	new	PascalTextIn
	dup
	invokenonvirtual	PascalTextIn/<init>()V
	putstatic	allocarraytest1/_standardIn LPascalTextIn;


	bipush	10
	newarray	int
	putstatic	allocarraytest1/a1 [I

	iconst_5
	iconst_4
	multianewarray	[[I 2
	putstatic	allocarraytest1/a2 [[I

	iconst_2
	iconst_3
	iconst_4
	multianewarray	[[[I 3
	putstatic	allocarraytest1/a3 [[[I

	nop

	getstatic	allocarraytest1/_runTimer LRunTimer;
	invokevirtual	RunTimer.printElapsedTime()V

	return

.limit locals 1
.limit stack 4
.end method
