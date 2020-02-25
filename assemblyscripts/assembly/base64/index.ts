export { wasiabort } from '../env';
export { getMemoryPtr } from '../memory';

import { JSON } from '../../node_modules/assemblyscript-json/assembly';
import { Base64 } from './base64';
import { loadStringFromMemory, saveStringIntoMemory } from '../memory';

export function _outgoing(len: usize): usize {
    const rawJson = loadStringFromMemory(len);
    const data: JSON.Obj = changetype<JSON.Obj>(JSON.parse(rawJson));
    const rawText = changetype<JSON.Str>(data.get("text"))._str;
    const keyword = changetype<JSON.Str>(data.get("keyword"))._str;
    const username = changetype<JSON.Str>(data.get("username"))._str;
    const text = rawText.substring(keyword.length + 1);
    const result = ['@', username, ': ', 'Base64.encode("', text, '") = `', Base64.encode(text), '`'];
    return saveStringIntoMemory(result.join(''));
}
