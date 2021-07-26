def to_u16(x)
  [
    (x & 0xff),
    ((x >> 8) & 0xff),
  ]
end

REGISTERS = { A: 0, B: 1, C: 2, D: 3, E: 4, F: 5, G: 6, H: 7 }

def reg(input)
  REGISTERS.fetch(input)
end

@program ||= []
@instruction_index = 0

def _MARK
  @instruction_index
end

def _DRW
  @program += [0x01]
  @instruction_index += 1
end

def _MOV
  @program += [0x02]
  @instruction_index += 1
end

def _STO_REG(register, value)
  @program += [0x03, reg(register)] + to_u16(value)
  @instruction_index += 1
end

def _INC_REG(register)
  @program += [0x04, reg(register)]
  @instruction_index += 1
end

def _INC_REG_BY(register, amount)
  @program += [0x05, reg(register)] + to_u16(amount)
  @instruction_index += 1
end

def _DEC_REG(register)
  @program += [0x06, reg(register)]
  @instruction_index += 1
end

def _JMP_NZ(register, addr)
  @program += [0x07, reg(register)] + to_u16(addr)
  @instruction_index += 1
end

def _MUL(input, output, value)
  @program += [0x09, reg(input), reg(output)] + to_u16(value)
  @instruction_index += 1
end

def _STO_REG_REG(r1, r2)
  @program += [0x0a, reg(r1), reg(r2)]
  @instruction_index += 1
end

def _END
  @program += [0x08]
  File.open("program.bin", "wb") do |f|
    f.write(@program.pack("C*"))
  end
  exit
end
