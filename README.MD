# Atomicity problem on IC

## Explanation

The transaction must either fully happen or not happen at all. It must not complete partially.

For more details: [source code](./src/main.rs)

## How to test

Deploy a local canister

```bash
$ make local
```

Perform update calls
```bash
$ for i in {0..99};
  do dfx canister call block_on update&
done
```

Verify
```bash
$ dfx canister call block_on query
```
