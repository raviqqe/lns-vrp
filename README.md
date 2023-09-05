# vrp

Vehicle Routing Problem (VRP) solver

A number of total cases is `(s + v - 1)! / (v - 1)!` where `s` is a number of stops and `v` is a number of vehicles.

## Solvers

### Exact

Note that those are currently equivalent to a brute force one unless you provide a cost calculator that may return infinity or computes proper lower bounds.

- Dynamic programming
- [Branch and bound](https://en.wikipedia.org/wiki/Branch_and_bound)

### Approximate

- Nearest neighbor
- [Ruin and recreate][ruin-and-recreate]
  - Ruined regions are the top-k closest stops and their surrounding stops in routes in an existing solution.
  - Sub-problems are solved by brute force.

## Examples

```sh
cargo run --release --features trace --bin ruin_and_recreate
```

## References

- [Heuristics for Vehicle Routing Problem: A Survey and Recent Advances](https://arxiv.org/abs/2303.04147)
- [Record Breaking Optimization Results Using the Ruin and Recreate Principle][ruin-and-recreate]
- [`graphhopper/jsprit`](https://github.com/graphhopper/jsprit)
- [`reinterpretcat/vrp`](https://github.com/reinterpretcat/vrp)

## License

[The Unlicense](UNLICENSE)

[ruin-and-recreate]: https://www.semanticscholar.org/paper/Record-Breaking-Optimization-Results-Using-the-Ruin-Schrimpf-Schneider/4f80e70e51e368858c3df0787f05c3aa2b9650b4
