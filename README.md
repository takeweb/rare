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

## 次の目標
tree --gitignore
.
├── Cargo.toml
├── LICENSE
├── README.md
├── src
│   ├── lib.rs
│   └── main.rs
└── tests
    ├── cli.rs
    ├── expected
    │   └── no_args_1.txt
    └── inputs
        ├── inputs1
        │   ├── aaa
        │   │   └── one.txt
        │   └── bbb
        │       ├── eleven.txt
        │       └── ten.txt
        └── inputs2
            ├── aaa
            │   └── one.txt
            ├── bbb
            │   ├── ccc
            │   │   ├── ddd
            │   │   │   └── empty.txt
            │   │   └── three.txt
            │   └── ten.txt
            └── two.txt

13 directories, 15 files

## 現状
cargo run -- -e .git,.vscode,target,.gitignore,Cargo.lock,.DS_Store
.
├── Cargo.toml
├── LICENSE
├── README.md
├── [src]
│    ├── lib.rs
│    └── main.rs
└── [tests]
     ├── cli.rs
     ├── [expected]
│    │    └── no_args_1.txt
     └── [inputs]
│    │    ├── [inputs1]
│    │    │    ├── [aaa]
│    │    │    │    └── one.txt
│    │    │    └── [bbb]
│    │    │    │    ├── eleven.txt
│    │    │    │    └── ten.txt
│    │    └── [inputs2]
│    │    │    ├── [aaa]
│    │    │    │    └── one.txt
│    │    │    ├── [bbb]
│    │    │    │    ├── [ccc]
│    │    │    │    │    ├── [ddd]
│    │    │    │    │    │    └── empty.txt
│    │    │    │    │    └── three.txt
│    │    │    │    └── ten.txt
│    │    │    └── two.txt

13 directories, 15 files