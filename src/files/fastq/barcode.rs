// Copyright (c) 2021 Kana LLC

extern crate shell;
#[macro_use] extern crate shell;
use shell::ShellResult;

// Pass an argument
let name = "shell";
cmd!("echo Hello rust {}!", name).run().unwrap();

// Extract environment variable
cmd!("echo HOME is $HOME").run().unwrap();