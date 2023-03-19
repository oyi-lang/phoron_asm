.class public allocarraytest2
.super java/lang/Object

.field private static _runTimer LRunTimer;
.field private static _standardIn LPascalTextIn;

.field private static a1 [Ljava/lang/StringBuilder;
.field private static a2 [[Ljava/lang/StringBuilder;
.field private static a3 [[[Ljava/lang/StringBuilder;

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
	putstatic	allocarraytest2/_runTimer LRunTimer;
	new	PascalTextIn
	dup
	invokenonvirtual	PascalTextIn/<init>()V
	putstatic	allocarraytest2/_standardIn LPascalTextIn;


	bipush	10
	anewarray	java/lang/StringBuilder

	iconst_0
	istore_1
L001:
	iload_1
	bipush	10
	if_icmpge	L002

	dup
	iload_1
	iconst_5
	invokestatic	PaddedString.create(I)Ljava/lang/StringBuilder;
	aastore
	iinc	1 1
	goto	L001
L002:
	putstatic	allocarraytest2/a1 [Ljava/lang/StringBuilder;

	iconst_5
	iconst_4
	multianewarray	[[Ljava/lang/StringBuilder; 2

	iconst_0
	istore_1
L003:
	iload_1
	iconst_5
	if_icmpge	L004

	dup
	iload_1
	aaload

	iconst_0
	istore_2
L005:
	iload_2
	iconst_4
	if_icmpge	L006

	dup
	iload_2
	iconst_5
	invokestatic	PaddedString.create(I)Ljava/lang/StringBuilder;
	aastore
	iinc	2 1
	goto	L005
L006:
	pop
	iinc	1 1
	goto	L003
L004:
	putstatic	allocarraytest2/a2 [[Ljava/lang/StringBuilder;

	iconst_2
	iconst_3
	iconst_4
	multianewarray	[[[Ljava/lang/StringBuilder; 3

	iconst_0
	istore_1
L007:
	iload_1
	iconst_2
	if_icmpge	L008

	dup
	iload_1
	aaload

	iconst_0
	istore_2
L009:
	iload_2
	iconst_3
	if_icmpge	L010

	dup
	iload_2
	aaload

	iconst_0
	istore_3
L011:
	iload_3
	iconst_4
	if_icmpge	L012

	dup
	iload_3
	iconst_5
	invokestatic	PaddedString.create(I)Ljava/lang/StringBuilder;
	aastore
	iinc	3 1
	goto	L011
L012:
	pop
	iinc	2 1
	goto	L009
L010:
	pop
	iinc	1 1
	goto	L007
L008:
	putstatic	allocarraytest2/a3 [[[Ljava/lang/StringBuilder;

	nop

	getstatic	allocarraytest2/_runTimer LRunTimer;
	invokevirtual	RunTimer.printElapsedTime()V

	return

.limit locals 4
.limit stack 7
.end method
