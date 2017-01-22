# Computes the first 25 elements of the Fibonacci sequence
# R3 is reserved for results
# R4 is reserved for acc
# R5 is always for loading and using constants
# R6 is reserved for printing

# 0x0000
LOADC R1, 0
LOADC R2, 1

# print R1
STORE 0x0000, R1
CPRINT R1

# print R2
# 0x0010
STORE 0x0000, R2
CPRINT R2

# Seen 2 of 25 elements
LOADC R4, 2

ADD R1, R2, R3
# R1: FREE
# 0x0020
LOADC R1, 25
EQ R5, R1, R3
# "break"
GOTOIF 0x0048, R5
# print R3
STORE 0x0000, R2
# 0x0030
CPRINT R2
# reset...
ADD R2, R5, R1
ADD R3, R5, R2
# increment
LOADC R5, 1
# 0x0040
ADD R5, R4, R4
# loop
GOTO 0x0014
EXIT
