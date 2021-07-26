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
E = 4
F = 5
G = 6
H = 7

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

def _MUL(input, output, value)
  @program += ["m".ord, input, output] + to_u16(value)
  @instruction_index += 1
end

def _INC_REG_BY(register, amount)
  @program += ["i".ord, register] + to_u16(amount)
  @instruction_index += 1
end

def _STO_REG(register, value)
  @program += ["S".ord, register] + to_u16(value)
  @instruction_index += 1
end

def _STO_REG_REG(r1, r2)
  @program += ["2".ord, r1, r2]
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
