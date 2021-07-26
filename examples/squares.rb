require_relative "util"

_DRW

    _MOV
  _LOOP -1, 100

  _INC_REG_BY A, 91
_LOOP -8, 50

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
