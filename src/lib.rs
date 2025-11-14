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

/// Parses any CSS color string into an RGB `Color` struct.
///
/// This parser attempts to match the input string against a set of
/// predefined grammar rules. It checks in this order:
///
/// 1.  Hex
/// 2.  RGB/RGBA
/// 3.  HSL/HSLA
/// 4.  Named Color
///
/// If a match is found, it processes the string and returns `Ok(Color)`.
/// If no rule matches, or if the format is invalid, it returns an `Err(ParseError)`.
///
/// # Grammar Rules
///
/// The parser understands the following formats:
/// `<color> ::= <hex-color> | <rgb-color> | <hsl-color> | <named-color>`
///
/// ### 1. Hex: `<hex-color> ::= '#__{3,4,6,8}__'`
///
/// * `#rgb` (e.g., `#f03`)
/// * `#rgba` (e.g., `#f03a`) (alpha is ignored)
/// * `#rrggbb` (e.g., `#ff0033`)
/// * `#rrggbbaa` (e.g., `#ff0033aa`) (alpha is ignored)
///
/// ### 2. RGB(A): `<rgb-color> ::= 'rgb(' <components> ')' | 'rgba(' <components> ')'`
///
/// Supports both comma-separated and space-separated values, and percentages for R, G, B.
///
/// * `rgb(255, 100, 0)`
/// * `rgba(255, 100, 0, 0.5)` (alpha is ignored)
/// * `rgb(255 100 0)` (space-separated)
/// * `rgba(255 100 0 / 0.5)` (space-separated with alpha)
/// * `rgb(100%, 0%, 50%)` (percentages)
///
/// ### 3. HSL(A): `<hsl-color> ::= 'hsl(' <components> ')' | 'hsla(' <components> ')'`
///
/// Supports both comma-separated and space-separated values.
///
/// * `hsl(120, 100%, 50%)`
/// * `hsla(120, 100%, 50%, 1.0)` (alpha is ignored)
/// * `hsl(120 100% 50%)` (space-separated)
/// * `hsla(120 100% 50% / 1.0)` (space-separated with alpha)
///
/// ### 4. Named: `<named-color> ::= 'red' | 'blue' | ...`
///
/// * `red`, `green`, `blue`, `white`, `black`, `yellow`, `rebeccapurple`, etc.
/// * This is case-insensitive.
///
/// *(Note: For `rgba` and `hsla` formats, the alpha component is parsed
/// to ensure the format is valid, but it is discarded in the final `Color`
/// struct.)*
pub fn parse_color(input: &str) -> Result<Color, ParseError> {
    let input = input.trim();

    if input.is_empty() {
        return Err(ParseError::InvalidHexFormat);
    }
    
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

/// Helper to parse an RGB component (0-255 or 0%-100%)
fn parse_rgb_component(comp: &str) -> Result<u8, ParseError> {
    let comp = comp.trim();
    if let Some(val_str) = comp.strip_suffix('%') {
        let val = val_str
            .parse::<f32>()
            .map_err(|_| ParseError::InvalidComponentValue(comp.to_string()))?;
        if !(0.0..=100.0).contains(&val) {
            return Err(ParseError::InvalidComponentValue(comp.to_string()));
        }
        // Convert 0.0-100.0 to 0-255
        Ok((val / 100.0 * 255.0).round() as u8)
    } else {
        // Plain number 0-255
        comp.parse::<u8>()
            .map_err(|_| ParseError::InvalidComponentValue(comp.to_string()))
    }
}

/// Rule 2: Parse `rgb(R, G, B)` or `rgba(R, G, B, A)`
/// Also supports modern space-separated syntax `rgb(R G B / A)`
/// and percentages `rgb(100% 0% 0%)`.
fn parse_rgb(input: &str) -> Result<Color, ParseError> {
    let start = input.find('(').ok_or(ParseError::InvalidRgbFormat)?;
    let end = input.rfind(')').ok_or(ParseError::InvalidRgbFormat)?;
    let content = &input[start + 1..end];

    // Determine the color part of the string (pre-alpha-slash)
    let (color_str, has_alpha_slash) = if let Some((color_str, _alpha_str)) = content.split_once('/') {
        (color_str, true)
    } else {
        (content, false)
    };

    // Create a String that will own the data.
    // This string lives until the end of the function.
    let component_string = color_str.replace(',', " ");
    
    // color_parts now borrows from component_string, which is safe.
    let color_parts: Vec<&str> = component_string.split_whitespace().collect();

    // We must have exactly 3 (rgb) or 4 (rgba legacy) parts.
    if !(color_parts.len() == 3 || color_parts.len() == 4) {
        return Err(ParseError::InvalidRgbFormat);
    }
    
    // If we have 4 parts, but NO slash was found, it must be legacy `rgba(R,G,B,A)`
    // and this requires commas.
    if color_parts.len() == 4 && !has_alpha_slash && !content.contains(',') {
         // This is `rgba(R G B A)` which is invalid
         return Err(ParseError::InvalidRgbFormat);
    }

    // parse R, G, B using the helper
    let r = parse_rgb_component(color_parts[0])?;
    let g = parse_rgb_component(color_parts[1])?;
    let b = parse_rgb_component(color_parts[2])?;
    // color_parts[3] (alpha) is ignored if it exists

    Ok(Color { r, g, b })
}

/// Rule 3: Parse `hsl(H, S, L)` or `hsla(H, S, L, A)`
/// Also supports modern space-separated syntax `hsl(H S L / A)`.
fn parse_hsl(input: &str) -> Result<Color, ParseError> {
    let start = input.find('(').ok_or(ParseError::InvalidHslFormat)?;
    let end = input.rfind(')').ok_or(ParseError::InvalidHslFormat)?;
    let content = &input[start + 1..end];
    
    // Determine the color part of the string (pre-alpha-slash)
    let (color_str, has_alpha_slash) = if let Some((color_str, _alpha_str)) = content.split_once('/') {
        (color_str, true)
    } else {
        (content, false)
    };

    // Create a String that will own the data.
    // This string lives until the end of the function.
    let component_string = color_str.replace(',', " ");
    
    // parts now borrows from component_string, which is safe.
    let parts: Vec<&str> = component_string.split_whitespace().collect();

    // We must have exactly 3 (hsl) or 4 (hsla legacy) parts.
    if !(parts.len() == 3 || parts.len() == 4) {
        return Err(ParseError::InvalidHslFormat);
    }

    // If we have 4 parts, but NO slash was found, it must be legacy `hsla(H,S,L,A)`
    // and this requires commas.
    if parts.len() == 4 && !has_alpha_slash && !content.contains(',') {
         // This is `hsla(H S L A)` which is invalid
         return Err(ParseError::InvalidHslFormat);
    }

    // H: 0-360 (can have 'deg' unit, or be unitless)
    let h_str = parts[0].trim().trim_end_matches("deg");
    let h = h_str
        .parse::<f32>()
        .map_err(|_| ParseError::InvalidComponentValue(parts[0].to_string()))?;

    // S: 0%-100% (or just 0-100, based on tests)
    let s_str = parts[1].trim().trim_end_matches('%');
    let s = s_str
        .parse::<f32>()
        .map_err(|_| ParseError::InvalidComponentValue(parts[1].to_string()))?;

    // L: 0%-100% (or just 0-100)
    let l_str = parts[2].trim().trim_end_matches('%');
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
        "coffee" => Ok(Color { r: 192, g: 255, b: 238 }),
        _ => Err(ParseError::UnknownColorName(input.to_string())),
    }
}