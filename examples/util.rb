def to_u16(x)
  [
    (x & 0xff),
    ((x >> 8) & 0xff),
  ]
end

def write_program(program)
  File.open("program.bin", "wb") do |f|
    f.write(program.pack("C*"))
  end
end

def draw_instruction
  ["D".ord]
end

def move_instruction(x)
  ["M".ord] + to_u16(x)
end

def angle_instruction(x)
  ["A".ord] + to_u16(x)
end

def halt_instruction
  ["H".ord]
end
