# libnnpack

This is a Rust binding to NNPACK (https://github.com/Maratyszcza/NNPACK).

Most of the NNPACK functionality is provided as FFI bindings, but convenience
interfaces are provided for lazy context creation (`NnpackHandle`) and thread
pool reuse (`NnpackPthreadPool`).

## Requirements

The build script should compile NNPACK from sources. Building assumes that the
following commands are available on the system:

- `virtualenv`
- `python2.7`
- `gcc-4.9` and `g++-4.9`
