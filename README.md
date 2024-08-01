# cue-rs -- use CUE from Rust

This is a work in progress, expect constant churn and breakage.

## Setup

### Prerequisites

Building `cue-rs` requires access to a statically-build [libcue](https://github.com/cue-lang/libcue).

### Build

```
cargo build
```

If `libcue` is not installed system wide, specify its location like so:

```
BINDGEN_EXTRA_CLANG_ARGS="-I /path/to/libcue" LIBRARY_PATH=/path/to/libcue cargo build

```

### Test

```
cargo test
```

Similarly, pass `BINDGEN_EXTRA_CLANG_ARGS` and `LIBRARY_PATH` if needed.

## Using

TODO.

## Issue tracking

Please raise all issues in
[the main CUE repository](https://github.com/cue-lang/cue/issues),
giving the title of the issue a `cue-rs: ` prefix.
