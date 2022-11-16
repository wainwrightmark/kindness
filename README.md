# kindness

[<img alt="github" src="https://img.shields.io/badge/github-wainwrightmark/kindness-8da0cb?style=for-the-badge&labelColor=555555&logo=github" height="22">](https://github.com/wainwrightmark/kindness)
[<img alt="crates.io" src="https://img.shields.io/crates/v/kindness.svg?style=for-the-badge&color=fc8d62&logo=rust" height="22">](https://crates.io/crates/kindness)
[<img alt="docs.rs" src="https://img.shields.io/docsrs/kindness/latest?style=for-the-badge&labelColor=555555&logoColor=white&logo=data:image/svg+xml;base64,PHN2ZyByb2xlPSJpbWciIHhtbG5zPSJodHRwOi8vd3d3LnczLm9yZy8yMDAwL3N2ZyIgdmlld0JveD0iMCAwIDUxMiA1MTIiPjxwYXRoIGZpbGw9IiNmNWY1ZjUiIGQ9Ik00ODguNiAyNTAuMkwzOTIgMjE0VjEwNS41YzAtMTUtOS4zLTI4LjQtMjMuNC0zMy43bC0xMDAtMzcuNWMtOC4xLTMuMS0xNy4xLTMuMS0yNS4zIDBsLTEwMCAzNy41Yy0xNC4xIDUuMy0yMy40IDE4LjctMjMuNCAzMy43VjIxNGwtOTYuNiAzNi4yQzkuMyAyNTUuNSAwIDI2OC45IDAgMjgzLjlWMzk0YzAgMTMuNiA3LjcgMjYuMSAxOS45IDMyLjJsMTAwIDUwYzEwLjEgNS4xIDIyLjEgNS4xIDMyLjIgMGwxMDMuOS01MiAxMDMuOSA1MmMxMC4xIDUuMSAyMi4xIDUuMSAzMi4yIDBsMTAwLTUwYzEyLjItNi4xIDE5LjktMTguNiAxOS45LTMyLjJWMjgzLjljMC0xNS05LjMtMjguNC0yMy40LTMzLjd6TTM1OCAyMTQuOGwtODUgMzEuOXYtNjguMmw4NS0zN3Y3My4zek0xNTQgMTA0LjFsMTAyLTM4LjIgMTAyIDM4LjJ2LjZsLTEwMiA0MS40LTEwMi00MS40di0uNnptODQgMjkxLjFsLTg1IDQyLjV2LTc5LjFsODUtMzguOHY3NS40em0wLTExMmwtMTAyIDQxLjQtMTAyLTQxLjR2LS42bDEwMi0zOC4yIDEwMiAzOC4ydi42em0yNDAgMTEybC04NSA0Mi41di03OS4xbDg1LTM4Ljh2NzUuNHptMC0xMTJsLTEwMiA0MS40LTEwMi00MS40di0uNmwxMDItMzguMiAxMDIgMzguMnYuNnoiPjwvcGF0aD48L3N2Zz4K" height="22">](https://docs.rs/kindness)
[<img alt="build status" src="https://img.shields.io/github/workflow/status/wainwrightmark/kindness/build/main?style=for-the-badge" height="22">](https://github.com/wainwrightmark/kindness/actions?query=branch%3Amain)

Methods for returning random elements from an iterator. 

Includes `random_item()`, `random_max()`, `random_max_by()`, `random_max_by_key()`, `random_min()`, `random_min_by()`, `random_min_by_key()`

Does not allocate or iterate iterators more than once.

---

This crate works with Cargo with a `Cargo.toml` like:

```toml
[dependencies]
kindness = "0.1.0"
rand = "0.8.5"
```

## Getting started

```rust
use kindness::*;
use rand::SeedableRng;

fn main() {    
    let mut rng = rand::rngs::StdRng::seed_from_u64(123);
    let m =[3,2,1,2,3].iter().random_max(&mut rng).unwrap();
    assert_eq!(*m, 3)
}
```

## Contributing

Contributions are welcome! Open a pull request to fix a bug, or [open an issue][]
to discuss a new feature or change.

Check out the [Contributing][] section in the docs for more info.

[Contributing]: CONTRIBUTING.md
[open an issue]: https://github.com/wainwrightmark/kindness/issues

## License

This project is proudly licensed under the MIT license ([LICENSE](LICENSE)
or http://opensource.org/licenses/MIT).

`kindness` can be distributed according to the MIT license. Contributions
will be accepted under the same license.

## Authors

* [Mark Wainwright](https://github.com/wainwrightmark)
