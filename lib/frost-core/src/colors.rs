use core::fmt;

pub enum Color
{
    RGB     { r: f32, g: f32, b: f32 },
    HSV     { h: f32, s: f32, v: f32 },
    HSL     { h: f32, s: f32, l: f32} ,
    CMYK    { c: f32, m: f32, y: f32, k: f32 },

    RGBA    { r: f32, g: f32, b: f32, a: f32 },
    HSVA    { h: f32, s: f32, v: f32, a: f32 },
    HSLA    { h: f32, s: f32, l: f32, a: f32 },
    CMYKA   { c: f32, m: f32, y: f32, k: f32, a: f32 },
}

/// Represents 32bit color (8 bits per chanel color)
pub struct RGBA8
{
    pub r : u8,
    pub g : u8,
    pub b : u8,
    pub a : u8
}
impl RGBA8 {
    /// Instantiate new 32 bit color from color rgba values in range [0, 255]
    pub fn new(r: u8, g: u8, b: u8, a: u8) -> Self
    {
        return Self {r, g, b, a};
    }
    /// Instantiate new 32 bit color from color rgba values in range [0.0, 1.0]
    pub fn from_rgba(r: f32, g: f32, b: f32, a: f32) -> Self
    {
        return Self { r: to_u8(r), g: to_u8(g), b: to_u8(b), a: to_u8(a)};
    }
    /// Instantiate new 32 bit color its hue, saturation, value, and alpha
    pub fn from_hsva(h: f32, s: f32, v: f32, a: f32) -> Self
    {
        let (r, g, b, a) = hsva_to_rgba(h, s, v, a);
        return Self::from_rgba(r, g, b, a);
    }
    /// Instantiate new 32 bit color its hue, saturation, lightness, and alpha
    pub fn from_hsla(h: f32, s: f32, l: f32, a: f32) -> Self
    {
        let (r, g, b, a) = hsla_to_rgba(h, s, l, a);
        return Self::from_rgba(r, g, b, a);
    }
    /// Instantiate new 32 bit color its cyan, magenta, yellow, key, and alpha 
    pub fn from_cmyka(c: f32, m: f32, y: f32, k: f32, a: f32) -> Self
    {
        let (r, g, b, a) = cmyka_to_rgba(c, m, y, k, a);
        return Self::from_rgba(r, g, b, a);
    }
}
impl TryFrom<&Color> for RGBA8 {
    type Error = &'static str;
    fn try_from(value: &Color) -> Result<Self, Self::Error>
    {
        match value
        {
            Color::RGB   { r, g, b }       => return Ok(Self::from_rgba(*r, *g, *b, 1.0f32)),
            Color::HSV   { h, s, v }       => return Ok(Self::from_hsva(*h, *s, *v, 1.0f32)),
            Color::HSL   { h, s, l }       => return Ok(Self::from_hsva(*h, *s, *l, 1.0f32)),
            Color::CMYK  { c, m, y, k }    => return Ok(Self::from_cmyka(*c, *m, *y, *k, 1.0f32)),
            Color::RGBA  { r, g, b, a }    => return Ok(Self::from_rgba(*r, *g, *b, *a)),
            Color::HSVA  { h, s, v, a }    => return Ok(Self::from_hsva(*h, *s, *v, *a)),
            Color::HSLA  { h, s, l, a }    => return Ok(Self::from_hsva(*h, *s, *l, *a)),
            Color::CMYKA { c, m, y, k, a } => return Ok(Self::from_cmyka(*c, *m, *y, *k, *a)),
            _ => Err("RGBA8.TryFrom<Color> Error - Bad color value!")
        }
    }
}
impl fmt::Display for RGBA8
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
    {
        return write!(f, "[r: {}, g: {}, b: {}, a: {}]", self.r, self.g, self.b, self.a);
    }
}


/// Represents HDR color (32 bits per chanel color)
pub struct RGBA32
{
    pub r : f32,
    pub g : f32,
    pub b : f32,
    pub a : f32
}
impl RGBA32 {
    /// Instantiate new 32 bit color from color rgba values in range [0, 255]
    pub fn new(r: u8, g: u8, b: u8, a: u8) -> Self
    {
        return Self {
            r : r as f32 * (1.0f32 / 255.0f32), 
            g : g as f32 * (1.0f32 / 255.0f32),
            b : b as f32 * (1.0f32 / 255.0f32), 
            a : a as f32 * (1.0f32 / 255.0f32)
        };
    }
    /// Instantiate new 32 bit color from color rgba values in range [0.0, 1.0]
    pub fn from_rgba(r: f32, g: f32, b: f32, a: f32) -> Self
    {
        return Self { r: r, g: g, b: b, a: a};
    }
    /// Instantiate new 32 bit color its hue, saturation, value, and alpha
    pub fn from_hsva(h: f32, s: f32, v: f32, a: f32) -> Self
    {
        let (r, g, b, a) = hsva_to_rgba(h, s, v, a);
        return Self::from_rgba(r, g, b, a);
    }
    /// Instantiate new 32 bit color its hue, saturation, lightness, and alpha
    pub fn from_hsla(h: f32, s: f32, l: f32, a: f32) -> Self
    {
        let (r, g, b, a) = hsla_to_rgba(h, s, l, a);
        return Self::from_rgba(r, g, b, a);
    }
    /// Instantiate new 32 bit color its cyan, magenta, yellow, key, and alpha 
    pub fn from_cmyka(c: f32, m: f32, y: f32, k: f32, a: f32) -> Self
    {
        let (r, g, b, a) = cmyka_to_rgba(c, m, y, k, a);
        return Self::from_rgba(r, g, b, a);
    }
}
impl TryFrom<&Color> for RGBA32 {
    type Error = &'static str;
    fn try_from(value: &Color) -> Result<Self, Self::Error>
    {
        match value
        {
            Color::RGB   { r, g, b }       => return Ok(Self::from_rgba(*r, *g, *b, 1.0f32)),
            Color::HSV   { h, s, v }       => return Ok(Self::from_hsva(*h, *s, *v, 1.0f32)),
            Color::HSL   { h, s, l }       => return Ok(Self::from_hsva(*h, *s, *l, 1.0f32)),
            Color::CMYK  { c, m, y, k }    => return Ok(Self::from_cmyka(*c, *m, *y, *k, 1.0f32)),
            Color::RGBA  { r, g, b, a }    => return Ok(Self::from_rgba(*r, *g, *b, *a)),
            Color::HSVA  { h, s, v, a }    => return Ok(Self::from_hsva(*h, *s, *v, *a)),
            Color::HSLA  { h, s, l, a }    => return Ok(Self::from_hsva(*h, *s, *l, *a)),
            Color::CMYKA { c, m, y, k, a } => return Ok(Self::from_cmyka(*c, *m, *y, *k, *a)),
            _ => Err("RGBA8.TryFrom<Color> Error - Bad color value!")
        }
    }
}
impl fmt::Display for RGBA32
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
    {
        return write!(f, "[r: {}, g: {}, b: {}, a: {}]", self.r, self.g, self.b, self.a);
    }
}

const D60 : f32 = 1.0f32 / 6.0f32;
#[inline(always)]
fn to_u8(val: f32) -> u8 { return (val * 255.0) as u8; }
//#[inline(always)]
//fn to_f32(val: u8) -> f32 { return (val as f32) / 255.0; }
#[inline(always)]
fn hsva_to_rgba(h: f32, s: f32, v: f32, a: f32) -> (f32, f32, f32, f32) {
    let hue : f32 = (h % 1.0f32 + 1.0f32) % 1.0f32;
    let c   : f32 = v * s;
    let x   : f32 = c * (1.0f32 - (1.0f32 - ((hue / D60) % 2.0f32).abs() - 1.0f32));
    let m   : f32 = v - c;

         if hue >= 0.0 * D60 && hue < 1.0 * D60 { return (c + m,   x + m,   0.0 + m, a); }
    else if hue >= 1.0 * D60 && hue < 2.0 * D60 { return (x + m,   c + m,   0.0 + m, a); }
    else if hue >= 2.0 * D60 && hue < 3.0 * D60 { return (0.0 + m, c + m,   x + m,   a); }
    else if hue >= 3.0 * D60 && hue < 4.0 * D60 { return (0.0 + m, x + m,   c + m,   a); }
    else if hue >= 4.0 * D60 && hue < 5.0 * D60 { return (x + m,   0.0 + m, c + m,   a); }
    else                                        { return (c + m,   0.0 + m, x + m,   a); }
}
#[inline(always)]
fn hsla_to_rgba(h: f32, s: f32, l: f32, a: f32) -> (f32, f32, f32, f32) {
    let hue : f32 = (h % 1.0f32 + 1.0f32) % 1.0f32;
    let c   : f32 = (1.0f32 - (l * 2.0f32 - 1.0f32).abs()) * s;
    let x   : f32 = c * (1.0f32 - (1.0f32 - ((hue / D60) % 2.032) - 1.0f32).abs());
    let m   : f32 = l - c * 0.5f32;
    
         if hue >= 0.0 * D60 && hue < 1.0 * D60 { return (c + m,   x + m,   0.0 + m, a); }
    else if hue >= 1.0 * D60 && hue < 2.0 * D60 { return (x + m,   c + m,   0.0 + m, a); }
    else if hue >= 2.0 * D60 && hue < 3.0 * D60 { return (0.0 + m, c + m,   x + m,   a); }
    else if hue >= 3.0 * D60 && hue < 4.0 * D60 { return (0.0 + m, x + m,   c + m,   a); }
    else if hue >= 4.0 * D60 && hue < 5.0 * D60 { return (x + m,   0.0 + m, c + m,   a); }
    else                                        { return (c + m,   0.0 + m, x + m,   a); }
}
#[inline(always)]
fn cmyka_to_rgba(c: f32, m: f32, y: f32, k: f32, a: f32) -> (f32, f32, f32, f32) {
    return ((1.0f32 - c) * (1.0f32 - k), (1.0f32 - m) * (1.0f32 - k), (1.0f32 - y) * (1.0f32 - k), a);
}
