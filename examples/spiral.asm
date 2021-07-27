DRW

STO E 100
STO D 1

# Start a loop
mark_1:

  # Store the current value D (counting up) in C
  STO C D

  # Move C number of times
  mark_2:
    DEC C
    MOV
    JNZ C mark_2:

  # Multiply the current value
  MUL B D 15
  # And then increment the ..
  STO A B

  # Increment our loop
  INC D
  DEC E

JNZ E mark_1:

HLT
