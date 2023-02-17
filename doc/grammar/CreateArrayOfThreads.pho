; create an array of java.lang.Thread

.class public CreateArrayOfThreads
.super java/lang/Object

.method public <init>()V
  aload_0
  invokespecial java/lang/Object/<init>()V
  return
.end method

.method public static main([Ljava/lang/String;)V
  .limit stack 3
  .limit locals 2

  bipush 10
  anewarray java/lang/Thread
  astore_1

  getstatic java/lang/System/out Ljava/io/PrintStream;
  aload_1
  instanceof [Ljava/lang/Thread;
  invokevirtual java/io/PrintStream/println(I)V ; should print 1
  return
.end method
