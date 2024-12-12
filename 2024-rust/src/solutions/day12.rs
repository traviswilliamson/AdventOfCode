use std::{borrow::Borrow, cell::RefCell, error::Error, rc::Rc};

struct Region {
    area: u16,
    perimeter: u16
}

pub fn first(_input: String) -> Result<String, Box<dyn Error>> {
    let mut garden: Vec<Vec<(u8, Option<Rc<RefCell<Region>>>)>> = _input.split_terminator("\n")
        .map(|line| {
            line.bytes().map(|b| (b, None)).collect::<Vec<(u8, Option<Rc<RefCell<Region>>>)>>()
        }).collect();
    let mut regions: Vec<Rc<RefCell<Region>>> = Vec::new();
    for i in 0..garden.len() {
        let on_top = i == 0;
        let on_bottom = i == garden.len() - 1;
        for j in 0..garden[i].len() {
            let on_left = j == 0;
            let on_right = j == garden[i].len() - 1;
            let plant = garden[i][j].0;
            if !on_top && garden[i - 1][j].0 == plant {
                garden[i][j].1 = Some(garden[i - 1][j].1.as_ref().unwrap().clone());
            }
            else if !on_left && garden[i][j - 1].0 == plant {
                garden[i][j].1 = Some(garden[i][j - 1].1.as_ref().unwrap().clone());
            }
            else {
                garden[i][j].1 = Some(Rc::new(RefCell::new(Region {
                    area: 0,
                    perimeter: 0
                })));
                regions.push(garden[i][j].1.as_ref().unwrap().clone());

            }
            let mut region = garden[i][j].1.as_ref().unwrap().borrow_mut();
            region.area += 1;
            if on_top || garden[i - 1][j].0 != plant {
                region.perimeter += 1;
            }
            if on_bottom || garden[i + 1][j].0 != plant {
                region.perimeter += 1;
            }
            if on_left || garden[i][j - 1].0 != plant {
                region.perimeter += 1;
            }
            if on_right || garden[i][j + 1].0 != plant {
                region.perimeter += 1;
            }
        }
    }
    Ok(regions.iter().map(|r| {
        let r: &RefCell<Region> = r.borrow();
        println!("{:?} {:?}", r.borrow().area, r.borrow().perimeter);
        r.borrow().area * r.borrow().perimeter
    }).sum::<u16>().to_string())
}

pub fn second(_input: String) -> Result<String, Box<dyn Error>> {
    return Err(Box::<dyn Error>::from("Not implemented!"));
}
