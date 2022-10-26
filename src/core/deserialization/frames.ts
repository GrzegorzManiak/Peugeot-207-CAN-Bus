// 0E6

import { Frame } from "../..";
import { arr_to_num, bool_arr_to_int, fixed_dlc_len, hex_to_bin, x_bits_from_arr } from "../common";
import { MESSAGE_TABLE } from "../message_table";

export interface FrameParserInterface {
    val: number;
    bin: Array<Array<boolean>>;
}

export type FrameExt = {
    data: {
        [key: string]: string | number | boolean | Array<Array<boolean>>;
    }
} & FrameParserInterface;



/*
    @name 220
    @desc Door state

    0 - Front left door
    1 - Front right door
    2 - Rear left door
    3 - Rear right door
    4 - Boot
*/
const f220 = (frame: Frame): FrameExt => {
    const data = hex_to_bin(fixed_dlc_len(1, frame));

    return {
        bin: data,
        val: bool_arr_to_int(data),
        data: {
            front_left_door: data[0][0],
            front_right_door: data[0][1],
            rear_left_door: data[0][2],
            rear_right_door: data[0][3],
            boot: data[0][4],
        },
    };
}



/*
    @name 0B6
    @desc Odometer + extra
*/
const f0B6 = (frame: Frame): FrameExt => {
    const data = hex_to_bin(fixed_dlc_len(8, frame));


    // -- Tachometer, 1-13
    const Tachometer = x_bits_from_arr(data, 1, 13);

    // -- Speed, 12, Bit 17 - 24
    const Speed = x_bits_from_arr(data, 17, 24);

    // -- Odometer, 28, bit 28 - 32
    const Odometer = x_bits_from_arr(data, 28, 32);

    // -- Fule consumption, bit 49 - 56
    const FuelConsumption = x_bits_from_arr(data, 49, 56);


    return {
        val: bool_arr_to_int(data),
        bin: data,

        data: {
            rpm: arr_to_num(Tachometer),
            speed: arr_to_num(Speed) * 100,
            driven: arr_to_num(Odometer),
            fuel_consumption_counter: arr_to_num(FuelConsumption),
        }
    }
}



/*
    @name 0E6
    @desc Wheel rotation
*/
const f0E6 = (frame: Frame): FrameExt => {
    const data = hex_to_bin(fixed_dlc_len(8, frame));


    // -- Left wheen roation, 9 - 24
    // Left wheel rotation counter, ABS impulses
    const LeftWheelRotation = x_bits_from_arr(data, 9, 24);

    // -- Right wheen roation, 25 - 40
    // Right wheel rotation counter, ABS impulses
    const RightWheelRotation = x_bits_from_arr(data, 25, 40);

    // -- Voltage, 41 - 48
    // Voltage = (V + 144) / 20 
    const Voltage = x_bits_from_arr(data, 41, 48);


    return {
        val: bool_arr_to_int(data),
        bin: data,

        data: {
            left_wheel_rotation: arr_to_num(LeftWheelRotation),
            right_wheel_rotation: arr_to_num(RightWheelRotation),
            voltage: (arr_to_num(Voltage) + 144) / 20,
        }
    }
}



/*
    @name 0F6
    @desc Ignition, cool tempt, odometer, temp, revgear, turn lr
*/
const f0F6 = (frame: Frame): FrameExt => {
    const data = hex_to_bin(fixed_dlc_len(8, frame));


    // -- Ignition, 5
    const Ignition = data[0][4];

    // -- Temp, byte 2
    // Temperature = C-39 °C 
    const Temp = data[1];

    // -- Odometer, Byte 3-5
    const Odometer = x_bits_from_arr(data, 17, 40);

    // -- Outside temp, Byte 7
    // Temperature = round(T/2.0 - 39.5) °C
    const OutsideTemp = data[6];

    // -- Revgear, Byte 8 bit 1
    const Revgear = data[7][0];

    // -- Right, Left signal, Byte 8 bit 7, 8
    const TurnLR = data[7][6],
        TurnRR = data[7][7];


    return {
        val: bool_arr_to_int(data),
        bin: data,

        data: {
            ignition: Ignition,
            coolant_temp: arr_to_num(Temp) - 39,
            odometer: arr_to_num(Odometer) / 10,
            outside_temp: arr_to_num(OutsideTemp) / 2 - 39.5,
            revgear: Revgear,
            turn_lr: TurnLR,
            turn_rr: TurnRR,
        }
    }
}



/*
    @name 128
    @desc Dashboard lightning
*/
const f128 = (frame: Frame): FrameExt => {
    const data = hex_to_bin(fixed_dlc_len(6, frame));


    // Driver seat belt warning light, bit 2
    const driver_belt = data[0][1];

    // Any door or trunk open, byte 2, bit 4
    const door_open = data[1][3];

    // Byte 5, Increasing by 1
    const side_lights = data[4][0],
        low_beam = data[4][1],
        high_beam = data[4][2],
        front_fog_lights = data[4][3],
        rear_fog_lights = data[4][4],
        right_indicator = data[4][5],
        left_indicator = data[4][6];

    // Low fuel warning light, byte 6, bit 1
    const low_fuel = data[5][0];


    return {
        val: bool_arr_to_int(data),
        bin: data,
        
        data: {
            driver_belt_warning: driver_belt,
            door_open: door_open,
            side_lights: side_lights,
            low_beam: low_beam,
            high_beam: high_beam,
            front_fog_lights: front_fog_lights,
            rear_fog_lights: rear_fog_lights,
            right_indicator: right_indicator,
            left_indicator: left_indicator,
            low_fuel: low_fuel,
        }
    }
}



/*
    @name 1A1
    @desc Informational message
*/
const f1A1 = (frame: Frame): FrameExt => {
    const data = hex_to_bin(fixed_dlc_len(2, frame));

    // Byte 1,
    // 0x80 - show window / 128
    // 0x7F - hide window / 127
    // 0xFF - clear window (default) / 255
    const show_hide_clear = arr_to_num(data[0]).toString();

    const MESSAGES: { [key: string]: string; }= {
        '128': 'show', '127': 'hide', '255': 'clear',
    };

    // Byte 2 message code
    const message_code = arr_to_num(data[1]).toString(16).toUpperCase();
    
    return {
        val: bool_arr_to_int(data),
        bin: data,

        data: {
            show_hide_clear: MESSAGES[show_hide_clear],
            message_code,
            message: MESSAGE_TABLE[message_code],
        }
    }
}



/*
    @name 221
    @desc Trip computer
*/
const f221 = (frame: Frame): FrameExt => {
    const data = hex_to_bin(fixed_dlc_len(8, frame));


    // Liters per 100 km, byte 2, 3
    const liters_per_100km = arr_to_num(x_bits_from_arr(data, 9, 16));

    // Rest of run, byte 4, 5
    const rest_of_run = arr_to_num(x_bits_from_arr(data, 25, 32));

    return {
        val: bool_arr_to_int(data),
        bin: data,

        data: {
            liters_per_100km: liters_per_100km / 10,
            rest_of_run: rest_of_run
        }
    }
}



/*
    @name 2B6
    @desc ASCII coded last 8 VIN digits
*/
const f2B6 = (frame: Frame): FrameExt => {
    const data = hex_to_bin(fixed_dlc_len(8, frame));

    // Byte 1-8
    const vin = data.map((byte) => {
        return String.fromCharCode(arr_to_num(byte));
    }).join('');

    return {
        val: bool_arr_to_int(data),
        bin: data,
        data: { last_8_vin: vin },
    }
}



/*
    @name 3B6
    @desc ASCII coded 4-9 letters of VIN
*/
const f3B6 = (frame: Frame): FrameExt => {
    const data = hex_to_bin(fixed_dlc_len(6, frame));

    // Byte 1-8
    const vin = data.map((byte) => {
        return String.fromCharCode(arr_to_num(byte));
    }).join('');

    return {
        val: bool_arr_to_int(data),
        bin: data,
        data: { vin_4_9: vin },
    }
}



/*
    @name 30D
    @desc Wheels rotation speed
*/
const f30D = (frame: Frame): FrameExt => {
    const data = hex_to_bin(fixed_dlc_len(8, frame));

    // FL 1,2
    const FL = [...data[0], ...data[1]];

    // FR 3,4
    const FR = [...data[2], ...data[3]];

    // RL 5,6
    const RL = [...data[4], ...data[5]];
    
    // RR 7,8
    const RR = [...data[6], ...data[7]];

    return {
        val: bool_arr_to_int(data),
        bin: data,
        
        data: {
            front_left: arr_to_num(FL),
            front_right: arr_to_num(FR),
            rear_left: arr_to_num(RL),
            rear_right: arr_to_num(RR),
        }
    }
}



/*
    @name 3F6
    @desc Date and time
*/
const f3F6 = (frame: Frame): FrameExt => {
    const data = hex_to_bin(fixed_dlc_len(8, frame));

    // Time in seconds bit 1-20
    const time = arr_to_num(x_bits_from_arr(data, 0, 20));

    // Format: Byte 6 bit 1
    const format = data[5][0];
    // 0 - 12h
    // 1 - 24h
    const formats = ['12h', '24h'];
    

    return {
        val: bool_arr_to_int(data),
        bin: data,
        
        data: {
            time_in_sec: time,
            format: formats[format === true ? 1 : 0],
        }
    }
}


export function parse_frame(frame: Frame): FrameExt | undefined {
    switch(frame.id) {
        case '0x220': return f220(frame);
        case '0x0B6': return f0B6(frame);
        case '0x0E6': return f0E6(frame);
        case '0x0F6': return f0F6(frame);
        case '0x128': return f128(frame);
        case '0x1A1': return f1A1(frame);
        case '0x221': return f221(frame);
        case '0x2B6': return f2B6(frame);
        case '0x30D': return f30D(frame);
        case '0x3B6': return f3B6(frame);
        case '0x3F6': return f3F6(frame);
        default: return undefined;
    }
}