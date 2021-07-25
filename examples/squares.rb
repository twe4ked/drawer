require_relative "util"

program = []

program += draw_instruction

x = 0
while x < 512 do
  if x < 508
    program += move_instruction(x)
    program += angle_instruction((x * 91) % 360)
  end

  x += 1
end

program += halt_instruction

write_program(program)
