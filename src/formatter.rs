pub fn format_squares_numbers(moon_squares: u32, sun_squares: u32) -> String {
    let moon_formatted = format!("{:03}", moon_squares);
    let sun_formatted = format!("{:03}", sun_squares);
    format!("Moon {} | Sun {}", moon_formatted, sun_formatted)
}
