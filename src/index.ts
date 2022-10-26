import { sleep } from "./core/common";
import { parse_frame } from "./core/deserialization/frames";
import { get_master, has_changed } from "./core/recall";
import { test_data } from "./core/test_data";

import { Frame, Mode } from './index.d';

const fs = require('fs');
const readline = require('readline');


// -- // CONFIG // -- //

export const DEBUG_MODE: boolean = true;
export const IGNORED_IDS: string[] = [];
export const WHITE_LIST: string[] = [];

export const MODE = Mode.ALL;

// -- // CONFIG // -- //



// -- Data -- // 
async function read_frames(fn: (arg0: Frame) => void, td = test_data) {
    switch(DEBUG_MODE) {
        case true:
            const fileStream = fs.createReadStream(td.data_path);
            const rl = readline.createInterface({
                input: fileStream,
                crlfDelay: Infinity
            });

            for await (const line of rl) {
                fn(td.parse(line));
                await sleep(1000 / td.feed_per_sec);
            }

            break;

        case false:
            break;
    }
}

function start() {
    read_frames((frame) => {
        // -- Check if white list has items,
        // if so, only process those items.
        if (WHITE_LIST.length > 0 && 
            WHITE_LIST.indexOf(frame.id) === -1) return;

        // -- Check if frame is ignored 
        if (IGNORED_IDS.includes(frame.id))
            return;
    
        // -- Parse the packet
        const parsed = parse_frame(frame);
        
        // -- Check if the packet can be parsed
        if (parsed === undefined) return;
    
        // -- Check if the packet has changed
        has_changed(frame.id, parsed.data, parsed.val);

        console.log(get_master());
    }).then(() => {
        // -- Print out the global object
        console.log(get_master());
    });
}
            
start();
// const frame = test_data.parse('21:53:01.677 -> Standard ID: 0x0B6       DLC: 8  Data: 0x22 0x05 0x00 0x00 0x00 0x00 0x1A 0xD0');
// console.log(parse_frame(frame));
