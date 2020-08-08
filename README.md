# async-oneshot-local

<!-- [![License](https://img.shields.io/crates/l/async-oneshot-local.svg)](https://github.com/irrustible/async-oneshot-local/blob/main/LICENSE) -->
<!-- [![Package](https://img.shields.io/crates/v/async-oneshot-local.svg)](https://crates.io/crates/async-oneshot-local) -->
<!-- [![Documentation](https://docs.rs/async-oneshot-local/badge.svg)](https://docs.rs/async-oneshot-local) -->

The single-threaded companion to
[async-oneshot](https://github.com/irresponsible/async-oneshot).

Features:

<!-- * Sender may wait for a receiver to be waiting. -->
* Fast and small, with easy to understand code.
* Only one dependency, which is also my library.
* Complete `no_std` support (with `alloc` for `Rc`).
* No `unsafe`!

## Usage

```rust
#[test]
fn success_one_thread() {
    let (s,r) = oneshot::<bool>();
    assert_eq!((), s.send(true).unwrap());
    assert_eq!(Ok(true), future::block_on(r));
}
```

## Performance

Crap numbers from my shitty 2015 macbook pro:

```
test create           ... bench:          91 ns/iter (+/- 11)
test create_send      ... bench:          88 ns/iter (+/- 14)
test create_send_recv ... bench:         100 ns/iter (+/- 4)
```

## Copyright and License

Copyright (c) 2020 James Laver, async-oneshot-local contributors.

This Source Code Form is subject to the terms of the Mozilla Public
License, v. 2.0. If a copy of the MPL was not distributed with this
file, You can obtain one at http://mozilla.org/MPL/2.0/.
