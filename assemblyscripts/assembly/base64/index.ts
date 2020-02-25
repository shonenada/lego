export { wasiabort } from '../env';
export { getMemoryPtr } from '../memory';

import { JSON } from '../../node_modules/assemblyscript-json/assembly';
import { Base64 } from './base64';
import { loadStringFromMemory, saveStringIntoMemory } from '../memory';

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

export function _outgoing(len: usize): usize {
    const rawJson = loadStringFromMemory(len);
    const data: JSON.Obj = changetype<JSON.Obj>(JSON.parse(rawJson));
    const text = changetype<JSON.Str>(data.get("text"))._str;
    const username = changetype<JSON.Str>(data.get("username"))._str;
    const result = ['@', username, ': ', 'Base64.encode("', text, '") = `', Base64.encode(text), '`'];
    return saveStringIntoMemory(result.join(''));
}
