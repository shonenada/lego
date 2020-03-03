#![feature(test)]

extern crate test;

use wasmer_runtime::{instantiate, imports, Func};

static WASM: &'static [u8] = include_bytes!("../src/wasm/add.wasm");

#[cfg(test)]
mod tests {
    use super::*;
    use test::Bencher;

    #[bench]
    fn bench_instance(b: &mut Bencher) {
        b.iter(|| {
            let import_objects = imports! {};
            instantiate(WASM, &import_objects).unwrap();
        });
    }

    #[bench]
    fn bench_get_add(b: &mut Bencher) {
        let import_objects = imports! {};
        let instance = instantiate(WASM, &import_objects).unwrap();
        b.iter(|| {
            let _add: Func<(i32, i32), i32> = instance.func("add").unwrap();
        });
    }

    #[bench]
    fn bench_add(b: &mut Bencher) {
        let import_objects = imports! {};
        let instance = instantiate(WASM, &import_objects).unwrap();
        let add: Func<(i32, i32), i32> = instance.func("add").unwrap();
        b.iter(|| {
            let result = add.call(5, 5).unwrap();
            assert_eq!(result, 10);
        });
    }
}
