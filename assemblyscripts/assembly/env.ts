import { Console } from '../node_modules/as-wasi/assembly';

@global
export function wasiabort(
    message: string = "",
    fileName: string = "",
    lineNumber: u32 = 0,
    colNumber: u32 = 0
): void {
    Console.error(message);
}