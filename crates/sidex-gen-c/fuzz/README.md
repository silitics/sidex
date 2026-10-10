# Sidex C JSON Runtime Fuzzing

This directory contains a libFuzzer target for the portable C JSON runtime.

Build it with Clang:

```sh
mkdir -p target/sidex-gen-c-fuzz
clang \
  -std=c99 -g -O1 \
  -fsanitize=fuzzer,address,undefined \
  -I crates/sidex-gen-c/runtime \
  crates/sidex-gen-c/runtime/sidex_types.c \
  crates/sidex-gen-c/runtime/sidex_json_reader.c \
  crates/sidex-gen-c/runtime/sidex_json_value.c \
  crates/sidex-gen-c/runtime/sidex_json_writer.c \
  crates/sidex-gen-c/fuzz/json_runtime_fuzzer.c \
  -o target/sidex-gen-c-fuzz/json_runtime_fuzzer
```

Run it with the seed corpus:

```sh
rm -rf target/sidex-gen-c-fuzz/json_runtime_corpus
mkdir -p target/sidex-gen-c-fuzz/json_runtime_corpus
cp crates/sidex-gen-c/fuzz/corpus/json_runtime/* \
  target/sidex-gen-c-fuzz/json_runtime_corpus/
target/sidex-gen-c-fuzz/json_runtime_fuzzer \
  target/sidex-gen-c-fuzz/json_runtime_corpus \
  -max_len=16384 \
  -rss_limit_mb=512
```

If Clang is not installed locally, the same commands can be run through Nix:

```sh
nix --extra-experimental-features 'nix-command flakes' shell nixpkgs#clang nixpkgs#clang-tools -c clang \
  -std=c99 -g -O1 \
  -fsanitize=fuzzer,address,undefined \
  -I crates/sidex-gen-c/runtime \
  crates/sidex-gen-c/runtime/sidex_types.c \
  crates/sidex-gen-c/runtime/sidex_json_reader.c \
  crates/sidex-gen-c/runtime/sidex_json_value.c \
  crates/sidex-gen-c/runtime/sidex_json_writer.c \
  crates/sidex-gen-c/fuzz/json_runtime_fuzzer.c \
  -o target/sidex-gen-c-fuzz/json_runtime_fuzzer
```

The fuzzer treats malformed JSON, arena exhaustion, and output-buffer exhaustion
as expected non-crashing outcomes. Successful decodes are encoded, parsed again,
and encoded a second time; the two encoded byte strings must match.
