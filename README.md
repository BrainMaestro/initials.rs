initials [![Build Status](https://travis-ci.org/sonmezonur/initials.rs.svg?branch=master)](https://travis-ci.org/sonmezonur/initials.rs)
=======

`initials` crate helps to generate customizable avatars with the initial characters from the names.

<p float="center">
  <img src="gen/a.jpg" width="100" height="100"/> 
  <img src="gen/铁.jpg" width="100" height="100" />
  <img src="gen/fm.jpg" width="100" height="100"/>
  <img src="gen/i.jpg" width="100" height="100"/>
</p>


Setup and Usage
--------

In your Cargo.toml, add the following:

```toml
[dependencies]
initials = "*"
```

Extern `initials` crate and draw the image on your project:

```rust
extern crate initials;

use initials::{AvatarBuilder, AvatarResult};

fn avatar() -> AvatarResult {
    AvatarBuilder::new("Avatar")
        .with_font_color("#000000")?
        .with_background_color("#FAFAFA")?
        .with_width(200)?
        .with_height(200)
}

fn main() {
    let avatar = avatar().unwrap();
    let image = avatar.draw();
    // use the generated image
}

```

See [Documentation](https://sonmezonur.github.io/initials.rs)

Example
--------
[App](https://iavatar.herokuapp.com) |
[Doc](https://github.com/sonmezonur/initials.rs/blob/master/api)

License
--------

MIT
