import { Frame } from '../index.d';
const path = require('path');

export interface TestDataInterface {
    data_path: string;
    feed_per_sec: number;
    parse: (data: string) => Frame;
}

export const test_data: TestDataInterface = {
    feed_per_sec: 100,
    data_path: path.join(__dirname, '../../test_data/2.txt'),
    parse: (data: string) => {
        // EG Frame => 20:13:04.801 -> 1Standard ID: 0x036       DLC: 8  Data: 0x00 0x00 0x00 0x0F 0x03 0x00 0x00 0xA0
        return {
            id: data.split('ID:')[1].split('DLC')[0].trim(),
            data: data.split('Data:')[1].trim().split(' '),
        };
    }
}
