.class public recordtest1
.super java/lang/Object

.field private static _runTimer LRunTimer;
.field private static _standardIn LPascalTextIn;

.field private static john Ljava/util/HashMap;
.field private static mary Ljava/util/HashMap;

.method public <init>()V

	aload_0
	invokenonvirtual	java/lang/Object/<init>()V
	return

.limit locals 1
.limit stack 1
.end method

.method private static print1(Ljava/util/HashMap;)V

.var 2 is addr Ljava/util/HashMap;
.var 1 is age I
.var 0 is pers Ljava/util/HashMap;
.var 3 is phs [Ljava/lang/StringBuilder;


	new	java/util/HashMap
	dup
	invokenonvirtual	java/util/HashMap/<init>()V
	dup
	ldc	"city"
	bipush	16
	invokestatic	PaddedString.create(I)Ljava/lang/StringBuilder;
	invokevirtual	java/util/HashMap.put(Ljava/lang/Object;Ljava/lang/Object;)Ljava/lang/Object;
	pop
	dup
	ldc	"state"
	iconst_2
	invokestatic	PaddedString.create(I)Ljava/lang/StringBuilder;
	invokevirtual	java/util/HashMap.put(Ljava/lang/Object;Ljava/lang/Object;)Ljava/lang/Object;
	pop
	dup
	ldc	"street"
	bipush	16
	invokestatic	PaddedString.create(I)Ljava/lang/StringBuilder;
	invokevirtual	java/util/HashMap.put(Ljava/lang/Object;Ljava/lang/Object;)Ljava/lang/Object;
	pop
	dup
	ldc	"zip"
	iconst_5
	invokestatic	PaddedString.create(I)Ljava/lang/StringBuilder;
	invokevirtual	java/util/HashMap.put(Ljava/lang/Object;Ljava/lang/Object;)Ljava/lang/Object;
	pop
	astore_2

	iconst_2
	anewarray	java/lang/StringBuilder

	iconst_0
	istore	4
L001:
	iload	4
	iconst_2
	if_icmpge	L002

	dup
	iload	4
	bipush	8
	invokestatic	PaddedString.create(I)Ljava/lang/StringBuilder;
	aastore
	iinc	4 1
	goto	L001
L002:
	astore_3

.line 36
	aload_0
	ldc	"age"
	invokevirtual	java/util/HashMap.get(Ljava/lang/Object;)Ljava/lang/Object;
	checkcast	java/lang/Integer
	invokevirtual	java/lang/Integer.intValue()I
	istore_1
.line 38
	aload_2
	ldc	"street"
	invokevirtual	java/util/HashMap.get(Ljava/lang/Object;)Ljava/lang/Object;
	checkcast	java/lang/StringBuilder
	dup
	iconst_0
	invokevirtual	java/lang/StringBuilder.setLength(I)V
	aload_0
	ldc	"address"
	invokevirtual	java/util/HashMap.get(Ljava/lang/Object;)Ljava/lang/Object;
	checkcast	java/util/HashMap
	ldc	"street"
	invokevirtual	java/util/HashMap.get(Ljava/lang/Object;)Ljava/lang/Object;
	checkcast	java/lang/StringBuilder
	invokevirtual	java/lang/StringBuilder.append(Ljava/lang/CharSequence;)Ljava/lang/StringBuilder;
	pop
.line 39
	aload_2
	ldc	"city"
	invokevirtual	java/util/HashMap.get(Ljava/lang/Object;)Ljava/lang/Object;
	checkcast	java/lang/StringBuilder
	dup
	iconst_0
	invokevirtual	java/lang/StringBuilder.setLength(I)V
	aload_0
	ldc	"address"
	invokevirtual	java/util/HashMap.get(Ljava/lang/Object;)Ljava/lang/Object;
	checkcast	java/util/HashMap
	ldc	"city"
	invokevirtual	java/util/HashMap.get(Ljava/lang/Object;)Ljava/lang/Object;
	checkcast	java/lang/StringBuilder
	invokevirtual	java/lang/StringBuilder.append(Ljava/lang/CharSequence;)Ljava/lang/StringBuilder;
	pop
.line 40
	aload_2
	ldc	"state"
	invokevirtual	java/util/HashMap.get(Ljava/lang/Object;)Ljava/lang/Object;
	checkcast	java/lang/StringBuilder
	dup
	iconst_0
	invokevirtual	java/lang/StringBuilder.setLength(I)V
	aload_0
	ldc	"address"
	invokevirtual	java/util/HashMap.get(Ljava/lang/Object;)Ljava/lang/Object;
	checkcast	java/util/HashMap
	ldc	"state"
	invokevirtual	java/util/HashMap.get(Ljava/lang/Object;)Ljava/lang/Object;
	checkcast	java/lang/StringBuilder
	invokevirtual	java/lang/StringBuilder.append(Ljava/lang/CharSequence;)Ljava/lang/StringBuilder;
	pop
.line 41
	aload_2
	ldc	"zip"
	invokevirtual	java/util/HashMap.get(Ljava/lang/Object;)Ljava/lang/Object;
	checkcast	java/lang/StringBuilder
	dup
	iconst_0
	invokevirtual	java/lang/StringBuilder.setLength(I)V
	aload_0
	ldc	"address"
	invokevirtual	java/util/HashMap.get(Ljava/lang/Object;)Ljava/lang/Object;
	checkcast	java/util/HashMap
	ldc	"zip"
	invokevirtual	java/util/HashMap.get(Ljava/lang/Object;)Ljava/lang/Object;
	checkcast	java/lang/StringBuilder
	invokevirtual	java/lang/StringBuilder.append(Ljava/lang/CharSequence;)Ljava/lang/StringBuilder;
	pop
.line 43
	aload_3
	iconst_0
	aaload
	dup
	iconst_0
	invokevirtual	java/lang/StringBuilder.setLength(I)V
	aload_0
	ldc	"phones"
	invokevirtual	java/util/HashMap.get(Ljava/lang/Object;)Ljava/lang/Object;
	checkcast	[Ljava/lang/StringBuilder;
	iconst_0
	aaload
	invokevirtual	java/lang/StringBuilder.append(Ljava/lang/CharSequence;)Ljava/lang/StringBuilder;
	pop
.line 44
	aload_3
	iconst_1
	aaload
	dup
	iconst_0
	invokevirtual	java/lang/StringBuilder.setLength(I)V
	aload_0
	ldc	"phones"
	invokevirtual	java/util/HashMap.get(Ljava/lang/Object;)Ljava/lang/Object;
	checkcast	[Ljava/lang/StringBuilder;
	iconst_1
	aaload
	invokevirtual	java/lang/StringBuilder.append(Ljava/lang/CharSequence;)Ljava/lang/StringBuilder;
	pop
.line 46
	getstatic	java/lang/System/out Ljava/io/PrintStream;
	invokevirtual	java/io/PrintStream.println()V
.line 47
	getstatic	java/lang/System/out Ljava/io/PrintStream;
	ldc	"First name: %s\n"
	iconst_1
	anewarray	java/lang/Object
	dup
	iconst_0
	aload_0
	ldc	"firstname"
	invokevirtual	java/util/HashMap.get(Ljava/lang/Object;)Ljava/lang/Object;
	checkcast	java/lang/StringBuilder
	aastore
	invokestatic	java/lang/String/format(Ljava/lang/String;[Ljava/lang/Object;)Ljava/lang/String;
	invokevirtual	java/io/PrintStream.print(Ljava/lang/String;)V
.line 48
	getstatic	java/lang/System/out Ljava/io/PrintStream;
	ldc	" Last name: %s\n"
	iconst_1
	anewarray	java/lang/Object
	dup
	iconst_0
	aload_0
	ldc	"lastname"
	invokevirtual	java/util/HashMap.get(Ljava/lang/Object;)Ljava/lang/Object;
	checkcast	java/lang/StringBuilder
	aastore
	invokestatic	java/lang/String/format(Ljava/lang/String;[Ljava/lang/Object;)Ljava/lang/String;
	invokevirtual	java/io/PrintStream.print(Ljava/lang/String;)V
.line 49
	getstatic	java/lang/System/out Ljava/io/PrintStream;
	ldc	"       Age: %d\n"
	iconst_1
	anewarray	java/lang/Object
	dup
	iconst_0
	iload_1
	invokestatic	java/lang/Integer.valueOf(I)Ljava/lang/Integer;
	aastore
	invokestatic	java/lang/String/format(Ljava/lang/String;[Ljava/lang/Object;)Ljava/lang/String;
	invokevirtual	java/io/PrintStream.print(Ljava/lang/String;)V
.line 50
	getstatic	java/lang/System/out Ljava/io/PrintStream;
	ldc	"    Street: %s\n"
	iconst_1
	anewarray	java/lang/Object
	dup
	iconst_0
	aload_2
	ldc	"street"
	invokevirtual	java/util/HashMap.get(Ljava/lang/Object;)Ljava/lang/Object;
	checkcast	java/lang/StringBuilder
	aastore
	invokestatic	java/lang/String/format(Ljava/lang/String;[Ljava/lang/Object;)Ljava/lang/String;
	invokevirtual	java/io/PrintStream.print(Ljava/lang/String;)V
.line 51
	getstatic	java/lang/System/out Ljava/io/PrintStream;
	ldc	"      City: %s\n"
	iconst_1
	anewarray	java/lang/Object
	dup
	iconst_0
	aload_2
	ldc	"city"
	invokevirtual	java/util/HashMap.get(Ljava/lang/Object;)Ljava/lang/Object;
	checkcast	java/lang/StringBuilder
	aastore
	invokestatic	java/lang/String/format(Ljava/lang/String;[Ljava/lang/Object;)Ljava/lang/String;
	invokevirtual	java/io/PrintStream.print(Ljava/lang/String;)V
.line 52
	getstatic	java/lang/System/out Ljava/io/PrintStream;
	ldc	"     State: %s\n"
	iconst_1
	anewarray	java/lang/Object
	dup
	iconst_0
	aload_2
	ldc	"state"
	invokevirtual	java/util/HashMap.get(Ljava/lang/Object;)Ljava/lang/Object;
	checkcast	java/lang/StringBuilder
	aastore
	invokestatic	java/lang/String/format(Ljava/lang/String;[Ljava/lang/Object;)Ljava/lang/String;
	invokevirtual	java/io/PrintStream.print(Ljava/lang/String;)V
.line 53
	getstatic	java/lang/System/out Ljava/io/PrintStream;
	ldc	"       ZIP: %s\n"
	iconst_1
	anewarray	java/lang/Object
	dup
	iconst_0
	aload_2
	ldc	"zip"
	invokevirtual	java/util/HashMap.get(Ljava/lang/Object;)Ljava/lang/Object;
	checkcast	java/lang/StringBuilder
	aastore
	invokestatic	java/lang/String/format(Ljava/lang/String;[Ljava/lang/Object;)Ljava/lang/String;
	invokevirtual	java/io/PrintStream.print(Ljava/lang/String;)V
.line 54
	getstatic	java/lang/System/out Ljava/io/PrintStream;
	ldc	"  Phone #1: %s\n"
	iconst_1
	anewarray	java/lang/Object
	dup
	iconst_0
	aload_3
	iconst_0
	aaload
	aastore
	invokestatic	java/lang/String/format(Ljava/lang/String;[Ljava/lang/Object;)Ljava/lang/String;
	invokevirtual	java/io/PrintStream.print(Ljava/lang/String;)V
.line 55
	getstatic	java/lang/System/out Ljava/io/PrintStream;
	ldc	"  Phone #2: %s\n"
	iconst_1
	anewarray	java/lang/Object
	dup
	iconst_0
	aload_3
	iconst_1
	aaload
	aastore
	invokestatic	java/lang/String/format(Ljava/lang/String;[Ljava/lang/Object;)Ljava/lang/String;
	invokevirtual	java/io/PrintStream.print(Ljava/lang/String;)V

	return

.limit locals 5
.limit stack 8
.end method

.method private static print2(Ljava/util/HashMap;)V

.var 2 is addr Ljava/util/HashMap;
.var 1 is age I
.var 0 is pers Ljava/util/HashMap;
.var 3 is phs [Ljava/lang/StringBuilder;


	new	java/util/HashMap
	dup
	invokenonvirtual	java/util/HashMap/<init>()V
	dup
	ldc	"city"
	bipush	16
	invokestatic	PaddedString.create(I)Ljava/lang/StringBuilder;
	invokevirtual	java/util/HashMap.put(Ljava/lang/Object;Ljava/lang/Object;)Ljava/lang/Object;
	pop
	dup
	ldc	"state"
	iconst_2
	invokestatic	PaddedString.create(I)Ljava/lang/StringBuilder;
	invokevirtual	java/util/HashMap.put(Ljava/lang/Object;Ljava/lang/Object;)Ljava/lang/Object;
	pop
	dup
	ldc	"street"
	bipush	16
	invokestatic	PaddedString.create(I)Ljava/lang/StringBuilder;
	invokevirtual	java/util/HashMap.put(Ljava/lang/Object;Ljava/lang/Object;)Ljava/lang/Object;
	pop
	dup
	ldc	"zip"
	iconst_5
	invokestatic	PaddedString.create(I)Ljava/lang/StringBuilder;
	invokevirtual	java/util/HashMap.put(Ljava/lang/Object;Ljava/lang/Object;)Ljava/lang/Object;
	pop
	astore_2

	iconst_2
	anewarray	java/lang/StringBuilder

	iconst_0
	istore	4
L003:
	iload	4
	iconst_2
	if_icmpge	L004

	dup
	iload	4
	bipush	8
	invokestatic	PaddedString.create(I)Ljava/lang/StringBuilder;
	aastore
	iinc	4 1
	goto	L003
L004:
	astore_3

.line 66
	getstatic	java/lang/System/out Ljava/io/PrintStream;
	invokevirtual	java/io/PrintStream.println()V
.line 67
	getstatic	java/lang/System/out Ljava/io/PrintStream;
	ldc	"First name: %s\n"
	iconst_1
	anewarray	java/lang/Object
	dup
	iconst_0
	aload_0
	ldc	"firstname"
	invokevirtual	java/util/HashMap.get(Ljava/lang/Object;)Ljava/lang/Object;
	checkcast	java/lang/StringBuilder
	aastore
	invokestatic	java/lang/String/format(Ljava/lang/String;[Ljava/lang/Object;)Ljava/lang/String;
	invokevirtual	java/io/PrintStream.print(Ljava/lang/String;)V
.line 68
	getstatic	java/lang/System/out Ljava/io/PrintStream;
	ldc	" Last name: %s\n"
	iconst_1
	anewarray	java/lang/Object
	dup
	iconst_0
	aload_0
	ldc	"lastname"
	invokevirtual	java/util/HashMap.get(Ljava/lang/Object;)Ljava/lang/Object;
	checkcast	java/lang/StringBuilder
	aastore
	invokestatic	java/lang/String/format(Ljava/lang/String;[Ljava/lang/Object;)Ljava/lang/String;
	invokevirtual	java/io/PrintStream.print(Ljava/lang/String;)V
.line 69
	getstatic	java/lang/System/out Ljava/io/PrintStream;
	ldc	"       Age: %d\n"
	iconst_1
	anewarray	java/lang/Object
	dup
	iconst_0
	aload_0
	ldc	"age"
	invokevirtual	java/util/HashMap.get(Ljava/lang/Object;)Ljava/lang/Object;
	checkcast	java/lang/Integer
	invokevirtual	java/lang/Integer.intValue()I
	invokestatic	java/lang/Integer.valueOf(I)Ljava/lang/Integer;
	aastore
	invokestatic	java/lang/String/format(Ljava/lang/String;[Ljava/lang/Object;)Ljava/lang/String;
	invokevirtual	java/io/PrintStream.print(Ljava/lang/String;)V
.line 70
	getstatic	java/lang/System/out Ljava/io/PrintStream;
	ldc	"    Street: %s\n"
	iconst_1
	anewarray	java/lang/Object
	dup
	iconst_0
	aload_0
	ldc	"address"
	invokevirtual	java/util/HashMap.get(Ljava/lang/Object;)Ljava/lang/Object;
	checkcast	java/util/HashMap
	ldc	"street"
	invokevirtual	java/util/HashMap.get(Ljava/lang/Object;)Ljava/lang/Object;
	checkcast	java/lang/StringBuilder
	aastore
	invokestatic	java/lang/String/format(Ljava/lang/String;[Ljava/lang/Object;)Ljava/lang/String;
	invokevirtual	java/io/PrintStream.print(Ljava/lang/String;)V
.line 71
	getstatic	java/lang/System/out Ljava/io/PrintStream;
	ldc	"      City: %s\n"
	iconst_1
	anewarray	java/lang/Object
	dup
	iconst_0
	aload_0
	ldc	"address"
	invokevirtual	java/util/HashMap.get(Ljava/lang/Object;)Ljava/lang/Object;
	checkcast	java/util/HashMap
	ldc	"city"
	invokevirtual	java/util/HashMap.get(Ljava/lang/Object;)Ljava/lang/Object;
	checkcast	java/lang/StringBuilder
	aastore
	invokestatic	java/lang/String/format(Ljava/lang/String;[Ljava/lang/Object;)Ljava/lang/String;
	invokevirtual	java/io/PrintStream.print(Ljava/lang/String;)V
.line 72
	getstatic	java/lang/System/out Ljava/io/PrintStream;
	ldc	"     State: %s\n"
	iconst_1
	anewarray	java/lang/Object
	dup
	iconst_0
	aload_0
	ldc	"address"
	invokevirtual	java/util/HashMap.get(Ljava/lang/Object;)Ljava/lang/Object;
	checkcast	java/util/HashMap
	ldc	"state"
	invokevirtual	java/util/HashMap.get(Ljava/lang/Object;)Ljava/lang/Object;
	checkcast	java/lang/StringBuilder
	aastore
	invokestatic	java/lang/String/format(Ljava/lang/String;[Ljava/lang/Object;)Ljava/lang/String;
	invokevirtual	java/io/PrintStream.print(Ljava/lang/String;)V
.line 73
	getstatic	java/lang/System/out Ljava/io/PrintStream;
	ldc	"       ZIP: %s\n"
	iconst_1
	anewarray	java/lang/Object
	dup
	iconst_0
	aload_0
	ldc	"address"
	invokevirtual	java/util/HashMap.get(Ljava/lang/Object;)Ljava/lang/Object;
	checkcast	java/util/HashMap
	ldc	"zip"
	invokevirtual	java/util/HashMap.get(Ljava/lang/Object;)Ljava/lang/Object;
	checkcast	java/lang/StringBuilder
	aastore
	invokestatic	java/lang/String/format(Ljava/lang/String;[Ljava/lang/Object;)Ljava/lang/String;
	invokevirtual	java/io/PrintStream.print(Ljava/lang/String;)V
.line 74
	getstatic	java/lang/System/out Ljava/io/PrintStream;
	ldc	"  Phone #1: %s\n"
	iconst_1
	anewarray	java/lang/Object
	dup
	iconst_0
	aload_0
	ldc	"phones"
	invokevirtual	java/util/HashMap.get(Ljava/lang/Object;)Ljava/lang/Object;
	checkcast	[Ljava/lang/StringBuilder;
	iconst_0
	aaload
	aastore
	invokestatic	java/lang/String/format(Ljava/lang/String;[Ljava/lang/Object;)Ljava/lang/String;
	invokevirtual	java/io/PrintStream.print(Ljava/lang/String;)V
.line 75
	getstatic	java/lang/System/out Ljava/io/PrintStream;
	ldc	"  Phone #2: %s\n"
	iconst_1
	anewarray	java/lang/Object
	dup
	iconst_0
	aload_0
	ldc	"phones"
	invokevirtual	java/util/HashMap.get(Ljava/lang/Object;)Ljava/lang/Object;
	checkcast	[Ljava/lang/StringBuilder;
	iconst_1
	aaload
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
	putstatic	recordtest1/_runTimer LRunTimer;
	new	PascalTextIn
	dup
	invokenonvirtual	PascalTextIn/<init>()V
	putstatic	recordtest1/_standardIn LPascalTextIn;


	new	java/util/HashMap
	dup
	invokenonvirtual	java/util/HashMap/<init>()V
	dup
	ldc	"address"

	new	java/util/HashMap
	dup
	invokenonvirtual	java/util/HashMap/<init>()V
	dup
	ldc	"city"
	bipush	16
	invokestatic	PaddedString.create(I)Ljava/lang/StringBuilder;
	invokevirtual	java/util/HashMap.put(Ljava/lang/Object;Ljava/lang/Object;)Ljava/lang/Object;
	pop
	dup
	ldc	"state"
	iconst_2
	invokestatic	PaddedString.create(I)Ljava/lang/StringBuilder;
	invokevirtual	java/util/HashMap.put(Ljava/lang/Object;Ljava/lang/Object;)Ljava/lang/Object;
	pop
	dup
	ldc	"street"
	bipush	16
	invokestatic	PaddedString.create(I)Ljava/lang/StringBuilder;
	invokevirtual	java/util/HashMap.put(Ljava/lang/Object;Ljava/lang/Object;)Ljava/lang/Object;
	pop
	dup
	ldc	"zip"
	iconst_5
	invokestatic	PaddedString.create(I)Ljava/lang/StringBuilder;
	invokevirtual	java/util/HashMap.put(Ljava/lang/Object;Ljava/lang/Object;)Ljava/lang/Object;
	pop
	invokevirtual	java/util/HashMap.put(Ljava/lang/Object;Ljava/lang/Object;)Ljava/lang/Object;
	pop
	dup
	ldc	"age"
	iconst_0
	invokestatic	java/lang/Integer.valueOf(I)Ljava/lang/Integer;
	invokevirtual	java/util/HashMap.put(Ljava/lang/Object;Ljava/lang/Object;)Ljava/lang/Object;
	pop
	dup
	ldc	"firstname"
	bipush	16
	invokestatic	PaddedString.create(I)Ljava/lang/StringBuilder;
	invokevirtual	java/util/HashMap.put(Ljava/lang/Object;Ljava/lang/Object;)Ljava/lang/Object;
	pop
	dup
	ldc	"lastname"
	bipush	16
	invokestatic	PaddedString.create(I)Ljava/lang/StringBuilder;
	invokevirtual	java/util/HashMap.put(Ljava/lang/Object;Ljava/lang/Object;)Ljava/lang/Object;
	pop
	dup
	ldc	"phones"

	iconst_2
	anewarray	java/lang/StringBuilder

	iconst_0
	istore_1
L005:
	iload_1
	iconst_2
	if_icmpge	L006

	dup
	iload_1
	bipush	8
	invokestatic	PaddedString.create(I)Ljava/lang/StringBuilder;
	aastore
	iinc	1 1
	goto	L005
L006:
	invokevirtual	java/util/HashMap.put(Ljava/lang/Object;Ljava/lang/Object;)Ljava/lang/Object;
	pop
	putstatic	recordtest1/john Ljava/util/HashMap;

	new	java/util/HashMap
	dup
	invokenonvirtual	java/util/HashMap/<init>()V
	dup
	ldc	"address"

	new	java/util/HashMap
	dup
	invokenonvirtual	java/util/HashMap/<init>()V
	dup
	ldc	"city"
	bipush	16
	invokestatic	PaddedString.create(I)Ljava/lang/StringBuilder;
	invokevirtual	java/util/HashMap.put(Ljava/lang/Object;Ljava/lang/Object;)Ljava/lang/Object;
	pop
	dup
	ldc	"state"
	iconst_2
	invokestatic	PaddedString.create(I)Ljava/lang/StringBuilder;
	invokevirtual	java/util/HashMap.put(Ljava/lang/Object;Ljava/lang/Object;)Ljava/lang/Object;
	pop
	dup
	ldc	"street"
	bipush	16
	invokestatic	PaddedString.create(I)Ljava/lang/StringBuilder;
	invokevirtual	java/util/HashMap.put(Ljava/lang/Object;Ljava/lang/Object;)Ljava/lang/Object;
	pop
	dup
	ldc	"zip"
	iconst_5
	invokestatic	PaddedString.create(I)Ljava/lang/StringBuilder;
	invokevirtual	java/util/HashMap.put(Ljava/lang/Object;Ljava/lang/Object;)Ljava/lang/Object;
	pop
	invokevirtual	java/util/HashMap.put(Ljava/lang/Object;Ljava/lang/Object;)Ljava/lang/Object;
	pop
	dup
	ldc	"age"
	iconst_0
	invokestatic	java/lang/Integer.valueOf(I)Ljava/lang/Integer;
	invokevirtual	java/util/HashMap.put(Ljava/lang/Object;Ljava/lang/Object;)Ljava/lang/Object;
	pop
	dup
	ldc	"firstname"
	bipush	16
	invokestatic	PaddedString.create(I)Ljava/lang/StringBuilder;
	invokevirtual	java/util/HashMap.put(Ljava/lang/Object;Ljava/lang/Object;)Ljava/lang/Object;
	pop
	dup
	ldc	"lastname"
	bipush	16
	invokestatic	PaddedString.create(I)Ljava/lang/StringBuilder;
	invokevirtual	java/util/HashMap.put(Ljava/lang/Object;Ljava/lang/Object;)Ljava/lang/Object;
	pop
	dup
	ldc	"phones"

	iconst_2
	anewarray	java/lang/StringBuilder

	iconst_0
	istore_1
L007:
	iload_1
	iconst_2
	if_icmpge	L008

	dup
	iload_1
	bipush	8
	invokestatic	PaddedString.create(I)Ljava/lang/StringBuilder;
	aastore
	iinc	1 1
	goto	L007
L008:
	invokevirtual	java/util/HashMap.put(Ljava/lang/Object;Ljava/lang/Object;)Ljava/lang/Object;
	pop
	putstatic	recordtest1/mary Ljava/util/HashMap;

.line 79
	getstatic	recordtest1/john Ljava/util/HashMap;
	ldc	"firstname"
	invokevirtual	java/util/HashMap.get(Ljava/lang/Object;)Ljava/lang/Object;
	checkcast	java/lang/StringBuilder
	dup
	iconst_0
	invokevirtual	java/lang/StringBuilder.setLength(I)V
	ldc	"John"
	invokevirtual	java/lang/StringBuilder.append(Ljava/lang/String;)Ljava/lang/StringBuilder;
	bipush	16
	iconst_4
	invokestatic	PaddedString.blanks(II)Ljava/lang/StringBuilder;
	invokevirtual	java/lang/StringBuilder.append(Ljava/lang/CharSequence;)Ljava/lang/StringBuilder;
	pop
.line 80
	getstatic	recordtest1/john Ljava/util/HashMap;
	ldc	"lastname"
	invokevirtual	java/util/HashMap.get(Ljava/lang/Object;)Ljava/lang/Object;
	checkcast	java/lang/StringBuilder
	dup
	iconst_0
	invokevirtual	java/lang/StringBuilder.setLength(I)V
	ldc	"Doe"
	invokevirtual	java/lang/StringBuilder.append(Ljava/lang/String;)Ljava/lang/StringBuilder;
	bipush	16
	iconst_3
	invokestatic	PaddedString.blanks(II)Ljava/lang/StringBuilder;
	invokevirtual	java/lang/StringBuilder.append(Ljava/lang/CharSequence;)Ljava/lang/StringBuilder;
	pop
.line 81
	getstatic	recordtest1/john Ljava/util/HashMap;
	ldc	"age"
	bipush	24
	invokestatic	java/lang/Integer.valueOf(I)Ljava/lang/Integer;
	invokevirtual	java/util/HashMap.put(Ljava/lang/Object;Ljava/lang/Object;)Ljava/lang/Object;
	pop
.line 82
	getstatic	recordtest1/john Ljava/util/HashMap;
	ldc	"address"
	invokevirtual	java/util/HashMap.get(Ljava/lang/Object;)Ljava/lang/Object;
	checkcast	java/util/HashMap
	ldc	"street"
	invokevirtual	java/util/HashMap.get(Ljava/lang/Object;)Ljava/lang/Object;
	checkcast	java/lang/StringBuilder
	dup
	iconst_0
	invokevirtual	java/lang/StringBuilder.setLength(I)V
	ldc	"1680 25th Street"
	invokevirtual	java/lang/StringBuilder.append(Ljava/lang/String;)Ljava/lang/StringBuilder;
	pop
.line 83
	getstatic	recordtest1/john Ljava/util/HashMap;
	ldc	"address"
	invokevirtual	java/util/HashMap.get(Ljava/lang/Object;)Ljava/lang/Object;
	checkcast	java/util/HashMap
	ldc	"city"
	invokevirtual	java/util/HashMap.get(Ljava/lang/Object;)Ljava/lang/Object;
	checkcast	java/lang/StringBuilder
	dup
	iconst_0
	invokevirtual	java/lang/StringBuilder.setLength(I)V
	ldc	"San Pablo"
	invokevirtual	java/lang/StringBuilder.append(Ljava/lang/String;)Ljava/lang/StringBuilder;
	bipush	16
	bipush	9
	invokestatic	PaddedString.blanks(II)Ljava/lang/StringBuilder;
	invokevirtual	java/lang/StringBuilder.append(Ljava/lang/CharSequence;)Ljava/lang/StringBuilder;
	pop
.line 84
	getstatic	recordtest1/john Ljava/util/HashMap;
	ldc	"address"
	invokevirtual	java/util/HashMap.get(Ljava/lang/Object;)Ljava/lang/Object;
	checkcast	java/util/HashMap
	ldc	"state"
	invokevirtual	java/util/HashMap.get(Ljava/lang/Object;)Ljava/lang/Object;
	checkcast	java/lang/StringBuilder
	dup
	iconst_0
	invokevirtual	java/lang/StringBuilder.setLength(I)V
	ldc	"CALIFORNIA"
	iconst_0
	iconst_2
	invokevirtual	java/lang/String.substring(II)Ljava/lang/String;
	invokevirtual	java/lang/StringBuilder.append(Ljava/lang/String;)Ljava/lang/StringBuilder;
	pop
.line 85
	getstatic	recordtest1/john Ljava/util/HashMap;
	ldc	"address"
	invokevirtual	java/util/HashMap.get(Ljava/lang/Object;)Ljava/lang/Object;
	checkcast	java/util/HashMap
	ldc	"zip"
	invokevirtual	java/util/HashMap.get(Ljava/lang/Object;)Ljava/lang/Object;
	checkcast	java/lang/StringBuilder
	dup
	iconst_0
	invokevirtual	java/lang/StringBuilder.setLength(I)V
	ldc	"94806"
	invokevirtual	java/lang/StringBuilder.append(Ljava/lang/String;)Ljava/lang/StringBuilder;
	pop
.line 86
	getstatic	recordtest1/john Ljava/util/HashMap;
	ldc	"phones"
	invokevirtual	java/util/HashMap.get(Ljava/lang/Object;)Ljava/lang/Object;
	checkcast	[Ljava/lang/StringBuilder;
	iconst_0
	aaload
	dup
	iconst_0
	invokevirtual	java/lang/StringBuilder.setLength(I)V
	ldc	"111-1111"
	invokevirtual	java/lang/StringBuilder.append(Ljava/lang/String;)Ljava/lang/StringBuilder;
	pop
.line 87
	getstatic	recordtest1/john Ljava/util/HashMap;
	ldc	"phones"
	invokevirtual	java/util/HashMap.get(Ljava/lang/Object;)Ljava/lang/Object;
	checkcast	[Ljava/lang/StringBuilder;
	iconst_1
	aaload
	dup
	iconst_0
	invokevirtual	java/lang/StringBuilder.setLength(I)V
	ldc	"222-2222"
	invokevirtual	java/lang/StringBuilder.append(Ljava/lang/String;)Ljava/lang/StringBuilder;
	pop
.line 89
	getstatic	recordtest1/mary Ljava/util/HashMap;
	ldc	"firstname"
	invokevirtual	java/util/HashMap.get(Ljava/lang/Object;)Ljava/lang/Object;
	checkcast	java/lang/StringBuilder
	dup
	iconst_0
	invokevirtual	java/lang/StringBuilder.setLength(I)V
	ldc	"Mary"
	invokevirtual	java/lang/StringBuilder.append(Ljava/lang/String;)Ljava/lang/StringBuilder;
	bipush	16
	iconst_4
	invokestatic	PaddedString.blanks(II)Ljava/lang/StringBuilder;
	invokevirtual	java/lang/StringBuilder.append(Ljava/lang/CharSequence;)Ljava/lang/StringBuilder;
	pop
.line 90
	getstatic	recordtest1/mary Ljava/util/HashMap;
	ldc	"lastname"
	invokevirtual	java/util/HashMap.get(Ljava/lang/Object;)Ljava/lang/Object;
	checkcast	java/lang/StringBuilder
	dup
	iconst_0
	invokevirtual	java/lang/StringBuilder.setLength(I)V
	ldc	"Jane"
	invokevirtual	java/lang/StringBuilder.append(Ljava/lang/String;)Ljava/lang/StringBuilder;
	bipush	16
	iconst_4
	invokestatic	PaddedString.blanks(II)Ljava/lang/StringBuilder;
	invokevirtual	java/lang/StringBuilder.append(Ljava/lang/CharSequence;)Ljava/lang/StringBuilder;
	pop
.line 91
	getstatic	recordtest1/mary Ljava/util/HashMap;
	ldc	"age"
	bipush	22
	invokestatic	java/lang/Integer.valueOf(I)Ljava/lang/Integer;
	invokevirtual	java/util/HashMap.put(Ljava/lang/Object;Ljava/lang/Object;)Ljava/lang/Object;
	pop
.line 92
	getstatic	recordtest1/mary Ljava/util/HashMap;
	ldc	"address"
	invokevirtual	java/util/HashMap.get(Ljava/lang/Object;)Ljava/lang/Object;
	checkcast	java/util/HashMap
	ldc	"street"
	invokevirtual	java/util/HashMap.get(Ljava/lang/Object;)Ljava/lang/Object;
	checkcast	java/lang/StringBuilder
	dup
	iconst_0
	invokevirtual	java/lang/StringBuilder.setLength(I)V
	ldc	"4899 Bela Drive "
	invokevirtual	java/lang/StringBuilder.append(Ljava/lang/String;)Ljava/lang/StringBuilder;
	pop
.line 93
	getstatic	recordtest1/mary Ljava/util/HashMap;
	ldc	"address"
	invokevirtual	java/util/HashMap.get(Ljava/lang/Object;)Ljava/lang/Object;
	checkcast	java/util/HashMap
	ldc	"city"
	invokevirtual	java/util/HashMap.get(Ljava/lang/Object;)Ljava/lang/Object;
	checkcast	java/lang/StringBuilder
	dup
	iconst_0
	invokevirtual	java/lang/StringBuilder.setLength(I)V
	ldc	"San Jose"
	invokevirtual	java/lang/StringBuilder.append(Ljava/lang/String;)Ljava/lang/StringBuilder;
	bipush	16
	bipush	8
	invokestatic	PaddedString.blanks(II)Ljava/lang/StringBuilder;
	invokevirtual	java/lang/StringBuilder.append(Ljava/lang/CharSequence;)Ljava/lang/StringBuilder;
	pop
.line 94
	getstatic	recordtest1/mary Ljava/util/HashMap;
	ldc	"address"
	invokevirtual	java/util/HashMap.get(Ljava/lang/Object;)Ljava/lang/Object;
	checkcast	java/util/HashMap
	ldc	"state"
	invokevirtual	java/util/HashMap.get(Ljava/lang/Object;)Ljava/lang/Object;
	checkcast	java/lang/StringBuilder
	dup
	iconst_0
	invokevirtual	java/lang/StringBuilder.setLength(I)V
	ldc	"CALIFORNIA"
	iconst_0
	iconst_2
	invokevirtual	java/lang/String.substring(II)Ljava/lang/String;
	invokevirtual	java/lang/StringBuilder.append(Ljava/lang/String;)Ljava/lang/StringBuilder;
	pop
.line 95
	getstatic	recordtest1/mary Ljava/util/HashMap;
	ldc	"address"
	invokevirtual	java/util/HashMap.get(Ljava/lang/Object;)Ljava/lang/Object;
	checkcast	java/util/HashMap
	ldc	"zip"
	invokevirtual	java/util/HashMap.get(Ljava/lang/Object;)Ljava/lang/Object;
	checkcast	java/lang/StringBuilder
	dup
	iconst_0
	invokevirtual	java/lang/StringBuilder.setLength(I)V
	ldc	"95129"
	invokevirtual	java/lang/StringBuilder.append(Ljava/lang/String;)Ljava/lang/StringBuilder;
	pop
.line 96
	getstatic	recordtest1/mary Ljava/util/HashMap;
	ldc	"phones"
	invokevirtual	java/util/HashMap.get(Ljava/lang/Object;)Ljava/lang/Object;
	checkcast	[Ljava/lang/StringBuilder;
	iconst_0
	aaload
	dup
	iconst_0
	invokevirtual	java/lang/StringBuilder.setLength(I)V
	ldc	"333-3333"
	invokevirtual	java/lang/StringBuilder.append(Ljava/lang/String;)Ljava/lang/StringBuilder;
	pop
.line 97
	getstatic	recordtest1/mary Ljava/util/HashMap;
	ldc	"phones"
	invokevirtual	java/util/HashMap.get(Ljava/lang/Object;)Ljava/lang/Object;
	checkcast	[Ljava/lang/StringBuilder;
	iconst_1
	aaload
	dup
	iconst_0
	invokevirtual	java/lang/StringBuilder.setLength(I)V
	ldc	"444-4444"
	invokevirtual	java/lang/StringBuilder.append(Ljava/lang/String;)Ljava/lang/StringBuilder;
	pop
.line 98
	getstatic	recordtest1/john Ljava/util/HashMap;
	invokestatic	recordtest1/print1(Ljava/util/HashMap;)V
.line 99
	getstatic	recordtest1/mary Ljava/util/HashMap;
	invokestatic	Cloner.deepClone(Ljava/lang/Object;)Ljava/lang/Object;
	checkcast	java/util/HashMap
	invokestatic	recordtest1/print2(Ljava/util/HashMap;)V

	getstatic	recordtest1/_runTimer LRunTimer;
	invokevirtual	RunTimer.printElapsedTime()V

	return

.limit locals 2
.limit stack 8
.end method
