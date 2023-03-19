; loop and print numbers from 0 to 9 (inclusive)

.class public CountJasmin2
.super java/lang/Object

.method public <init>()V
  aload_0 ; load this
  invokespecial java/lang/Object/<init>()V ; super()
  return
.end method

.method public static main([Ljava/lang/String;)V
  .limit stack 3
  .limit locals 4 ; variable 0 is `this`

  iconst_0
  istore_1 ; i = 0

  getstatic java/lang/System/out Ljava/io/PrintStream; ; `out`
  astore_2 ; store in var 2

loop:
  aload_2 ; load `out`
  iload_1 ; load i
  invokevirtual java/io/PrintStream/println(I)V ; print i
  iinc 1 1 ; i += 1
  iload 1 ; load i onto stack
  bipush 10 ; load the limit onto stack
  if_icmplt loop ; i < limit ?
  return
.end method