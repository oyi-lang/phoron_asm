.class public hellomany
.super java/lang/Object

.field private static _runTimer LRunTimer;
.field private static _standardIn LPascalTextIn;

.field private static count I

.method public <init>()V

	aload_0
	invokenonvirtual	java/lang/Object/<init>()V
	return

.limit locals 1
.limit stack 1
.end method

.method private static sayhello(I)V

.var 0 is howmanytimes I
.var 1 is i I


.line 12
	iconst_1
	istore_1
.line 12
L001:
	iload_1
	iload_0
	if_icmpgt	L003
	iconst_0
	goto	L004
L003:
	iconst_1
L004:
	ifne	L002
.line 13
	getstatic	java/lang/System/out Ljava/io/PrintStream;
	ldc	"%3d: Hello, world.\n"
	iconst_1
	anewarray	java/lang/Object
	dup
	iconst_0
	iload_1
	invokestatic	java/lang/Integer.valueOf(I)Ljava/lang/Integer;
	aastore
	invokestatic	java/lang/String/format(Ljava/lang/String;[Ljava/lang/Object;)Ljava/lang/String;
	invokevirtual	java/io/PrintStream.print(Ljava/lang/String;)V
.line 12
	iload_1
	iconst_1
	iadd
	istore_1
	goto	L001
L002:

	return

.limit locals 2
.limit stack 7
.end method

.method public static main([Ljava/lang/String;)V

	new	RunTimer
	dup
	invokenonvirtual	RunTimer/<init>()V
	putstatic	hellomany/_runTimer LRunTimer;
	new	PascalTextIn
	dup
	invokenonvirtual	PascalTextIn/<init>()V
	putstatic	hellomany/_standardIn LPascalTextIn;


.line 18
	getstatic	java/lang/System/out Ljava/io/PrintStream;
	ldc	"How many times? "
	invokevirtual	java/io/PrintStream.print(Ljava/lang/String;)V
.line 19
	getstatic	hellomany/_standardIn LPascalTextIn;
	invokevirtual	PascalTextIn.readInteger()I
	putstatic	hellomany/count I
.line 20
	getstatic	java/lang/System/out Ljava/io/PrintStream;
	invokevirtual	java/io/PrintStream.println()V
.line 22
	getstatic	hellomany/count I
	invokestatic	hellomany/sayhello(I)V

	getstatic	hellomany/_runTimer LRunTimer;
	invokevirtual	RunTimer.printElapsedTime()V

	return

.limit locals 1
.limit stack 3
.end method
