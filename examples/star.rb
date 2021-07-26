require_relative "util"

_DRW

_STO_REG B, 5
  mark_1 = _MARK

  _STO_REG A, 200
    mark_2 = _MARK
    _MOV
    _DEC_REG A
    _JMP_NZ A, mark_2

  _INC_ANG 144
_DEC_REG B
_JMP_NZ B, mark_1

_END
