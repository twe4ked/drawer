require_relative "util"

_DRW

_STO_REG :B, 50
  mark_1 = _MARK

  _STO_REG :C, 200
    mark_2 = _MARK
    _MOV
    _DEC_REG :C
    _JMP_NZ :C, mark_2

  _INC_REG_BY :A, 91

_DEC_REG :B
_JMP_NZ :B, mark_1

# This is a worse version of the commented out version below

# x = 0
# while x < 512 do
#   if x < 508
#     program += move_instruction(x)
#     program += angle_instruction((x * 91) % 360)
#   end
#
#   x += 1
# end

_END
