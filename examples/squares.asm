DRW

STO B 50
  mark_1:

  STO C 200
    mark_2:
    MOV
    DEC C
    JNZ C mark_2:

  ADD A 91

DEC B
JNZ B mark_1:

HLT

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
