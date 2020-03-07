Lego
====

Lego is a serverless service with [WebAssembly](https://webassembly.org/) and [wasmer](https://wasmer.io).


## Quick Guide

### Prepare wasm

1. Write your wasm in any language you like, in AssemblyScript for example:

```js
export function helloWorld() {
    return "Hello, Lego";
}
```

2. Tell `Lego` where to get/set bytes into memory by defining and exporting `memory_ptr` function:

```js
let memory = new ArrayBuffer(1024);

export function memory_ptr(): usize {
    return changetype<usize>(memory);
}

// Help for set string into memory
function saveStringIntoMemory(data: String): usize {
    const len = raw.length;
    let view = new DataView(memory);
    for (let i=0;i<(len as i32);i++) {
        view.setUint8(i, raw.charCodeAt(i) as i8);
    }
    return len;
}
```

3. Export `http_get` hook to `Lego`:

```
export function http_get(inputLen: usize): usize {
    return saveStringIntoMemory(helloWorld());
}
```

4. Compiles into wasm and store in any path you like:

```sh
$ npm run build
$ cp ./build/helloworld.wasm /path/to/wasm/helloworld.wasm
```

### Run `Lego`

1. Setup environment varialble `LEGO_WASM_ROOT` as the root locatoin of your wasms.

```
export LEGO_WASM_ROOT="/path/to/wasm"
```

2. Here we go ~

```
$ /path/to/lego
```

### Request `Lego`

```
$ curl http://localhost:8000/helloworld | jq    # `helloworld` is the name of your wasm file without extension
{
  "result": "hello world"
}
```

You can find more AssemblyScript examples written in [assemblyscripts](./assemblyscripts/assembly).
