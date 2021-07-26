require_relative "util"

_DRW

_STO_REG :B, 5
  mark_1 = _MARK

  _STO_REG :C, 200
    mark_2 = _MARK
    _MOV
    _DEC_REG :C
    _JMP_NZ :C, mark_2

  _INC_REG_BY :A, 144
_DEC_REG :B
_JMP_NZ :B, mark_1

_END
