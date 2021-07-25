require_relative "util"

program = []

program += draw_instruction

(1..100).each do |i|
  program += move_instruction(i)
  program += angle_instruction(15 * i)
end

program += halt_instruction

write_program(program)
