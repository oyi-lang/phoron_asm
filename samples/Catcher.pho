.class public Catcher
.super java/lang/Object

.method public <init>()V
  aload_0
  invokespecial java/lang/Object/<init>()V
  return
.end method

.method public static main([Ljava/lang/String;)V
  .limit stack 3
  .limit locals 3
  .catch java/lang/Exception from Label1 to Label2 using Handler

Label1:
  new java/lang/Exception
  dup
  invokespecial java/lang/Exception/<init>()V
  athrow
Label2:
Handler:
  pop
  getstatic java/lang/System/out Ljava/io/PrintStream;
  ldc "Exception Caught"
  invokevirtual java/io/PrintStream/println(Ljava/lang/String;)V
  return
.end method