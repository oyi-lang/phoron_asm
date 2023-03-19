.class public hilbert
.super java/lang/Object

.field private static _runTimer LRunTimer;
.field private static _standardIn LPascalTextIn;

.field private static b [F
.field private static h [[F
.field private static i I
.field private static j I
.field private static n I
.field private static ps [I
.field private static x [F

.method public <init>()V

	aload_0
	invokenonvirtual	java/lang/Object/<init>()V
	return

.limit locals 1
.limit stack 1
.end method

.method private static singular(I)V

.var 0 is why I


.line 25
	iload_0

	lookupswitch
	  0: L002
	  1: L003
	  2: L004
	  default: L001

L002:
.line 26
	getstatic	java/lang/System/out Ljava/io/PrintStream;
	ldc	"Matrix with zero row in decompose.\n"
	invokevirtual	java/io/PrintStream.print(Ljava/lang/String;)V
	goto	L001
L003:
.line 27
	getstatic	java/lang/System/out Ljava/io/PrintStream;
	ldc	"Singular matrix in decompose.\n"
	invokevirtual	java/io/PrintStream.print(Ljava/lang/String;)V
	goto	L001
L004:
.line 28
	getstatic	java/lang/System/out Ljava/io/PrintStream;
	ldc	"No convergence in improve.\n"
	invokevirtual	java/io/PrintStream.print(Ljava/lang/String;)V
	goto	L001
L001:


	return

.limit locals 1
.limit stack 2
.end method

.method private static decompose(I[[F[[F)V

.var 1 is a [[F
.var 11 is biggest F
.var 4 is i I
.var 5 is j I
.var 6 is k I
.var 2 is lu [[F
.var 12 is mult F
.var 0 is n I
.var 8 is normrow F
.var 9 is pivot F
.var 7 is pivotindex I
.var 3 is scales [F
.var 10 is size F


	iconst_5
	newarray	float
	astore_3

.line 47
	iconst_1
	istore	4
.line 47
L005:
	iload	4
	iload_0
	if_icmpgt	L007
	iconst_0
	goto	L008
L007:
	iconst_1
L008:
	ifne	L006
.line 48
	getstatic	hilbert/ps [I
	iload	4
	iconst_1
	isub
	iload	4
	iastore
.line 49
	fconst_0
	fstore	8
.line 51
	iconst_1
	istore	5
.line 51
L009:
	iload	5
	iload_0
	if_icmpgt	L011
	iconst_0
	goto	L012
L011:
	iconst_1
L012:
	ifne	L010
.line 52
	aload_2
	iload	4
	iconst_1
	isub
	aaload
	iload	5
	iconst_1
	isub
	aload_1
	iload	4
	iconst_1
	isub
	aaload
	iload	5
	iconst_1
	isub
	faload
	fastore
.line 55
	fload	8
	aload_2
	iload	4
	iconst_1
	isub
	aaload
	iload	5
	iconst_1
	isub
	faload
	invokestatic	java/lang/Math/abs(F)F
	fcmpg
	iflt	L014
	iconst_0
	goto	L015
L014:
	iconst_1
L015:
	ifeq	L013
.line 55
	aload_2
	iload	4
	iconst_1
	isub
	aaload
	iload	5
	iconst_1
	isub
	faload
	invokestatic	java/lang/Math/abs(F)F
	fstore	8
L013:
.line 51
	iload	5
	iconst_1
	iadd
	istore	5
	goto	L009
L010:
.line 59
	fload	8
	iconst_0
	i2f
	fcmpg
	ifne	L017
	iconst_0
	goto	L018
L017:
	iconst_1
L018:
	ifeq	L019
.line 59
	aload_3
	iload	4
	iconst_1
	isub
	iconst_1
	i2f
	fload	8
	fdiv
	fastore
	goto	L016
L019:
.line 61
	aload_3
	iload	4
	iconst_1
	isub
	fconst_0
	fastore
.line 62
	iconst_0
	invokestatic	hilbert/singular(I)V
L016:
.line 47
	iload	4
	iconst_1
	iadd
	istore	4
	goto	L005
L006:
.line 67
	iconst_1
	istore	6
.line 67
L020:
	iload	6
	iload_0
	iconst_1
	isub
	if_icmpgt	L022
	iconst_0
	goto	L023
L022:
	iconst_1
L023:
	ifne	L021
.line 68
	iconst_0
	istore	7
.line 69
	fconst_0
	fstore	11
.line 72
	iload	6
	istore	4
.line 72
L024:
	iload	4
	iload_0
	if_icmpgt	L026
	iconst_0
	goto	L027
L026:
	iconst_1
L027:
	ifne	L025
.line 75
	aload_2
	getstatic	hilbert/ps [I
	iload	4
	iconst_1
	isub
	iaload
	iconst_1
	isub
	aaload
	iload	6
	iconst_1
	isub
	faload
	invokestatic	java/lang/Math/abs(F)F
	aload_3
	getstatic	hilbert/ps [I
	iload	4
	iconst_1
	isub
	iaload
	iconst_1
	isub
	faload
	fmul
	fstore	10
.line 77
	fload	11
	fload	10
	fcmpg
	iflt	L029
	iconst_0
	goto	L030
L029:
	iconst_1
L030:
	ifeq	L028
.line 78
	fload	10
	fstore	11
.line 79
	iload	4
	istore	7
L028:
.line 72
	iload	4
	iconst_1
	iadd
	istore	4
	goto	L024
L025:
.line 83
	fload	11
	iconst_0
	i2f
	fcmpg
	ifeq	L032
	iconst_0
	goto	L033
L032:
	iconst_1
L033:
	ifeq	L034
.line 83
	iconst_1
	invokestatic	hilbert/singular(I)V
	goto	L031
L034:
.line 85
	iload	7
	iload	6
	if_icmpne	L036
	iconst_0
	goto	L037
L036:
	iconst_1
L037:
	ifeq	L035
.line 88
	getstatic	hilbert/ps [I
	iload	6
	iconst_1
	isub
	iaload
	istore	5
.line 89
	getstatic	hilbert/ps [I
	iload	6
	iconst_1
	isub
	getstatic	hilbert/ps [I
	iload	7
	iconst_1
	isub
	iaload
	iastore
.line 90
	getstatic	hilbert/ps [I
	iload	7
	iconst_1
	isub
	iload	5
	iastore
L035:
.line 93
	aload_2
	getstatic	hilbert/ps [I
	iload	6
	iconst_1
	isub
	iaload
	iconst_1
	isub
	aaload
	iload	6
	iconst_1
	isub
	faload
	fstore	9
.line 96
	iload	6
	iconst_1
	iadd
	istore	4
.line 96
L038:
	iload	4
	iload_0
	if_icmpgt	L040
	iconst_0
	goto	L041
L040:
	iconst_1
L041:
	ifne	L039
.line 97
	aload_2
	getstatic	hilbert/ps [I
	iload	4
	iconst_1
	isub
	iaload
	iconst_1
	isub
	aaload
	iload	6
	iconst_1
	isub
	faload
	fload	9
	fdiv
	fstore	12
.line 98
	aload_2
	getstatic	hilbert/ps [I
	iload	4
	iconst_1
	isub
	iaload
	iconst_1
	isub
	aaload
	iload	6
	iconst_1
	isub
	fload	12
	fastore
.line 100
	fload	12
	iconst_0
	i2f
	fcmpg
	ifne	L043
	iconst_0
	goto	L044
L043:
	iconst_1
L044:
	ifeq	L042
.line 103
	iload	6
	iconst_1
	iadd
	istore	5
.line 103
L045:
	iload	5
	iload_0
	if_icmpgt	L047
	iconst_0
	goto	L048
L047:
	iconst_1
L048:
	ifne	L046
.line 104
	aload_2
	getstatic	hilbert/ps [I
	iload	4
	iconst_1
	isub
	iaload
	iconst_1
	isub
	aaload
	iload	5
	iconst_1
	isub
	aload_2
	getstatic	hilbert/ps [I
	iload	4
	iconst_1
	isub
	iaload
	iconst_1
	isub
	aaload
	iload	5
	iconst_1
	isub
	faload
	fload	12
	aload_2
	getstatic	hilbert/ps [I
	iload	6
	iconst_1
	isub
	iaload
	iconst_1
	isub
	aaload
	iload	5
	iconst_1
	isub
	faload
	fmul
	fsub
	fastore
.line 103
	iload	5
	iconst_1
	iadd
	istore	5
	goto	L045
L046:
L042:
.line 96
	iload	4
	iconst_1
	iadd
	istore	4
	goto	L038
L039:
L031:
.line 67
	iload	6
	iconst_1
	iadd
	istore	6
	goto	L020
L021:
.line 113
	aload_2
	getstatic	hilbert/ps [I
	iload_0
	iconst_1
	isub
	iaload
	iconst_1
	isub
	aaload
	iload_0
	iconst_1
	isub
	faload
	iconst_0
	i2f
	fcmpg
	ifeq	L050
	iconst_0
	goto	L051
L050:
	iconst_1
L051:
	ifeq	L049
.line 113
	iconst_1
	invokestatic	hilbert/singular(I)V
L049:

	return

.limit locals 13
.limit stack 8
.end method

.method private static solve(I[[F[F[F)V

.var 2 is b [F
.var 6 is dot F
.var 4 is i I
.var 5 is j I
.var 1 is lu [[F
.var 0 is n I
.var 3 is x [F


.line 127
	iconst_1
	istore	4
.line 127
L052:
	iload	4
	iload_0
	if_icmpgt	L054
	iconst_0
	goto	L055
L054:
	iconst_1
L055:
	ifne	L053
.line 128
	fconst_0
	fstore	6
.line 129
	iconst_1
	istore	5
.line 129
L056:
	iload	5
	iload	4
	iconst_1
	isub
	if_icmpgt	L058
	iconst_0
	goto	L059
L058:
	iconst_1
L059:
	ifne	L057
.line 130
	fload	6
	aload_1
	getstatic	hilbert/ps [I
	iload	4
	iconst_1
	isub
	iaload
	iconst_1
	isub
	aaload
	iload	5
	iconst_1
	isub
	faload
	aload_3
	iload	5
	iconst_1
	isub
	faload
	fmul
	fadd
	fstore	6
.line 129
	iload	5
	iconst_1
	iadd
	istore	5
	goto	L056
L057:
.line 132
	aload_3
	iload	4
	iconst_1
	isub
	aload_2
	getstatic	hilbert/ps [I
	iload	4
	iconst_1
	isub
	iaload
	iconst_1
	isub
	faload
	fload	6
	fsub
	fastore
.line 127
	iload	4
	iconst_1
	iadd
	istore	4
	goto	L052
L053:
.line 136
	iload_0
	istore	4
.line 136
L060:
	iload	4
	iconst_1
	if_icmplt	L062
	iconst_0
	goto	L063
L062:
	iconst_1
L063:
	ifne	L061
.line 137
	fconst_0
	fstore	6
.line 138
	iload	4
	iconst_1
	iadd
	istore	5
.line 138
L064:
	iload	5
	iload_0
	if_icmpgt	L066
	iconst_0
	goto	L067
L066:
	iconst_1
L067:
	ifne	L065
.line 139
	fload	6
	aload_1
	getstatic	hilbert/ps [I
	iload	4
	iconst_1
	isub
	iaload
	iconst_1
	isub
	aaload
	iload	5
	iconst_1
	isub
	faload
	aload_3
	iload	5
	iconst_1
	isub
	faload
	fmul
	fadd
	fstore	6
.line 138
	iload	5
	iconst_1
	iadd
	istore	5
	goto	L064
L065:
.line 141
	aload_3
	iload	4
	iconst_1
	isub
	aload_3
	iload	4
	iconst_1
	isub
	faload
	fload	6
	fsub
	aload_1
	getstatic	hilbert/ps [I
	iload	4
	iconst_1
	isub
	iaload
	iconst_1
	isub
	aaload
	iload	4
	iconst_1
	isub
	faload
	fdiv
	fastore
.line 136
	iload	4
	iconst_1
	isub
	istore	4
	goto	L060
L061:

	return

.limit locals 7
.limit stack 7
.end method

.method private static invert(I[[F[[F)V

.var 1 is a [[F
.var 2 is ainv [[F
.var 4 is b [F
.var 6 is i I
.var 7 is j I
.var 3 is lu [[F
.var 0 is n I
.var 5 is x [F


	iconst_5
	newarray	float
	astore	4

	iconst_5
	iconst_5
	multianewarray	[[F 2
	astore_3

	iconst_5
	newarray	float
	astore	5

.line 156
	iload_0
	aload_1
	aload_3
	invokestatic	hilbert/decompose(I[[F[[F)V
.line 158
	iconst_1
	istore	7
.line 158
L068:
	iload	7
	iload_0
	if_icmpgt	L070
	iconst_0
	goto	L071
L070:
	iconst_1
L071:
	ifne	L069
.line 159
	iconst_1
	istore	6
.line 159
L072:
	iload	6
	iload_0
	if_icmpgt	L074
	iconst_0
	goto	L075
L074:
	iconst_1
L075:
	ifne	L073
.line 160
	iload	6
	iload	7
	if_icmpeq	L077
	iconst_0
	goto	L078
L077:
	iconst_1
L078:
	ifeq	L079
.line 160
	aload	4
	iload	6
	iconst_1
	isub
	fconst_1
	fastore
	goto	L076
L079:
.line 161
	aload	4
	iload	6
	iconst_1
	isub
	fconst_0
	fastore
L076:
.line 159
	iload	6
	iconst_1
	iadd
	istore	6
	goto	L072
L073:
.line 164
	iload_0
	aload_3
	aload	4
	aload	5
	invokestatic	hilbert/solve(I[[F[F[F)V
.line 166
	iconst_1
	istore	6
.line 166
L080:
	iload	6
	iload_0
	if_icmpgt	L082
	iconst_0
	goto	L083
L082:
	iconst_1
L083:
	ifne	L081
.line 166
	aload_2
	iload	6
	iconst_1
	isub
	aaload
	iload	7
	iconst_1
	isub
	aload	5
	iload	6
	iconst_1
	isub
	faload
	fastore
.line 166
	iload	6
	iconst_1
	iadd
	istore	6
	goto	L080
L081:
.line 158
	iload	7
	iconst_1
	iadd
	istore	7
	goto	L068
L069:

	return

.limit locals 8
.limit stack 5
.end method

.method private static printmatrix([[F)V

.var 1 is i I
.var 2 is j I
.var 0 is m [[F


.line 176
	getstatic	java/lang/System/out Ljava/io/PrintStream;
	invokevirtual	java/io/PrintStream.println()V
.line 177
	iconst_1
	istore_1
.line 177
L084:
	iload_1
	getstatic	hilbert/n I
	if_icmpgt	L086
	iconst_0
	goto	L087
L086:
	iconst_1
L087:
	ifne	L085
.line 178
	iconst_1
	istore_2
.line 178
L088:
	iload_2
	getstatic	hilbert/n I
	if_icmpgt	L090
	iconst_0
	goto	L091
L090:
	iconst_1
L091:
	ifne	L089
.line 178
	getstatic	java/lang/System/out Ljava/io/PrintStream;
	ldc	"%15.6f"
	iconst_1
	anewarray	java/lang/Object
	dup
	iconst_0
	aload_0
	iload_1
	iconst_1
	isub
	aaload
	iload_2
	iconst_1
	isub
	faload
	invokestatic	java/lang/Float.valueOf(F)Ljava/lang/Float;
	aastore
	invokestatic	java/lang/String/format(Ljava/lang/String;[Ljava/lang/Object;)Ljava/lang/String;
	invokevirtual	java/io/PrintStream.print(Ljava/lang/String;)V
.line 178
	iload_2
	iconst_1
	iadd
	istore_2
	goto	L088
L089:
.line 179
	getstatic	java/lang/System/out Ljava/io/PrintStream;
	invokevirtual	java/io/PrintStream.println()V
.line 177
	iload_1
	iconst_1
	iadd
	istore_1
	goto	L084
L085:
.line 181
	getstatic	java/lang/System/out Ljava/io/PrintStream;
	invokevirtual	java/io/PrintStream.println()V

	return

.limit locals 3
.limit stack 9
.end method

.method public static main([Ljava/lang/String;)V

	new	RunTimer
	dup
	invokenonvirtual	RunTimer/<init>()V
	putstatic	hilbert/_runTimer LRunTimer;
	new	PascalTextIn
	dup
	invokenonvirtual	PascalTextIn/<init>()V
	putstatic	hilbert/_standardIn LPascalTextIn;


	iconst_5
	newarray	float
	putstatic	hilbert/b [F

	iconst_5
	iconst_5
	multianewarray	[[F 2
	putstatic	hilbert/h [[F

	iconst_5
	newarray	int
	putstatic	hilbert/ps [I

	iconst_5
	newarray	float
	putstatic	hilbert/x [F

.line 185
	getstatic	java/lang/System/out Ljava/io/PrintStream;
	invokevirtual	java/io/PrintStream.println()V
.line 186
	getstatic	java/lang/System/out Ljava/io/PrintStream;
	ldc	"Rank of Hilbert matrix (1-%1d)? "
	iconst_1
	anewarray	java/lang/Object
	dup
	iconst_0
	iconst_5
	invokestatic	java/lang/Integer.valueOf(I)Ljava/lang/Integer;
	aastore
	invokestatic	java/lang/String/format(Ljava/lang/String;[Ljava/lang/Object;)Ljava/lang/String;
	invokevirtual	java/io/PrintStream.print(Ljava/lang/String;)V
.line 187
	getstatic	hilbert/_standardIn LPascalTextIn;
	invokevirtual	PascalTextIn.readInteger()I
	putstatic	hilbert/n I
.line 190
	iconst_1
	putstatic	hilbert/i I
.line 190
L092:
	getstatic	hilbert/i I
	getstatic	hilbert/n I
	if_icmpgt	L094
	iconst_0
	goto	L095
L094:
	iconst_1
L095:
	ifne	L093
.line 191
	iconst_1
	putstatic	hilbert/j I
.line 191
L096:
	getstatic	hilbert/j I
	getstatic	hilbert/n I
	if_icmpgt	L098
	iconst_0
	goto	L099
L098:
	iconst_1
L099:
	ifne	L097
.line 192
	getstatic	hilbert/h [[F
	getstatic	hilbert/i I
	iconst_1
	isub
	aaload
	getstatic	hilbert/j I
	iconst_1
	isub
	iconst_1
	i2f
	getstatic	hilbert/i I
	getstatic	hilbert/j I
	iadd
	iconst_1
	isub
	i2f
	fdiv
	fastore
.line 191
	getstatic	hilbert/j I
	iconst_1
	iadd
	putstatic	hilbert/j I
	goto	L096
L097:
.line 190
	getstatic	hilbert/i I
	iconst_1
	iadd
	putstatic	hilbert/i I
	goto	L092
L093:
.line 196
	getstatic	java/lang/System/out Ljava/io/PrintStream;
	invokevirtual	java/io/PrintStream.println()V
.line 197
	getstatic	java/lang/System/out Ljava/io/PrintStream;
	ldc	"Hilbert matrix:\n"
	invokevirtual	java/io/PrintStream.print(Ljava/lang/String;)V
.line 198
	getstatic	hilbert/h [[F
	invokestatic	hilbert/printmatrix([[F)V
.line 201
	getstatic	hilbert/n I
	getstatic	hilbert/h [[F
	getstatic	hilbert/h [[F
	invokestatic	hilbert/invert(I[[F[[F)V
.line 203
	getstatic	java/lang/System/out Ljava/io/PrintStream;
	ldc	"Hilbert matrix inverted:\n"
	invokevirtual	java/io/PrintStream.print(Ljava/lang/String;)V
.line 204
	getstatic	hilbert/h [[F
	invokestatic	hilbert/printmatrix([[F)V
.line 207
	getstatic	hilbert/n I
	getstatic	hilbert/h [[F
	getstatic	hilbert/h [[F
	invokestatic	hilbert/invert(I[[F[[F)V
.line 209
	getstatic	java/lang/System/out Ljava/io/PrintStream;
	ldc	"Inverse matrix inverted:\n"
	invokevirtual	java/io/PrintStream.print(Ljava/lang/String;)V
.line 210
	getstatic	hilbert/h [[F
	invokestatic	hilbert/printmatrix([[F)V

	getstatic	hilbert/_runTimer LRunTimer;
	invokevirtual	RunTimer.printElapsedTime()V

	return

.limit locals 1
.limit stack 7
.end method
