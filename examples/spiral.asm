WIDTH 1024
HEIGHT 1024

DRW

STO E 100
STO D 1

# Start a loop
mark_1:

  # Store the current value D (counting up) in C
  STO C D

  # Move forward C number of times
  mark_2:
    DEC C
    FWD
    JNZ C mark_2:

  # Set the A register to by D * 15
  STO A D
  MUL A 15

  # Increment our loop
  INC D
  DEC E

JNZ E mark_1:

HLT
