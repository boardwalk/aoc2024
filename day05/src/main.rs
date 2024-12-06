#![feature(map_many_mut)]
use anyhow::Error;
use std::collections::HashMap;

fn load_rules() -> Vec<(usize, usize)> {
    let mut rules = Vec::new();

    for ln in std::io::stdin().lines() {
        let ln = ln.unwrap();

        if ln.is_empty() {
            break;
        }

        let tokens: Vec<_> = ln.split('|').collect();

        assert!(tokens.len() == 2);

        let lhs = usize::from_str_radix(tokens[0], 10).unwrap();
        let rhs = usize::from_str_radix(tokens[1], 10).unwrap();

        rules.push((lhs, rhs));
    }

    rules
}

fn load_updates() -> Vec<Vec<usize>> {
    let mut updates = Vec::new();

    for ln in std::io::stdin().lines() {
        let ln = ln.unwrap();
        let update: Vec<usize> = ln
            .split(',')
            .map(|token| usize::from_str_radix(token, 10).unwrap())
            .collect();

        updates.push(update);
    }

    updates
}

// returns true if a change was required
fn fix_update(update: &mut [usize], rules: &[(usize, usize)]) -> bool {
    let mut page_to_index: HashMap<usize, usize> = update
        .iter()
        .enumerate()
        .map(|(t, page)| (*page, t))
        .collect();

    let mut changed = false;

    loop {
        let mut changed_this_cycle = false;

        for (page_x, page_y) in rules {
            let [Some(page_x_index), Some(page_y_index)] =
                page_to_index.get_many_mut([page_x, page_y])
            else {
                continue;
            };

            if *page_x_index >= *page_y_index {
                // x not printed before y, rule broken
                // get slightly closely to an ok update by swapping the two
                update.swap(*page_x_index, *page_y_index);
                std::mem::swap(page_x_index, page_y_index);

                changed = true;
                changed_this_cycle = true;
            }
        }

        if !changed_this_cycle {
            break;
        }
    }

    changed
}

fn get_update_middle_page(update: &[usize]) -> usize {
    assert!(update.len() % 2 == 1);
    update[update.len() / 2]
}

fn main() -> Result<(), Error> {
    let rules = load_rules();
    let updates = load_updates();

    let mut good_updates = Vec::new();
    let mut bad_updates = Vec::new();

    for update in updates.into_iter() {
        let mut copy = update.clone();
        if fix_update(&mut copy, &rules) {
            bad_updates.push(update);
        } else {
            good_updates.push(update)
        }
    }

    let mut p1_res = 0;
    for update in &good_updates {
        p1_res += get_update_middle_page(update);
    }

    for update in &mut bad_updates {
        fix_update(update, &rules);
    }

    let mut p2_res = 0;
    for update in &bad_updates {
        p2_res += get_update_middle_page(update);
    }

    println!("p1 = {p1_res:?}");
    println!("p2 = {p2_res:?}");
    Ok(())
}
