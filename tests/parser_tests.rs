use colourss::{parse_color, Color};

#[test]
fn test_rule1_hex_short() {
    // #rgb
    assert_eq!(
        parse_color("#f00").unwrap(),
        Color { r: 255, g: 0, b: 0 }
    );
    // #rgba (alpha ignored)
    assert_eq!(
        parse_color("#0f08").unwrap(),
        Color { r: 0, g: 255, b: 0 }
    );
}

#[test]
fn test_rule1_hex_long() {
    // #rrggbb
    assert_eq!(
        parse_color("#FF0000").unwrap(),
        Color { r: 255, g: 0, b: 0 }
    );
    // #rrggbbaa (alpha ignored)
    assert_eq!(
        parse_color("#0000FFaa").unwrap(),
        Color { r: 0, g: 0, b: 255 }
    );
}

#[test]
fn test_rule1_hex_fail() {
    assert!(parse_color("#f0").is_err()); // too short
    assert!(parse_color("#ff00a").is_err()); // wrong length
    assert!(parse_color("#GGHHII").is_err()); // bad chars
    assert!(parse_color("f00").is_err()); // missing hash
}

#[test]
fn test_rule2_rgb() {
    // rgb
    assert_eq!(
        parse_color("rgb(255, 0, 0)").unwrap(),
        Color { r: 255, g: 0, b: 0 }
    );
    // rgba (alpha ignored)
    assert_eq!(
        parse_color("rgba(0, 128, 0, 0.5)").unwrap(),
        Color { r: 0, g: 128, b: 0 }
    );
    // with whitespace
    assert_eq!(
        parse_color("  rgb( 0 , 255 , 0 )  ").unwrap(),
        Color { r: 0, g: 255, b: 0 }
    );

    // New tests for space-separated and percentages
    assert_eq!(
        parse_color("rgb(255 0 0)").unwrap(), // spaces
        Color { r: 255, g: 0, b: 0 }
    );
    assert_eq!(
        parse_color("rgba(0 128 0 / 0.5)").unwrap(), // spaces + alpha
        Color { r: 0, g: 128, b: 0 }
    );
    assert_eq!(
        parse_color("rgb(100%, 0%, 0%)").unwrap(), // percentages
        Color { r: 255, g: 0, b: 0 }
    );
    assert_eq!(
        parse_color("rgb(0% 100% 0%)").unwrap(), // percentages + spaces
        Color { r: 0, g: 255, b: 0 }
    );
    assert_eq!(
        parse_color("rgba(0% 0% 100% / 1.0)").unwrap(), // percentages + spaces + alpha
        Color { r: 0, g: 0, b: 255 }
    );
}

#[test]
fn test_rule2_rgb_fail() {
    assert!(parse_color("rgb(255, 0)").is_err()); // too few parts
    assert!(parse_color("rgb(255, 0, 0, 1, 5)").is_err()); // too many parts
    assert!(parse_color("rgb(256, 0, 0)").is_err()); // bad number
    assert!(parse_color("rgb(255, 0, 0").is_err()); // missing paren
    assert!(parse_color("rgb(101%, 0%, 0%)").is_err()); // bad percentage
    assert!(parse_color("rgb(255 0 0 0.5)").is_err()); // 4 parts in rgb() without slash
}

#[test]
fn test_rule3_hsl() {
    // hsl with %
    assert_eq!(
        parse_color("hsl(120, 100%, 50%)").unwrap(), // green
        Color { r: 0, g: 255, b: 0 }
    );
    // hsla (alpha ignored)
    assert_eq!(
        parse_color("hsla(0, 100%, 50%, 1.0)").unwrap(), // red
        Color { r: 255, g: 0, b: 0 }
    );
    // hsl without % (should still work)
    assert_eq!(
        parse_color("hsl(240, 100, 50)").unwrap(), // blue
        Color { r: 0, g: 0, b: 255 }
    );
    // hsl black
    assert_eq!(
        parse_color("hsl(0, 0%, 0%)").unwrap(),
        Color { r: 0, g: 0, b: 0 }
    );

    // New tests for space-separated
    assert_eq!(
        parse_color("hsl(120 100% 50%)").unwrap(), // spaces
        Color { r: 0, g: 255, b: 0 }
    );
    assert_eq!(
        parse_color("hsla(0 100% 50% / 1.0)").unwrap(), // spaces + alpha
        Color { r: 255, g: 0, b: 0 }
    );
    assert_eq!(
        parse_color("hsl(240deg 100% 50%)").unwrap(), // 'deg' unit
        Color { r: 0, g: 0, b: 255 }
    );
}

#[test]
fn test_rule3_hsl_fail() {
    assert!(parse_color("hsl(400, 100%, 50%)").is_err()); // bad hue
    assert!(parse_color("hsl(120, 101%, 50%)").is_err()); // bad sat
    assert!(parse_color("hsl(120, 100, 50, 1, 2)").is_err()); // too many
    assert!(parse_color("hsl(120, 100, 50a)").is_err()); // bad number
}

#[test]
fn test_rule4_named() {
    assert_eq!(
        parse_color("red").unwrap(),
        Color { r: 255, g: 0, b: 0 }
    );
    // check case-insensitivity
    assert_eq!(
        parse_color("WHITE").unwrap(),
        Color {
            r: 255,
            g: 255,
            b: 255
        }
    );
    assert_eq!(
        parse_color("rebeccapurple").unwrap(),
        Color {
            r: 102,
            g: 51,
            b: 153
        }
    );
    // This was in fail test, but it's implemented
    assert_eq!(
        parse_color("orange").unwrap(),
        Color { r: 255, g: 165, b: 0 }
    );
}

#[test]
fn test_rule4_named_fail() {
    // not in our small list
    assert!(parse_color("notacolor").is_err());
}

#[test]
fn test_overall_fail() {
    // test empty
    assert!(parse_color("").is_err());
    assert!(parse_color("   ").is_err());
    // test junk
    assert!(parse_color("rgb(255, 0, 0)a").is_err()); // junk at end
    assert!(parse_color("hello").is_err());
}