export { wasiabort } from './env';

import { Base64 } from './base64';

let memory = new ArrayBuffer(1024);
export function getMemoryPtr(): usize {
    return changetype<usize>(memory);
}

function loadStringFromMemory(len: usize): string {
    let view = new DataView(memory);
    return String.UTF8.decode(view.buffer, true);
}

function saveStringIntoMemory(raw: string): usize {
    const len = raw.length;
    let view = new DataView(memory);
    for (let i=0;i<(len as i32);i++) {
        view.setUint8(i, raw.charCodeAt(i) as i8);
    }
    return len;
}

export function b64encode(len: usize): usize {
    const inputString = loadStringFromMemory(len);
    const encoded = Base64.encode(inputString);
    return saveStringIntoMemory(encoded);
}

export function b64decode(len: usize): usize {
    const inputString = loadStringFromMemory(len);
    const encoded = Base64.decode(inputString);
    return saveStringIntoMemory(encoded);
}