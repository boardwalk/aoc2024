use anyhow::Error;

struct Prize {
    a: (usize, usize),
    b: (usize, usize),

    prize: (usize, usize),
}

fn main() -> Result<(), Error> {
    let prizes = vec![
        Prize {
            a: (94, 34),
            b: (22, 67),
            prize: (8400, 5400),
        },
        Prize {
            a: (26, 66),
            b: (67, 21),
            prize: (12748, 12176),
        },
        Prize {
            a: (17, 86),
            b: (84, 37),
            prize: (7870, 6450),
        },
        Prize {
            a: (69, 23),
            b: (27, 71),
            prize: (18641, 10279),
        },
    ];

    Ok(())
}
