let cache: {
    [key: string]: {
        data: any;
        num: number;
    };
} = {};

export function has_changed(id: string, data: any, num: number): boolean {
    if (cache[id] === undefined) {
        cache[id] = {
            data: data,
            num: num,
        };

        return true;
    }

    if (cache[id].data !== data || cache[id].num !== num) {
        cache[id] = {
            data: data,
            num: num,
        };

        return true;
    }

    return false;
}

export function get_latest(id: string): any {
    return cache[id];
}

export function get_master(): any {
    let master = {};

    for (let key in cache) {
        master = Object.assign(master, cache[key].data);
    }

    return master;
}

export function test_for_dupe_key(): boolean {
    let all_keys: Array<string> = [];

    for (let key in cache) {
        for (let value in cache[key].data) {
            if (all_keys.indexOf(value) === -1)
                all_keys.push(value);
            
            else return true;
        }
    }

    return false;
}