# LastGitCommit-rs
A simple wrapper arround [`git2-rs`](https://github.com/rust-lang/git2-rs) to easily get info about the last commit. Useful when you want to show the last commit message or the current git hash.

[![CircleCI](https://circleci.com/gh/olback/lgc-rs.svg?style=svg)](https://circleci.com/gh/olback/lgc-rs) [![docs](https://docs.rs/last-git-commit/badge.svg)](https://docs.rs/last-git-commit) [![](https://meritbadge.herokuapp.com/last-git-commit)](https://crates.io/crates/last-git-commit)

### Simple Git Hash Example
```rust
use last_git_commit::LastGitCommit;

let lgc = LastGitCommit::new().build().unwrap();
let long = lgc.id().long();
let short = lgc.id().short();
let range = lgc.id().range(0..3).unwrap();

println!("Long: {}", long); // "c4f94258c12b8905f3d57f879ae1171ce367cd29"
println!("Short: {}", short); // "c4f9425"
println!("Range: {}", range); // "c4f"
```

#### Please see the [documentation](https://docs.rs/last-git-commit) and [examples](https://github.com/olback/lgc-rs/tree/master/examples).
