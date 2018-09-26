use flat;

impl flat::ColorArgs {
    /// Create a color with the given **a**lpha, **r**ed, **g**reen, and
    /// **b**lue.
    ///
    /// An alpha of 255 is fully opaque, and 0 is fully transparent.
    pub fn argb(a: u8, r: u8, g: u8, b: u8) -> Self {
        Self { a, r, g, b }
    }

    /// Create an opaque color with the given, **r**ed, **g**reen, and **b**lue.
    pub fn rgb(r: u8, g: u8, b: u8) -> Self {
        Self { a: 255, r, g, b }
    }
}
