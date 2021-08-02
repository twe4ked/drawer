all_examples:
	for f in examples/*.asm; do cargo run --bin=assembler < $$f && QUIT_ON_VM_TERM=1 cargo run < program.bin --release; done;
