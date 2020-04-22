# The Yage decoder

This is a lib which aimed to give a convinent way to add instruction with a simple synthax

# macro synthax
`#[declare_instruction(opcode=0xff, regs=[], lenght=2, cycles=8, flags=[])]`


# Usage 
Here is a example of usage:
```Rust
enum instructions {
	#[declare_instruction(opcode=0x00, regs=[], lenght=1, cycles=4, flags=[])]
	NOP
}
```

This sample will generate the following struct

```Rust
pub struct NOP {
	opcode: 0x00,
	regs: [],
	lenght: 1,
	cycles: 4,
	flags: [],
}
```

For now, this is very experimental
