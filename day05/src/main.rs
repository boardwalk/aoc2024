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

fn is_update_ok(update: &[usize], rules: &[(usize, usize)]) -> bool {
    let page_to_print_time: HashMap<usize, usize> = update
        .iter()
        .enumerate()
        .map(|(t, page)| (*page, t))
        .collect();

    for (page_x, page_y) in rules {
        let Some(page_x_time) = page_to_print_time.get(page_x) else {
            // page never printed, rule doesn't apply
            continue;
        };

        let Some(page_y_time) = page_to_print_time.get(page_y) else {
            // page never printed, rule doesn't apply
            continue;
        };

        if *page_x_time >= *page_y_time {
            // x not printed before y, rule broken
            return false;
        }
    }

    // all rules obeyed
    true
}

fn get_update_middle_page(update: &[usize]) -> usize {
    assert!(update.len() % 2 == 1);
    update[update.len() / 2]
}

fn main() -> Result<(), Error> {
    let rules = load_rules();
    let updates = load_updates();

    let mut res = 0;

    for update in &updates {
        if is_update_ok(update, &rules) {
            res += get_update_middle_page(update);
        }
    }

    println!("{res:?}");
    Ok(())
}
