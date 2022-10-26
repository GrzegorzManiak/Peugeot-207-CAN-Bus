export interface Frame {
    id: string;
    data: Array<string>;
}

export enum Mode {
    DESERIZALIZATION = 'DESERIALIZATION',
    ALL = 'ALL',
}
