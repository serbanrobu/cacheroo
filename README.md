# Cacheroo

A simple in-memory cache library that stores key-value pairs with an optional
time based expiration of keys.

The store can be used within a process that includes multiple threads as it
contains a shared
[`RwLock`](https://doc.rust-lang.org/stable/std/sync/struct.RwLock.html) which
allow any number of readers to acquire the lock as long as a writer is not
holding the lock.

The underlying data structure is a
[`HashMap`](https://doc.rust-lang.org/std/collections/struct.HashMap.html) so
performance characteristics and memory usage are similar. So you get on average
a constant time *O(1)* for insert/remove/lookup operations. In addition to the
inserted value, an optional
[`JoinHandle`](https://docs.rs/tokio/latest/tokio/task/struct.JoinHandle.html)
is also stored to be able to cancel the key expiration task.

## Tradeoffs

- Because the store uses a `HashMap`, the key type must implement the
[`Hash`](https://doc.rust-lang.org/std/hash/trait.Hash.html) trait.
- For expiration, a [tokio](https://tokio.rs/) task is spawned instead of a
thread for less overhead.  So to run the operations for the store you will need
a tokio runtime which can be a disadvantage depending on the purpose for which
the library is used.
- Since a key expiration task is scheduled, the inserted key must be cloned, so
the key type must implement the
[`Clone`](https://doc.rust-lang.org/std/clone/trait.Clone.html) trait.
- To release the lock immediately after it is accessed, the value is cloned, so
the value type must also implement the
[`Clone`](https://doc.rust-lang.org/std/clone/trait.Clone.html) trait.
- For the store to be used from multiple threads, both the key and the value
type must implement `Send + Sync + 'static`.

## Development

In [flake.nix](flake.nix) file, a [nix
shell](https://nixos.wiki/wiki/Development_environment_with_nix-shell) is
defined that includes everything you need for development. You can enter the
shell by running the following command:

```sh
nix develop
```

The default nix shell uses a stable rust compiler, but there is also a *nightly*
nix shell that you might use for your IDE or to run the benchmarks that require
the nightly feature
[`test`](https://doc.rust-lang.org/beta/unstable-book/library-features/test.html):

```sh
nix develop .#nightly --command cargo bench
```

To test the library run the following command in the default nix shell:

```sh
cargo test
```

or for the expensive, ignored tests:

```sh
cargo test -- --ignored
```