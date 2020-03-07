let memory = new ArrayBuffer(1024);

export function memory_ptr(): usize {
    return changetype<usize>(memory);
}

export function loadStringFromMemory(len: usize): string {
    let view = new DataView(memory);
    return String.UTF8.decode(view.buffer, true);
}

export function saveStringIntoMemory(raw: string): usize {
    const len = raw.length;
    let view = new DataView(memory);
    for (let i=0;i<(len as i32);i++) {
        view.setUint8(i, raw.charCodeAt(i) as i8);
    }
    return len;
}