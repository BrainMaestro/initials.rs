//! `initials` crate helps to generate customizable avatars with the initial characters from the names. 
//!
//! # Usage
//!
//! - Extern `initials` crate on your project.
//! ```
//! extern crate initials;
//!
//! ```
//!
//! - Draw the avatar by using `initials::AvatarBuilder`
//! ```
//! use initials::AvatarBuilder;
//! 
//! let image = AvatarBuilder::new("Anakin Skywalker")
//!   .draw();
//!
//! ```
//! This will import dynamic RGBA image.
//! 
//! - If you wish to save the image:
//! ```
//!   use initials::AvatarBuilder;
//!
//!   let image = AvatarBuilder::new("Anakin Skywalker")
//!     .draw();
//!
//!   image.save("avatar.jpg").unwrap();
//! ```
//! 
//! Or, you may manipulate `DynamicImage` according to your needs after building.([Docs](https://docs.rs/image))
//!
//! # Customization
//!
//! `initials` allows to fully customize the attributes of the image.
//! 
//! ##### Default Attributes
//!   -  **font:** Hirgino Sans
//!   -  **font_scale:** 150.0
//!   -  **length:** 2
//!   -  **width:** 300
//!   -  **height:** 300
//!   -  **contrast_ratio:** 4.5
//!   -  **font_color:** randomly generated
//!   -  **background_color:** randomly generated
//!
//! ##### Manipulation
//!
//! |  method | description |
//! |-----------|-------------|
//! |  with_font(str) | Font file path(.ttf)  |
//! |  with_font_color(str)   | Font hex color code  |
//! |  with_font_scale(f32)  | Uniform scale of the text |
//! |  with_background_color(str)  | Background hex color code  |
//! |  with_length(usize)  |  Font length |
//! |  with_height(u32)  | Image height  |
//! |  with_width(u32)  | Image width  |
//! |  with_contrast_ratio(u32)  | Contrast ratio for the randomly generated colors  |
//! 
//! ##### Example
//! 
//! ```
//!   use initials::AvatarBuilder;
//!
//!   let image = AvatarBuilder::new("Anakin Skywalker")
//!     .with_background_color("#FAFAFA")
//!     .with_font_color("#000000")
//!     .with_length(1)
//!     .draw();
//!
//! ```
//! - This will export an initials avatar `A` with black font and white background.
//!
//!
//! # Randomization
//!
//! - By default, `background color` and `font color` will be generated by considering the contrast ratio.
//!
//! ```
//!   use initials::AvatarBuilder;
//!
//!   let image = AvatarBuilder::new("Lucky Seven")
//!     .draw();
//!
//!   let image_with_random_background = AvatarBuilder::new("Lucky Seven")
//!     .with_font_color("#000000")
//!     .draw();
//!
//!   let image_with_random_font = AvatarBuilder::new("Lucky Seven")
//!     .with_background_color("#FAFAFA")
//!     .draw();
//! ```
//! 
//! - Means that you may fully customize the colors or unsetted colors will be automatically generated
//! by providing clear and readable avatars according to the contrast ratio.

#[macro_use]
extern crate failure;

extern crate rand;
extern crate image;
extern crate rusttype;

pub mod hex;
pub mod contrast;
pub mod avatar;

pub use avatar::AvatarBuilder;