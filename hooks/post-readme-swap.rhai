if file::exists("README.generated.md") {
    if file::exists("README.md") {
        file::delete("README.md");
    }
    file::rename("README.generated.md", "README.md");
}

if file::exists("cargo-generate.toml") {
    file::delete("cargo-generate.toml");
}

if file::exists("hooks") {
    file::delete("hooks");
}

if file::exists("scripts") {
    file::delete("scripts");
}
