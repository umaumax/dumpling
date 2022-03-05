# 🥟dumpling

This is a rust crate for dumping variables for debugging.

## how to add
``` toml
[dependencies]
dumpling = { git = "https://github.com/umaumax/dumpling.git" }
```

or

`git clone https://github.com/umaumax/dumpling`

``` toml
[dependencies]
dumpling = { path = "./dumpling" }
```

## how to use
``` rust
use dumpling::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    {
        println!("# lower case macro!");
        let v = 123_i32;
        println!("{} -> {}", v, dump_hex!(v));
        let v = 1.23_f32;
        println!("{} -> {}", v, dump_hex!(v));
        let v = vec![1.0_f64, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0, 10.0];
        println!("{:?} -> {}", v, dump_iter_hex!(v));
        let v = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
        println!("{:?} -> {}", v, dump_binary_hex!(v));
        let v = "Hello world!";
        println!("{:?} -> {}", v, dump_binary_hex!(v.as_bytes()));
        let v = "Hello world!";
        println!("sha1: {:?} -> {}", v, dump_sha1!(v.as_bytes()));
        let v = "Hello world!";
        println!("sha256: {:?} -> {}", v, dump_sha256!(v.as_bytes()));
        let v = "Hello world!";
        println!("md5: {:?} -> {}", v, dump_md5!(v.as_bytes()));
    }
    println!();
    {
        println!("# upper case macro!");
        let v = 123_i32;
        println!("{} -> {}", v, dump_HEX!(v));
        let v = 1.23_f32;
        println!("{} -> {}", v, dump_HEX!(v));
        let v = vec![1.0_f64, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0, 10.0];
        println!("{:?} -> {}", v, dump_iter_HEX!(v));
        let v = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
        println!("{:?} -> {}", v, dump_binary_HEX!(v));
        let v = "Hello world!";
        println!("{:?} -> {}", v, dump_binary_HEX!(v.as_bytes()));
        let v = "Hello world!";
        println!("sha1: {:?} -> {}", v, dump_SHA1!(v.as_bytes()));
        let v = "Hello world!";
        println!("sha256: {:?} -> {}", v, dump_SHA256!(v.as_bytes()));
        let v = "Hello world!";
        println!("md5: {:?} -> {}", v, dump_MD5!(v.as_bytes()));
    }

    Ok(())
}
```
