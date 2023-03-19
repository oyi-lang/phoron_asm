.class public casetest1
.super java/lang/Object

.field private static _runTimer LRunTimer;
.field private static _standardIn LPascalTextIn;

.field private static ch C
.field private static even I
.field private static i I
.field private static j I
.field private static odd I
.field private static prime I
.field private static size I
.field private static str C

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
	putstatic	casetest1/_runTimer LRunTimer;
	new	PascalTextIn
	dup
	invokenonvirtual	PascalTextIn/<init>()V
	putstatic	casetest1/_standardIn LPascalTextIn;


.line 12
	iconst_3
	putstatic	casetest1/i I
.line 12
	bipush	98
	putstatic	casetest1/ch C
.line 13
	iconst_1
	putstatic	casetest1/size I
.line 14
	sipush	990
	ineg
	putstatic	casetest1/even I
.line 14
	sipush	999
	ineg
	putstatic	casetest1/odd I
.line 14
	iconst_0
	putstatic	casetest1/prime I
.line 16
	getstatic	casetest1/i I
	iconst_1
	iadd

	lookupswitch
	  -8: L003
	  1: L002
	  4: L004
	  5: L004
	  7: L004
	  default: L001

L002:
.line 17
	getstatic	casetest1/i I
	putstatic	casetest1/j I
	goto	L001
L003:
.line 18
	bipush	8
	getstatic	casetest1/i I
	imul
	putstatic	casetest1/j I
	goto	L001
L004:
.line 19
	sipush	574
	getstatic	casetest1/i I
	imul
	putstatic	casetest1/j I
	goto	L001
L001:

.line 22
	getstatic	java/lang/System/out Ljava/io/PrintStream;
	ldc	"j = %d\n"
	iconst_1
	anewarray	java/lang/Object
	dup
	iconst_0
	getstatic	casetest1/j I
	invokestatic	java/lang/Integer.valueOf(I)Ljava/lang/Integer;
	aastore
	invokestatic	java/lang/String/format(Ljava/lang/String;[Ljava/lang/Object;)Ljava/lang/String;
	invokevirtual	java/io/PrintStream.print(Ljava/lang/String;)V
.line 24
	getstatic	casetest1/ch C

	lookupswitch
	  97: L007
	  98: L006
	  99: L006
	  default: L005

L006:
.line 25
	bipush	112
	putstatic	casetest1/str C
	goto	L005
L007:
.line 26
	bipush	113
	putstatic	casetest1/str C
	goto	L005
L005:

.line 29
	getstatic	java/lang/System/out Ljava/io/PrintStream;
	ldc	"str = '%c'\n"
	iconst_1
	anewarray	java/lang/Object
	dup
	iconst_0
	getstatic	casetest1/str C
	invokestatic	java/lang/Character.valueOf(C)Ljava/lang/Character;
	aastore
	invokestatic	java/lang/String/format(Ljava/lang/String;[Ljava/lang/Object;)Ljava/lang/String;
	invokevirtual	java/io/PrintStream.print(Ljava/lang/String;)V
.line 31
	getstatic	casetest1/size I

	lookupswitch
	  0: L009
	  1: L010
	  2: L011
	  default: L008

L009:
.line 32
	bipush	115
	putstatic	casetest1/str C
	goto	L008
L010:
.line 33
	bipush	109
	putstatic	casetest1/str C
	goto	L008
L011:
.line 34
	bipush	108
	putstatic	casetest1/str C
	goto	L008
L008:

.line 37
	getstatic	java/lang/System/out Ljava/io/PrintStream;
	ldc	"str = '%c'\n"
	iconst_1
	anewarray	java/lang/Object
	dup
	iconst_0
	getstatic	casetest1/str C
	invokestatic	java/lang/Character.valueOf(C)Ljava/lang/Character;
	aastore
	invokestatic	java/lang/String/format(Ljava/lang/String;[Ljava/lang/Object;)Ljava/lang/String;
	invokevirtual	java/io/PrintStream.print(Ljava/lang/String;)V
.line 39
	iconst_5
	ineg
	putstatic	casetest1/i I
.line 39
L012:
	getstatic	casetest1/i I
	bipush	15
	if_icmpgt	L014
	iconst_0
	goto	L015
L014:
	iconst_1
L015:
	ifne	L013
.line 40
	getstatic	casetest1/i I

	lookupswitch
	  -4: L018
	  -3: L019
	  -2: L018
	  -1: L019
	  0: L018
	  1: L019
	  2: L017
	  3: L019
	  4: L018
	  5: L019
	  6: L018
	  7: L019
	  8: L018
	  9: L019
	  10: L018
	  11: L019
	  12: L018
	  default: L016

L017:
.line 41
	getstatic	casetest1/i I
	putstatic	casetest1/prime I
	goto	L016
L018:
.line 42
	getstatic	casetest1/i I
	putstatic	casetest1/even I
	goto	L016
L019:
.line 43
	getstatic	casetest1/i I

	lookupswitch
	  -3: L021
	  -1: L021
	  1: L021
	  2: L022
	  3: L022
	  5: L022
	  7: L022
	  9: L021
	  11: L022
	  default: L020

L021:
.line 44
	getstatic	casetest1/i I
	putstatic	casetest1/odd I
	goto	L020
L022:
.line 45
	getstatic	casetest1/i I
	putstatic	casetest1/prime I
	goto	L020
L020:

	goto	L016
L016:

.line 49
	getstatic	java/lang/System/out Ljava/io/PrintStream;
	ldc	"i = %d, even = %d, odd = %d, prime = %d\n"
	iconst_4
	anewarray	java/lang/Object
	dup
	iconst_0
	getstatic	casetest1/i I
	invokestatic	java/lang/Integer.valueOf(I)Ljava/lang/Integer;
	aastore
	dup
	iconst_1
	getstatic	casetest1/even I
	invokestatic	java/lang/Integer.valueOf(I)Ljava/lang/Integer;
	aastore
	dup
	iconst_2
	getstatic	casetest1/odd I
	invokestatic	java/lang/Integer.valueOf(I)Ljava/lang/Integer;
	aastore
	dup
	iconst_3
	getstatic	casetest1/prime I
	invokestatic	java/lang/Integer.valueOf(I)Ljava/lang/Integer;
	aastore
	invokestatic	java/lang/String/format(Ljava/lang/String;[Ljava/lang/Object;)Ljava/lang/String;
	invokevirtual	java/io/PrintStream.print(Ljava/lang/String;)V
.line 39
	getstatic	casetest1/i I
	iconst_1
	iadd
	putstatic	casetest1/i I
	goto	L012
L013:

	getstatic	casetest1/_runTimer LRunTimer;
	invokevirtual	RunTimer.printElapsedTime()V

	return

.limit locals 1
.limit stack 7
.end method
