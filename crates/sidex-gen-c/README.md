# Sidex: C Code Generation

This crate is part of [Sidex](https://oss.silitics.com/sidex/). It contains
the C code generation backend and the small portable C runtime used by
generated C bindings.

The backend is intentionally generic and independent of any product or RTOS.
The C runtime is C99 and avoids dynamic allocation.
