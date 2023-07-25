# Ch. 2 - Setting Up A Project for rust using Cargo

- `cargo new ...` - creates a brand new project, `--vcs [git, hg, pijul, fossil, none]` allows one to choose the type of version conrol used
- `cargo build` - Compiles the Project, change aware, meaning it won't compile if there aren't any changes detected
- `cargo run` - Compiles & Runs the Project, change aware, meaning it won't compile if there aren't any changes detected
- `cargo check` - Builds project without producing a binary to check for errors


# Cargo

- Project binary files end up being put in `target/debug` directory
- Commands stay the save accross Operating Systems
