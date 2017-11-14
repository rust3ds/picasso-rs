This is a convenience wrapper around the `picasso` CLI
and is primarily intended to be used in build scripts.

Here is an example `build.rs`:

```rust
extern crate picasso;

use std::env;
use std::path::PathBuf;

fn main() {
    let out_dir = PathBuf::from(env::var("OUT_DIR").unwrap());

    picasso::assemble(&["vshader.v.pica"], out_dir.join("vshader.v.shbin"))
        .unwrap_or_else(|e| panic!("{}", e));
}
```

To embed the assembled shaders in your binary you can do the following:

```rust
static VSHADER_SHBIN: &[u8] = include_bytes!(concat!(env!("OUT_DIR"), "/vshader.v.shbin"));
```
