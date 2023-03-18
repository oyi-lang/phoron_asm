.class public strings2
.super java/lang/Object

.field private static _runTimer LRunTimer;
.field private static _standardIn LPascalTextIn;

.field private static cube1 [[[Ljava/lang/StringBuilder;
.field private static cube2 [[[Ljava/lang/StringBuilder;
.field private static name1 Ljava/lang/StringBuilder;
.field private static name2 Ljava/lang/StringBuilder;

.method public <init>()V

	aload_0
	invokenonvirtual	java/lang/Object/<init>()V
	return

.limit locals 1
.limit stack 1
.end method

.method private static xfer1(Ljava/lang/StringBuilder;Ljava/lang/StringBuilder;)V

.var 0 is str1 Ljava/lang/StringBuilder;
.var 1 is str2 Ljava/lang/StringBuilder;


.line 13
	aload_1
	dup
	iconst_0
	invokevirtual	java/lang/StringBuilder.setLength(I)V
	aload_0
	invokevirtual	java/lang/StringBuilder.append(Ljava/lang/CharSequence;)Ljava/lang/StringBuilder;
	pop

	return

.limit locals 2
.limit stack 3
.end method

.method private static noxfer1(Ljava/lang/StringBuilder;Ljava/lang/StringBuilder;)V

.var 0 is str1 Ljava/lang/StringBuilder;
.var 1 is str2 Ljava/lang/StringBuilder;


.line 18
	aload_1
	dup
	iconst_0
	invokevirtual	java/lang/StringBuilder.setLength(I)V
	aload_0
	invokevirtual	java/lang/StringBuilder.append(Ljava/lang/CharSequence;)Ljava/lang/StringBuilder;
	pop

	return

.limit locals 2
.limit stack 3
.end method

.method private static xfer2([[[Ljava/lang/StringBuilder;[[[Ljava/lang/StringBuilder;)V

.var 0 is cb1 [[[Ljava/lang/StringBuilder;
.var 1 is cb2 [[[Ljava/lang/StringBuilder;


.line 23
	aload_1
	iconst_1
	aaload
	iconst_0
	aaload
	iconst_1
	aaload
	dup
	iconst_0
	invokevirtual	java/lang/StringBuilder.setLength(I)V
	aload_0
	iconst_1
	aaload
	iconst_0
	aaload
	iconst_1
	aaload
	invokevirtual	java/lang/StringBuilder.append(Ljava/lang/CharSequence;)Ljava/lang/StringBuilder;
	pop

	return

.limit locals 2
.limit stack 3
.end method

.method private static noxfer2([[[Ljava/lang/StringBuilder;[[[Ljava/lang/StringBuilder;)V

.var 0 is cb1 [[[Ljava/lang/StringBuilder;
.var 1 is cb2 [[[Ljava/lang/StringBuilder;


.line 28
	aload_1
	iconst_1
	aaload
	iconst_0
	aaload
	iconst_1
	aaload
	dup
	iconst_0
	invokevirtual	java/lang/StringBuilder.setLength(I)V
	aload_0
	iconst_1
	aaload
	iconst_0
	aaload
	iconst_1
	aaload
	invokevirtual	java/lang/StringBuilder.append(Ljava/lang/CharSequence;)Ljava/lang/StringBuilder;
	pop

	return

.limit locals 2
.limit stack 3
.end method

.method public static main([Ljava/lang/String;)V

	new	RunTimer
	dup
	invokenonvirtual	RunTimer/<init>()V
	putstatic	strings2/_runTimer LRunTimer;
	new	PascalTextIn
	dup
	invokenonvirtual	PascalTextIn/<init>()V
	putstatic	strings2/_standardIn LPascalTextIn;


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
	putstatic	strings2/cube1 [[[Ljava/lang/StringBuilder;

	iconst_2
	iconst_2
	iconst_2
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
	iconst_2
	if_icmpge	L010

	dup
	iload_2
	aaload

	iconst_0
	istore_3
L011:
	iload_3
	iconst_2
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
	putstatic	strings2/cube2 [[[Ljava/lang/StringBuilder;
	iconst_5
	invokestatic	PaddedString.create(I)Ljava/lang/StringBuilder;
	putstatic	strings2/name1 Ljava/lang/StringBuilder;
	iconst_5
	invokestatic	PaddedString.create(I)Ljava/lang/StringBuilder;
	putstatic	strings2/name2 Ljava/lang/StringBuilder;

.line 32
	getstatic	strings2/name1 Ljava/lang/StringBuilder;
	dup
	iconst_0
	invokevirtual	java/lang/StringBuilder.setLength(I)V
	ldc	"AAAAA"
	invokevirtual	java/lang/StringBuilder.append(Ljava/lang/String;)Ljava/lang/StringBuilder;
	pop
.line 33
	getstatic	strings2/name2 Ljava/lang/StringBuilder;
	dup
	iconst_0
	invokevirtual	java/lang/StringBuilder.setLength(I)V
	ldc	"BBBBB"
	invokevirtual	java/lang/StringBuilder.append(Ljava/lang/String;)Ljava/lang/StringBuilder;
	pop
.line 34
	getstatic	java/lang/System/out Ljava/io/PrintStream;
	ldc	"'%s, %s'\n"
	iconst_2
	anewarray	java/lang/Object
	dup
	iconst_0
	getstatic	strings2/name1 Ljava/lang/StringBuilder;
	aastore
	dup
	iconst_1
	getstatic	strings2/name2 Ljava/lang/StringBuilder;
	aastore
	invokestatic	java/lang/String/format(Ljava/lang/String;[Ljava/lang/Object;)Ljava/lang/String;
	invokevirtual	java/io/PrintStream.print(Ljava/lang/String;)V
.line 36
	getstatic	strings2/name1 Ljava/lang/StringBuilder;
	invokestatic	Cloner.deepClone(Ljava/lang/Object;)Ljava/lang/Object;
	checkcast	java/lang/StringBuilder
	getstatic	strings2/name2 Ljava/lang/StringBuilder;
	invokestatic	Cloner.deepClone(Ljava/lang/Object;)Ljava/lang/Object;
	checkcast	java/lang/StringBuilder
	invokestatic	strings2/noxfer1(Ljava/lang/StringBuilder;Ljava/lang/StringBuilder;)V
.line 37
	getstatic	java/lang/System/out Ljava/io/PrintStream;
	ldc	"'%s, %s'\n"
	iconst_2
	anewarray	java/lang/Object
	dup
	iconst_0
	getstatic	strings2/name1 Ljava/lang/StringBuilder;
	aastore
	dup
	iconst_1
	getstatic	strings2/name2 Ljava/lang/StringBuilder;
	aastore
	invokestatic	java/lang/String/format(Ljava/lang/String;[Ljava/lang/Object;)Ljava/lang/String;
	invokevirtual	java/io/PrintStream.print(Ljava/lang/String;)V
.line 39
	getstatic	strings2/name1 Ljava/lang/StringBuilder;
	invokestatic	Cloner.deepClone(Ljava/lang/Object;)Ljava/lang/Object;
	checkcast	java/lang/StringBuilder
	getstatic	strings2/name2 Ljava/lang/StringBuilder;
	invokestatic	strings2/xfer1(Ljava/lang/StringBuilder;Ljava/lang/StringBuilder;)V
.line 40
	getstatic	java/lang/System/out Ljava/io/PrintStream;
	ldc	"'%s, %s'\n"
	iconst_2
	anewarray	java/lang/Object
	dup
	iconst_0
	getstatic	strings2/name1 Ljava/lang/StringBuilder;
	aastore
	dup
	iconst_1
	getstatic	strings2/name2 Ljava/lang/StringBuilder;
	aastore
	invokestatic	java/lang/String/format(Ljava/lang/String;[Ljava/lang/Object;)Ljava/lang/String;
	invokevirtual	java/io/PrintStream.print(Ljava/lang/String;)V
.line 42
	getstatic	strings2/cube1 [[[Ljava/lang/StringBuilder;
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
.line 43
	getstatic	strings2/cube2 [[[Ljava/lang/StringBuilder;
	iconst_1
	aaload
	iconst_0
	aaload
	iconst_1
	aaload
	dup
	iconst_0
	invokevirtual	java/lang/StringBuilder.setLength(I)V
	ldc	"xxxxx"
	invokevirtual	java/lang/StringBuilder.append(Ljava/lang/String;)Ljava/lang/StringBuilder;
	pop
.line 44
	getstatic	java/lang/System/out Ljava/io/PrintStream;
	ldc	"'%s, %s'\n"
	iconst_2
	anewarray	java/lang/Object
	dup
	iconst_0
	getstatic	strings2/cube1 [[[Ljava/lang/StringBuilder;
	iconst_1
	aaload
	iconst_0
	aaload
	iconst_1
	aaload
	aastore
	dup
	iconst_1
	getstatic	strings2/cube2 [[[Ljava/lang/StringBuilder;
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
	getstatic	strings2/cube1 [[[Ljava/lang/StringBuilder;
	invokestatic	Cloner.deepClone(Ljava/lang/Object;)Ljava/lang/Object;
	checkcast	[[[Ljava/lang/StringBuilder;
	getstatic	strings2/cube2 [[[Ljava/lang/StringBuilder;
	invokestatic	Cloner.deepClone(Ljava/lang/Object;)Ljava/lang/Object;
	checkcast	[[[Ljava/lang/StringBuilder;
	invokestatic	strings2/noxfer2([[[Ljava/lang/StringBuilder;[[[Ljava/lang/StringBuilder;)V
.line 47
	getstatic	java/lang/System/out Ljava/io/PrintStream;
	ldc	"'%s, %s'\n"
	iconst_2
	anewarray	java/lang/Object
	dup
	iconst_0
	getstatic	strings2/cube1 [[[Ljava/lang/StringBuilder;
	iconst_1
	aaload
	iconst_0
	aaload
	iconst_1
	aaload
	aastore
	dup
	iconst_1
	getstatic	strings2/cube2 [[[Ljava/lang/StringBuilder;
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
	getstatic	strings2/cube1 [[[Ljava/lang/StringBuilder;
	invokestatic	Cloner.deepClone(Ljava/lang/Object;)Ljava/lang/Object;
	checkcast	[[[Ljava/lang/StringBuilder;
	getstatic	strings2/cube2 [[[Ljava/lang/StringBuilder;
	invokestatic	strings2/xfer2([[[Ljava/lang/StringBuilder;[[[Ljava/lang/StringBuilder;)V
.line 50
	getstatic	java/lang/System/out Ljava/io/PrintStream;
	ldc	"'%s, %s'\n"
	iconst_2
	anewarray	java/lang/Object
	dup
	iconst_0
	getstatic	strings2/cube1 [[[Ljava/lang/StringBuilder;
	iconst_1
	aaload
	iconst_0
	aaload
	iconst_1
	aaload
	aastore
	dup
	iconst_1
	getstatic	strings2/cube2 [[[Ljava/lang/StringBuilder;
	iconst_1
	aaload
	iconst_0
	aaload
	iconst_1
	aaload
	aastore
	invokestatic	java/lang/String/format(Ljava/lang/String;[Ljava/lang/Object;)Ljava/lang/String;
	invokevirtual	java/io/PrintStream.print(Ljava/lang/String;)V

	getstatic	strings2/_runTimer LRunTimer;
	invokevirtual	RunTimer.printElapsedTime()V

	return

.limit locals 4
.limit stack 8
.end method
