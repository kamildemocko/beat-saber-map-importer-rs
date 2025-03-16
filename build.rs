use std::io;
use winresource::WindowsResource;

fn main() -> io::Result<()> {
    WindowsResource::new()
        .set_icon("assets/icon.ico")
        .compile()?;

    Ok(())
}