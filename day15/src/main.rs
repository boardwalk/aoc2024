use anyhow::{anyhow, Error};

fn main() -> Result<(), Error> {
    let (grid, extra) = tools::load_grid(std::io::stdin().lock())?;
    let extra = extra.ok_or_else(|| anyhow!("missing extra"))?;
    println!("{grid:?}, {extra:?}");

    Ok(())
}
