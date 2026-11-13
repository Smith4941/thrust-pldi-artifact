# RQ3 input families

Generic functions with other bodies than the RQ3 family of
[../scalability-evaluation/](../scalability-evaluation/README.md), each verified once
(`generic`) and per instance (`per_instance`, `THRUST_PER_INSTANCE_GENERICS=1`) at k instances, for
the supplementary Figure `fig:scalability-families`.

## Families

Every family except `satint` uses a trait `Step` (`step` requires `inv(x)`, ensures `inv(result) &&
result > x`). Type `S_j` adds `j mod 3 + 1` and has `inv(x)` = `x >= j`, unless the row says
otherwise. Every program has a fail twin.

| family | generic function | types |
| --- | --- | --- |
| `base` | one loop, invariant inferred (the RQ3 `inferred` variant) | `S_j` |
| `base_parity` | the same | `inv(x)` = `x >= j && (x - j) % 2 == 0`, step adds 2 |
| `twovarw` | two values stepped in one loop, invariants written | `S_j` |
| `twovar`, `nestloop`, `vecfill` | two values in one loop / two nested loops / values pushed into a `Vec` with a `forall` in `ensures`, invariants inferred | `S_j` |
| `long<n>` | no loop: n calls of `step`, each followed by an `assert!` | `S_j` |
| `satint<n>` | no loop: n saturating additions through `SatInt::sat_add` | bounded integers `I8, U8, I16, U16, I32, U31, Pct, Deg` (`Pct` 0 to 100, `Deg` 0 to 359) |

## Files

- `programs/<family>_k<k>_{pass,fail}.rs`: the programs, written by the generator of
  thrust-research 7767cd1 and inputs now.
- `runs.csv`: the inputs of `../scripts/run.py`: the pass side of every program in both settings,
  one run each with a 60 s solver timeout.

`../scripts/tables.py` draws the figure from these rows: pass-side wall time of the Thrust command
(decisions.md D123) against k per
family, the setting's open marker at 60 s where the solver timed out or ran out of memory; it stops on any other verdict.
