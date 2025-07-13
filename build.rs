fn main() {
    let mut res = winres::WindowsResource::new();
    res.set_icon("media/simpler-icon.ico");
    res.compile().expect("Failed to compile resources");
}
