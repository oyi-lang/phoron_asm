; swap the top two items on the stack

.class public SwapTopTwoItems
.super java/lang/Object

.method public <init>()V
  aload_0
  invokespecial java/lang/Object/<init>()V
  return
.end method

.method public static main([Ljava/lang/String;)V
  .limit stack 3
  .limit locals 1

  getstatic java/lang/System/out Ljava/io/PrintStream;
  iconst_1 ; push 1 onto the stack
  iconst_2 ; push 2 onto the stack
  swap ; swap the 1 and 2
  pop ; drop 1
  invokevirtual java/io/PrintStream/println(I)V ; print 2, pops off 2 as well as `out`
  return
.end method