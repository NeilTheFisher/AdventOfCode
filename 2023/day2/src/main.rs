use std::{
    fs::File,
    io::{prelude::*, BufReader},
};

fn main() -> std::io::Result<()> {
    // let file = File::open("test_input.txt")?;
    let file = File::open("input.txt")?;
    let reader = BufReader::new(file);

    let mut sum = 0;

    for line in reader.lines() {
        let line = line.unwrap();
        let game_text = line.split(": ").last().unwrap();

        let mut max_red = 0;
        let mut max_green = 0;
        let mut max_blue = 0;

        let sub_games = game_text.split("; ");
        for sub_game in sub_games {
            let cubes = sub_game.split(", ");
            for cube in cubes {
                let [cube_count, cube_color]: [&str; 2] =
                    cube.split(" ").collect::<Vec<_>>().try_into().unwrap();
                let cube_count = cube_count.parse::<i32>().unwrap();

                match cube_color {
                    "red" => {
                        max_red = max_red.max(cube_count);
                    }
                    "green" => {
                        max_green = max_green.max(cube_count);
                    }
                    "blue" => {
                        max_blue = max_blue.max(cube_count);
                    }
                    _ => unreachable!(),
                }
            }
        }

        let power = max_red * max_green * max_blue;

        sum += power;
    }

    println!("sum: {sum}");

    Ok(())
}
