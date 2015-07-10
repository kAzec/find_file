# find_file
A simple tool to search files&amp;folders in give directory

# Usage
```rust
println!("{:?}", Search::dir("/Path/to/directory").with_depth(3).for_folder("src"));
println!("{:?}", Search::dir("/Path/to/directory").with_depth(3).for_file("src/lib.rs"));
println!("{:?}", Search::dir("/Path/to/directory").with_depth(3).for_both("src/lib.rs"));
// Search current working dir
println!("{:?}", Search::cwd(0).for_file("src/lib.rs"));
// Search parent dir of cwd
println!("{:?}", Search::cwd(1).with_depth(2).for_file("src/lib.rs"));
```
# Note
1. Search depth defaults to 1.
2. If search path like "src/lib.rs", depth determines the first dir in the path, i.e "src".
3. Search may fail with NotFound error in such conditions:
```rust
//$ tree directory 
///directory
//├── subdir1
//│   └── name1
//│       └── name2-diff
//└── subdir2
//    └── name1
//        └── name2
println!("{:?}", Search::dir(directory).with_depth(2).for_both("name1/name2"));
```
