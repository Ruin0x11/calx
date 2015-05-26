use std::convert::{From};
use std::cell::RefCell;
use std::collections::HashMap;
use std::ascii::{AsciiExt};
use std::ops::{Add, Sub, Mul};
use std::fmt;
use num::{Float, Num};

/// Color in sRGB color space.
///
/// This is the physical color definition on computer monitors, also the
/// color format most often used when writing out RGB values of computer
/// graphics colors.
#[derive(Copy, Clone, PartialEq, PartialOrd, Debug, RustcEncodable, RustcDecodable)]
pub struct SRgba {
    /// sRGB red component
    pub r: u8,
    /// sRGB green component
    pub g: u8,
    /// sRGB blue component
    pub b: u8,
    /// sRGB alpha channel
    pub a: u8,
}

impl SRgba {
    pub fn new(r: u8, g: u8, b: u8, a: u8) -> SRgba {
        SRgba { r: r, g: g, b: b, a: a }
    }
}

impl fmt::Display for SRgba {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "#{:02X}{:02X}{:02X}{:02X}", self.r, self.g, self.b, self.a)
    }
}

impl From<Rgba> for SRgba {
    #[inline]
    fn from(c: Rgba) -> SRgba {
        SRgba::new(
            (to_srgb(c.r) * 255.0).round() as u8,
            (to_srgb(c.g) * 255.0).round() as u8,
            (to_srgb(c.b) * 255.0).round() as u8,
            (to_srgb(c.a) * 255.0).round() as u8)
    }
}

impl<'a> From<&'a str> for SRgba {
    fn from(c: &'a str) -> SRgba {
        let rgba: Rgba = c.into();
        rgba.into()
    }
}

/// Color in linear color space.
///
/// This is the canonical color representation that the rendering engine
/// expects to get.
///
/// The `::from(&str)` constructor will try to parse the color string
/// and will panic if the parsing fails.
#[derive(Copy, Clone, PartialEq, PartialOrd, Debug, RustcEncodable, RustcDecodable)]
pub struct Rgba {
    /// Linear red component
    pub r: f32,
    /// Linear green component
    pub g: f32,
    /// Linear blue component
    pub b: f32,
    /// Alpha channel
    pub a: f32,
}

impl Rgba {
    pub fn new(r: f32, g: f32, b: f32, a: f32) -> Rgba {
        Rgba { r: r, g: g, b: b, a: a }
    }

    /// Turn color to monochrome preserving brightness.
    pub fn to_monochrome(&self) -> Rgba {
        let luma = self.r * 0.2126 + self.g * 0.7152 + self.b * 0.0722;
        Rgba::new(luma, luma, luma, self.a)
    }

    pub fn into_array(self) -> [f32; 4] {
        use std::mem;
        unsafe { mem::transmute(self) }
    }
}

impl From<SRgba> for Rgba {
    #[inline]
    fn from(s: SRgba) -> Rgba {
        Rgba::new(
            to_linear(s.r as f32 / 255.0),
            to_linear(s.g as f32 / 255.0),
            to_linear(s.b as f32 / 255.0),
            to_linear(s.a as f32 / 255.0))
    }
}

impl From<u32> for Rgba {
    fn from(u: u32) -> Rgba {
        SRgba::new(
            (u >> 24) as u8,
            (u >> 16) as u8,
            (u >> 8)  as u8,
            u         as u8).into()
    }
}

impl<'a> From<&'a str> for Rgba {
    fn from(s: &'a str) -> Rgba {
        thread_local!(static MEMOIZER: RefCell<HashMap<String, Rgba>> =
                      RefCell::new(HashMap::new()));

        let ret = MEMOIZER.with(|c| c.borrow().get(s).map(|&x| x));
        match ret {
            Some(color) => color,
            None => {
                // XXX: Panic if parsing fails.
                let parsed = parse_color(s).expect(&format!("Bad color string '{}'", s));
                MEMOIZER.with(|c| c.borrow_mut().insert(s.to_string(), parsed));
                parsed
            }
        }
    }
}

impl Add<Rgba> for Rgba {
    type Output = Rgba;
    fn add(self, rhs: Rgba) -> Rgba {
        Rgba::new(
            self.r + rhs.r,
            self.g + rhs.g,
            self.b + rhs.b,
            self.a + rhs.a)
    }
}

impl Sub<Rgba> for Rgba {
    type Output = Rgba;
    fn sub(self, rhs: Rgba) -> Rgba {
        Rgba::new(
            self.r - rhs.r,
            self.g - rhs.g,
            self.b - rhs.b,
            self.a - rhs.a)
    }
}

impl Mul<f32> for Rgba {
    type Output = Rgba;
    fn mul(self, rhs: f32) -> Rgba {
        Rgba::new(
            self.r * rhs,
            self.g * rhs,
            self.b * rhs,
            self.a)
    }
}

impl Mul<Rgba> for Rgba {
    type Output = Rgba;
    fn mul(self, rhs: Rgba) -> Rgba {
        Rgba::new(
            self.r * rhs.r,
            self.g * rhs.g,
            self.b * rhs.b,
            self.a * rhs.a)
    }
}

#[inline]
pub fn to_linear(srgb: f32) -> f32 {
   if srgb <= 0.04045 {
       srgb / 12.92
   } else {
       ((srgb + 0.055) / (1.055)).powf(2.4)
   }
}

#[inline]
pub fn to_srgb(linear: f32) -> f32 {
    if linear < 0.0031308 {
        12.92 * linear
    } else {
        (1.0 + 0.055) * linear.powf(1.0 / 2.4) - 0.055
    }
}

/// Try to parse a string representation into a color value.
///
/// Accepts case-insensitive SVG color names ("Green", "powderblue") and
/// hex `#RGB` or `#RGBA` color names with 4 or 8 bits per channel.
/// "RED", "red", "#F00", "#F00F", "#FF0000" and "#FF0000FF" all
/// correspond to the same opaque pure red color.
pub fn parse_color(name: &str) -> Option<Rgba> {
    if let Some(color) = parse_color_name(&name.to_string().to_ascii_uppercase()[..]) {
        return Some(color);
    }

    if name.starts_with("#") {
        let name = &name[1..];

        // Hex digits per color channel, either 1 or 2. Single digit values
        // get doubled for the color, #420 becomes #442200.
        let digits: usize;

        // Does the color include the alpha channel. If not, assume alpha is
        // fully opaque.
        let alpha: bool;

        match name.len() {
            3 => {
                digits = 1;
                alpha = false;
            }
            4 => {
                digits = 1;
                alpha = true;
            }
            6 => {
                digits = 2;
                alpha = false;
            }
            8 => {
                digits = 2;
                alpha = true;
            }
            _ => { return None; }
        }

        assert!(digits == 1 || digits == 2);

        let r = Num::from_str_radix(&name[0..(digits)], 16);
        let g = Num::from_str_radix(&name[(digits)..(2 * digits)], 16);
        let b = Num::from_str_radix(&name[(2 * digits)..(3 * digits)], 16);
        let a = if alpha {
            Num::from_str_radix(&name[(3 * digits)..(4 * digits)], 16)
        } else {
            if digits == 1 { Ok(0xFu8) } else { Ok(0xFFu8) }
        };

        return match (r, g, b, a) {
            (Ok(mut r), Ok(mut g), Ok(mut b), Ok(mut a)) => {
                if digits == 1 {
                    r = (r << 4) + r;
                    g = (g << 4) + g;
                    b = (b << 4) + b;
                    a = (a << 4) + a;
                }

                Some(Rgba::from(SRgba::new(r, g, b, a)))
            }
            _ => None
        };
    }

    return None;
}

macro_rules! color_constants {
    {
        $([$name:ident, $r:expr, $g:expr, $b:expr])+
    } => {
        pub mod color {
            /*! Color constants */
            use super::Rgba;
            $(pub static $name: Rgba = Rgba { r: $r, g: $g, b: $b, a: 1.0 };)+
        }

        fn parse_color_name(upper_case_name: &str) -> Option<Rgba> {
            match upper_case_name {
                $(stringify!($name) => Some(color::$name),)+
                _ => None
            }
        }
    }
}

color_constants!{
[ALICEBLUE, 0.8714, 0.9387, 1.0]
[ANTIQUEWHITE, 0.956, 0.8308, 0.6795]
[AQUA, 0.0, 1.0, 1.0]
[AQUAMARINE, 0.2122, 1.0, 0.6584]
[AZURE, 0.8714, 1.0, 1.0]
[BEIGE, 0.9131, 0.9131, 0.7157]
[BISQUE, 1.0, 0.7758, 0.552]
[BLACK, 0.0, 0.0, 0.0]
[BLANCHEDALMOND, 1.0, 0.8308, 0.6105]
[BLUE, 0.0, 0.0, 1.0]
[BLUEVIOLET, 0.2542, 0.02416, 0.7605]
[BROWN, 0.3763, 0.02315, 0.02315]
[BURLYWOOD, 0.7305, 0.4793, 0.2423]
[CADETBLUE, 0.1144, 0.3419, 0.3515]
[CHARTREUSE, 0.2122, 1.0, 0.0]
[CHOCOLATE, 0.6445, 0.1413, 0.01298]
[CORAL, 1.0, 0.2122, 0.08022]
[CORNFLOWERBLUE, 0.1274, 0.3005, 0.8469]
[CORNSILK, 1.0, 0.9387, 0.7157]
[CRIMSON, 0.7157, 0.006995, 0.04519]
[CYAN, 0.0, 1.0, 1.0]
[DARKBLUE, 0.0, 0.0, 0.2582]
[DARKCYAN, 0.0, 0.2582, 0.2582]
[DARKGOLDENROD, 0.4793, 0.2384, 0.003347]
[DARKGRAY, 0.3968, 0.3968, 0.3968]
[DARKGREEN, 0.0, 0.1274, 0.0]
[DARKKHAKI, 0.5089, 0.4735, 0.147]
[DARKMAGENTA, 0.2582, 0.0, 0.2582]
[DARKOLIVEGREEN, 0.09084, 0.147, 0.02843]
[DARKORANGE, 1.0, 0.2623, 0.0]
[DARKORCHID, 0.3185, 0.0319, 0.6038]
[DARKRED, 0.2582, 0.0, 0.0]
[DARKSALMON, 0.8148, 0.305, 0.1946]
[DARKSEAGREEN, 0.2747, 0.5029, 0.2747]
[DARKSLATEBLUE, 0.0648, 0.04667, 0.2582]
[DARKSLATEGRAY, 0.02843, 0.07819, 0.07819]
[DARKTURQUOISE, 0.0, 0.6172, 0.6376]
[DARKVIOLET, 0.2961, 0.0, 0.6514]
[DEEPPINK, 1.0, 0.006995, 0.2918]
[DEEPSKYBLUE, 0.0, 0.521, 1.0]
[DIMGRAY, 0.1413, 0.1413, 0.1413]
[DODGERBLUE, 0.01298, 0.2789, 1.0]
[FIREBRICK, 0.4452, 0.016, 0.016]
[FLORALWHITE, 1.0, 0.956, 0.8714]
[FORESTGREEN, 0.016, 0.2582, 0.016]
[FUCHSIA, 1.0, 0.0, 1.0]
[GAINSBORO, 0.7157, 0.7157, 0.7157]
[GHOSTWHITE, 0.9387, 0.9387, 1.0]
[GOLD, 1.0, 0.6795, 0.0]
[GOLDENROD, 0.7011, 0.3763, 0.01444]
[GRAY, 0.2159, 0.2159, 0.2159]
[GREEN, 0.0, 0.2159, 0.0]
[GREENYELLOW, 0.4179, 1.0, 0.02843]
[HONEYDEW, 0.8714, 1.0, 0.8714]
[HOTPINK, 1.0, 0.1413, 0.4564]
[INDIANRED, 0.6105, 0.107, 0.107]
[INDIGO, 0.07036, 0.0, 0.2232]
[IVORY, 1.0, 1.0, 0.8714]
[KHAKI, 0.8714, 0.7913, 0.2623]
[LAVENDER, 0.7913, 0.7913, 0.956]
[LAVENDERBLUSH, 1.0, 0.8714, 0.9131]
[LAWNGREEN, 0.2016, 0.9734, 0.0]
[LEMONCHIFFON, 1.0, 0.956, 0.6105]
[LIGHTBLUE, 0.4179, 0.6867, 0.7913]
[LIGHTCORAL, 0.8714, 0.2159, 0.2159]
[LIGHTCYAN, 0.7454, 1.0, 1.0]
[LIGHTGOLDENRODYELLOW, 0.956, 0.956, 0.6445]
[LIGHTGREEN, 0.2789, 0.855, 0.2789]
[LIGHTGRAY, 0.6514, 0.6514, 0.6514]
[LIGHTPINK, 1.0, 0.4678, 0.5333]
[LIGHTSALMON, 1.0, 0.3515, 0.1946]
[LIGHTSEAGREEN, 0.01444, 0.4452, 0.402]
[LIGHTSKYBLUE, 0.2423, 0.6172, 0.956]
[LIGHTSLATEGRAY, 0.1845, 0.2462, 0.3185]
[LIGHTSTEELBLUE, 0.4342, 0.552, 0.7305]
[LIGHTYELLOW, 1.0, 1.0, 0.7454]
[LIME, 0.0, 1.0, 0.0]
[LIMEGREEN, 0.0319, 0.6105, 0.0319]
[LINEN, 0.956, 0.8714, 0.7913]
[MAGENTA, 1.0, 0.0, 1.0]
[MAROON, 0.2159, 0.0, 0.0]
[MEDIUMAQUAMARINE, 0.1329, 0.6105, 0.402]
[MEDIUMBLUE, 0.0, 0.0, 0.6105]
[MEDIUMORCHID, 0.491, 0.09084, 0.6514]
[MEDIUMPURPLE, 0.2918, 0.162, 0.7084]
[MEDIUMSEAGREEN, 0.04519, 0.4508, 0.1651]
[MEDIUMSLATEBLUE, 0.1981, 0.1384, 0.855]
[MEDIUMSPRINGGREEN, 0.0, 0.956, 0.3231]
[MEDIUMTURQUOISE, 0.0648, 0.6376, 0.6038]
[MEDIUMVIOLETRED, 0.5711, 0.007499, 0.2346]
[MIDNIGHTBLUE, 0.009721, 0.009721, 0.162]
[MINTCREAM, 0.9131, 1.0, 0.956]
[MISTYROSE, 1.0, 0.7758, 0.7529]
[MOCCASIN, 1.0, 0.7758, 0.4621]
[NAVAJOWHITE, 1.0, 0.7305, 0.4179]
[NAVY, 0.0, 0.0, 0.2159]
[OLDLACE, 0.9823, 0.9131, 0.7913]
[OLIVE, 0.2159, 0.2159, 0.0]
[OLIVEDRAB, 0.147, 0.2705, 0.01681]
[ORANGE, 1.0, 0.3763, 0.0]
[ORANGERED, 1.0, 0.05951, 0.0]
[ORCHID, 0.7011, 0.162, 0.6724]
[PALEGOLDENROD, 0.855, 0.807, 0.402]
[PALEGREEN, 0.314, 0.9647, 0.314]
[PALEVIOLETRED, 0.7084, 0.162, 0.2918]
[PAPAYAWHIP, 1.0, 0.8632, 0.6654]
[PEACHPUFF, 1.0, 0.7011, 0.4851]
[PERU, 0.6105, 0.2346, 0.04971]
[PINK, 1.0, 0.5271, 0.5972]
[PLUM, 0.7231, 0.3515, 0.7231]
[POWDERBLUE, 0.4342, 0.7454, 0.7913]
[PURPLE, 0.2159, 0.0, 0.2159]
[RED, 1.0, 0.0, 0.0]
[ROSYBROWN, 0.5029, 0.2747, 0.2747]
[ROYALBLUE, 0.05286, 0.1413, 0.7529]
[SADDLEBROWN, 0.2582, 0.05951, 0.006512]
[SALMON, 0.956, 0.2159, 0.1683]
[SANDYBROWN, 0.956, 0.3712, 0.117]
[SEAGREEN, 0.02732, 0.2582, 0.09531]
[SEASHELL, 1.0, 0.9131, 0.855]
[SIENNA, 0.3515, 0.08438, 0.02624]
[SILVER, 0.5271, 0.5271, 0.5271]
[SKYBLUE, 0.2423, 0.6172, 0.8308]
[SLATEBLUE, 0.1441, 0.1022, 0.6105]
[SLATEGRAY, 0.162, 0.2159, 0.2789]
[SNOW, 1.0, 0.956, 0.956]
[SPRINGGREEN, 0.0, 1.0, 0.2122]
[STEELBLUE, 0.06125, 0.2232, 0.4564]
[TAN, 0.6445, 0.4564, 0.2623]
[TEAL, 0.0, 0.2159, 0.2159]
[THISTLE, 0.6867, 0.521, 0.6867]
[TOMATO, 1.0, 0.1248, 0.06301]
[TURQUOISE, 0.05127, 0.7454, 0.6308]
[VIOLET, 0.855, 0.2232, 0.855]
[WHEAT, 0.9131, 0.7305, 0.4508]
[WHITE, 1.0, 1.0, 1.0]
[WHITESMOKE, 0.9131, 0.9131, 0.9131]
[YELLOW, 1.0, 1.0, 0.0]
[YELLOWGREEN, 0.3231, 0.6105, 0.0319]
}

#[cfg(test)]
mod test {
    #[test]
    fn test_parse_color() {
        use std::convert::{From};
        use super::{Rgba, SRgba, parse_color};

        assert_eq!(None, parse_color(""));
        assert_eq!(None, parse_color("#"));
        assert_eq!(None, parse_color("#12"));
        assert_eq!(None, parse_color("#123456789ABC"));
        assert_eq!(None, parse_color("#ff0000garbage"));
        assert_eq!(None, parse_color("#ffjunk"));
        assert_eq!(None, parse_color("actuallynotacolorname"));
        assert_eq!(None, parse_color("redd"));

        assert_eq!(Some(Rgba::new(1.0, 0.0, 0.0, 1.0)), parse_color("#f00"));
        assert_eq!(Some(Rgba::new(1.0, 0.0, 0.0, 1.0)), parse_color("#f00f"));
        assert_eq!(Some(Rgba::new(1.0, 0.0, 0.0, 1.0)), parse_color("#ff0000"));
        assert_eq!(Some(Rgba::new(1.0, 0.0, 0.0, 1.0)), parse_color("#ff0000ff"));
        assert_eq!(Some(Rgba::new(1.0, 0.0, 0.0, 1.0)), parse_color("#FF0000FF"));
        assert_eq!(Some(Rgba::new(1.0, 0.0, 0.0, 1.0)), parse_color("red"));
        assert_eq!(Some(Rgba::new(1.0, 0.0, 0.0, 1.0)), parse_color("Red"));
        assert_eq!(Some(Rgba::new(1.0, 0.0, 0.0, 1.0)), parse_color("RED"));

        assert_eq!(0x00, SRgba::from("#000").r);
        assert_eq!(0x22, SRgba::from("#200").r);
        assert_eq!(0xFF, SRgba::from("#F00").r);
    }
}
