.class public strings1
.super java/lang/Object

.field private static _runTimer LRunTimer;
.field private static _standardIn LPascalTextIn;

.field private static cube [[[Ljava/lang/StringBuilder;
.field private static list [Ljava/lang/StringBuilder;
.field private static matrix [[Ljava/lang/StringBuilder;
.field private static str5 Ljava/lang/StringBuilder;
.field private static str8 Ljava/lang/StringBuilder;

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
	putstatic	strings1/_runTimer LRunTimer;
	new	PascalTextIn
	dup
	invokenonvirtual	PascalTextIn/<init>()V
	putstatic	strings1/_standardIn LPascalTextIn;


	iconst_2
	iconst_2
	iconst_2
	multianewarray	[[[Ljava/lang/StringBuilder; 3

	iconst_0
	istore_1
L001:
	iload_1
	iconst_2
	if_icmpge	L002

	dup
	iload_1
	aaload

	iconst_0
	istore_2
L003:
	iload_2
	iconst_2
	if_icmpge	L004

	dup
	iload_2
	aaload

	iconst_0
	istore_3
L005:
	iload_3
	iconst_2
	if_icmpge	L006

	dup
	iload_3
	iconst_5
	invokestatic	PaddedString.create(I)Ljava/lang/StringBuilder;
	aastore
	iinc	3 1
	goto	L005
L006:
	pop
	iinc	2 1
	goto	L003
L004:
	pop
	iinc	1 1
	goto	L001
L002:
	putstatic	strings1/cube [[[Ljava/lang/StringBuilder;

	iconst_3
	anewarray	java/lang/StringBuilder

	iconst_0
	istore_1
L007:
	iload_1
	iconst_3
	if_icmpge	L008

	dup
	iload_1
	iconst_5
	invokestatic	PaddedString.create(I)Ljava/lang/StringBuilder;
	aastore
	iinc	1 1
	goto	L007
L008:
	putstatic	strings1/list [Ljava/lang/StringBuilder;

	iconst_2
	iconst_3
	multianewarray	[[Ljava/lang/StringBuilder; 2

	iconst_0
	istore_1
L009:
	iload_1
	iconst_2
	if_icmpge	L010

	dup
	iload_1
	aaload

	iconst_0
	istore_2
L011:
	iload_2
	iconst_3
	if_icmpge	L012

	dup
	iload_2
	iconst_5
	invokestatic	PaddedString.create(I)Ljava/lang/StringBuilder;
	aastore
	iinc	2 1
	goto	L011
L012:
	pop
	iinc	1 1
	goto	L009
L010:
	putstatic	strings1/matrix [[Ljava/lang/StringBuilder;
	iconst_5
	invokestatic	PaddedString.create(I)Ljava/lang/StringBuilder;
	putstatic	strings1/str5 Ljava/lang/StringBuilder;
	bipush	8
	invokestatic	PaddedString.create(I)Ljava/lang/StringBuilder;
	putstatic	strings1/str8 Ljava/lang/StringBuilder;

.line 15
	getstatic	strings1/str5 Ljava/lang/StringBuilder;
	dup
	iconst_0
	invokevirtual	java/lang/StringBuilder.setLength(I)V
	ldc	"hello"
	invokevirtual	java/lang/StringBuilder.append(Ljava/lang/String;)Ljava/lang/StringBuilder;
	pop
.line 16
	getstatic	strings1/str8 Ljava/lang/StringBuilder;
	dup
	iconst_0
	invokevirtual	java/lang/StringBuilder.setLength(I)V
	ldc	"everyone"
	invokevirtual	java/lang/StringBuilder.append(Ljava/lang/String;)Ljava/lang/StringBuilder;
	pop
.line 17
	getstatic	java/lang/System/out Ljava/io/PrintStream;
	ldc	"'%s, %s'\n"
	iconst_2
	anewarray	java/lang/Object
	dup
	iconst_0
	getstatic	strings1/str5 Ljava/lang/StringBuilder;
	aastore
	dup
	iconst_1
	getstatic	strings1/str8 Ljava/lang/StringBuilder;
	aastore
	invokestatic	java/lang/String/format(Ljava/lang/String;[Ljava/lang/Object;)Ljava/lang/String;
	invokevirtual	java/io/PrintStream.print(Ljava/lang/String;)V
.line 19
	getstatic	strings1/str5 Ljava/lang/StringBuilder;
	invokevirtual	java/lang/StringBuilder.toString()Ljava/lang/String;
	getstatic	strings1/str8 Ljava/lang/StringBuilder;
	invokevirtual	java/lang/StringBuilder.toString()Ljava/lang/String;
	invokevirtual	java/lang/String.compareTo(Ljava/lang/String;)I
	ifgt	L014
	iconst_0
	goto	L015
L014:
	iconst_1
L015:
	ifeq	L013
.line 19
	getstatic	java/lang/System/out Ljava/io/PrintStream;
	ldc	"greater\n"
	invokevirtual	java/io/PrintStream.print(Ljava/lang/String;)V
L013:
.line 20
	getstatic	strings1/str5 Ljava/lang/StringBuilder;
	invokevirtual	java/lang/StringBuilder.toString()Ljava/lang/String;
	ldc	"goodbye"
	invokevirtual	java/lang/String.compareTo(Ljava/lang/String;)I
	ifgt	L017
	iconst_0
	goto	L018
L017:
	iconst_1
L018:
	ifeq	L016
.line 20
	getstatic	java/lang/System/out Ljava/io/PrintStream;
	ldc	"greater\n"
	invokevirtual	java/io/PrintStream.print(Ljava/lang/String;)V
L016:
.line 21
	ldc	"nobody"
	getstatic	strings1/str8 Ljava/lang/StringBuilder;
	invokevirtual	java/lang/StringBuilder.toString()Ljava/lang/String;
	invokevirtual	java/lang/String.compareTo(Ljava/lang/String;)I
	ifge	L020
	iconst_0
	goto	L021
L020:
	iconst_1
L021:
	ifeq	L019
.line 21
	getstatic	java/lang/System/out Ljava/io/PrintStream;
	ldc	"greater\n"
	invokevirtual	java/io/PrintStream.print(Ljava/lang/String;)V
L019:
.line 23
	getstatic	strings1/str5 Ljava/lang/StringBuilder;
	dup
	iconst_0
	invokevirtual	java/lang/StringBuilder.setLength(I)V
	getstatic	strings1/str8 Ljava/lang/StringBuilder;
	iconst_0
	iconst_5
	invokevirtual	java/lang/StringBuilder.substring(II)Ljava/lang/String;
	invokevirtual	java/lang/StringBuilder.append(Ljava/lang/CharSequence;)Ljava/lang/StringBuilder;
	pop
.line 24
	getstatic	java/lang/System/out Ljava/io/PrintStream;
	ldc	"'%s'\n"
	iconst_1
	anewarray	java/lang/Object
	dup
	iconst_0
	getstatic	strings1/str5 Ljava/lang/StringBuilder;
	aastore
	invokestatic	java/lang/String/format(Ljava/lang/String;[Ljava/lang/Object;)Ljava/lang/String;
	invokevirtual	java/io/PrintStream.print(Ljava/lang/String;)V
.line 26
	getstatic	strings1/str5 Ljava/lang/StringBuilder;
	dup
	iconst_0
	invokevirtual	java/lang/StringBuilder.setLength(I)V
	ldc	"hello"
	invokevirtual	java/lang/StringBuilder.append(Ljava/lang/String;)Ljava/lang/StringBuilder;
	pop
.line 27
	getstatic	strings1/str8 Ljava/lang/StringBuilder;
	dup
	iconst_0
	invokevirtual	java/lang/StringBuilder.setLength(I)V
	getstatic	strings1/str5 Ljava/lang/StringBuilder;
	invokevirtual	java/lang/StringBuilder.append(Ljava/lang/CharSequence;)Ljava/lang/StringBuilder;
	bipush	8
	iconst_5
	invokestatic	PaddedString.blanks(II)Ljava/lang/StringBuilder;
	invokevirtual	java/lang/StringBuilder.append(Ljava/lang/CharSequence;)Ljava/lang/StringBuilder;
	pop
.line 28
	getstatic	java/lang/System/out Ljava/io/PrintStream;
	ldc	"'%s'\n"
	iconst_1
	anewarray	java/lang/Object
	dup
	iconst_0
	getstatic	strings1/str8 Ljava/lang/StringBuilder;
	aastore
	invokestatic	java/lang/String/format(Ljava/lang/String;[Ljava/lang/Object;)Ljava/lang/String;
	invokevirtual	java/io/PrintStream.print(Ljava/lang/String;)V
.line 30
	getstatic	strings1/str8 Ljava/lang/StringBuilder;
	bipush	8
	iconst_1
	isub
	bipush	33
	invokevirtual	java/lang/StringBuilder.setCharAt(IC)V
.line 31
	getstatic	java/lang/System/out Ljava/io/PrintStream;
	ldc	"'%s'\n"
	iconst_1
	anewarray	java/lang/Object
	dup
	iconst_0
	getstatic	strings1/str8 Ljava/lang/StringBuilder;
	aastore
	invokestatic	java/lang/String/format(Ljava/lang/String;[Ljava/lang/Object;)Ljava/lang/String;
	invokevirtual	java/io/PrintStream.print(Ljava/lang/String;)V
.line 33
	getstatic	strings1/list [Ljava/lang/StringBuilder;
	iconst_1
	iconst_1
	isub
	aaload
	dup
	iconst_0
	invokevirtual	java/lang/StringBuilder.setLength(I)V
	ldc	"Ron"
	invokevirtual	java/lang/StringBuilder.append(Ljava/lang/String;)Ljava/lang/StringBuilder;
	iconst_5
	iconst_3
	invokestatic	PaddedString.blanks(II)Ljava/lang/StringBuilder;
	invokevirtual	java/lang/StringBuilder.append(Ljava/lang/CharSequence;)Ljava/lang/StringBuilder;
	pop
.line 34
	getstatic	strings1/list [Ljava/lang/StringBuilder;
	iconst_2
	iconst_1
	isub
	aaload
	dup
	iconst_0
	invokevirtual	java/lang/StringBuilder.setLength(I)V
	ldc	"John"
	invokevirtual	java/lang/StringBuilder.append(Ljava/lang/String;)Ljava/lang/StringBuilder;
	iconst_5
	iconst_4
	invokestatic	PaddedString.blanks(II)Ljava/lang/StringBuilder;
	invokevirtual	java/lang/StringBuilder.append(Ljava/lang/CharSequence;)Ljava/lang/StringBuilder;
	pop
.line 35
	getstatic	java/lang/System/out Ljava/io/PrintStream;
	ldc	"'%s'\n"
	iconst_1
	anewarray	java/lang/Object
	dup
	iconst_0
	getstatic	strings1/list [Ljava/lang/StringBuilder;
	iconst_1
	iconst_1
	isub
	aaload
	aastore
	invokestatic	java/lang/String/format(Ljava/lang/String;[Ljava/lang/Object;)Ljava/lang/String;
	invokevirtual	java/io/PrintStream.print(Ljava/lang/String;)V
.line 36
	getstatic	java/lang/System/out Ljava/io/PrintStream;
	ldc	"'%s'\n"
	iconst_1
	anewarray	java/lang/Object
	dup
	iconst_0
	getstatic	strings1/list [Ljava/lang/StringBuilder;
	iconst_2
	iconst_1
	isub
	aaload
	aastore
	invokestatic	java/lang/String/format(Ljava/lang/String;[Ljava/lang/Object;)Ljava/lang/String;
	invokevirtual	java/io/PrintStream.print(Ljava/lang/String;)V
.line 38
	getstatic	strings1/matrix [[Ljava/lang/StringBuilder;
	iconst_0
	aaload
	iconst_1
	aaload
	dup
	iconst_0
	invokevirtual	java/lang/StringBuilder.setLength(I)V
	ldc	"  0,1"
	invokevirtual	java/lang/StringBuilder.append(Ljava/lang/String;)Ljava/lang/StringBuilder;
	pop
.line 39
	getstatic	strings1/matrix [[Ljava/lang/StringBuilder;
	iconst_1
	aaload
	iconst_2
	aaload
	dup
	iconst_0
	invokevirtual	java/lang/StringBuilder.setLength(I)V
	ldc	"  1,2"
	invokevirtual	java/lang/StringBuilder.append(Ljava/lang/String;)Ljava/lang/StringBuilder;
	pop
.line 40
	getstatic	java/lang/System/out Ljava/io/PrintStream;
	ldc	"'%s, %s'\n"
	iconst_2
	anewarray	java/lang/Object
	dup
	iconst_0
	getstatic	strings1/matrix [[Ljava/lang/StringBuilder;
	iconst_0
	aaload
	iconst_1
	aaload
	aastore
	dup
	iconst_1
	getstatic	strings1/matrix [[Ljava/lang/StringBuilder;
	iconst_1
	aaload
	iconst_2
	aaload
	aastore
	invokestatic	java/lang/String/format(Ljava/lang/String;[Ljava/lang/Object;)Ljava/lang/String;
	invokevirtual	java/io/PrintStream.print(Ljava/lang/String;)V
.line 42
	getstatic	strings1/cube [[[Ljava/lang/StringBuilder;
	iconst_0
	aaload
	iconst_1
	aaload
	iconst_1
	aaload
	dup
	iconst_0
	invokevirtual	java/lang/StringBuilder.setLength(I)V
	ldc	"0,1,1"
	invokevirtual	java/lang/StringBuilder.append(Ljava/lang/String;)Ljava/lang/StringBuilder;
	pop
.line 43
	getstatic	strings1/cube [[[Ljava/lang/StringBuilder;
	iconst_1
	aaload
	iconst_0
	aaload
	iconst_1
	aaload
	dup
	iconst_0
	invokevirtual	java/lang/StringBuilder.setLength(I)V
	ldc	"1,0,1"
	invokevirtual	java/lang/StringBuilder.append(Ljava/lang/String;)Ljava/lang/StringBuilder;
	pop
.line 44
	getstatic	java/lang/System/out Ljava/io/PrintStream;
	ldc	"'%s, %s'\n"
	iconst_2
	anewarray	java/lang/Object
	dup
	iconst_0
	getstatic	strings1/cube [[[Ljava/lang/StringBuilder;
	iconst_0
	aaload
	iconst_1
	aaload
	iconst_1
	aaload
	aastore
	dup
	iconst_1
	getstatic	strings1/cube [[[Ljava/lang/StringBuilder;
	iconst_1
	aaload
	iconst_0
	aaload
	iconst_1
	aaload
	aastore
	invokestatic	java/lang/String/format(Ljava/lang/String;[Ljava/lang/Object;)Ljava/lang/String;
	invokevirtual	java/io/PrintStream.print(Ljava/lang/String;)V
.line 46
	getstatic	strings1/cube [[[Ljava/lang/StringBuilder;
	iconst_1
	aaload
	iconst_0
	aaload
	iconst_1
	aaload
	iconst_3
	iconst_1
	isub
	getstatic	strings1/cube [[[Ljava/lang/StringBuilder;
	iconst_0
	aaload
	iconst_1
	aaload
	iconst_1
	aaload
	iconst_5
	iconst_1
	isub
	invokevirtual	java/lang/StringBuilder.charAt(I)C
	invokevirtual	java/lang/StringBuilder.setCharAt(IC)V
.line 47
	getstatic	java/lang/System/out Ljava/io/PrintStream;
	ldc	"'%s'\n"
	iconst_1
	anewarray	java/lang/Object
	dup
	iconst_0
	getstatic	strings1/cube [[[Ljava/lang/StringBuilder;
	iconst_1
	aaload
	iconst_0
	aaload
	iconst_1
	aaload
	aastore
	invokestatic	java/lang/String/format(Ljava/lang/String;[Ljava/lang/Object;)Ljava/lang/String;
	invokevirtual	java/io/PrintStream.print(Ljava/lang/String;)V
.line 49
	getstatic	strings1/cube [[[Ljava/lang/StringBuilder;
	iconst_1
	aaload
	iconst_0
	aaload
	iconst_1
	aaload
	invokevirtual	java/lang/StringBuilder.toString()Ljava/lang/String;
	getstatic	strings1/cube [[[Ljava/lang/StringBuilder;
	iconst_0
	aaload
	iconst_1
	aaload
	iconst_1
	aaload
	invokevirtual	java/lang/StringBuilder.toString()Ljava/lang/String;
	invokevirtual	java/lang/String.compareTo(Ljava/lang/String;)I
	ifgt	L023
	iconst_0
	goto	L024
L023:
	iconst_1
L024:
	ifeq	L022
.line 49
	getstatic	java/lang/System/out Ljava/io/PrintStream;
	ldc	"greater\n"
	invokevirtual	java/io/PrintStream.print(Ljava/lang/String;)V
L022:
.line 50
	getstatic	strings1/cube [[[Ljava/lang/StringBuilder;
	iconst_1
	aaload
	iconst_0
	aaload
	iconst_1
	aaload
	iconst_3
	iconst_1
	isub
	invokevirtual	java/lang/StringBuilder.charAt(I)C
	bipush	48
	if_icmpgt	L026
	iconst_0
	goto	L027
L026:
	iconst_1
L027:
	ifeq	L025
.line 50
	getstatic	java/lang/System/out Ljava/io/PrintStream;
	ldc	"greater\n"
	invokevirtual	java/io/PrintStream.print(Ljava/lang/String;)V
L025:

	getstatic	strings1/_runTimer LRunTimer;
	invokevirtual	RunTimer.printElapsedTime()V

	return

.limit locals 4
.limit stack 9
.end method
