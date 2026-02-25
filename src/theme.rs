use plotters::prelude::*;
#[derive(Clone)]
pub struct Theme {
    pub bg: RGBColor,
    pub grid: RGBColor,
    pub text: RGBColor,
    pub box_fill: RGBAColor,
    pub box_edge: RGBColor,
    pub whisker: RGBColor,
    pub median: RGBColor,
    pub outlier: RGBColor,
}

impl Default for Theme {
    fn default() -> Self {
        Self {
            bg: RGBColor(12, 12, 14),
            grid: RGBColor(60, 60, 70),
            text: RGBColor(220, 220, 230),
            box_fill: RGBColor(120, 90, 160).mix(0.35),
            box_edge: RGBColor(180, 150, 220),
            whisker: RGBColor(210, 210, 220),
            median: RGBColor(240, 240, 245),
            outlier: RGBColor(230, 230, 235),
        }
    }
}

impl Theme {
    pub fn white() -> Self {
        Self {
            bg: RGBColor(240, 242, 246),
            grid: RGBColor(255, 255, 255),
            text: RGBColor(60, 60, 70),
            box_fill: RGBColor(170, 130, 200).mix(0.7),
            box_edge: RGBColor(140, 90, 180),
            whisker: RGBColor(60, 60, 70),
            median: RGBColor(196, 78, 82),
            outlier: RGBColor(60, 60, 70),
        }
    }
}