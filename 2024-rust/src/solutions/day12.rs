use std::{borrow::Borrow, cell::RefCell, collections::HashSet, error::Error, rc::Rc};

struct Region {
    area: u32,
    perimeter: u32,
}

pub fn first(_input: String) -> Result<String, Box<dyn Error>> {
    let mut garden: Vec<Vec<(u8, usize)>> = _input.split_terminator("\n")
        .map(|line| {
            line.bytes().map(|b| (b, 0)).collect::<Vec<(u8, usize)>>()
        }).collect();
    let mut regions: Vec<Region> = Vec::new();
    for i in 0..garden.len() {
        let on_top = i == 0;
        let on_bottom = i == garden.len() - 1;
        for j in 0..garden[i].len() {
            let on_left = j == 0;
            let on_right = j == garden[i].len() - 1;
            let plant = garden[i][j].0;
            if !on_top && garden[i - 1][j].0 == plant {
                garden[i][j].1 = garden[i - 1][j].1;
                if !on_left && garden[i][j - 1].0 == plant && garden[i][j - 1].1 != garden[i][j].1 {
                    // Merge regions
                    regions[garden[i][j].1].area += regions[garden[i][j - 1].1].area;
                    regions[garden[i][j].1].perimeter += regions[garden[i][j - 1].1].perimeter;
                    regions[garden[i][j - 1].1].area = 0;
                    regions[garden[i][j - 1].1].perimeter = 0;
                    for ii in 0..(i+1) {
                        for jj in 0..garden[ii].len() {
                            if garden[ii][jj].1 == garden[i][j - 1].1 {
                                garden[ii][jj].1 = garden[i][j].1;
                            }
                        }
                    }
                }
            }
            else if !on_left && garden[i][j - 1].0 == plant {
                garden[i][j].1 = garden[i][j - 1].1;
            }
            else {
                regions.push(Region {
                    area: 0,
                    perimeter: 0
                });
                garden[i][j].1 = regions.len() - 1;

            }
            regions[garden[i][j].1].area += 1;
            if on_top || garden[i - 1][j].0 != plant {
                regions[garden[i][j].1].perimeter += 1;
            }
            if on_bottom || garden[i + 1][j].0 != plant {
                regions[garden[i][j].1].perimeter += 1;
            }
            if on_left || garden[i][j - 1].0 != plant {
                regions[garden[i][j].1].perimeter += 1;
            }
            if on_right || garden[i][j + 1].0 != plant {
                regions[garden[i][j].1].perimeter += 1;
            }
        }
    }
    Ok(regions.iter().map(|r| {
        r.area * r.perimeter
    }).sum::<u32>().to_string())
}

struct CheapRegion {
    area: u32,
    corners: u32,
}

pub fn second(_input: String) -> Result<String, Box<dyn Error>> {
    let mut garden: Vec<Vec<(u8, usize)>> = _input.split_terminator("\n")
        .map(|line| {
            line.bytes().map(|b| (b, 0)).collect::<Vec<(u8, usize)>>()
        }).collect();
    let mut regions: Vec<CheapRegion> = Vec::new();
    for i in 0..garden.len() {
        let on_top = i == 0;
        let on_bottom = i == garden.len() - 1;
        for j in 0..garden[i].len() {
            let on_left = j == 0;
            let on_right = j == garden[i].len() - 1;
            let plant = garden[i][j].0;
            if !on_top && garden[i - 1][j].0 == plant {
                garden[i][j].1 = garden[i - 1][j].1;
                if !on_left && garden[i][j - 1].0 == plant && garden[i][j - 1].1 != garden[i][j].1 {
                    // Merge regions
                    regions[garden[i][j].1].area += regions[garden[i][j - 1].1].area;
                    regions[garden[i][j].1].corners += regions[garden[i][j - 1].1].corners;
                    regions[garden[i][j - 1].1].area = 0;
                    regions[garden[i][j - 1].1].corners = 0;
                    for ii in 0..(i+1) {
                        for jj in 0..garden[ii].len() {
                            if garden[ii][jj].1 == garden[i][j - 1].1 {
                                garden[ii][jj].1 = garden[i][j].1;
                            }
                        }
                    }
                }
            }
            else if !on_left && garden[i][j - 1].0 == plant {
                garden[i][j].1 = garden[i][j - 1].1;
            }
            else {
                regions.push(CheapRegion {
                    area: 0,
                    corners: 0
                });
                garden[i][j].1 = regions.len() - 1;
            }

            // TODO: Fix logic for corners
            regions[garden[i][j].1].area += 1;
            if on_top || garden[i - 1][j].0 != plant {
                regions[garden[i][j].1].corners += 1;
            }
            if on_bottom || garden[i + 1][j].0 != plant {
                regions[garden[i][j].1].corners += 1;
            }
            if on_left || garden[i][j - 1].0 != plant {
                regions[garden[i][j].1].corners += 1;
            }
            if on_right || garden[i][j + 1].0 != plant {
                regions[garden[i][j].1].corners += 1;
            }
        }
    }
    Ok(regions.iter().map(|r| {
        r.area * r.corners
    }).sum::<u32>().to_string())
}
