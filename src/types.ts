export interface Frame {
    id: string;
    data: Array<string>;
}

export enum Mode {
    DESERIZALIZATION = 'DESERIALIZATION',
    ALL = 'ALL',
}

export type FrameExt = {
    val: number;
    bin: Array<Array<boolean>>;
    data: { [key: string]: string | number | boolean }
};
