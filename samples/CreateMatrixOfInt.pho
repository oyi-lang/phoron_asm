; createa three dimensional matrix of int.

.class public CreateMatrixOfInt
.super java/lang/Object

.method public <init>()V
  aload_0
  invokespecial java/lang/Object/<init>()V
  return
.end method

.method public static main([Ljava/lang/String;)V
  .limit stack 5
  .limit locals 2

  bipush 2
  bipush 3
  bipush 7
  ; create  2 x 3 x 7 int matrix
  multianewarray [[[I 3 
  astore_1

  getstatic java/lang/System/out Ljava/io/PrintStream;
  aload_1
  instanceof [[[I
  invokevirtual java/io/PrintStream/println(I)V ; should print 1
  return
.end method
