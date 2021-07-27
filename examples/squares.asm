DRW

STO B 512
STO C 1

main_loop:
	; If C > 507, skip to continue:
	JGT C 507 continue:

		; Move C times
		STO E C
		move_loop:
			MOV
			DEC E
			JNZ E move_loop:

		; Multiply the main loop count by 91 and store in D, then add D to A
		MUL D C 91
		STO A D

	continue:

	INC C

	; Main loop increment
	DEC B
	JNZ B main_loop:

HLT
