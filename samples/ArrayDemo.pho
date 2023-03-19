; general demonstration of the creation and manipulation of an int array

.class public ArrayDemo
.super java/lang/Object

.method public <init>()V
  aload_0
  invokespecial java/lang/Object/<init>()V
  return
.end method

.method private static setArr([III)V
  .limit stack 4
  .limit locals 4

  aload_0 
  iload_1
  iload_2
  iastore
  return
.end method

.method private static printArr([II)V
  .limit stack 4
  .limit locals 2

  getstatic java/lang/System/out Ljava/io/PrintStream;
  aload_0
  iload_1
  iaload
  invokevirtual java/io/PrintStream/println(I)V
  return
.end method

.method public static main([Ljava/lang/String;)V
  .limit stack 3
  .limit locals 2

  ; create an int array of size 5 in local 1
  bipush 5
  newarray int
  astore_1

  ; for primitives, the arrays are zeroed, test by printing the last element
  getstatic java/lang/System/out Ljava/io/PrintStream;
  aload_1
  bipush 4
  iaload 
  invokevirtual java/io/PrintStream/println(I)V ; should print 0

  ; set elements to the value of their respective index + 1
  aload_1
  bipush 0
  bipush 1
  invokestatic ArrayDemo/setArr([III)V

  aload_1
  bipush 1
  bipush 2
  iastore ; a[1] = 2

  aload_1
  bipush 2
  bipush 3
  iastore ; a[2] = 3

  aload_1
  bipush 3
  bipush 4
  iastore ; a[3] = 4

  aload_1
  bipush 4
  bipush 5
  iastore ; a[4] = 5

  ; now print the elements of the array
  aload_1
  bipush 0
  invokestatic ArrayDemo/printArr([II)V

  getstatic java/lang/System/out Ljava/io/PrintStream;
  aload_1
  bipush 1
  iaload
  invokevirtual java/io/PrintStream/println(I)V

  getstatic java/lang/System/out Ljava/io/PrintStream;
  aload_1
  bipush 2
  iaload
  invokevirtual java/io/PrintStream/println(I)V

  getstatic java/lang/System/out Ljava/io/PrintStream;
  aload_1
  bipush 3
  iaload
  invokevirtual java/io/PrintStream/println(I)V

  getstatic java/lang/System/out Ljava/io/PrintStream;
  aload_1
  bipush 4
  iaload
  invokevirtual java/io/PrintStream/println(I)V
  return
.end method
