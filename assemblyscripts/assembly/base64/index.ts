export { wasiabort } from '../env';
export { memory_ptr } from '../memory';  // export for wasmer

import { JSON } from '../../node_modules/assemblyscript-json/assembly';
import { Base64 } from './base64';
import { loadStringFromMemory, saveStringIntoMemory } from '../memory';

export function http_post(len: usize): usize {
    const rawJson = loadStringFromMemory(len);
    const data: JSON.Obj = changetype<JSON.Obj>(JSON.parse(rawJson));
    const text = changetype<JSON.Str>(data.get("text"))._str;
    const result = ['Base64.encode("', text, '") = `', Base64.encode(text), '`'];
    return saveStringIntoMemory(result.join(''));
}
