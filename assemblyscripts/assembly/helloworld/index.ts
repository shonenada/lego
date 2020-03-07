export { wasiabort } from '../env';
export { memory_ptr } from '../memory';  // export for wasmer

import { loadStringFromMemory, saveStringIntoMemory } from '../memory';

export function http_get(inputLen: usize): usize {
    const result = ['Hello,', 'Lego'];
    return saveStringIntoMemory(result.join(' '));
}
