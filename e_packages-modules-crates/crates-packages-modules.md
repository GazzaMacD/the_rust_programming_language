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
-   **Modules**

    -   Start from the crate root: When compiling a crate, the compiler first looks in the crate root file (usually src/lib.rs for a library crate or src/main.rs for a binary crate) for code to compile.
    -   Declaring modules: In the crate root file, you can declare new modules; say, you declare a “garden” module with mod garden;. The compiler will look for the module’s code in these places:
        -   Inline, within curly brackets that replace the semicolon following mod garden
        -   In the file src/garden.rs
        -   In the file src/garden/mod.rs
    -   Declaring submodules: In any file other than the crate root, you can declare submodules. For example, you might declare mod vegetables; in src/garden.rs. The compiler will look for the submodule’s code within the directory named for the parent module in these places:
        -   Inline, directly following mod vegetables, within curly brackets instead of the semicolon
        -   In the file src/garden/vegetables.rs
        -   In the file src/garden/vegetables/mod.rs
    -   Paths to code in modules: Once a module is part of your crate, you can refer to code in that module from anywhere else in that same crate, as long as the privacy rules allow, using the path to the code. For example, an Asparagus type in the garden vegetables module would be found at crate::garden::vegetables::Asparagus.
    -   Private vs public: Code within a module is private from its parent modules by default. To make a module public, declare it with pub mod instead of mod. To make items within a public module public as well, use pub before their declarations.
    -   The use keyword: Within a scope, the use keyword creates shortcuts to items to reduce repetition of long paths. In any scope that can refer to crate::garden::vegetables::Asparagus, you can create a shortcut with use crate::garden::vegetables::Asparagus; and from then on you only need to write Asparagus to make use of that type in the scope.
    -   modules are declared using the `mod` keyword
    -   example of a restaurant library crate below in src/lib.rs

        ```rs
        mod front_of_house {
            mod hosting {
                fn add_to_waitlist() {}

                fn seat_at_table() {}
            }

            mod serving {
                fn take_order() {}

                fn serve_order() {}

                fn take_payment() {}
            }
        }
        ```

    -   this would result in the following structure where crate is the src/lib.rs file
        ```
            crate
            └── front_of_house
                ├── hosting
                │   ├── add_to_waitlist
                │   └── seat_at_table
                └── serving
                    ├── take_order
                    ├── serve_order
                    └── take_payment
        ```
    -   note the parent / child relationships and also the sibling relationships
    -   crate is an implicit name

-   **Paths for referring to an item in the module tree**

    -   absolute path is the full path starting from the crate root (either crate name or 'crate')
    -   relative paths use self, super or identifier in current module
    -   separators are `::`
    -   example of relative and absolute below

    ```
    mod front_of_house {
        mod hosting {
            fn add_to_waitlist() {}
        }
    }

    pub fn eat_at_restaurant() {
        // Absolute path
        crate::front_of_house::hosting::add_to_waitlist();

        // Relative path
        front_of_house::hosting::add_to_waitlist();
    }
    ```
