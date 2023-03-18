.class public stringtest
.super java/lang/Object

.field private static _runTimer LRunTimer;
.field private static _standardIn LPascalTextIn;

.field private static str5 Ljava/lang/StringBuilder;
.field private static str8 Ljava/lang/StringBuilder;
.field private static strv Ljava/lang/StringBuilder;

.method public <init>()V

	aload_0
	invokenonvirtual	java/lang/Object/<init>()V
	return

.limit locals 1
.limit stack 1
.end method

.method private static teststrings(Ljava/lang/StringBuilder;Ljava/lang/StringBuilder;)V

.var 2 is b1 Z
.var 3 is b2 Z
.var 4 is b3 Z
.var 0 is s5 Ljava/lang/StringBuilder;
.var 1 is s8 Ljava/lang/StringBuilder;


.line 17
	aload_0
	invokevirtual	java/lang/StringBuilder.toString()Ljava/lang/String;
	aload_1
	invokevirtual	java/lang/StringBuilder.toString()Ljava/lang/String;
	invokevirtual	java/lang/String.compareTo(Ljava/lang/String;)I
	ifgt	L001
	iconst_0
	goto	L002
L001:
	iconst_1
L002:
	istore_2
.line 18
	aload_0
	invokevirtual	java/lang/StringBuilder.toString()Ljava/lang/String;
	ldc	"goodbye"
	invokevirtual	java/lang/String.compareTo(Ljava/lang/String;)I
	iflt	L003
	iconst_0
	goto	L004
L003:
	iconst_1
L004:
	istore_3
.line 19
	ldc	"nobody"
	aload_1
	invokevirtual	java/lang/StringBuilder.toString()Ljava/lang/String;
	invokevirtual	java/lang/String.compareTo(Ljava/lang/String;)I
	ifge	L005
	iconst_0
	goto	L006
L005:
	iconst_1
L006:
	istore	4
.line 21
	getstatic	java/lang/System/out Ljava/io/PrintStream;
	ldc	"%6s%6s%6s\n"
	iconst_3
	anewarray	java/lang/Object
	dup
	iconst_0
	iload_2
	ifne	L007
	ldc	"false"
	goto	L008
L007:
	ldc	"true"
L008:
	aastore
	dup
	iconst_1
	iload_3
	ifne	L009
	ldc	"false"
	goto	L010
L009:
	ldc	"true"
L010:
	aastore
	dup
	iconst_2
	iload	4
	ifne	L011
	ldc	"false"
	goto	L012
L011:
	ldc	"true"
L012:
	aastore
	invokestatic	java/lang/String/format(Ljava/lang/String;[Ljava/lang/Object;)Ljava/lang/String;
	invokevirtual	java/io/PrintStream.print(Ljava/lang/String;)V

	return

.limit locals 5
.limit stack 8
.end method

.method public static main([Ljava/lang/String;)V

	new	RunTimer
	dup
	invokenonvirtual	RunTimer/<init>()V
	putstatic	stringtest/_runTimer LRunTimer;
	new	PascalTextIn
	dup
	invokenonvirtual	PascalTextIn/<init>()V
	putstatic	stringtest/_standardIn LPascalTextIn;

	iconst_5
	invokestatic	PaddedString.create(I)Ljava/lang/StringBuilder;
	putstatic	stringtest/str5 Ljava/lang/StringBuilder;
	bipush	8
	invokestatic	PaddedString.create(I)Ljava/lang/StringBuilder;
	putstatic	stringtest/str8 Ljava/lang/StringBuilder;
	iconst_5
	invokestatic	PaddedString.create(I)Ljava/lang/StringBuilder;
	putstatic	stringtest/strv Ljava/lang/StringBuilder;

.line 25
	getstatic	stringtest/str5 Ljava/lang/StringBuilder;
	dup
	iconst_0
	invokevirtual	java/lang/StringBuilder.setLength(I)V
	ldc	"hello"
	invokevirtual	java/lang/StringBuilder.append(Ljava/lang/String;)Ljava/lang/StringBuilder;
	pop
.line 26
	getstatic	stringtest/str8 Ljava/lang/StringBuilder;
	dup
	iconst_0
	invokevirtual	java/lang/StringBuilder.setLength(I)V
	ldc	"everyone"
	invokevirtual	java/lang/StringBuilder.append(Ljava/lang/String;)Ljava/lang/StringBuilder;
	pop
.line 27
	getstatic	java/lang/System/out Ljava/io/PrintStream;
	ldc	"'%s, %s'\n"
	iconst_2
	anewarray	java/lang/Object
	dup
	iconst_0
	getstatic	stringtest/str5 Ljava/lang/StringBuilder;
	aastore
	dup
	iconst_1
	getstatic	stringtest/str8 Ljava/lang/StringBuilder;
	aastore
	invokestatic	java/lang/String/format(Ljava/lang/String;[Ljava/lang/Object;)Ljava/lang/String;
	invokevirtual	java/io/PrintStream.print(Ljava/lang/String;)V
.line 29
	getstatic	stringtest/str5 Ljava/lang/StringBuilder;
	getstatic	stringtest/str8 Ljava/lang/StringBuilder;
	invokestatic	Cloner.deepClone(Ljava/lang/Object;)Ljava/lang/Object;
	checkcast	java/lang/StringBuilder
	invokestatic	stringtest/teststrings(Ljava/lang/StringBuilder;Ljava/lang/StringBuilder;)V
.line 31
	getstatic	stringtest/str5 Ljava/lang/StringBuilder;
	dup
	iconst_0
	invokevirtual	java/lang/StringBuilder.setLength(I)V
	getstatic	stringtest/str8 Ljava/lang/StringBuilder;
	iconst_0
	iconst_5
	invokevirtual	java/lang/StringBuilder.substring(II)Ljava/lang/String;
	invokevirtual	java/lang/StringBuilder.append(Ljava/lang/CharSequence;)Ljava/lang/StringBuilder;
	pop
.line 32
	getstatic	stringtest/strv Ljava/lang/StringBuilder;
	dup
	iconst_0
	invokevirtual	java/lang/StringBuilder.setLength(I)V
	getstatic	stringtest/str5 Ljava/lang/StringBuilder;
	invokevirtual	java/lang/StringBuilder.append(Ljava/lang/CharSequence;)Ljava/lang/StringBuilder;
	pop
.line 33
	getstatic	java/lang/System/out Ljava/io/PrintStream;
	ldc	"'%s, %s, %s'\n"
	iconst_3
	anewarray	java/lang/Object
	dup
	iconst_0
	getstatic	stringtest/str5 Ljava/lang/StringBuilder;
	aastore
	dup
	iconst_1
	getstatic	stringtest/strv Ljava/lang/StringBuilder;
	aastore
	dup
	iconst_2
	getstatic	stringtest/str8 Ljava/lang/StringBuilder;
	aastore
	invokestatic	java/lang/String/format(Ljava/lang/String;[Ljava/lang/Object;)Ljava/lang/String;
	invokevirtual	java/io/PrintStream.print(Ljava/lang/String;)V
.line 35
	getstatic	stringtest/str5 Ljava/lang/StringBuilder;
	dup
	iconst_0
	invokevirtual	java/lang/StringBuilder.setLength(I)V
	ldc	"hello"
	invokevirtual	java/lang/StringBuilder.append(Ljava/lang/String;)Ljava/lang/StringBuilder;
	pop
.line 36
	getstatic	stringtest/str8 Ljava/lang/StringBuilder;
	dup
	iconst_0
	invokevirtual	java/lang/StringBuilder.setLength(I)V
	getstatic	stringtest/str5 Ljava/lang/StringBuilder;
	invokevirtual	java/lang/StringBuilder.append(Ljava/lang/CharSequence;)Ljava/lang/StringBuilder;
	bipush	8
	iconst_5
	invokestatic	PaddedString.blanks(II)Ljava/lang/StringBuilder;
	invokevirtual	java/lang/StringBuilder.append(Ljava/lang/CharSequence;)Ljava/lang/StringBuilder;
	pop
.line 37
	getstatic	java/lang/System/out Ljava/io/PrintStream;
	ldc	"'%s'\n"
	iconst_1
	anewarray	java/lang/Object
	dup
	iconst_0
	getstatic	stringtest/str8 Ljava/lang/StringBuilder;
	aastore
	invokestatic	java/lang/String/format(Ljava/lang/String;[Ljava/lang/Object;)Ljava/lang/String;
	invokevirtual	java/io/PrintStream.print(Ljava/lang/String;)V

	getstatic	stringtest/_runTimer LRunTimer;
	invokevirtual	RunTimer.printElapsedTime()V

	return

.limit locals 1
.limit stack 7
.end method
