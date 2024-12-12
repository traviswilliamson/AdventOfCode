use std::{collections::{HashMap, HashSet}, error::Error};

struct Map {
    map : HashMap<(i8, i8), (usize, Option<HashSet<(i8, i8)>>)> 
}

impl Map {
    fn eval_point(&mut self, (i, j): (i8, i8)) {
        if self.map.get(&(i, j)).unwrap().1.is_some() {
            return;
        }
        let current_height = self.map.get(&(i, j)).unwrap().0;
        let mut this_points_set = HashSet::new();
        let up = (i - 1, j);
        if self.map.contains_key(&up) && self.map.get(&up).unwrap().0 == current_height + 1 {
            self.eval_point(up);
            this_points_set.extend(self.map.get(&up).unwrap().1.clone().unwrap().iter());
        }
        let down = (i + 1, j);
        if self.map.contains_key(&down) && self.map.get(&down).unwrap().0 == current_height + 1 {
            self.eval_point(down);
            this_points_set.extend(self.map.get(&down).unwrap().1.clone().unwrap().iter());
        }
        let left = (i, j - 1);
        if self.map.contains_key(&left) && self.map.get(&left).unwrap().0 == current_height + 1 {
            self.eval_point(left);
            this_points_set.extend(self.map.get(&left).unwrap().1.clone().unwrap().iter());
        }
        let right = (i, j + 1);
        if self.map.contains_key(&right) && self.map.get(&right).unwrap().0 == current_height + 1 {
            self.eval_point(right);
            this_points_set.extend(self.map.get(&right).unwrap().1.clone().unwrap().iter());
        }
        self.map.get_mut(&(i, j)).unwrap().1 = Some(this_points_set);
    }
}

pub fn first(_input: String) -> Result<String, Box<dyn Error>> {
    let mut trail_heads = Vec::new();
    let map: HashMap<(i8, i8), (usize, Option<HashSet<(i8, i8)>>)> = _input.split_terminator("\n")
        .enumerate()
        .fold(HashMap::new(), |map: HashMap<(i8, i8), (usize, Option<HashSet<(i8, i8)>>)>, (i, line)| {
            line.bytes().enumerate().fold(map, |mut map: HashMap<(i8, i8), (usize, Option<HashSet<(i8, i8)>>)>, (j, c)| {
                let height =(c - b'0') as usize;
                if height == 0 {
                    trail_heads.push((i as i8, j as i8));
                }
                let set: Option<HashSet<(i8, i8)>> = match height {
                    9 => Some(HashSet::from_iter(vec![(i as i8, j as i8)])),
                    _ => None
                };
                map.insert((i as i8, j as i8), (height, set));
                map
            })
        });
    let mut s_map = Map {
        map: map
    };
    let mut score = 0;
    for trail_head in trail_heads {
        s_map.eval_point(trail_head);
        score += s_map.map.get(&trail_head).unwrap().1.as_ref().unwrap().len();
    }
    Ok(score.to_string())
}

struct Mup {
    map : HashMap<(i8, i8), (usize, Option<i32>)>
}

impl Mup {
    fn eval_point(&mut self, (i, j): (i8, i8)) {
        if self.map.get(&(i, j)).unwrap().1.is_some() {
            return;
        }
        let current_height = self.map.get(&(i, j)).unwrap().0;
        let mut this_points_score = 0;
        let up = (i - 1, j);
        if self.map.contains_key(&up) && self.map.get(&up).unwrap().0 == current_height + 1 {
            self.eval_point(up);
            this_points_score += self.map.get(&up).unwrap().1.unwrap();
        }
        let down = (i + 1, j);
        if self.map.contains_key(&down) && self.map.get(&down).unwrap().0 == current_height + 1 {
            self.eval_point(down);
            this_points_score += self.map.get(&down).unwrap().1.unwrap();
        }
        let left = (i, j - 1);
        if self.map.contains_key(&left) && self.map.get(&left).unwrap().0 == current_height + 1 {
            self.eval_point(left);
            this_points_score += self.map.get(&left).unwrap().1.unwrap();
        }
        let right = (i, j + 1);
        if self.map.contains_key(&right) && self.map.get(&right).unwrap().0 == current_height + 1 {
            self.eval_point(right);
            this_points_score += self.map.get(&right).unwrap().1.unwrap();
        }
        self.map.get_mut(&(i, j)).unwrap().1 = Some(this_points_score);
    }
}

pub fn second(_input: String) -> Result<String, Box<dyn Error>> {
    let mut trail_heads = Vec::new();
    let map: HashMap<(i8, i8), (usize, Option<i32>)> = _input.split_terminator("\n")
        .enumerate()
        .fold(HashMap::new(), |map: HashMap<(i8, i8), (usize, Option<i32>)>, (i, line)| {
            line.bytes().enumerate().fold(map, |mut map: HashMap<(i8, i8), (usize, Option<i32>)>, (j, c)| {
                let height =(c - b'0') as usize;
                if height == 0 {
                    trail_heads.push((i as i8, j as i8));
                }
                let paths: Option<i32> = match height {
                    9 => Some(1),
                    _ => None
                };
                map.insert((i as i8, j as i8), (height, paths));
                map
            })
        });
    let mut s_map = Mup {
        map: map
    };
    let mut score = 0;
    for trail_head in trail_heads {
        s_map.eval_point(trail_head);
        score += s_map.map.get(&trail_head).unwrap().1.unwrap();
    }
    Ok(score.to_string())
}
