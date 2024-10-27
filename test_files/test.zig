fn add(a: usize, b: usize) usize {
    return a + b;
}

var global: usize = 1;
const global2: usize = 1;

pub fn main() !void {
    var arr: [10]usize = undefined;
    const local: usize = for (0..arr.len) |i| {
        arr[i] = i;
        if (i == 9) break 5;
    } else 3;

    for (&arr, 0..) |*a, i| {
        a.* = i;
    }

    arr[0] = while (global < 100) {
        global = add(global, local);
        if (global == 38) break 7;
    } else 8;

    while (global < 100) {
        global = add(global, local);
    }
}
