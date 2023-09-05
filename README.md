# vrp

Vehicle Routing Problem (VRP) solver

A number of total cases is `(s + v - 1)! / (v - 1)!` where `s` is a number of stops and `v` is a number of vehicles.

## Solvers

### Exact

Note that those are currently equivalent to a brute force one unless you provide a cost calculator that may return infinity or computes proper lower bounds.

- Dynamic programming
- Branch and bound

### Approximate

- Nearest neighbor
- Ruin and recreate

## References

- [Heuristics for Vehicle Routing Problem: A Survey and Recent Advances](https://arxiv.org/abs/2303.04147)
- [Record Breaking Optimization Results Using the Ruin and Recreate Principle](https://www.semanticscholar.org/paper/Record-Breaking-Optimization-Results-Using-the-Ruin-Schrimpf-Schneider/4f80e70e51e368858c3df0787f05c3aa2b9650b4)
- [`graphhopper/jsprit`](https://github.com/graphhopper/jsprit)
- [`reinterpretcat/vrp`](https://github.com/reinterpretcat/vrp)

## License

[The Unlicense](UNLICENSE)
