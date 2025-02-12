# Changelog

This project follows semantic versioning.

Possible header types:

- `Features` for any new features added, or for backwards-compatible
  changes to existing functionality.
- `Bug Fixes` for any bug fixes.
- `Breaking Changes` for any backwards-incompatible changes.#

## v0.5.0 (2024-02-06)

- Bumped rand to 0.9. This has led to a change in reproducibility for `choose_item`. Also, choose item is no longer necessary as `rand` has now incorporated its performance improvements into `choose`.
- Bumped Hashbrown to 0.15

## v0.4.0 (2022-12-12)

- Add `std` feature
- Add `choose-unique()` and `choose_unique_by_key()` behind the std feature
- Performance improvements

## v0.3.0 (2022-11-17)

- Use a different method to perform comparisons, improving performance for long iterators.

## v0.2.0 (2022-11-16)

- Renamed `random_max` etc. to `choose_max`

## v0.1.0 (2022-11-15)

- Initial Release on [crates.io] :tada:

[crates.io]: https://crates.io/crates/kindness
