.class public iftest
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
	putstatic	iftest/_runTimer LRunTimer;
	new	PascalTextIn
	dup
	invokenonvirtual	PascalTextIn/<init>()V
	putstatic	iftest/_standardIn LPascalTextIn;


.line 7
	iconst_3
	putstatic	iftest/i I
.line 7
	iconst_4
	putstatic	iftest/j I
.line 7
	iconst_0
	putstatic	iftest/t I
.line 7
	iconst_0
	putstatic	iftest/f I
.line 9
	getstatic	iftest/i I
	getstatic	iftest/j I
	if_icmplt	L002
	iconst_0
	goto	L003
L002:
	iconst_1
L003:
	ifeq	L001
.line 9
	sipush	300
	putstatic	iftest/t I
L001:
.line 11
	getstatic	iftest/i I
	getstatic	iftest/j I
	if_icmpeq	L005
	iconst_0
	goto	L006
L005:
	iconst_1
L006:
	ifeq	L007
.line 11
	sipush	200
	putstatic	iftest/t I
	goto	L004
L007:
.line 12
	sipush	200
	ineg
	putstatic	iftest/f I
L004:
.line 15
	getstatic	iftest/i I
	iconst_1
	if_icmpeq	L009
	iconst_0
	goto	L010
L009:
	iconst_1
L010:
	ifeq	L011
.line 15
	bipush	10
	putstatic	iftest/f I
	goto	L008
L011:
.line 16
	getstatic	iftest/i I
	iconst_2
	if_icmpeq	L013
	iconst_0
	goto	L014
L013:
	iconst_1
L014:
	ifeq	L015
.line 16
	bipush	20
	putstatic	iftest/f I
	goto	L012
L015:
.line 17
	getstatic	iftest/i I
	iconst_3
	if_icmpeq	L017
	iconst_0
	goto	L018
L017:
	iconst_1
L018:
	ifeq	L019
.line 17
	bipush	30
	putstatic	iftest/t I
	goto	L016
L019:
.line 18
	getstatic	iftest/i I
	iconst_4
	if_icmpeq	L021
	iconst_0
	goto	L022
L021:
	iconst_1
L022:
	ifeq	L023
.line 18
	bipush	40
	putstatic	iftest/f I
	goto	L020
L023:
.line 19
	iconst_1
	ineg
	putstatic	iftest/f I
L020:
L016:
L012:
L008:
.line 22
	getstatic	iftest/i I
	iconst_3
	if_icmpeq	L025
	iconst_0
	goto	L026
L025:
	iconst_1
L026:
	ifeq	L024
.line 22
	getstatic	iftest/j I
	iconst_2
	if_icmpeq	L028
	iconst_0
	goto	L029
L028:
	iconst_1
L029:
	ifeq	L030
.line 22
	sipush	500
	putstatic	iftest/t I
	goto	L027
L030:
.line 22
	sipush	500
	ineg
	putstatic	iftest/f I
L027:
L024:

	getstatic	iftest/_runTimer LRunTimer;
	invokevirtual	RunTimer.printElapsedTime()V

	return

.limit locals 1
.limit stack 3
.end method
