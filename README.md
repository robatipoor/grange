# granger
[![Crates.io](https://img.shields.io/crates/v/grange.svg?style=plastic)](http://crates.io/crates/grange)
[![Build Status](https://travis-ci.org/robatipoor/grange.svg?branch=master)](https://travis-ci.org/robatipoor/grange)
[![Build status](https://ci.appveyor.com/api/projects/status/017f6i47aqspsgav?svg=true)](https://ci.appveyor.com/project/robatipoor/grange)
[![License: MIT](https://img.shields.io/badge/license-MIT-blue.svg)](LICENSE)

create range numbers in rust

**add dependency**

```sh
cargo add grange
```
or 
```toml
grange = "0.1.0"
```

**example**

```rs
use grange::{Range,range};

fn main() {
    for i in range!(1,,2,,=20){ // equal to (1..=20).step_by(2)
        println!("{}", i);
    }
    for i in range!(10,,-1,,=1){ // equal to (1..=10).rev() 
        println!("{}", i);
    }
    for i in range!(10,,-2,,=1){  
        println!("{}", i);
    }
    for i in range!(,,3,,10){ // equal to (0..10).step_by(3)
        println!("{}", i);
    }
    for i in range!(10,,1){  
        println!("{}", i);
    }
}
```
