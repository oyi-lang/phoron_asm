.class public casetest
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
	putstatic	casetest/_runTimer LRunTimer;
	new	PascalTextIn
	dup
	invokenonvirtual	PascalTextIn/<init>()V
	putstatic	casetest/_standardIn LPascalTextIn;


.line 12
	iconst_3
	putstatic	casetest/i I
.line 12
	bipush	98
	putstatic	casetest/ch C
.line 13
	iconst_1
	putstatic	casetest/size I
.line 14
	sipush	990
	ineg
	putstatic	casetest/even I
.line 14
	sipush	999
	ineg
	putstatic	casetest/odd I
.line 14
	iconst_0
	putstatic	casetest/prime I
.line 16
	getstatic	casetest/i I
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
	getstatic	casetest/i I
	putstatic	casetest/j I
	goto	L001
L003:
.line 18
	bipush	8
	getstatic	casetest/i I
	imul
	putstatic	casetest/j I
	goto	L001
L004:
.line 19
	sipush	574
	getstatic	casetest/i I
	imul
	putstatic	casetest/j I
	goto	L001
L001:

.line 22
	getstatic	casetest/ch C

	lookupswitch
	  97: L007
	  98: L006
	  99: L006
	  default: L005

L006:
.line 23
	bipush	112
	putstatic	casetest/str C
	goto	L005
L007:
.line 24
	bipush	113
	putstatic	casetest/str C
	goto	L005
L005:

.line 27
	getstatic	casetest/size I

	lookupswitch
	  0: L009
	  1: L010
	  2: L011
	  default: L008

L009:
.line 28
	bipush	115
	putstatic	casetest/str C
	goto	L008
L010:
.line 29
	bipush	109
	putstatic	casetest/str C
	goto	L008
L011:
.line 30
	bipush	108
	putstatic	casetest/str C
	goto	L008
L008:

.line 33
	iconst_5
	ineg
	putstatic	casetest/i I
.line 33
L012:
	getstatic	casetest/i I
	bipush	15
	if_icmpgt	L014
	iconst_0
	goto	L015
L014:
	iconst_1
L015:
	ifne	L013
.line 34
	getstatic	casetest/i I

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
.line 35
	getstatic	casetest/i I
	putstatic	casetest/prime I
	goto	L016
L018:
.line 36
	getstatic	casetest/i I
	putstatic	casetest/even I
	goto	L016
L019:
.line 37
	getstatic	casetest/i I

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
.line 38
	getstatic	casetest/i I
	putstatic	casetest/odd I
	goto	L020
L022:
.line 39
	getstatic	casetest/i I
	putstatic	casetest/prime I
	goto	L020
L020:

	goto	L016
L016:

.line 33
	getstatic	casetest/i I
	iconst_1
	iadd
	putstatic	casetest/i I
	goto	L012
L013:

	getstatic	casetest/_runTimer LRunTimer;
	invokevirtual	RunTimer.printElapsedTime()V

	return

.limit locals 1
.limit stack 3
.end method
