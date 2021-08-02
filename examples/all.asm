WIDTH 1024
HEIGHT 1024

DRW

MOV

; STO Rx n
; STO Rx Ry
STO A 1
STO S A

; MUL Rx n
; MUL Rx Ry
MUL A 1
MUL S A

; DIV Rx n
; DIV Rx Ry
DIV A 1
DIV S A

; ADD Rx n
; ADD Rx Ry
ADD A 1
ADD S A

; SUB Rx n
; SUB Rx Ry
SUB A 1
SUB S A

; INC Rx
INC A
INC S

; DEC Rx
DEC A
DEC S

STO A 1
STO S 0

; JNZ Rx label:
JNZ A l1:
JNZ S l2:

l1:
		STO A 0

l2:

STO A 1
STO S 0

l3:
		STO A 0

; JEQ Rx n label:
; JEQ Rx Ry label:
JEQ A 1 l3:
JEQ S A l4:

l4:

STO A 1
STO S 1

l5:
		STO A 0

; JNE Rx n label:
; JNE Rx Ry label:
JNE A 0 l5:
JNE S A l6:

l6:

STO A 1
STO S 1

l7:
		STO A 0

; JGT Rx n label:
; JGT Rx Ry label:
JGT A 0 l7:
JGT S A l8:

l8:

STO A 2
STO S 1

l9:
		STO A 1

; JLT Rx n label:
; JLT Rx Ry label:
JLT A 1 l9:
JLT S A l10:

l10:
		HLT
