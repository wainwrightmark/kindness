# kindness

![GITHUB](https://img.shields.io/github/last-commit/wainwrightmark/kindness)
![Crates.io](https://img.shields.io/crates/v/kindness)
![GitHub Workflow Status](https://img.shields.io/github/actions/workflow/status/wainwrightmark/kindness/build.yml)
![docs](https://img.shields.io/docsrs/kindness)

Methods for choosing random elements from an iterator.

Includes `choose_item()`, `choose_max()`, `choose_max_by()`, `choose_max_by_key()`, `choose_min()`, `choose_min_by()`, `choose_min_by_key()`

`no_std` by default. The `std` feature unlocks the `choose_unique()` and `choose_unique_by_key()` methods.


If you are confused by the name of the crate, think "random max of".

---

This crate works with Cargo with a `Cargo.toml` like:

```toml
[dependencies]
kindness = "0.4.0"
rand = "0.8.5"
```

## Getting started

```rust
use kindness::*;
use rand::SeedableRng;

fn main() {
    let mut rng = rand::rngs::StdRng::seed_from_u64(123);
    let m =[3,2,1,2,3].iter().choose_max(&mut rng).unwrap();
    assert_eq!(*m, 3)
}
```

## Contributing

Contributions are welcome! Open a pull request to fix a bug, or [open an issue][]
to discuss a new feature or change.

Check out the [Contributing][] section in the docs for more info.

[contributing]: CONTRIBUTING.md
[open an issue]: https://github.com/wainwrightmark/kindness/issues

## License

This project is proudly licensed under the MIT license ([LICENSE](LICENSE)
or http://opensource.org/licenses/MIT).

`kindness` can be distributed according to the MIT license. Contributions
will be accepted under the same license.

## Authors

- [Mark Wainwright](https://github.com/wainwrightmark)
