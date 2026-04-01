use std::{fs::File, io::Read};

fn main() {
    let input_file = "input.txt";
    let mut input_file = File::open(input_file).expect("input file not found");

    let mut input = String::new();
    input_file
        .read_to_string(&mut input)
        .expect("error reading input file");

    let mut dial_value = 50;
    let mut password = 0;

    for rotation in input.split_whitespace() {
        let (dir, rotation_value) = rotation.split_at(1);

        let rotation_value: i32 = rotation_value
            .parse()
            .expect("rotation value is not an integer");

        let num_rotations = rotation_value / 100;
        let remaining_rotation = rotation_value % 100;

        password += num_rotations;

        if dir == "L" {
            if (remaining_rotation >= dial_value) && (dial_value != 0) {
                password += 1;
            }
            dial_value -= rotation_value;
            dial_value = ((dial_value % 100) + 100) % 100;
        } else if dir == "R" {
            if (remaining_rotation >= 100 - dial_value) && (dial_value != 0) {
                password += 1;
            }
            dial_value += rotation_value;
            dial_value = ((dial_value % 100) + 100) % 100;
        }
    }

    println!("password is {password}");
}
