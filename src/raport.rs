use handlebars::Handlebars;
use serde::Serialize;
use std::fs;
use std::path::Path;
use std::process::Command;

use crate::math::SummaryStats;
use crate::plot::PlotRef;

#[derive(Serialize)]
#[serde(tag = "type")]
pub enum Block {
    Text { content: String },
    Subtitle { content: String },
    Note { content: String },
    Html { content: String },
    Plot { plot: PlotRef },
    PlotPair { left: PlotRef, right: PlotRef },
    PlotGrid { plots: Vec<PlotRef>, columns: usize },
    Table { headers: Vec<String>, rows: Vec<Vec<String>> },
    Conclusion { content: String },
    PageBreak,
}

// =========================
// INTO BLOCK
// =========================

pub trait IntoBlock {
    fn into_block(self) -> Block;
}

impl IntoBlock for Block {
    fn into_block(self) -> Block {
        self
    }
}

impl IntoBlock for PlotRef {
    fn into_block(self) -> Block {
        Block::Plot { plot: self }
    }
}

impl IntoBlock for Table {
    fn into_block(self) -> Block {
        Block::Table {
            headers: self.headers,
            rows: self.rows,
        }
    }
}

impl IntoBlock for String {
    fn into_block(self) -> Block {
        Block::Text { content: self }
    }
}

impl IntoBlock for &str {
    fn into_block(self) -> Block {
        Block::Text {
            content: self.to_string(),
        }
    }
}

// =========================
// TABLE DSL
// =========================

pub struct Table {
    headers: Vec<String>,
    rows: Vec<Vec<String>>,
}

impl Table {
    pub fn new<S: ToString, const N: usize>(headers: [S; N]) -> Self {
        Self {
            headers: headers.into_iter().map(|x| x.to_string()).collect(),
            rows: Vec::new(),
        }
    }

    pub fn row<S: ToString, const N: usize>(mut self, row: [S; N]) -> Self {
        self.rows
            .push(row.into_iter().map(|x| x.to_string()).collect());
        self
    }

    pub fn from_stats(stats: &SummaryStats) -> Self {
        Self::new(["Statystyka", "Wartość"])
            .row(["Średnia", &format!("{:.2}", stats.mean)])
            .row(["Mediana", &format!("{:.2}", stats.median)])
            .row(["Minimum", &format!("{:.2}", stats.min)])
            .row(["Maksimum", &format!("{:.2}", stats.max)])
            .row(["Q1", &format!("{:.2}", stats.q1)])
            .row(["Q3", &format!("{:.2}", stats.q3)])
            .row(["Wariancja", &format!("{:.2}", stats.variance_sample)])
            .row(["Odchylenie std.", &format!("{:.2}", stats.std_dev_sample)])
            .row(["CV (%)", &format!("{:.2}", stats.cv_sample * 100.0)])
    }
    pub fn stats_by_group(rows: &[(&str, &SummaryStats)]) -> Self {
        let mut table = Self::new([
            "Grupa",
            "Średnia",
            "Mediana",
            "Min",
            "Max",
            "Q1",
            "Q3",
            "Odch. std.",
            "CV (%)",
        ]);

        for (label, stats) in rows {
            table = table.row([
                *label,
                &format!("{:.2}", stats.mean),
                &format!("{:.2}", stats.median),
                &format!("{:.2}", stats.min),
                &format!("{:.2}", stats.max),
                &format!("{:.2}", stats.q1),
                &format!("{:.2}", stats.q3),
                &format!("{:.2}", stats.std_dev_sample),
                &format!("{:.2}", stats.cv_sample * 100.0),
            ]);
        }

        table
    }
    pub fn compare_two_groups(
        left_label: &str,
        right_label: &str,
        left: &SummaryStats,
        right: &SummaryStats,
    ) -> Self {
        Self::from_string_rows(
            ["Statystyka", left_label, right_label],
            vec![
                ["Średnia".into(), format!("{:.2}", left.mean), format!("{:.2}", right.mean)],
                ["Mediana".into(), format!("{:.2}", left.median), format!("{:.2}", right.median)],
                ["Minimum".into(), format!("{:.2}", left.min), format!("{:.2}", right.min)],
                ["Maksimum".into(), format!("{:.2}", left.max), format!("{:.2}", right.max)],
                ["Kwartyl dolny (Q1)".into(), format!("{:.2}", left.q1), format!("{:.2}", right.q1)],
                ["Kwartyl górny (Q3)".into(), format!("{:.2}", left.q3), format!("{:.2}", right.q3)],
                ["Wariancja".into(), format!("{:.2}", left.variance_sample), format!("{:.2}", right.variance_sample)],
                ["Odchylenie standardowe".into(), format!("{:.2}", left.std_dev_sample), format!("{:.2}", right.std_dev_sample)],
                [
                    "Współczynnik zmienności (CV)".into(),
                    format!("{:.2}%", left.cv_sample * 100.0),
                    format!("{:.2}%", right.cv_sample * 100.0),
                ],
            ],
        )
    }
    pub fn from_usize_f64_pairs(headers: [&str; 2], rows: &[(usize, f64)]) -> Self {
        rows.iter().fold(
            Self::new(headers),
            |table, (a, b)| table.row([a.to_string(), format!("{:.4}", b)]),
        )
    }

    pub fn from_string_rows<S: ToString, const N: usize>(
        headers: [S; N],
        rows: Vec<[String; N]>,
    ) -> Self {
        let mut table = Self::new(headers);
        for row in rows {
            table = table.row(row);
        }
        table
    }
}

// =========================
// SECTION
// =========================

#[derive(Serialize)]
pub struct Section {
    pub title: String,
    pub blocks: Vec<Block>,
}

// =========================
// SECTION BUILDER
// =========================

pub struct SectionBuilder {
    title: String,
    blocks: Vec<Block>,
}

impl SectionBuilder {
    pub fn new(title: &str) -> Self {
        Self {
            title: title.to_string(),
            blocks: Vec::new(),
        }
    }

    pub fn plot_grid(&mut self, plots: Vec<PlotRef>, columns: usize) -> &mut Self {
        self.blocks.push(Block::PlotGrid { plots, columns });
        self
    }

    pub fn add<B: IntoBlock>(&mut self, block: B) -> &mut Self {
        self.blocks.push(block.into_block());
        self
    }

    pub fn add_pair(&mut self, left: PlotRef, right: PlotRef) -> &mut Self {
        self.blocks.push(Block::PlotPair { left, right });
        self
    }

    pub fn text(&mut self, content: &str) -> &mut Self {
        self.blocks.push(Block::Text {
            content: content.to_string(),
        });
        self
    }

    pub fn note(&mut self, content: &str) -> &mut Self {
        self.blocks.push(Block::Note {
            content: content.to_string(),
        });
        self
    }

    pub fn html(&mut self, content: &str) -> &mut Self {
        self.blocks.push(Block::Html {
            content: content.to_string(),
        });
        self
    }

    pub fn subtitle(&mut self, content: &str) -> &mut Self {
        self.blocks.push(Block::Subtitle {
            content: content.to_string(),
        });
        self
    }

    pub fn plot(&mut self, plot: PlotRef) -> &mut Self {
        self.blocks.push(Block::Plot { plot });
        self
    }

    pub fn plot_pair(&mut self, left: PlotRef, right: PlotRef) -> &mut Self {
        self.blocks.push(Block::PlotPair { left, right });
        self
    }

    pub fn table(&mut self, table: Table) -> &mut Self {
        self.blocks.push(Block::Table {
            headers: table.headers,
            rows: table.rows,
        });
        self
    }

    pub fn raw_table<S: ToString>(&mut self, headers: Vec<S>, rows: Vec<Vec<S>>) -> &mut Self {
        let headers = headers.into_iter().map(|x| x.to_string()).collect();
        let rows = rows
            .into_iter()
            .map(|r| r.into_iter().map(|x| x.to_string()).collect())
            .collect();

        self.blocks.push(Block::Table { headers, rows });
        self
    }

    pub fn stats_table(&mut self, stats: &SummaryStats) -> &mut Self {
        self.table(Table::from_stats(stats))
    }

    pub fn stats_by_group(&mut self, rows: &[(&str, &SummaryStats)]) -> &mut Self {
        self.table(Table::stats_by_group(rows))
    }

    pub fn variable_report(
        &mut self,
        stats: &SummaryStats,
        hist: PlotRef,
        boxplot: PlotRef,
    ) -> &mut Self {
        self.stats_table(stats).plot_pair(hist, boxplot)
    }

    pub fn comparison_report(
        &mut self,
        rows: &[(&str, &SummaryStats)],
        compare_plot: PlotRef,
    ) -> &mut Self {
        self.stats_by_group(rows).plot(compare_plot)
    }

    pub fn conclusion(&mut self, content: &str) -> &mut Self {
        self.blocks.push(Block::Conclusion {
            content: content.to_string(),
        });
        self
    }

    pub fn page_break(&mut self) -> &mut Self {
        self.blocks.push(Block::PageBreak);
        self
    }

    fn build(self) -> Section {
        Section {
            title: self.title,
            blocks: self.blocks,
        }
    }
}

// =========================
// REPORT META
// =========================

#[derive(Serialize, Default)]
pub struct ReportMeta {
    pub title: String,
    pub subtitle: Option<String>,
    pub author: Option<String>,
    pub date: Option<String>,
}

// =========================
// REPORT DOC
// =========================

#[derive(Serialize)]
pub struct ReportDocument {
    pub meta: ReportMeta,
    pub sections: Vec<Section>,
}

// =========================
// REPORT BUILDER
// =========================

pub struct ReportBuilder {
    meta: ReportMeta,
    sections: Vec<Section>,
}

pub fn report() -> ReportBuilder {
    ReportBuilder {
        meta: ReportMeta {
            title: "Raport Statystyczny".to_string(),
            subtitle: None,
            author: None,
            date: None,
        },
        sections: Vec::new(),
    }
}

impl ReportBuilder {
    pub fn title(mut self, title: &str) -> Self {
        self.meta.title = title.to_string();
        self
    }

    pub fn subtitle(mut self, subtitle: &str) -> Self {
        self.meta.subtitle = Some(subtitle.to_string());
        self
    }

    pub fn author(mut self, author: &str) -> Self {
        self.meta.author = Some(author.to_string());
        self
    }

    pub fn date(mut self, date: &str) -> Self {
        self.meta.date = Some(date.to_string());
        self
    }

    pub fn date_now(mut self) -> Self {
        // Bez dodatkowych crate'ów — prosty placeholder z czasu systemowego jako string
        // Jak zechcesz, można potem podmienić na chrono.
        let now = std::time::SystemTime::now();
        self.meta.date = Some(format!("{:?}", now));
        self
    }

    pub fn section<F>(mut self, title: &str, f: F) -> Self
    where
        F: FnOnce(&mut SectionBuilder),
    {
        let mut builder = SectionBuilder::new(title);
        f(&mut builder);
        self.sections.push(builder.build());
        self
    }

    pub fn section_standard<F>(self, title: &str, f: F) -> Self
    where
        F: FnOnce(&mut SectionBuilder),
    {
        self.section(title, |s| {
            f(s);
            s.page_break();
        })
    }

    pub fn build(self, output: &str) -> Result<(), Box<dyn std::error::Error>> {
        let doc = ReportDocument {
            meta: self.meta,
            sections: self.sections,
        };

        generate_html(doc, output)
    }

    pub fn build_with_pdf(
        self,
        html_output: &str,
        pdf_output: &str,
    ) -> Result<(), Box<dyn std::error::Error>> {
        self.build(html_output)?;
        export_pdf(Path::new(html_output), Path::new(pdf_output))
    }
}

// =========================
// RENDER
// =========================

fn generate_html(
    doc: ReportDocument,
    output: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    let template = fs::read_to_string("templates/base.html.hbs")?;

    let mut reg = Handlebars::new();
    reg.register_escape_fn(handlebars::no_escape);

    reg.register_helper(
        "eq",
        Box::new(
            |h: &handlebars::Helper,
             _: &Handlebars,
             _: &handlebars::Context,
             _: &mut handlebars::RenderContext,
             out: &mut dyn handlebars::Output|
             -> handlebars::HelperResult {
                let p0 = h.param(0).unwrap().value().as_str().unwrap();
                let p1 = h.param(1).unwrap().value().as_str().unwrap();

                if p0 == p1 {
                    out.write("true")?;
                }

                Ok(())
            },
        ),
    );

    reg.register_template_string("tpl", template)?;

    let html = reg.render("tpl", &doc)?;
    fs::write(output, html)?;

    Ok(())
}

// =========================
// PDF EXPORT
// =========================

pub fn export_pdf(
    html_path: &Path,
    pdf_path: &Path,
) -> Result<(), Box<dyn std::error::Error>> {
    let current_dir = std::env::current_dir()?;

    let html_abs = if html_path.is_absolute() {
        html_path.to_path_buf()
    } else {
        current_dir.join(html_path)
    };

    let pdf_abs = if pdf_path.is_absolute() {
        pdf_path.to_path_buf()
    } else {
        current_dir.join(pdf_path)
    };

    if let Some(parent) = pdf_abs.parent() {
        std::fs::create_dir_all(parent)?;
    }

    let browsers = [
        "C:\\Program Files (x86)\\Microsoft\\Edge\\Application\\msedge.exe",
        "C:\\Program Files\\Microsoft\\Edge\\Application\\msedge.exe",
        "C:\\Program Files\\Google\\Chrome\\Application\\chrome.exe",
        "google-chrome",
        "chromium",
        "chromium-browser",
        "/Applications/Google Chrome.app/Contents/MacOS/Google Chrome",
    ];

    let mut last_err = None;

    for browser in browsers {
        let result = Command::new(browser)
            .args([
                "--headless",
                "--disable-gpu",
                "--no-pdf-header-footer",
                &format!("--print-to-pdf={}", pdf_abs.display()),
                html_abs.to_str().ok_or("Niepoprawna ścieżka HTML")?,
            ])
            .status();

        match result {
            Ok(status) if status.success() => {
                println!("PDF gotowy: {}", pdf_abs.display());
                return Ok(());
            }
            Ok(status) => {
                last_err = Some(format!("{} -> kod wyjścia: {}", browser, status));
            }
            Err(e) => {
                last_err = Some(format!("{} -> {}", browser, e));
            }
        }
    }

    Err(format!("Nie udało się wygenerować PDF: {:?}", last_err).into())
}