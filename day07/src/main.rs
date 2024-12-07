use anyhow::Error;

fn main() -> Result<(), Error> {
    let grid = tools::load_grid(std::io::stdin().lock())?;
    println!("{grid:?}");
    Ok(())
}
