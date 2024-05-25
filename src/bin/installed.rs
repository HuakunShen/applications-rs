fn main() -> Result<(), Box<dyn std::error::Error>> {
    let apps = installed::list()?;
    let mut i = 0;
    for app in apps {
        // metadata accessor fns, these are only evaluated when used
        let name = app.name();
        let version = app.version();
        let publisher = app.publisher();
        println!("{name} v{version} by {publisher}");
        i += 1;
    }
    println!("{} apps found", i);
    Ok(())
}