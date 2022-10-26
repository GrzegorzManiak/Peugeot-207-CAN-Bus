import { Frame } from '../index.d';

export async function sleep(ms: number) {
    return new Promise(resolve => setTimeout(resolve, ms));
}

export function hex_to_bin(hex: Array<string>): Array<Array<boolean>> {
    // EG: ['0x32', '0xA5'] => [00110010, 10100101]
    let bin: Array<Array<boolean>> = [];

    hex.forEach((h) => {
        let x = parseInt(h.replace('0x', ''), 16).toString(2).padStart(8, '0');
        let arr: Array<boolean> = [];

        x.split('').forEach((b) => arr.push(b === '1'));

        bin.push(arr);
    });

    return bin;
}

export function fixed_dlc_len(dlc: number, frame: Frame): Array<string> {
    // EG: 8, ['0x32', '0xA5'] => ['0x32', '0xA5', '0x00', '0x00', '0x00', '0x00', '0x00', '0x00']
    // EG: 1, ['0x32', '0xA5'] => ['0x32']
    let data = frame.data;

    if (data.length < dlc) {
        for (let i = 0; i < dlc - data.length; i++) {
            data.push('0x00');
        }
    }

    return data;
}

export function bool_arr_to_int(arr: Array<Array<boolean>>): number {
    // EG: [00110010, 10100101] => 3237
    let x = '';

    arr.forEach((a) => {
        a.forEach((b) => x += b ? '1' : '0');
    });

    return parseInt(x, 2);
}

export function bool_arr_to_bytes(arr: Array<Array<boolean>>): string {
    // EG: [[true, false, true, true], 
    // [false, true, false, true]] => [ 1011, 0101 ]
    let bytes = '';

    arr.forEach((a) => {
        let x = '';

        a.forEach((b) => x += b ? '1' : '0');

        bytes += x + ' ';
    });

    return bytes.trim();

}

export function x_bits_from_arr(arr: Array<Array<boolean>>, start: number, end: number): Array<boolean> {
    let x: Array<boolean> = [];

    start = start - 1;

    let i = 0;
    arr.forEach((sub) => sub.forEach((b) => {
        if (i >= start && i < end) x.push(b); i++; }));

    return x;
}

export function arr_to_num(arr: Array<boolean>): number {
    // EG: [true, false, true, true] => 11
    let x = '';

    arr.forEach((b) => x += b ? '1' : '0');

    return parseInt(x, 2);
}