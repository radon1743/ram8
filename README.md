# ram8
Trying to simulate a RAM and ALU in Rust 🦀

input.txt (in /src ) contains instructions which the programs performs.

1) LOD _ => Loads a number into accumulator (acc) of specified address.
2) ADD _ => Performs arithmatic addition of the number in acc and number in specified address.
3) STR _ => Stores the acc number into specified address.
4) JMP _ => Jumps to an intruction (note its range in 0-15).
5) HLT 0 => Halts the program and prints "Program Halted".
6) INC _ => Incremets the number in the specified address.
7) ITP _ => Stores a number to acc.
8) OTP _ => Prints the number in the specified address. 
