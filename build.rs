fn main() {
    if std::env::var("CARGO_CFG_TARGET_OS").unwrap() == "windows" {
        let mut res = winres::WindowsResource::new();
        res.set_icon("assets/icon.ico");
        res.set("FileDescription", "Dependencies cleaner");
        res.set("ProductName", "Dependencies cleaner By Mk");
        res.set("OriginalFilename", "Cleaner.exe");
        res.set("LegalCopyright", "Copyright (c) Mk");
        res.set("FileVersion", "1.0.0.0");
        res.set("ProductVersion", "1.0.0.0");
        res.set("CompanyName", "Majestic World Studio");
        res.compile().unwrap();
    }
}