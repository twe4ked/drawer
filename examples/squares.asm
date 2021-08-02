WIDTH 1024
HEIGHT 1024

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

		; Store the main loop count in A then multiply by 91
		STO A C
		MUL A 91

	continue:

	INC C

	; Main loop increment
	DEC B
	JNZ B main_loop:

HLT
