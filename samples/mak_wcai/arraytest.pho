.class public arraytest
.super java/lang/Object

.field private static _runTimer LRunTimer;
.field private static _standardIn LPascalTextIn;

.field private static a1 [I
.field private static a2 [[I
.field private static a3 [[[I
.field private static i I
.field private static j I
.field private static k I
.field private static n I

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
	putstatic	arraytest/_runTimer LRunTimer;
	new	PascalTextIn
	dup
	invokenonvirtual	PascalTextIn/<init>()V
	putstatic	arraytest/_standardIn LPascalTextIn;


	bipush	10
	newarray	int
	putstatic	arraytest/a1 [I

	iconst_5
	iconst_5
	multianewarray	[[I 2
	putstatic	arraytest/a2 [[I

	iconst_2
	iconst_3
	iconst_4
	multianewarray	[[[I 3
	putstatic	arraytest/a3 [[[I

.line 15
	iconst_0
	putstatic	arraytest/i I
.line 15
	iconst_0
	putstatic	arraytest/j I
.line 15
	iconst_0
	putstatic	arraytest/k I
.line 17
	getstatic	arraytest/a1 [I
	getstatic	arraytest/i I
	iaload
	putstatic	arraytest/j I
.line 18
	getstatic	arraytest/a2 [[I
	getstatic	arraytest/i I
	aaload
	getstatic	arraytest/j I
	iaload
	putstatic	arraytest/k I
.line 19
	getstatic	arraytest/a3 [[[I
	getstatic	arraytest/i I
	aaload
	getstatic	arraytest/j I
	aaload
	getstatic	arraytest/k I
	iaload
	putstatic	arraytest/n I
.line 21
	getstatic	arraytest/a1 [I
	getstatic	arraytest/i I
	getstatic	arraytest/j I
	iastore
.line 22
	getstatic	arraytest/a2 [[I
	getstatic	arraytest/i I
	aaload
	getstatic	arraytest/j I
	getstatic	arraytest/k I
	iastore
.line 23
	getstatic	arraytest/a3 [[[I
	getstatic	arraytest/i I
	aaload
	getstatic	arraytest/j I
	aaload
	getstatic	arraytest/k I
	getstatic	arraytest/n I
	iastore
.line 25
	getstatic	arraytest/a3 [[[I
	getstatic	arraytest/i I
	aaload
	getstatic	arraytest/a1 [I
	getstatic	arraytest/j I
	iaload
	aaload
	getstatic	arraytest/k I
	getstatic	arraytest/a2 [[I
	getstatic	arraytest/i I
	aaload
	getstatic	arraytest/j I
	iaload
	getstatic	arraytest/a3 [[[I
	getstatic	arraytest/k I
	aaload
	iconst_2
	getstatic	arraytest/n I
	imul
	aaload
	getstatic	arraytest/k I
	iconst_1
	iadd
	iaload
	isub
	iastore

	getstatic	arraytest/_runTimer LRunTimer;
	invokevirtual	RunTimer.printElapsedTime()V

	return

.limit locals 1
.limit stack 6
.end method
