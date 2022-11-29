# Managing Growing projects with Packages, Crates and Modules

## Packages and Crates

-   **A Crate**
    -   Smalllest amount of code
    -   Binary crates - have main fn and compile to executable
    -   Library crates - don't have main fun and don't compile (often used as just 'crate')
-   **A package**
    -   a bundle of one or more crates, must contain Cargo.toml (describes how to build)
    -   package can contain many binary crates
    -   package can contain max 1 library crate
    -   package must contain at least one crate (bin or lib)
    -   src/main.rs = the crate root of binary crate
    -   src/lib.rs = the crate root of library crate
    -   if src/main.rs & src/lib.rs then two crates(one bin and one lib)
    -   if mulitiple binary crates then files placed in src/bin directory, each file being
        a seperate crate
