# Independent grant-policy verification

Unison executes the one-period grant and separate-donation-ledger specification
in `distribution.u`. This checks policy caps independently of the Rust engine.
It does not establish legal suitability, investment performance or sustainability.

```ucm
scratch/main> builtins.merge
```

```ucm
scratch/main> load policy/distribution.u
```

```ucm
scratch/main> add
```

```ucm
scratch/main> test
```
