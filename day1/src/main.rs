const INPUT: &'static str = include_str!("../input.txt");

fn main() {
    part_a();
    part_b();
}

fn part_a() {
    let mut floor: i64 = 0;

    for (_, ch) in INPUT.char_indices() {
        floor += match ch {
            '(' => 1,
            ')' => -1,
            _ => 0,
        };
    }

    println!("final: {}", floor);
}

fn part_b() {
    let mut floor = 0;
    let mut index = 0;

    for (idx, ch) in INPUT.char_indices() {
        floor += match ch {
            '(' => 1,
            ')' => -1,
            _ => 0,
        };

        if floor == -1 {
            index = idx + 1;
            break;
        }
    }

    println!("index: {}", index);
}
