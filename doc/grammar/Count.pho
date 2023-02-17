.class public Count
.super java/lang/Object

.method public <init>()V
  aload_0
  invokespecial java/lang/Object/<init>()V
  return
.end method

.method public static main([Ljava/lang/String;)V
  .limit stack 3
  .limit locals 4

  getstatic java/lang/System/out Ljava/io/PrintStream;
  astore_1

  bipush 10
  istore_2

Loop:
  bipush 10
  iload_2
  isub
  invokestatic java/lang/String/valueOf(I)Ljava/lang/String;
  astore_3

  aload_1
  aload_3
  invokevirtual java/io/PrintStream/println(Ljava/lang/String;)V

  iinc 2 -1
  iload_2
  ifne Loop

  return
.end method