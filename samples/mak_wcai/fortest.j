.class public fortest
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
	putstatic	fortest/_runTimer LRunTimer;
	new	PascalTextIn
	dup
	invokenonvirtual	PascalTextIn/<init>()V
	putstatic	fortest/_standardIn LPascalTextIn;


.line 12
	iconst_2
	putstatic	fortest/j I
.line 13
	bipush	120
	putstatic	fortest/ch C
.line 15
	getstatic	fortest/j I
	putstatic	fortest/k I
.line 15
L001:
	getstatic	fortest/k I
	iconst_5
	if_icmpgt	L003
	iconst_0
	goto	L004
L003:
	iconst_1
L004:
	ifne	L002
.line 16
	getstatic	fortest/k I
	putstatic	fortest/n I
.line 15
	getstatic	fortest/k I
	iconst_1
	iadd
	putstatic	fortest/k I
	goto	L001
L002:
.line 19
	getstatic	fortest/n I
	iconst_1
	iadd
	putstatic	fortest/k I
.line 19
L005:
	getstatic	fortest/k I
	getstatic	fortest/j I
	iconst_2
	iadd
	if_icmplt	L007
	iconst_0
	goto	L008
L007:
	iconst_1
L008:
	ifne	L006
.line 19
	getstatic	fortest/k I
	putstatic	fortest/i I
.line 19
	getstatic	fortest/k I
	iconst_1
	isub
	putstatic	fortest/k I
	goto	L005
L006:
.line 21
	iconst_1
	putstatic	fortest/i I
.line 21
L009:
	getstatic	fortest/i I
	iconst_2
	if_icmpgt	L011
	iconst_0
	goto	L012
L011:
	iconst_1
L012:
	ifne	L010
.line 22
	iconst_1
	putstatic	fortest/j I
.line 22
L013:
	getstatic	fortest/j I
	iconst_3
	if_icmpgt	L015
	iconst_0
	goto	L016
L015:
	iconst_1
L016:
	ifne	L014
.line 23
	getstatic	fortest/i I
	getstatic	fortest/j I
	imul
	putstatic	fortest/k I
.line 22
	getstatic	fortest/j I
	iconst_1
	iadd
	putstatic	fortest/j I
	goto	L013
L014:
.line 21
	getstatic	fortest/i I
	iconst_1
	iadd
	putstatic	fortest/i I
	goto	L009
L010:
.line 27
	iconst_4
	putstatic	fortest/grade I
.line 27
L017:
	getstatic	fortest/grade I
	iconst_0
	if_icmplt	L019
	iconst_0
	goto	L020
L019:
	iconst_1
L020:
	ifne	L018
.line 27
	getstatic	fortest/grade I
	putstatic	fortest/i I
.line 27
	getstatic	fortest/grade I
	iconst_1
	isub
	putstatic	fortest/grade I
	goto	L017
L018:
.line 29
	getstatic	fortest/ch C
	putstatic	fortest/i I
.line 29
L021:
	getstatic	fortest/i I
	bipush	122
	if_icmpgt	L023
	iconst_0
	goto	L024
L023:
	iconst_1
L024:
	ifne	L022
.line 29
	getstatic	fortest/i I
	putstatic	fortest/j I
.line 29
	getstatic	fortest/i I
	iconst_1
	iadd
	putstatic	fortest/i I
	goto	L021
L022:

	getstatic	fortest/_runTimer LRunTimer;
	invokevirtual	RunTimer.printElapsedTime()V

	return

.limit locals 1
.limit stack 3
.end method
