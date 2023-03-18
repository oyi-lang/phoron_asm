.class public helloonce
.super java/lang/Object

.field private static _runTimer LRunTimer;
.field private static _standardIn LPascalTextIn;


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
	putstatic	helloonce/_runTimer LRunTimer;
	new	PascalTextIn
	dup
	invokenonvirtual	PascalTextIn/<init>()V
	putstatic	helloonce/_standardIn LPascalTextIn;


.line 4
	getstatic	java/lang/System/out Ljava/io/PrintStream;
	ldc	"Hello, world.\n"
	invokevirtual	java/io/PrintStream.print(Ljava/lang/String;)V

	getstatic	helloonce/_runTimer LRunTimer;
	invokevirtual	RunTimer.printElapsedTime()V

	return

.limit locals 1
.limit stack 3
.end method
