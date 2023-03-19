.class public reads
.super java/lang/Object

.field private static _runTimer LRunTimer;
.field private static _standardIn LPascalTextIn;

.field private static b Z
.field private static c C
.field private static i I
.field private static x F

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
	putstatic	reads/_runTimer LRunTimer;
	new	PascalTextIn
	dup
	invokenonvirtual	PascalTextIn/<init>()V
	putstatic	reads/_standardIn LPascalTextIn;


.line 10
	getstatic	java/lang/System/out Ljava/io/PrintStream;
	ldc	"Type an integer: "
	invokevirtual	java/io/PrintStream.print(Ljava/lang/String;)V
.line 11
	getstatic	reads/_standardIn LPascalTextIn;
	invokevirtual	PascalTextIn.readInteger()I
	putstatic	reads/i I
	getstatic	reads/_standardIn LPascalTextIn;
	invokevirtual	PascalTextIn.nextLine()V
.line 12
	getstatic	java/lang/System/out Ljava/io/PrintStream;
	ldc	"You typed: %d\n"
	iconst_1
	anewarray	java/lang/Object
	dup
	iconst_0
	getstatic	reads/i I
	invokestatic	java/lang/Integer.valueOf(I)Ljava/lang/Integer;
	aastore
	invokestatic	java/lang/String/format(Ljava/lang/String;[Ljava/lang/Object;)Ljava/lang/String;
	invokevirtual	java/io/PrintStream.print(Ljava/lang/String;)V
.line 13
	getstatic	java/lang/System/out Ljava/io/PrintStream;
	invokevirtual	java/io/PrintStream.println()V
.line 15
	getstatic	java/lang/System/out Ljava/io/PrintStream;
	ldc	"Type a real: "
	invokevirtual	java/io/PrintStream.print(Ljava/lang/String;)V
.line 16
	getstatic	reads/_standardIn LPascalTextIn;
	invokevirtual	PascalTextIn.readReal()F
	putstatic	reads/x F
	getstatic	reads/_standardIn LPascalTextIn;
	invokevirtual	PascalTextIn.nextLine()V
.line 17
	getstatic	java/lang/System/out Ljava/io/PrintStream;
	ldc	"You typed: %f\n"
	iconst_1
	anewarray	java/lang/Object
	dup
	iconst_0
	getstatic	reads/x F
	invokestatic	java/lang/Float.valueOf(F)Ljava/lang/Float;
	aastore
	invokestatic	java/lang/String/format(Ljava/lang/String;[Ljava/lang/Object;)Ljava/lang/String;
	invokevirtual	java/io/PrintStream.print(Ljava/lang/String;)V
.line 18
	getstatic	java/lang/System/out Ljava/io/PrintStream;
	invokevirtual	java/io/PrintStream.println()V
.line 20
	getstatic	java/lang/System/out Ljava/io/PrintStream;
	ldc	"Type a character: "
	invokevirtual	java/io/PrintStream.print(Ljava/lang/String;)V
.line 21
	getstatic	reads/_standardIn LPascalTextIn;
	invokevirtual	PascalTextIn.readChar()C
	putstatic	reads/c C
	getstatic	reads/_standardIn LPascalTextIn;
	invokevirtual	PascalTextIn.nextLine()V
.line 22
	getstatic	java/lang/System/out Ljava/io/PrintStream;
	ldc	"You typed: %c\n"
	iconst_1
	anewarray	java/lang/Object
	dup
	iconst_0
	getstatic	reads/c C
	invokestatic	java/lang/Character.valueOf(C)Ljava/lang/Character;
	aastore
	invokestatic	java/lang/String/format(Ljava/lang/String;[Ljava/lang/Object;)Ljava/lang/String;
	invokevirtual	java/io/PrintStream.print(Ljava/lang/String;)V
.line 23
	getstatic	java/lang/System/out Ljava/io/PrintStream;
	invokevirtual	java/io/PrintStream.println()V
.line 25
	getstatic	java/lang/System/out Ljava/io/PrintStream;
	ldc	"Type a boolean: "
	invokevirtual	java/io/PrintStream.print(Ljava/lang/String;)V
.line 26
	getstatic	reads/_standardIn LPascalTextIn;
	invokevirtual	PascalTextIn.readBoolean()Z
	putstatic	reads/b Z
	getstatic	reads/_standardIn LPascalTextIn;
	invokevirtual	PascalTextIn.nextLine()V
.line 27
	getstatic	java/lang/System/out Ljava/io/PrintStream;
	ldc	"You typed: %s\n"
	iconst_1
	anewarray	java/lang/Object
	dup
	iconst_0
	getstatic	reads/b Z
	ifne	L001
	ldc	"false"
	goto	L002
L001:
	ldc	"true"
L002:
	aastore
	invokestatic	java/lang/String/format(Ljava/lang/String;[Ljava/lang/Object;)Ljava/lang/String;
	invokevirtual	java/io/PrintStream.print(Ljava/lang/String;)V
.line 28
	getstatic	java/lang/System/out Ljava/io/PrintStream;
	invokevirtual	java/io/PrintStream.println()V

	getstatic	reads/_runTimer LRunTimer;
	invokevirtual	RunTimer.printElapsedTime()V

	return

.limit locals 1
.limit stack 8
.end method
