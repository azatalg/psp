use plotters::prelude::*;
use crate::math;
use crate::theme;
pub struct ChartConfig<'a> {
    pub out_path: &'a str,
    pub title: &'a str,
    pub theme: theme::Theme,
    pub size: (u32, u32),
}

impl<'a> ChartConfig<'a> {
    pub fn new(out_path: &'a str, title: &'a str) -> Self {
        Self {
            out_path,
            title,
            theme: theme::Theme::default(),
            size: (1200, 700),
        }
    }

    pub fn with_theme(mut self, theme: theme::Theme) -> Self {
        self.theme = theme;
        self
    }

    pub fn with_size(mut self, width: u32, height: u32) -> Self {
        self.size = (width, height);
        self
    }

    pub fn draw_histogram(&self, values: &[f64], bins: usize) -> Result<(), Box<dyn std::error::Error>> {
        if values.is_empty() || bins == 0 {
            return Err("Brak danych lub liczba klas wynosi 0".into());
        }

        let min_v = math::min(values).unwrap_or(0.0);
        let max_v = math::max(values).unwrap_or(0.0);
        let (min_x, max_x) = if (max_v - min_v).abs() < f64::EPSILON {
            (min_v - 0.5, max_v + 0.5)
        } else {
            (min_v, max_v)
        };

        let bin_width = (max_x - min_x) / (bins as f64);
        let mut counts = vec![0u32; bins];

        for &value in values {
            let raw = (value - min_x) / bin_width;
            let idx = if raw.is_finite() {
                (raw.floor() as usize).clamp(0, bins.saturating_sub(1))
            } else {
                0
            };
            counts[idx] += 1;
        }


        let max_count = counts.iter().copied().max().unwrap_or(0);
        let y_max = max_count + (max_count as f64 * 0.15).ceil() as u32 + 2;

        let root = BitMapBackend::new(self.out_path, self.size).into_drawing_area();
        root.fill(&self.theme.bg)?;

        let mut chart = ChartBuilder::on(&root)
            .caption(self.title, ("Roboto", 40).into_font().color(&self.theme.text))

            .margin_top(30)
            .margin_bottom(20)
            .margin_left(20)
            .margin_right(30)
            .x_label_area_size(60)
            .y_label_area_size(70)
            .build_cartesian_2d(min_x..max_x, 0u32..y_max)?;


        chart
            .configure_mesh()

            .disable_x_mesh()

            .bold_line_style(self.theme.grid.stroke_width(2))

            .light_line_style(TRANSPARENT)

            .axis_style(self.theme.text.stroke_width(1))
            .label_style(("Roboto", 20).into_font().color(&self.theme.text))

            .x_label_formatter(&|v| format!("{:.1}", v))
            .draw()?;


        for (i, &c) in counts.iter().enumerate() {
            let x0 = min_x + i as f64 * bin_width;
            let x1 = x0 + bin_width;


            chart.draw_series(std::iter::once(Rectangle::new(
                [(x0, 0), (x1, c)],
                self.theme.box_fill.filled(),
            )))?;


            chart.draw_series(std::iter::once(Rectangle::new(
                [(x0, 0), (x1, c)],
                self.theme.box_edge.stroke_width(2),
            )))?;


            if c > 0 {
                let x_center = x0 + bin_width / 2.0;
                chart.draw_series(std::iter::once(
                    EmptyElement::at((x_center, c))
                        + Text::new(
                        c.to_string(),
                        (-8, -25),
                        ("Roboto", 20).into_font().color(&self.theme.text)
                    )
                ))?;
            }
        }
        root.present()?;
        Ok(())
    }

    pub fn draw_boxplots(&self, groups: &[(&str, &[f64])]) -> Result<(), Box<dyn std::error::Error>> {
        if groups.is_empty() {
            return Err("Brak grup do narysowania".into());
        }

        let mut global_min = f64::MAX;
        let mut global_max = f64::MIN;

        for &(_, data) in groups {
            if data.is_empty() { continue; }
            global_min = global_min.min(math::min(data).unwrap_or(0.0));
            global_max = global_max.max(math::max(data).unwrap_or(0.0));
        }

        let y_margin = (global_max - global_min).abs() * 0.05 + 1.0;
        let y_range = (global_min - y_margin)..(global_max + y_margin);

        let root = BitMapBackend::new(self.out_path, self.size).into_drawing_area();

        root.fill(&self.theme.bg)?;

        let x_max = groups.len() as f64 + 1.0;

        let mut chart = ChartBuilder::on(&root)

            .caption(self.title, ("Roboto", 40).into_font().color(&self.theme.text))

            .margin_top(30)
            .margin_bottom(20)
            .margin_left(20)
            .margin_right(30)
            .x_label_area_size(60)
            .y_label_area_size(70)
            .build_cartesian_2d(0.0..x_max, y_range)?;


        chart
            .configure_mesh()

            .disable_x_mesh()

            .bold_line_style(self.theme.grid.stroke_width(2))

            .light_line_style(TRANSPARENT)

            .axis_style(self.theme.text.stroke_width(1))
            .label_style(("Roboto", 20).into_font().color(&self.theme.text))

            .x_label_formatter(&|v: &f64| {
                let idx = v.round() as usize;
                if idx > 0 && idx <= groups.len() {
                    groups[idx - 1].0.to_string()
                } else {
                    "".to_string()
                }
            })
            .x_labels(groups.len() + 2)
            .draw()?;



        for (i, &(_, data)) in groups.iter().enumerate() {
            if data.is_empty() { continue; }

            let stats = math::summarize(data);
            let (fence_low, fence_high) = math::boxplot_fences(stats.q1, stats.q3);

            let mut whisker_min = stats.q1;
            let mut whisker_max = stats.q3;
            let mut outliers = Vec::new();

            for &v in data {
                if v < fence_low || v > fence_high {
                    outliers.push(v);
                } else {
                    whisker_min = whisker_min.min(v);
                    whisker_max = whisker_max.max(v);
                }
            }

            let x_center = (i + 1) as f64;
            let box_width = 0.35;
            let cap_width = 0.15;


            chart.draw_series(std::iter::once(Rectangle::new(
                [(x_center - box_width, stats.q1), (x_center + box_width, stats.q3)],
                self.theme.box_fill.filled(),
            )))?;


            chart.draw_series(std::iter::once(Rectangle::new(
                [(x_center - box_width, stats.q1), (x_center + box_width, stats.q3)],
                self.theme.box_edge.stroke_width(2),
            )))?;


            chart.draw_series(std::iter::once(PathElement::new(
                vec![(x_center - box_width, stats.q2), (x_center + box_width, stats.q2)],
                self.theme.median.stroke_width(2),
            )))?;


            chart.draw_series(std::iter::once(PathElement::new(
                vec![(x_center, stats.q3), (x_center, whisker_max)],
                self.theme.whisker.stroke_width(2),
            )))?;
            chart.draw_series(std::iter::once(PathElement::new(
                vec![(x_center - cap_width, whisker_max), (x_center + cap_width, whisker_max)],
                self.theme.whisker.stroke_width(2),
            )))?;

            chart.draw_series(std::iter::once(PathElement::new(
                vec![(x_center, stats.q1), (x_center, whisker_min)],
                self.theme.whisker.stroke_width(2),
            )))?;
            chart.draw_series(std::iter::once(PathElement::new(
                vec![(x_center - cap_width, whisker_min), (x_center + cap_width, whisker_min)],
                self.theme.whisker.stroke_width(2),
            )))?;


            for outlier in outliers {
                chart.draw_series(std::iter::once(Circle::new(
                    (x_center, outlier),
                    5,
                    self.theme.outlier.filled(),
                )))?;
            }
        }

        root.present()?;
        Ok(())
    }
}