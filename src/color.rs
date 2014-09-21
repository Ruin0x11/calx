#[deriving(Clone, PartialEq, Eq, Show)]
pub struct Rgb { pub r: u8, pub g: u8, pub b: u8 }

impl Rgb {
    pub fn new(r: u8, g: u8, b: u8) -> Rgb {
        Rgb { r: r, g: g, b: b }
    }

    pub fn to_array(&self) -> [f32, ..4] {
        [self.r as f32 / 255.0,
         self.g as f32 / 255.0,
         self.b as f32 / 255.0,
         1.0]
    }
}

// From https://github.com/bjz/color-rs
pub static ALICEBLUE:               Rgb = Rgb { r: 0xF0, g: 0xF8, b: 0xFF };
pub static ANTIQUEWHITE:            Rgb = Rgb { r: 0xFA, g: 0xEB, b: 0xD7 };
pub static AQUA:                    Rgb = Rgb { r: 0x00, g: 0xFF, b: 0xFF };
pub static AQUAMARINE:              Rgb = Rgb { r: 0x7F, g: 0xFF, b: 0xD4 };
pub static AZURE:                   Rgb = Rgb { r: 0xF0, g: 0xFF, b: 0xFF };
pub static BEIGE:                   Rgb = Rgb { r: 0xF5, g: 0xF5, b: 0xDC };
pub static BISQUE:                  Rgb = Rgb { r: 0xFF, g: 0xE4, b: 0xC4 };
pub static BLACK:                   Rgb = Rgb { r: 0x00, g: 0x00, b: 0x00 };
pub static BLANCHEDALMOND:          Rgb = Rgb { r: 0xFF, g: 0xEB, b: 0xCD };
pub static BLUE:                    Rgb = Rgb { r: 0x00, g: 0x00, b: 0xFF };
pub static BLUEVIOLET:              Rgb = Rgb { r: 0x8A, g: 0x2B, b: 0xE2 };
pub static BROWN:                   Rgb = Rgb { r: 0xA5, g: 0x2A, b: 0x2A };
pub static BURLYWOOD:               Rgb = Rgb { r: 0xDE, g: 0xB8, b: 0x87 };
pub static CADETBLUE:               Rgb = Rgb { r: 0x5F, g: 0x9E, b: 0xA0 };
pub static CHARTREUSE:              Rgb = Rgb { r: 0x7F, g: 0xFF, b: 0x00 };
pub static CHOCOLATE:               Rgb = Rgb { r: 0xD2, g: 0x69, b: 0x1E };
pub static CORAL:                   Rgb = Rgb { r: 0xFF, g: 0x7F, b: 0x50 };
pub static CORNFLOWERBLUE:          Rgb = Rgb { r: 0x64, g: 0x95, b: 0xED };
pub static CORNSILK:                Rgb = Rgb { r: 0xFF, g: 0xF8, b: 0xDC };
pub static CRIMSON:                 Rgb = Rgb { r: 0xDC, g: 0x14, b: 0x3C };
pub static CYAN:                    Rgb = Rgb { r: 0x00, g: 0xFF, b: 0xFF };
pub static DARKBLUE:                Rgb = Rgb { r: 0x00, g: 0x00, b: 0x8B };
pub static DARKCYAN:                Rgb = Rgb { r: 0x00, g: 0x8B, b: 0x8B };
pub static DARKGOLDENROD:           Rgb = Rgb { r: 0xB8, g: 0x86, b: 0x0B };
pub static DARKGRAY:                Rgb = Rgb { r: 0xA9, g: 0xA9, b: 0xA9 };
pub static DARKGREEN:               Rgb = Rgb { r: 0x00, g: 0x64, b: 0x00 };
pub static DARKKHAKI:               Rgb = Rgb { r: 0xBD, g: 0xB7, b: 0x6B };
pub static DARKMAGENTA:             Rgb = Rgb { r: 0x8B, g: 0x00, b: 0x8B };
pub static DARKOLIVEGREEN:          Rgb = Rgb { r: 0x55, g: 0x6B, b: 0x2F };
pub static DARKORANGE:              Rgb = Rgb { r: 0xFF, g: 0x8C, b: 0x00 };
pub static DARKORCHID:              Rgb = Rgb { r: 0x99, g: 0x32, b: 0xCC };
pub static DARKRED:                 Rgb = Rgb { r: 0x8B, g: 0x00, b: 0x00 };
pub static DARKSALMON:              Rgb = Rgb { r: 0xE9, g: 0x96, b: 0x7A };
pub static DARKSEAGREEN:            Rgb = Rgb { r: 0x8F, g: 0xBC, b: 0x8F };
pub static DARKSLATEBLUE:           Rgb = Rgb { r: 0x48, g: 0x3D, b: 0x8B };
pub static DARKSLATEGRAY:           Rgb = Rgb { r: 0x2F, g: 0x4F, b: 0x4F };
pub static DARKTURQUOISE:           Rgb = Rgb { r: 0x00, g: 0xCE, b: 0xD1 };
pub static DARKVIOLET:              Rgb = Rgb { r: 0x94, g: 0x00, b: 0xD3 };
pub static DEEPPINK:                Rgb = Rgb { r: 0xFF, g: 0x14, b: 0x93 };
pub static DEEPSKYBLUE:             Rgb = Rgb { r: 0x00, g: 0xBF, b: 0xFF };
pub static DIMGRAY:                 Rgb = Rgb { r: 0x69, g: 0x69, b: 0x69 };
pub static DODGERBLUE:              Rgb = Rgb { r: 0x1E, g: 0x90, b: 0xFF };
pub static FIREBRICK:               Rgb = Rgb { r: 0xB2, g: 0x22, b: 0x22 };
pub static FLORALWHITE:             Rgb = Rgb { r: 0xFF, g: 0xFA, b: 0xF0 };
pub static FORESTGREEN:             Rgb = Rgb { r: 0x22, g: 0x8B, b: 0x22 };
pub static FUCHSIA:                 Rgb = Rgb { r: 0xFF, g: 0x00, b: 0xFF };
pub static GAINSBORO:               Rgb = Rgb { r: 0xDC, g: 0xDC, b: 0xDC };
pub static GHOSTWHITE:              Rgb = Rgb { r: 0xF8, g: 0xF8, b: 0xFF };
pub static GOLD:                    Rgb = Rgb { r: 0xFF, g: 0xD7, b: 0x00 };
pub static GOLDENROD:               Rgb = Rgb { r: 0xDA, g: 0xA5, b: 0x20 };
pub static GRAY:                    Rgb = Rgb { r: 0x80, g: 0x80, b: 0x80 };
pub static GREEN:                   Rgb = Rgb { r: 0x00, g: 0x80, b: 0x00 };
pub static GREENYELLOW:             Rgb = Rgb { r: 0xAD, g: 0xFF, b: 0x2F };
pub static HONEYDEW:                Rgb = Rgb { r: 0xF0, g: 0xFF, b: 0xF0 };
pub static HOTPINK:                 Rgb = Rgb { r: 0xFF, g: 0x69, b: 0xB4 };
pub static INDIANRED:               Rgb = Rgb { r: 0xCD, g: 0x5C, b: 0x5C };
pub static INDIGO:                  Rgb = Rgb { r: 0x4B, g: 0x00, b: 0x82 };
pub static IVORY:                   Rgb = Rgb { r: 0xFF, g: 0xFF, b: 0xF0 };
pub static KHAKI:                   Rgb = Rgb { r: 0xF0, g: 0xE6, b: 0x8C };
pub static LAVENDER:                Rgb = Rgb { r: 0xE6, g: 0xE6, b: 0xFA };
pub static LAVENDERBLUSH:           Rgb = Rgb { r: 0xFF, g: 0xF0, b: 0xF5 };
pub static LAWNGREEN:               Rgb = Rgb { r: 0x7C, g: 0xFC, b: 0x00 };
pub static LEMONCHIFFON:            Rgb = Rgb { r: 0xFF, g: 0xFA, b: 0xCD };
pub static LIGHTBLUE:               Rgb = Rgb { r: 0xAD, g: 0xD8, b: 0xE6 };
pub static LIGHTCORAL:              Rgb = Rgb { r: 0xF0, g: 0x80, b: 0x80 };
pub static LIGHTCYAN:               Rgb = Rgb { r: 0xE0, g: 0xFF, b: 0xFF };
pub static LIGHTGOLDENRODYELLOW:    Rgb = Rgb { r: 0xFA, g: 0xFA, b: 0xD2 };
pub static LIGHTGREEN:              Rgb = Rgb { r: 0x90, g: 0xEE, b: 0x90 };
pub static LIGHTGREY:               Rgb = Rgb { r: 0xD3, g: 0xD3, b: 0xD3 };
pub static LIGHTPINK:               Rgb = Rgb { r: 0xFF, g: 0xB6, b: 0xC1 };
pub static LIGHTSALMON:             Rgb = Rgb { r: 0xFF, g: 0xA0, b: 0x7A };
pub static LIGHTSEAGREEN:           Rgb = Rgb { r: 0x20, g: 0xB2, b: 0xAA };
pub static LIGHTSKYBLUE:            Rgb = Rgb { r: 0x87, g: 0xCE, b: 0xFA };
pub static LIGHTSLATEGRAY:          Rgb = Rgb { r: 0x77, g: 0x88, b: 0x99 };
pub static LIGHTSTEELBLUE:          Rgb = Rgb { r: 0xB0, g: 0xC4, b: 0xDE };
pub static LIGHTYELLOW:             Rgb = Rgb { r: 0xFF, g: 0xFF, b: 0xE0 };
pub static LIME:                    Rgb = Rgb { r: 0x00, g: 0xFF, b: 0x00 };
pub static LIMEGREEN:               Rgb = Rgb { r: 0x32, g: 0xCD, b: 0x32 };
pub static LINEN:                   Rgb = Rgb { r: 0xFA, g: 0xF0, b: 0xE6 };
pub static MAGENTA:                 Rgb = Rgb { r: 0xFF, g: 0x00, b: 0xFF };
pub static MAROON:                  Rgb = Rgb { r: 0x80, g: 0x00, b: 0x00 };
pub static MEDIUMAQUAMARINE:        Rgb = Rgb { r: 0x66, g: 0xCD, b: 0xAA };
pub static MEDIUMBLUE:              Rgb = Rgb { r: 0x00, g: 0x00, b: 0xCD };
pub static MEDIUMORCHID:            Rgb = Rgb { r: 0xBA, g: 0x55, b: 0xD3 };
pub static MEDIUMPURPLE:            Rgb = Rgb { r: 0x93, g: 0x70, b: 0xDB };
pub static MEDIUMSEAGREEN:          Rgb = Rgb { r: 0x3C, g: 0xB3, b: 0x71 };
pub static MEDIUMSLATEBLUE:         Rgb = Rgb { r: 0x7B, g: 0x68, b: 0xEE };
pub static MEDIUMSPRINGGREEN:       Rgb = Rgb { r: 0x00, g: 0xFA, b: 0x9A };
pub static MEDIUMTURQUOISE:         Rgb = Rgb { r: 0x48, g: 0xD1, b: 0xCC };
pub static MEDIUMVIOLETRED:         Rgb = Rgb { r: 0xC7, g: 0x15, b: 0x85 };
pub static MIDNIGHTBLUE:            Rgb = Rgb { r: 0x19, g: 0x19, b: 0x70 };
pub static MINTCREAM:               Rgb = Rgb { r: 0xF5, g: 0xFF, b: 0xFA };
pub static MISTYROSE:               Rgb = Rgb { r: 0xFF, g: 0xE4, b: 0xE1 };
pub static MOCCASIN:                Rgb = Rgb { r: 0xFF, g: 0xE4, b: 0xB5 };
pub static NAVAJOWHITE:             Rgb = Rgb { r: 0xFF, g: 0xDE, b: 0xAD };
pub static NAVY:                    Rgb = Rgb { r: 0x00, g: 0x00, b: 0x80 };
pub static OLDLACE:                 Rgb = Rgb { r: 0xFD, g: 0xF5, b: 0xE6 };
pub static OLIVE:                   Rgb = Rgb { r: 0x80, g: 0x80, b: 0x00 };
pub static OLIVEDRAB:               Rgb = Rgb { r: 0x6B, g: 0x8E, b: 0x23 };
pub static ORANGE:                  Rgb = Rgb { r: 0xFF, g: 0xA5, b: 0x00 };
pub static ORANGERED:               Rgb = Rgb { r: 0xFF, g: 0x45, b: 0x00 };
pub static ORCHID:                  Rgb = Rgb { r: 0xDA, g: 0x70, b: 0xD6 };
pub static PALEGOLDENROD:           Rgb = Rgb { r: 0xEE, g: 0xE8, b: 0xAA };
pub static PALEGREEN:               Rgb = Rgb { r: 0x98, g: 0xFB, b: 0x98 };
pub static PALEVIOLETRED:           Rgb = Rgb { r: 0xDB, g: 0x70, b: 0x93 };
pub static PAPAYAWHIP:              Rgb = Rgb { r: 0xFF, g: 0xEF, b: 0xD5 };
pub static PEACHPUFF:               Rgb = Rgb { r: 0xFF, g: 0xDA, b: 0xB9 };
pub static PERU:                    Rgb = Rgb { r: 0xCD, g: 0x85, b: 0x3F };
pub static PINK:                    Rgb = Rgb { r: 0xFF, g: 0xC0, b: 0xCB };
pub static PLUM:                    Rgb = Rgb { r: 0xDD, g: 0xA0, b: 0xDD };
pub static POWDERBLUE:              Rgb = Rgb { r: 0xB0, g: 0xE0, b: 0xE6 };
pub static PURPLE:                  Rgb = Rgb { r: 0x80, g: 0x00, b: 0x80 };
pub static RED:                     Rgb = Rgb { r: 0xFF, g: 0x00, b: 0x00 };
pub static ROSYBROWN:               Rgb = Rgb { r: 0xBC, g: 0x8F, b: 0x8F };
pub static ROYALBLUE:               Rgb = Rgb { r: 0x41, g: 0x69, b: 0xE1 };
pub static SADDLEBROWN:             Rgb = Rgb { r: 0x8B, g: 0x45, b: 0x13 };
pub static SALMON:                  Rgb = Rgb { r: 0xFA, g: 0x80, b: 0x72 };
pub static SANDYBROWN:              Rgb = Rgb { r: 0xFA, g: 0xA4, b: 0x60 };
pub static SEAGREEN:                Rgb = Rgb { r: 0x2E, g: 0x8B, b: 0x57 };
pub static SEASHELL:                Rgb = Rgb { r: 0xFF, g: 0xF5, b: 0xEE };
pub static SIENNA:                  Rgb = Rgb { r: 0xA0, g: 0x52, b: 0x2D };
pub static SILVER:                  Rgb = Rgb { r: 0xC0, g: 0xC0, b: 0xC0 };
pub static SKYBLUE:                 Rgb = Rgb { r: 0x87, g: 0xCE, b: 0xEB };
pub static SLATEBLUE:               Rgb = Rgb { r: 0x6A, g: 0x5A, b: 0xCD };
pub static SLATEGRAY:               Rgb = Rgb { r: 0x70, g: 0x80, b: 0x90 };
pub static SNOW:                    Rgb = Rgb { r: 0xFF, g: 0xFA, b: 0xFA };
pub static SPRINGGREEN:             Rgb = Rgb { r: 0x00, g: 0xFF, b: 0x7F };
pub static STEELBLUE:               Rgb = Rgb { r: 0x46, g: 0x82, b: 0xB4 };
pub static TAN:                     Rgb = Rgb { r: 0xD2, g: 0xB4, b: 0x8C };
pub static TEAL:                    Rgb = Rgb { r: 0x00, g: 0x80, b: 0x80 };
pub static THISTLE:                 Rgb = Rgb { r: 0xD8, g: 0xBF, b: 0xD8 };
pub static TOMATO:                  Rgb = Rgb { r: 0xFF, g: 0x63, b: 0x47 };
pub static TURQUOISE:               Rgb = Rgb { r: 0x40, g: 0xE0, b: 0xD0 };
pub static VIOLET:                  Rgb = Rgb { r: 0xEE, g: 0x82, b: 0xEE };
pub static WHEAT:                   Rgb = Rgb { r: 0xF5, g: 0xDE, b: 0xB3 };
pub static WHITE:                   Rgb = Rgb { r: 0xFF, g: 0xFF, b: 0xFF };
pub static WHITESMOKE:              Rgb = Rgb { r: 0xF5, g: 0xF5, b: 0xF5 };
pub static YELLOW:                  Rgb = Rgb { r: 0xFF, g: 0xFF, b: 0x00 };
pub static YELLOWGREEN:             Rgb = Rgb { r: 0x9A, g: 0xCD, b: 0x32 };
