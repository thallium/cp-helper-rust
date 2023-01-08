## CP Helper

A CLI utility that receives test cases from Competitive Companion and generates files accordingly. The file structure will look something like:
```
├── AtCoder
│   └── AtCoder_Beginner_Contest_284
│       ├── A__Sequence_of_Strings
│       │   ├── 1.in
│       │   ├── 1.out
│       │   ├── 2.in
│       │   ├── 2.out
│       │   └── main.cpp
└── Codeforces
    └── Educational_Codeforces_Round_58_Rated_for_Div_2
        └── A_Minimum_Integer
            ├── 1.in
            ├── 1.out
            └── main.cpp
```

## Installation

Download from [Releases](https://github.com/thallium/cp-helper-rust/releases)

## Configuration

The location for the config file is:
- Linux: /home/<user>/.config/cp-helper/config.toml
- Windows: C:\Users\<user>\AppData\Roaming\cp-helper\config.toml
- macOS: /Users/<user>/Library/Application Support/cp-helper/config.toml or /Users/<user>/.config/cp-helper/config.toml

Possible configurations:
- contest_path:
    - The path where all the data should be stored, if it's not set, the data will be stored under the current directory.

