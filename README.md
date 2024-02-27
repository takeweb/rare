# rtree

## ~/.ssh配下のディレクトリを.DS_Storeを除外して、tree出力
```
cargo run -- ~/dev/Rust/rust_tools/rtree/tests/inputs -e .DS_Store

/Users/xxxxx/dev/Rust/rust_tools/rtree/tests/inputs
├── aaa
│   │   ├── empty.txt
│   └── one.txt
├── bbb
│   ├── ccc
│   │   │   │   └── three.txt
│   └── ten.txt
└── two.txt
```
