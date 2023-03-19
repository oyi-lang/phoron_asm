.class public iftest1
.super java/lang/Object

.field private static _runTimer LRunTimer;
.field private static _standardIn LPascalTextIn;

.field private static f I
.field private static i I
.field private static j I
.field private static t I

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
	putstatic	iftest1/_runTimer LRunTimer;
	new	PascalTextIn
	dup
	invokenonvirtual	PascalTextIn/<init>()V
	putstatic	iftest1/_standardIn LPascalTextIn;


.line 7
	iconst_3
	putstatic	iftest1/i I
.line 7
	iconst_4
	putstatic	iftest1/j I
.line 7
	iconst_0
	putstatic	iftest1/t I
.line 7
	iconst_0
	putstatic	iftest1/f I
.line 9
	getstatic	iftest1/i I
	getstatic	iftest1/j I
	if_icmplt	L002
	iconst_0
	goto	L003
L002:
	iconst_1
L003:
	ifeq	L001
.line 9
	sipush	300
	putstatic	iftest1/t I
L001:
.line 11
	getstatic	java/lang/System/out Ljava/io/PrintStream;
	ldc	"t = %d, f = %d\n"
	iconst_2
	anewarray	java/lang/Object
	dup
	iconst_0
	getstatic	iftest1/t I
	invokestatic	java/lang/Integer.valueOf(I)Ljava/lang/Integer;
	aastore
	dup
	iconst_1
	getstatic	iftest1/f I
	invokestatic	java/lang/Integer.valueOf(I)Ljava/lang/Integer;
	aastore
	invokestatic	java/lang/String/format(Ljava/lang/String;[Ljava/lang/Object;)Ljava/lang/String;
	invokevirtual	java/io/PrintStream.print(Ljava/lang/String;)V
.line 13
	getstatic	iftest1/i I
	getstatic	iftest1/j I
	if_icmpeq	L005
	iconst_0
	goto	L006
L005:
	iconst_1
L006:
	ifeq	L007
.line 13
	sipush	200
	putstatic	iftest1/t I
	goto	L004
L007:
.line 14
	sipush	200
	ineg
	putstatic	iftest1/f I
L004:
.line 16
	getstatic	java/lang/System/out Ljava/io/PrintStream;
	ldc	"t = %d, f = %d\n"
	iconst_2
	anewarray	java/lang/Object
	dup
	iconst_0
	getstatic	iftest1/t I
	invokestatic	java/lang/Integer.valueOf(I)Ljava/lang/Integer;
	aastore
	dup
	iconst_1
	getstatic	iftest1/f I
	invokestatic	java/lang/Integer.valueOf(I)Ljava/lang/Integer;
	aastore
	invokestatic	java/lang/String/format(Ljava/lang/String;[Ljava/lang/Object;)Ljava/lang/String;
	invokevirtual	java/io/PrintStream.print(Ljava/lang/String;)V
.line 19
	getstatic	iftest1/i I
	iconst_1
	if_icmpeq	L009
	iconst_0
	goto	L010
L009:
	iconst_1
L010:
	ifeq	L011
.line 19
	bipush	10
	putstatic	iftest1/f I
	goto	L008
L011:
.line 20
	getstatic	iftest1/i I
	iconst_2
	if_icmpeq	L013
	iconst_0
	goto	L014
L013:
	iconst_1
L014:
	ifeq	L015
.line 20
	bipush	20
	putstatic	iftest1/f I
	goto	L012
L015:
.line 21
	getstatic	iftest1/i I
	iconst_3
	if_icmpeq	L017
	iconst_0
	goto	L018
L017:
	iconst_1
L018:
	ifeq	L019
.line 21
	bipush	30
	putstatic	iftest1/t I
	goto	L016
L019:
.line 22
	getstatic	iftest1/i I
	iconst_4
	if_icmpeq	L021
	iconst_0
	goto	L022
L021:
	iconst_1
L022:
	ifeq	L023
.line 22
	bipush	40
	putstatic	iftest1/f I
	goto	L020
L023:
.line 23
	iconst_1
	ineg
	putstatic	iftest1/f I
L020:
L016:
L012:
L008:
.line 25
	getstatic	java/lang/System/out Ljava/io/PrintStream;
	ldc	"t = %d, f = %d\n"
	iconst_2
	anewarray	java/lang/Object
	dup
	iconst_0
	getstatic	iftest1/t I
	invokestatic	java/lang/Integer.valueOf(I)Ljava/lang/Integer;
	aastore
	dup
	iconst_1
	getstatic	iftest1/f I
	invokestatic	java/lang/Integer.valueOf(I)Ljava/lang/Integer;
	aastore
	invokestatic	java/lang/String/format(Ljava/lang/String;[Ljava/lang/Object;)Ljava/lang/String;
	invokevirtual	java/io/PrintStream.print(Ljava/lang/String;)V
.line 28
	getstatic	iftest1/i I
	iconst_3
	if_icmpeq	L025
	iconst_0
	goto	L026
L025:
	iconst_1
L026:
	ifeq	L024
.line 28
	getstatic	iftest1/j I
	iconst_2
	if_icmpeq	L028
	iconst_0
	goto	L029
L028:
	iconst_1
L029:
	ifeq	L030
.line 28
	sipush	500
	putstatic	iftest1/t I
	goto	L027
L030:
.line 28
	sipush	500
	ineg
	putstatic	iftest1/f I
L027:
L024:
.line 30
	getstatic	java/lang/System/out Ljava/io/PrintStream;
	ldc	"t = %d, f = %d\n"
	iconst_2
	anewarray	java/lang/Object
	dup
	iconst_0
	getstatic	iftest1/t I
	invokestatic	java/lang/Integer.valueOf(I)Ljava/lang/Integer;
	aastore
	dup
	iconst_1
	getstatic	iftest1/f I
	invokestatic	java/lang/Integer.valueOf(I)Ljava/lang/Integer;
	aastore
	invokestatic	java/lang/String/format(Ljava/lang/String;[Ljava/lang/Object;)Ljava/lang/String;
	invokevirtual	java/io/PrintStream.print(Ljava/lang/String;)V

	getstatic	iftest1/_runTimer LRunTimer;
	invokevirtual	RunTimer.printElapsedTime()V

	return

.limit locals 1
.limit stack 7
.end method
