fn print(i: usize) {
    println!("{}", i);
}

const global: usize = 10;
static static_global: usize = 10;

fn main() {
    let mut i = 0;
    while i < 10 {
        print(i);
        i += 1;
    }
    for j in 0..10 {
        print(global);
    }
    let k = 10;
    loop {
        print(k);
        i += 1;
        if i == 10 {
            break;
        }
    }
}
