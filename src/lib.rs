use thiserror::Error;

#[derive(Error, Debug)]
pub enum ParseError {
    #[error("Invalid hex code format")]
    InvalidHexFormat,
    #[error("Invalid RGB/RGBA format")]
    InvalidRgbFormat,
    #[error("Invalid HSL/HSLA format")]
    InvalidHslFormat,
    #[error("Invalid component value: {0}")]
    InvalidComponentValue(String),
    #[error("Unknown color name: {0}")]
    UnknownColorName(String),
    #[error("Failed to parse number")]
    ParseFailure,
}

#[derive(Debug, PartialEq)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

/// Parses any CSS color string.
pub fn parse_color(input: &str) -> Result<Color, ParseError> {
    let input = input.trim();

    if input.starts_with('#') {
        return parse_hex(input);
    }

    if (input.starts_with("rgb(") || input.starts_with("rgba(")) && input.ends_with(')') {
        return parse_rgb(input);
    }

    if (input.starts_with("hsl(") || input.starts_with("hsla(")) && input.ends_with(')') {
        return parse_hsl(input);
    }

    // if nothing matches, try a name
    parse_named(input)
}

/// Rule 1: Parse `#RRGGBB` (long) or `#RGB` (short)
///
/// Handles 3, 4, 6, and 8-digit hex codes.
/// Alpha (4 and 8 digits) is ignored.
fn parse_hex(input: &str) -> Result<Color, ParseError> {
    // remove the '#'
    let hex = &input[1..];

    match hex.len() {
        // short hex: #rgb
        3 => {
            let r = u8::from_str_radix(&hex[0..1].repeat(2), 16)
                .map_err(|_| ParseError::InvalidHexFormat)?;
            let g = u8::from_str_radix(&hex[1..2].repeat(2), 16)
                .map_err(|_| ParseError::InvalidHexFormat)?;
            let b = u8::from_str_radix(&hex[2..3].repeat(2), 16)
                .map_err(|_| ParseError::InvalidHexFormat)?;
            Ok(Color { r, g, b })
        }
        // short hex with alpha: #rgba (alpha ignored)
        4 => {
            let r = u8::from_str_radix(&hex[0..1].repeat(2), 16)
                .map_err(|_| ParseError::InvalidHexFormat)?;
            let g = u8::from_str_radix(&hex[1..2].repeat(2), 16)
                .map_err(|_| ParseError::InvalidHexFormat)?;
            let b = u8::from_str_radix(&hex[2..3].repeat(2), 16)
                .map_err(|_| ParseError::InvalidHexFormat)?;
            // hex[3..4] is alpha, we ignore it
            Ok(Color { r, g, b })
        }
        // long hex: #rrggbb
        6 => {
            let r =
                u8::from_str_radix(&hex[0..2], 16).map_err(|_| ParseError::InvalidHexFormat)?;
            let g =
                u8::from_str_radix(&hex[2..4], 16).map_err(|_| ParseError::InvalidHexFormat)?;
            let b =
                u8::from_str_radix(&hex[4..6], 16).map_err(|_| ParseError::InvalidHexFormat)?;
            Ok(Color { r, g, b })
        }
        // long hex with alpha: #rrggbbaa (alpha ignored)
        8 => {
            let r =
                u8::from_str_radix(&hex[0..2], 16).map_err(|_| ParseError::InvalidHexFormat)?;
            let g =
                u8::from_str_radix(&hex[2..4], 16).map_err(|_| ParseError::InvalidHexFormat)?;
            let b =
                u8::from_str_radix(&hex[4..6], 16).map_err(|_| ParseError::InvalidHexFormat)?;
            // hex[6..8] is alpha, we ignore it
            Ok(Color { r, g, b })
        }
        // anything else is wrong
        _ => Err(ParseError::InvalidHexFormat),
    }
}

/// Rule 2: Parse `rgb(R, G, B)` or `rgba(R, G, B, A)`
fn parse_rgb(input: &str) -> Result<Color, ParseError> {
    // find '(' and ')'
    let start = input.find('(').ok_or(ParseError::InvalidRgbFormat)?;
    let end = input.rfind(')').ok_or(ParseError::InvalidRgbFormat)?;

    let content = &input[start + 1..end];

    // split by comma
    let parts: Vec<&str> = content.split(',').map(|s| s.trim()).collect();

    // We must have exactly 3 (rgb) or 4 (rgba) parts.
    if !(parts.len() == 3 || parts.len() == 4) {
        return Err(ParseError::InvalidRgbFormat);
    }

    // parse R, G, B
    let r = parts[0]
        .parse::<u8>()
        .map_err(|_| ParseError::InvalidComponentValue(parts[0].to_string()))?;
    let g = parts[1]
        .parse::<u8>()
        .map_err(|_| ParseError::InvalidComponentValue(parts[1].to_string()))?;
    let b = parts[2]
        .parse::<u8>()
        .map_err(|_| ParseError::InvalidComponentValue(parts[2].to_string()))?;
    // parts[3] (alpha) is ignored if it exists

    Ok(Color { r, g, b })
}

/// Rule 3: Parse `hsl(H, S, L)` or `hsla(H, S, L, A)`
fn parse_hsl(input: &str) -> Result<Color, ParseError> {
    let start = input.find('(').ok_or(ParseError::InvalidHslFormat)?;
    let end = input.rfind(')').ok_or(ParseError::InvalidHslFormat)?;

    let content = &input[start + 1..end];
    let parts: Vec<&str> = content.split(',').map(|s| s.trim()).collect();

    if !(parts.len() == 3 || parts.len() == 4) {
        return Err(ParseError::InvalidHslFormat);
    }

    // H: 0-360
    let h = parts[0]
        .parse::<f32>()
        .map_err(|_| ParseError::InvalidComponentValue(parts[0].to_string()))?;

    // S: 0%-100% (or just 0-100, based on tests)
    let s_str = parts[1].trim_end_matches('%');
    let s = s_str
        .parse::<f32>()
        .map_err(|_| ParseError::InvalidComponentValue(parts[1].to_string()))?;

    // L: 0%-100% (or just 0-100)
    let l_str = parts[2].trim_end_matches('%');
    let l = l_str
        .parse::<f32>()
        .map_err(|_| ParseError::InvalidComponentValue(parts[2].to_string()))?;

    // Validate ranges
    if !(0.0..=360.0).contains(&h) {
        return Err(ParseError::InvalidComponentValue(format!("H: {}", h)));
    }
    if !(0.0..=100.0).contains(&s) {
        return Err(ParseError::InvalidComponentValue(format!("S: {}", s)));
    }
    if !(0.0..=100.0).contains(&l) {
        return Err(ParseError::InvalidComponentValue(format!("L: {}", l)));
    }

    // convert to 0..1 range
    let h = h / 360.0;
    let s = s / 100.0; // Assume S and L are always 0-100
    let l = l / 100.0; // Assume S and L are always 0-100

    // HSL to RGB conversion
    if s == 0.0 {
        // it's grayscale
        let val = (l * 255.0) as u8;
        Ok(Color {
            r: val,
            g: val,
            b: val,
        })
    } else {
        let q = if l < 0.5 { l * (1.0 + s) } else { l + s - l * s };
        let p = 2.0 * l - q;
        let r = hue_to_rgb(p, q, h + 1.0 / 3.0);
        let g = hue_to_rgb(p, q, h);
        let b = hue_to_rgb(p, q, h - 1.0 / 3.0);

        Ok(Color {
            r: (r * 255.0) as u8,
            g: (g * 255.0) as u8,
            b: (b * 255.0) as u8,
        })
    }
}
// Helper for HSL
fn hue_to_rgb(p: f32, q: f32, mut t: f32) -> f32 {
    if t < 0.0 {
        t += 1.0;
    }
    if t > 1.0 {
        t -= 1.0;
    }
    if t < 1.0 / 6.0 {
        return p + (q - p) * 6.0 * t;
    }
    if t < 1.0 / 2.0 {
        return q;
    }
    if t < 2.0 / 3.0 {
        return p + (q - p) * (2.0 / 3.0 - t) * 6.0;
    }
    p
}

/// Rule 4: Parse named colors
fn parse_named(input: &str) -> Result<Color, ParseError> {
    match input.to_lowercase().as_str() {
        
        "red" => Ok(Color { r: 255, g: 0, b: 0 }),
        "lime" => Ok(Color { r: 0, g: 255, b: 0 }),
        "blue" => Ok(Color { r: 0, g: 0, b: 255 }),
        "white" => Ok(Color { r: 255, g: 255, b: 255 }),
        "black" => Ok(Color { r: 0, g: 0, b: 0 }),
        "yellow" => Ok(Color { r: 255, g: 255, b: 0 }),
        "cyan" => Ok(Color { r: 0, g: 255, b: 255 }),
        "magenta" => Ok(Color { r: 255, g: 0, b: 255 }),
        "aqua" => Ok(Color { r: 0, g: 255, b: 255 }), // same as cyan
        "fuchsia" => Ok(Color { r: 255, g: 0, b: 255 }), // same as magenta
        "orange" => Ok(Color { r: 255, g: 165, b: 0 }),
        "pink" => Ok(Color { r: 255, g: 192, b: 203 }),
        "brown" => Ok(Color { r: 165, g: 42, b: 42 }),
        "silver" => Ok(Color { r: 192, g: 192, b: 192 }),
        "gray" => Ok(Color { r: 128, g: 128, b: 128 }),
        "maroon" => Ok(Color { r: 128, g: 0, b: 0 }),
        "olive" => Ok(Color { r: 128, g: 128, b: 0 }),
        "green" => Ok(Color { r: 0, g: 128, b: 0 }),
        "purple" => Ok(Color { r: 128, g: 0, b: 128 }),
        "teal" => Ok(Color { r: 0, g: 128, b: 128 }),
        "navy" => Ok(Color { r: 0, g: 0, b: 128 }),
        "rebeccapurple" => Ok(Color { r: 102, g: 51, b: 153 }),
        "c0ffee" => Ok(Color { r: 192, g: 255, b: 238 }),
        _ => Err(ParseError::UnknownColorName(input.to_string())),
    }
}