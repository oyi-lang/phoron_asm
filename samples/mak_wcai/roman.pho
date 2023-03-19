.class public roman
.super java/lang/Object

.field private static _runTimer LRunTimer;
.field private static _standardIn LPascalTextIn;

.field private static x I
.field private static y I

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
	putstatic	roman/_runTimer LRunTimer;
	new	PascalTextIn
	dup
	invokenonvirtual	PascalTextIn/<init>()V
	putstatic	roman/_standardIn LPascalTextIn;


.line 7
	iconst_1
	putstatic	roman/y I
.line 8
L001:
.line 9
	getstatic	roman/y I
	putstatic	roman/x I
.line 10
	getstatic	java/lang/System/out Ljava/io/PrintStream;
	ldc	"%4d "
	iconst_1
	anewarray	java/lang/Object
	dup
	iconst_0
	getstatic	roman/x I
	invokestatic	java/lang/Integer.valueOf(I)Ljava/lang/Integer;
	aastore
	invokestatic	java/lang/String/format(Ljava/lang/String;[Ljava/lang/Object;)Ljava/lang/String;
	invokevirtual	java/io/PrintStream.print(Ljava/lang/String;)V
.line 11
L003:
	getstatic	roman/x I
	sipush	1000
	if_icmpge	L005
	iconst_0
	goto	L006
L005:
	iconst_1
L006:
	iconst_1
	ixor
	ifne	L004
.line 12
	getstatic	java/lang/System/out Ljava/io/PrintStream;
	ldc	"m"
	invokevirtual	java/io/PrintStream.print(Ljava/lang/String;)V
.line 13
	getstatic	roman/x I
	sipush	1000
	isub
	putstatic	roman/x I
	goto	L003
L004:
.line 15
L007:
	getstatic	roman/x I
	sipush	500
	if_icmpge	L009
	iconst_0
	goto	L010
L009:
	iconst_1
L010:
	iconst_1
	ixor
	ifne	L008
.line 16
	getstatic	java/lang/System/out Ljava/io/PrintStream;
	ldc	"d"
	invokevirtual	java/io/PrintStream.print(Ljava/lang/String;)V
.line 17
	getstatic	roman/x I
	sipush	500
	isub
	putstatic	roman/x I
	goto	L007
L008:
.line 19
L011:
	getstatic	roman/x I
	bipush	100
	if_icmpge	L013
	iconst_0
	goto	L014
L013:
	iconst_1
L014:
	iconst_1
	ixor
	ifne	L012
.line 20
	getstatic	java/lang/System/out Ljava/io/PrintStream;
	ldc	"c"
	invokevirtual	java/io/PrintStream.print(Ljava/lang/String;)V
.line 21
	getstatic	roman/x I
	bipush	100
	isub
	putstatic	roman/x I
	goto	L011
L012:
.line 23
L015:
	getstatic	roman/x I
	bipush	50
	if_icmpge	L017
	iconst_0
	goto	L018
L017:
	iconst_1
L018:
	iconst_1
	ixor
	ifne	L016
.line 24
	getstatic	java/lang/System/out Ljava/io/PrintStream;
	ldc	"l"
	invokevirtual	java/io/PrintStream.print(Ljava/lang/String;)V
.line 25
	getstatic	roman/x I
	bipush	50
	isub
	putstatic	roman/x I
	goto	L015
L016:
.line 27
L019:
	getstatic	roman/x I
	bipush	10
	if_icmpge	L021
	iconst_0
	goto	L022
L021:
	iconst_1
L022:
	iconst_1
	ixor
	ifne	L020
.line 28
	getstatic	java/lang/System/out Ljava/io/PrintStream;
	ldc	"x"
	invokevirtual	java/io/PrintStream.print(Ljava/lang/String;)V
.line 29
	getstatic	roman/x I
	bipush	10
	isub
	putstatic	roman/x I
	goto	L019
L020:
.line 31
L023:
	getstatic	roman/x I
	iconst_5
	if_icmpge	L025
	iconst_0
	goto	L026
L025:
	iconst_1
L026:
	iconst_1
	ixor
	ifne	L024
.line 32
	getstatic	java/lang/System/out Ljava/io/PrintStream;
	ldc	"v"
	invokevirtual	java/io/PrintStream.print(Ljava/lang/String;)V
.line 33
	getstatic	roman/x I
	iconst_5
	isub
	putstatic	roman/x I
	goto	L023
L024:
.line 35
L027:
	getstatic	roman/x I
	iconst_1
	if_icmpge	L029
	iconst_0
	goto	L030
L029:
	iconst_1
L030:
	iconst_1
	ixor
	ifne	L028
.line 36
	getstatic	java/lang/System/out Ljava/io/PrintStream;
	ldc	"i"
	invokevirtual	java/io/PrintStream.print(Ljava/lang/String;)V
.line 37
	getstatic	roman/x I
	iconst_1
	isub
	putstatic	roman/x I
	goto	L027
L028:
.line 39
	getstatic	java/lang/System/out Ljava/io/PrintStream;
	invokevirtual	java/io/PrintStream.println()V
.line 40
	iconst_2
	getstatic	roman/y I
	imul
	putstatic	roman/y I
	getstatic	roman/y I
	sipush	5000
	if_icmpgt	L031
	iconst_0
	goto	L032
L031:
	iconst_1
L032:
	ifne	L002
	goto	L001
L002:

	getstatic	roman/_runTimer LRunTimer;
	invokevirtual	RunTimer.printElapsedTime()V

	return

.limit locals 1
.limit stack 7
.end method
