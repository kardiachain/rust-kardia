# VM
This package contains KVM, KardiaChain Virtual Machine (EVM-liked). It implements:
- Stack-based machine
- Opcodes and instruction set
- Expose `call()` method to execute smart contract code
- ...

This KVM should preserve the functionalities of the existing KVM in `go-kardia` and then simplify as `akula` doing.