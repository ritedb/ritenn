extern crate ritemlcr;

fn simple() {
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
    cache.touch(2);
    cache.touch(2);
    cache.touch(100);
    cache.touch(2);
    cache.touch(2);
    cache.touch(2);
    cache.touch(100);
    cache.touch(100);
    cache.touch(100);
    cache.touch(1);
    cache.touch(2);
}
