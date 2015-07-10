# find_file
A simple tool to search files&amp;folders in give directory

# Usage
```rust
println!("{:?}", Search::dir("/Path/to/directory").with_depth(3).for_folder("src"));
println!("{:?}", Search::dir("/Path/to/directory").with_depth(3).for_file("src/lib.rs"));
println!("{:?}", Search::cwd().for_file("lib.rs"));
```
# Note
1. Search depth defaults to 1.
2. If search path like "src/lib.rs", depth determines the first dir in the path, i.e "src".
