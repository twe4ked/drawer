WIDTH 1024
HEIGHT 1024

DRW

STO C 1

main_loop:
	; If C > 507, skip to continue:
	JGT C 507 continue:

		; Move forward C times
		STO E C
		move_loop:
			FWD
			DEC E
			NZ E move:

		; Set A (the angle) to C * 91
		TO A C
		MUL A 91

	continue:

	; Main loop increment
       INC C
       JLT C 512 main_loop:

HLT
