use console::style;

fn main() -> std::io::Result<()> {
    claquer::clear_screen()?;

    claquer::intro(style(" create-app ").on_cyan().black())?;

    let _: u8 = claquer::text("Input a number (not greater than 256)")
        .placeholder("0")
        .interact()?;

    claquer::group(vec![
        claquer::item("path", |_| {
            claquer::text("Where should we create your project?")
                .placeholder("./sparkling-solid")
                .validate(|input: &String| {
                    if input.is_empty() {
                        Err("Please enter a path.")
                    } else if !input.starts_with("./") {
                        Err("Please enter a relative path")
                    } else {
                        Ok(())
                    }
                })
                .interact()
        }),
        claquer::item("password", |_| {
            claquer::password("Provide a password").mask('●').interact()
        }),
    ])?;

    claquer::outro(format!(
        "Problems? {}",
        style("https://example.com/issues").cyan().underlined()
    ))?;

    Ok(())
}
