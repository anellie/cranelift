# regalloc no_std

`regalloc` modified to be `no_std` compatible.
Not in a separate repo because it seemed overkill (and I'm somewhat lazy).

Thanks to the Cranelift team!

**Original README below**


regalloc.rs
===

A work-in-progress modular register allocation algorithm, implemented so as to
be used in [Cranelift](https://github.com/bytecodealliance/wasmtime/tree/main/cranelift).

WARNING: This is not production ready, you should expect API changes, failures,
etc.
