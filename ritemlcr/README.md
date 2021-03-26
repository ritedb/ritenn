# RiteMLCR: MLCR from TFS

> MLCR: Machine-Learning-based Cache Replacement
>
> MLCR trains a neural network to "guess" how long time will pass before the cache block is
> accessed again. In other words, it provides a qualified guess to approximate the ideal Bélády's
> algorithm without a time machine.
> 
> MLCR is slow, because it needs to train a neural network, but in many cases, the added
> precision pays off by greatly reducing the number of cache misses. As such, it should only be
> used when the cached medium is significantly slower than training the network (e.g. hard disks or
> internet downloads).

## Usage

```rust
let mut cache = ritemlcr::Cache::new();

cache.insert(1);
cache.insert(2);
cache.insert(3);
cache.insert(4);
cache.insert(100);
cache.insert(200);

cache.touch(100);
cache.touch(100);
cache.touch(1);
cache.touch(2);
cache.touch(2);
cache.touch(2);
```

## Credit

It is a fork of the crate [mlcr](https://crates.io/crates/mlcr/) , but some adjustments and improvements have been made to the code .

## License

This library is licensed under either of:

* MIT license [LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT
* Apache License 2.0 [LICENSE-APACHE](LICENSE-APACHE) or https://opensource.org/licenses/Apache-2.0

at your option.
