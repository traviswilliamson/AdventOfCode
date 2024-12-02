use std::error::Error;

pub fn first(_input: String) -> Result<String, Box<dyn Error>> {
    let safe = _input.split_terminator("\n")
        .filter(|&s|
            s
            .split_ascii_whitespace()
            .try_fold((None, None), |state:(Option<bool>, Option<i32>), element| {
                let el_num = element.parse::<i32>().unwrap();
                if state.1.is_some() {
                    let increasing = state.1? > el_num;
                    let diff = state.1?.abs_diff(el_num);
                    if state.0.is_some() && state.0? != increasing {
                        None
                    }
                    else if diff < 1 || diff > 3 {
                        None
                    }
                    else {
                        Some((Some(increasing), Some(el_num)))
                    }
                }
                else {
                    Some((None, Some(el_num)))
                }
            }).is_some()
        )
        .count();
    return Ok(safe.to_string());
}

fn test_safe(mut _row : std::slice::Iter<'_, i32>) -> (bool, usize) {
    let mut prev = _row.next().unwrap();
    let mut increasing = None;
    for (index, el) in _row.enumerate() {
        let bigger = el > prev;
        let diff = el.abs_diff(*prev);
        prev = el;
        if diff < 1 || diff > 3 {
            return (false, index)
        }
        if increasing.is_some() {
            if bigger != increasing.unwrap() {
                return (false, index)
            }
        }
        else {
            increasing = Some(bigger);
        }
    }
    (true, 0)
}

pub fn second(_input: String) -> Result<String, Box<dyn Error>> {
    let safe = _input.split_terminator("\n")
        .filter(|&s| {
            let mut row : Vec::<i32> = s.split_ascii_whitespace().map(|el| el.parse().unwrap()).collect();
            // First, does the while thing work?
            let base_works = test_safe(row.iter());
            if base_works.0 { true }
            else {
                // If not, does it work without the element before the failing index (Which is one less than the real position)?
                let removed = row.remove(base_works.1);
                let first_works = test_safe(row.iter());
                if first_works.0 {
                    return true
                }
                else {
                    // If not, does it work replacing the failure point with the removed value?
                    let replaced_2 = row[base_works.1];
                    row[base_works.1] = removed;
                    let second_works = test_safe(row.iter());
                    if second_works.0 { 
                        return true 
                    }
                    else if base_works.1 > 0 {
                        // We could have gotten in a bad state, check the one before
                        row[base_works.1 - 1] = row[base_works.1];
                        row[base_works.1] = replaced_2;
                        return test_safe(row.iter()).0;
                    }
                }
                false
            }
        })
        .count();
    return Ok(safe.to_string());
}
