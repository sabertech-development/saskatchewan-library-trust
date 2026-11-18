# Independent cash-policy verification

Unison executes the specification in `distribution.u`. This is an independent
check of the Rust engine, not a claim that the proposal is commercially viable.

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
