DRW

STO B 5

mark_1:
  STO C 200
		mark_2:
    MOV
    DEC C
    JNZ C mark_2:

  ADD A 144
DEC B
JNZ B mark_1:

HLT
