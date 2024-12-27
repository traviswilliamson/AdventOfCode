use std::error::Error;

pub fn first(_input: String) -> Result<String, Box<dyn Error>> {
    let mut locks = Vec::new();
    let mut keys = Vec::new();
    _input.split_terminator("\n\n").for_each(|grid| {
        let lock = grid.starts_with('#');
        let mut set = vec![0; 5];
        grid.split_terminator("\n").skip(1).take(5).for_each(|line| {
            line.chars().enumerate().filter(|(_, c)| *c == '#').for_each(|(i, _)| {
                set[i] += 1;
            });
        });
        match lock {
            true => locks.push(set),
            false => keys.push(set),
        }
    });
    let mut count_matches = 0;
    for lock in locks {
        for key in keys.iter() {
            if lock.iter().enumerate().all(|(i, v)| {
                v + key[i] <= 5
            }) {
                count_matches += 1;
            }
        }
    }
    Ok(count_matches.to_string())
}

pub fn second(_input: String) -> Result<String, Box<dyn Error>> {
    return Err(Box::<dyn Error>::from("Not implemented!"));
}
