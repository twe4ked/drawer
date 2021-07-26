require_relative "util"

_DRW

_STO_REG E, 100
_STO_REG D, 1

# Start a loop
mark_1 = _MARK

  # Store the current value D (counting up) in C
  _STO_REG_REG C, D

  # Move C number of times
  mark_2 = _MARK
    _DEC_REG C
    _MOV
    _JMP_NZ C, mark_2

  # Multiply the current value
  _MUL D, B, 15
  # And then increment the ..
  _STO_REG_REG A, B

  # Increment our loop
  _INC_REG D
  _DEC_REG E

_JMP_NZ E, mark_1

_END
