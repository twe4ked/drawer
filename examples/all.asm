; NOTE: All of these instructions won't run because of the jumping.

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

label:

; JNZ Rx label:
JNZ A label:
JNZ S label:

; JEQ Rx n label:
; JEQ Rx Ry label:
JEQ A 1 label:
JEQ S A label:

; JNE Rx n label:
; JNE Rx Ry label:
JNE A 1 label:
JNE S A label:

; JGT Rx n label:
; JGT Rx Ry label:
JGT A 1 label:
JGT S A label:

; JLT Rx n label:
; JLT Rx Ry label:
JLT A 1 label:
JLT S A label:

HLT
