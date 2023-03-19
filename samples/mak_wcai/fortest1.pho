.class public fortest1
.super java/lang/Object

.field private static _runTimer LRunTimer;
.field private static _standardIn LPascalTextIn;

.field private static ch C
.field private static grade I
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
	putstatic	fortest1/_runTimer LRunTimer;
	new	PascalTextIn
	dup
	invokenonvirtual	PascalTextIn/<init>()V
	putstatic	fortest1/_standardIn LPascalTextIn;


.line 12
	iconst_2
	putstatic	fortest1/j I
.line 14
	getstatic	fortest1/j I
	putstatic	fortest1/k I
.line 14
L001:
	getstatic	fortest1/k I
	iconst_5
	if_icmpgt	L003
	iconst_0
	goto	L004
L003:
	iconst_1
L004:
	ifne	L002
.line 15
	getstatic	fortest1/k I
	putstatic	fortest1/n I
.line 16
	getstatic	java/lang/System/out Ljava/io/PrintStream;
	ldc	"k = %d, n = %d\n"
	iconst_2
	anewarray	java/lang/Object
	dup
	iconst_0
	getstatic	fortest1/k I
	invokestatic	java/lang/Integer.valueOf(I)Ljava/lang/Integer;
	aastore
	dup
	iconst_1
	getstatic	fortest1/n I
	invokestatic	java/lang/Integer.valueOf(I)Ljava/lang/Integer;
	aastore
	invokestatic	java/lang/String/format(Ljava/lang/String;[Ljava/lang/Object;)Ljava/lang/String;
	invokevirtual	java/io/PrintStream.print(Ljava/lang/String;)V
.line 14
	getstatic	fortest1/k I
	iconst_1
	iadd
	putstatic	fortest1/k I
	goto	L001
L002:
.line 19
	getstatic	java/lang/System/out Ljava/io/PrintStream;
	invokevirtual	java/io/PrintStream.println()V
.line 20
	getstatic	fortest1/n I
	iconst_1
	iadd
	putstatic	fortest1/k I
.line 20
L005:
	getstatic	fortest1/k I
	getstatic	fortest1/j I
	iconst_2
	iadd
	if_icmplt	L007
	iconst_0
	goto	L008
L007:
	iconst_1
L008:
	ifne	L006
.line 20
	getstatic	java/lang/System/out Ljava/io/PrintStream;
	ldc	"k = %d\n"
	iconst_1
	anewarray	java/lang/Object
	dup
	iconst_0
	getstatic	fortest1/k I
	invokestatic	java/lang/Integer.valueOf(I)Ljava/lang/Integer;
	aastore
	invokestatic	java/lang/String/format(Ljava/lang/String;[Ljava/lang/Object;)Ljava/lang/String;
	invokevirtual	java/io/PrintStream.print(Ljava/lang/String;)V
.line 20
	getstatic	fortest1/k I
	iconst_1
	isub
	putstatic	fortest1/k I
	goto	L005
L006:
.line 22
	getstatic	java/lang/System/out Ljava/io/PrintStream;
	invokevirtual	java/io/PrintStream.println()V
.line 23
	iconst_1
	putstatic	fortest1/i I
.line 23
L009:
	getstatic	fortest1/i I
	iconst_2
	if_icmpgt	L011
	iconst_0
	goto	L012
L011:
	iconst_1
L012:
	ifne	L010
.line 24
	iconst_1
	putstatic	fortest1/j I
.line 24
L013:
	getstatic	fortest1/j I
	iconst_3
	if_icmpgt	L015
	iconst_0
	goto	L016
L015:
	iconst_1
L016:
	ifne	L014
.line 25
	getstatic	fortest1/i I
	getstatic	fortest1/j I
	imul
	putstatic	fortest1/k I
.line 26
	getstatic	java/lang/System/out Ljava/io/PrintStream;
	ldc	"i = %d, j = %d, k = %d\n"
	iconst_3
	anewarray	java/lang/Object
	dup
	iconst_0
	getstatic	fortest1/i I
	invokestatic	java/lang/Integer.valueOf(I)Ljava/lang/Integer;
	aastore
	dup
	iconst_1
	getstatic	fortest1/j I
	invokestatic	java/lang/Integer.valueOf(I)Ljava/lang/Integer;
	aastore
	dup
	iconst_2
	getstatic	fortest1/k I
	invokestatic	java/lang/Integer.valueOf(I)Ljava/lang/Integer;
	aastore
	invokestatic	java/lang/String/format(Ljava/lang/String;[Ljava/lang/Object;)Ljava/lang/String;
	invokevirtual	java/io/PrintStream.print(Ljava/lang/String;)V
.line 24
	getstatic	fortest1/j I
	iconst_1
	iadd
	putstatic	fortest1/j I
	goto	L013
L014:
.line 23
	getstatic	fortest1/i I
	iconst_1
	iadd
	putstatic	fortest1/i I
	goto	L009
L010:
.line 30
	getstatic	java/lang/System/out Ljava/io/PrintStream;
	invokevirtual	java/io/PrintStream.println()V
.line 31
	iconst_4
	putstatic	fortest1/grade I
.line 31
L017:
	getstatic	fortest1/grade I
	iconst_0
	if_icmplt	L019
	iconst_0
	goto	L020
L019:
	iconst_1
L020:
	ifne	L018
.line 31
	getstatic	java/lang/System/out Ljava/io/PrintStream;
	ldc	"%2d"
	iconst_1
	anewarray	java/lang/Object
	dup
	iconst_0
	getstatic	fortest1/grade I
	invokestatic	java/lang/Integer.valueOf(I)Ljava/lang/Integer;
	aastore
	invokestatic	java/lang/String/format(Ljava/lang/String;[Ljava/lang/Object;)Ljava/lang/String;
	invokevirtual	java/io/PrintStream.print(Ljava/lang/String;)V
.line 31
	getstatic	fortest1/grade I
	iconst_1
	isub
	putstatic	fortest1/grade I
	goto	L017
L018:
.line 32
	getstatic	java/lang/System/out Ljava/io/PrintStream;
	invokevirtual	java/io/PrintStream.println()V
.line 34
	bipush	120
	putstatic	fortest1/ch C
.line 36
	getstatic	java/lang/System/out Ljava/io/PrintStream;
	invokevirtual	java/io/PrintStream.println()V
.line 37
	getstatic	fortest1/ch C
	putstatic	fortest1/i I
.line 37
L021:
	getstatic	fortest1/i I
	bipush	122
	if_icmpgt	L023
	iconst_0
	goto	L024
L023:
	iconst_1
L024:
	ifne	L022
.line 37
	getstatic	java/lang/System/out Ljava/io/PrintStream;
	ldc	"%c"
	iconst_1
	anewarray	java/lang/Object
	dup
	iconst_0
	getstatic	fortest1/i I
	i2c
	invokestatic	java/lang/Character.valueOf(C)Ljava/lang/Character;
	aastore
	invokestatic	java/lang/String/format(Ljava/lang/String;[Ljava/lang/Object;)Ljava/lang/String;
	invokevirtual	java/io/PrintStream.print(Ljava/lang/String;)V
.line 37
	getstatic	fortest1/i I
	iconst_1
	iadd
	putstatic	fortest1/i I
	goto	L021
L022:
.line 38
	getstatic	java/lang/System/out Ljava/io/PrintStream;
	invokevirtual	java/io/PrintStream.println()V

	getstatic	fortest1/_runTimer LRunTimer;
	invokevirtual	RunTimer.printElapsedTime()V

	return

.limit locals 1
.limit stack 7
.end method
