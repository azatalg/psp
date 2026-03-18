use plotters::prelude::*;
use serde::Serialize;

use crate::math;
use crate::theme;

// =========================
// PLOT REF
// =========================

#[derive(Clone, Serialize)]
pub struct PlotRef {
    pub name: String,
    pub path: String,
}

impl PlotRef {
    pub fn new(name: impl Into<String>, path: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            path: path.into(),
        }
    }
}

// =========================
// PLOT FACTORY
// =========================

#[derive(Clone)]
pub struct PlotFactory {
    out_dir: String,
    theme: theme::Theme,
    size: (u32, u32),
}

impl PlotFactory {
    pub fn new(out_dir: impl Into<String>, theme: theme::Theme) -> Self {
        Self {
            out_dir: out_dir.into(),
            theme,
            size: (1200, 700),
        }
    }

    pub fn with_size(mut self, width: u32, height: u32) -> Self {
        self.size = (width, height);
        self
    }

    fn abs_png_path(&self, key: &str) -> String {
        format!("{}/{}.png", self.out_dir, key)
    }

    fn rel_png_path(&self, key: &str) -> String {
        format!("{}.png", key)
    }

    fn make_plot_ref(&self, key: &str, title: &str) -> PlotRef {
        PlotRef {
            name: title.to_string(),
            path: self.rel_png_path(key),
        }
    }

    pub fn histogram(
        &self,
        key: &str,
        title: &str,
        data: &[f64],
        bins: usize,
    ) -> Result<PlotRef, Box<dyn std::error::Error>> {
        let abs = self.abs_png_path(key);

        ChartConfig::new(&abs, title)
            .with_theme(self.theme.clone())
            .with_size(self.size.0, self.size.1)
            .draw_histogram(data, bins.max(1))?;

        Ok(self.make_plot_ref(key, title))
    }

    pub fn histogram_auto(
        &self,
        key: &str,
        title: &str,
        data: &[f64],
    ) -> Result<PlotRef, Box<dyn std::error::Error>> {
        let bins = math::optimal_bins_sturges(data.len()).max(1);
        self.histogram(key, title, data, bins)
    }

    pub fn boxplot_single(
        &self,
        key: &str,
        title: &str,
        label: &str,
        data: &[f64],
    ) -> Result<PlotRef, Box<dyn std::error::Error>> {
        let abs = self.abs_png_path(key);

        ChartConfig::new(&abs, title)
            .with_theme(self.theme.clone())
            .with_size(self.size.0, self.size.1)
            .draw_boxplots(&[(label, data)])?;

        Ok(self.make_plot_ref(key, title))
    }

    pub fn boxplot_groups(
        &self,
        key: &str,
        title: &str,
        groups: &[(&str, &[f64])],
    ) -> Result<PlotRef, Box<dyn std::error::Error>> {
        let abs = self.abs_png_path(key);

        ChartConfig::new(&abs, title)
            .with_theme(self.theme.clone())
            .with_size(self.size.0, self.size.1)
            .draw_boxplots(groups)?;

        Ok(self.make_plot_ref(key, title))
    }

    pub fn hist_and_box(
        &self,
        key: &str,
        variable_name: &str,
        data: &[f64],
    ) -> Result<(PlotRef, PlotRef), Box<dyn std::error::Error>> {
        let hist = self.histogram_auto(
            &format!("hist_{}", key),
            &format!("Rozkład {}", variable_name),
            data,
        )?;

        let boxp = self.boxplot_single(
            &format!("boxplot_{}", key),
            &format!("Wykres ramkowy {}", variable_name),
            variable_name,
            data,
        )?;

        Ok((hist, boxp))
    }

    pub fn normalized_histogram(
        &self,
        key: &str,
        title: &str,
        means: &[f64],
        mu: f64,
        sigma: f64,
        n: usize,
        bins: usize,
    ) -> Result<PlotRef, Box<dyn std::error::Error>> {
        let normalized = math::normalize_means(means, mu, sigma, n);
        self.histogram(key, title, &normalized, bins)
    }

    pub fn normalized_histogram_auto(
        &self,
        key: &str,
        title: &str,
        means: &[f64],
        mu: f64,
        sigma: f64,
        n: usize,
    ) -> Result<PlotRef, Box<dyn std::error::Error>> {
        let normalized = math::normalize_means(means, mu, sigma, n);
        self.histogram_auto(key, title, &normalized)
    }

    pub fn normalized_histogram_with_normal_overlay(
        &self,
        key: &str,
        title: &str,
        values: &[f64],
        bins: usize,
    ) -> Result<PlotRef, Box<dyn std::error::Error>> {
        let abs = self.abs_png_path(key);

        ChartConfig::new(&abs, title)
            .with_theme(self.theme.clone())
            .with_size(self.size.0, self.size.1)
            .draw_density_histogram_with_standard_normal(values, bins.max(1))?;

        Ok(self.make_plot_ref(key, title))
    }
    pub fn line_plot_with_reference(
        &self,
        key: &str,
        title: &str,
        xs: &[f64],
        ys: &[f64],
        reference_y: f64,
    ) -> Result<PlotRef, Box<dyn std::error::Error>> {
        let abs = self.abs_png_path(key);

        ChartConfig::new(&abs, title)
            .with_theme(self.theme.clone())
            .with_size(self.size.0, self.size.1)
            .draw_line_plot_with_reference(xs, ys, reference_y)?;

        Ok(self.make_plot_ref(key, title))
    }

    pub fn convergence_plot(
        &self,
        key: &str,
        title: &str,
        xs: &[f64],
        ys: &[f64],
        reference_y: f64,
    ) -> Result<PlotRef, Box<dyn std::error::Error>> {
        self.line_plot_with_reference(key, title, xs, ys, reference_y)
    }

    pub fn clt_hist(
        &self,
        key: &str,
        title: &str,
        normalized: &[f64],
    ) -> Result<PlotRef, Box<dyn std::error::Error>> {
        self.normalized_histogram_with_normal_overlay(
            key,
            title,
            normalized,
            math::optimal_bins_sturges(normalized.len()).max(1),
        )
    }

    pub fn population_hist(
        &self,
        key: &str,
        title: &str,
        data: &[f64],
    ) -> Result<PlotRef, Box<dyn std::error::Error>> {
        self.histogram_auto(key, title, data)
    }
}

// =========================
// LOW LEVEL CHART CONFIG
// =========================

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

    pub fn draw_line_plot_with_reference(
        &self,
        xs: &[f64],
        ys: &[f64],
        reference_y: f64,
    ) -> Result<(), Box<dyn std::error::Error>> {
        if xs.is_empty() || ys.is_empty() || xs.len() != ys.len() {
            return Err("Niepoprawne dane do wykresu liniowego".into());
        }

        let min_x = xs.iter().copied().fold(f64::INFINITY, f64::min);
        let max_x = xs.iter().copied().fold(f64::NEG_INFINITY, f64::max);

        let mut min_y = ys.iter().copied().fold(f64::INFINITY, f64::min);
        let mut max_y = ys.iter().copied().fold(f64::NEG_INFINITY, f64::max);

        min_y = min_y.min(reference_y);
        max_y = max_y.max(reference_y);

        let x_margin = ((max_x - min_x).abs() * 0.03).max(1.0);
        let y_margin = ((max_y - min_y).abs() * 0.08).max(0.5);

        let x0 = min_x - x_margin;
        let x1 = max_x + x_margin;
        let y0 = min_y - y_margin;
        let y1 = max_y + y_margin;

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
            .build_cartesian_2d(x0..x1, y0..y1)?;

        chart
            .configure_mesh()
            .bold_line_style(self.theme.grid.stroke_width(2))
            .light_line_style(TRANSPARENT)
            .axis_style(self.theme.text.stroke_width(1))
            .label_style(("Roboto", 20).into_font().color(&self.theme.text))
            .x_label_formatter(&|v| format!("{:.0}", v))
            .y_label_formatter(&|v| format!("{:.2}", v))
            .draw()?;

        let points: Vec<(f64, f64)> = xs.iter().copied().zip(ys.iter().copied()).collect();

        chart.draw_series(std::iter::once(PathElement::new(
            vec![(x0, reference_y), (x1, reference_y)],
            self.theme.median.stroke_width(2),
        )))?;

        chart.draw_series(std::iter::once(PathElement::new(
            points.clone(),
            self.theme.box_edge.stroke_width(3),
        )))?;

        chart.draw_series(
            points
                .into_iter()
                .map(|p| Circle::new(p, 4, self.theme.outlier.filled())),
        )?;

        root.present()?;
        Ok(())
    }

    pub fn draw_histogram(
        &self,
        values: &[f64],
        bins: usize,
    ) -> Result<(), Box<dyn std::error::Error>> {
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
                        ("Roboto", 20).into_font().color(&self.theme.text),
                    ),
                ))?;
            }
        }

        root.present()?;
        Ok(())
    }

    pub fn draw_boxplots(
        &self,
        groups: &[(&str, &[f64])],
    ) -> Result<(), Box<dyn std::error::Error>> {
        if groups.is_empty() {
            return Err("Brak grup do narysowania".into());
        }

        let mut global_min = f64::MAX;
        let mut global_max = f64::MIN;

        for &(_, data) in groups {
            if data.is_empty() {
                continue;
            }
            global_min = global_min.min(math::min(data).unwrap_or(0.0));
            global_max = global_max.max(math::max(data).unwrap_or(0.0));
        }

        if global_min == f64::MAX || global_max == f64::MIN {
            return Err("Wszystkie grupy są puste".into());
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
            if data.is_empty() {
                continue;
            }

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

    pub fn draw_density_histogram_with_standard_normal(
        &self,
        values: &[f64],
        bins: usize,
    ) -> Result<(), Box<dyn std::error::Error>> {
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

        let bin_width = (max_x - min_x) / bins as f64;
        let n = values.len() as f64;
        let mut counts = vec![0usize; bins];

        for &value in values {
            let raw = (value - min_x) / bin_width;
            let idx = if raw.is_finite() {
                (raw.floor() as usize).clamp(0, bins.saturating_sub(1))
            } else {
                0
            };
            counts[idx] += 1;
        }

        let densities: Vec<f64> = counts
            .iter()
            .map(|&c| c as f64 / (n * bin_width))
            .collect();

        let max_density_hist = densities.iter().copied().fold(0.0, f64::max);

        let normal_pdf = |x: f64| {
            (1.0 / (2.0 * std::f64::consts::PI).sqrt()) * (-0.5 * x * x).exp()
        };

        let max_density_normal = 1.0 / (2.0 * std::f64::consts::PI).sqrt();
        let y_max = (max_density_hist.max(max_density_normal) * 1.15).max(0.5);

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
            .build_cartesian_2d(min_x..max_x, 0.0..y_max)?;

        chart
            .configure_mesh()
            .disable_x_mesh()
            .bold_line_style(self.theme.grid.stroke_width(2))
            .light_line_style(TRANSPARENT)
            .axis_style(self.theme.text.stroke_width(1))
            .label_style(("Roboto", 20).into_font().color(&self.theme.text))
            .x_label_formatter(&|v| format!("{:.2}", v))
            .y_label_formatter(&|v| format!("{:.2}", v))
            .draw()?;

        for (i, &d) in densities.iter().enumerate() {
            let x0 = min_x + i as f64 * bin_width;
            let x1 = x0 + bin_width;

            chart.draw_series(std::iter::once(Rectangle::new(
                [(x0, 0.0), (x1, d)],
                self.theme.box_fill.filled(),
            )))?;

            chart.draw_series(std::iter::once(Rectangle::new(
                [(x0, 0.0), (x1, d)],
                self.theme.box_edge.stroke_width(2),
            )))?;
        }

        let steps = 400usize;
        let curve: Vec<(f64, f64)> = (0..=steps)
            .map(|i| {
                let x = min_x + (max_x - min_x) * i as f64 / steps as f64;
                (x, normal_pdf(x))
            })
            .collect();

        chart.draw_series(std::iter::once(PathElement::new(
            curve,
            self.theme.median.stroke_width(3),
        )))?;

        root.present()?;
        Ok(())
    }
}