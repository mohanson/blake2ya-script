# Blake2 Yet Another Script Test

**Benchmark**

To perform this test, you first need to install [ckb-debugger](https://github.com/nervosnetwork/ckb-standalone-debugger).

```sh
$ cargo run --release

# Script log: bench blake2b_ref
# Script log: 90682
# Script log: bench blake2ya
# Script log: 68534
```

25% faster than [blake2b_ref](https://github.com/jjyr/blake2b-ref.rs).
