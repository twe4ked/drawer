def to_u16(x)
  [
    (x & 0xff),
    ((x >> 8) & 0xff),
  ]
end

A = 0
B = 1
C = 2
D = 3

@program ||= []
@instruction_index = 0

def _MARK
  @instruction_index
end

def _DRW
  @program += ["D".ord]
  @instruction_index += 1
end

def _MOV
  @program += ["M".ord]
  @instruction_index += 1
end

def _ANG(angle)
  @program += ["a".ord] + to_u16(angle)
  @instruction_index += 1
end

def _INC_ANG(angle)
  @program += ["A".ord] + to_u16(angle)
  @instruction_index += 1
end

def _LOOP(relative_addr, times)
  addr = @program.length - 1 + relative_addr
  @program += ["L".ord] + to_u16(addr) + to_u16(times)
  @instruction_index += 1
end

def _STO_REG(register, value)
  @program += ["S".ord, register] + to_u16(value)
  @instruction_index += 1
end

def _DEC_REG(register)
  @program += ["d".ord, register]
  @instruction_index += 1
end

def _INC_REG(register)
  @program += ["I".ord, register]
  @instruction_index += 1
end

def _JMP_NZ(register, addr)
  @program += ["J".ord, register] + to_u16(addr)
  @instruction_index += 1
end

def _END
  @program += ["H".ord]
  File.open("program.bin", "wb") do |f|
    f.write(@program.pack("C*"))
  end
  exit
end
