use std::fs::File;
use std::io::{BufRead, BufReader};


#[derive(Debug, Clone)]
pub struct Variable {
    pub name: &'static str,
    pub all: Vec<f64>,
    pub boys: Vec<f64>,
    pub girls: Vec<f64>,
}

#[derive(Debug, Clone)]
pub struct GradesDataset {
    pub n_total: usize,
    pub n_boys: usize,
    pub n_girls: usize,
    pub variables: Vec<Variable>,
}

pub fn load_grades_data(
    filepath: &str,
) -> Result<(Vec<f64>, Vec<f64>, Vec<f64>, Vec<f64>, Vec<f64>, Vec<f64>), Box<dyn std::error::Error>> {
    let file = File::open(filepath)?;
    let reader = BufReader::new(file);

    let mut iq_b = Vec::new();
    let mut iq_g = Vec::new();
    let mut sroc_b = Vec::new();
    let mut sroc_g = Vec::new();
    let mut ph_b = Vec::new();
    let mut ph_g = Vec::new();

    for line_result in reader.lines() {
        let line = line_result?;
        let parts: Vec<&str> = line.trim().split_whitespace().collect();

        if parts.len() < 5 {
            continue;
        }

        let sroc: f64 = parts[1].parse().unwrap_or(0.0);
        let iq: f64 = parts[2].parse().unwrap_or(0.0);
        let gender = parts[3];
        let ph: f64 = parts[4].parse().unwrap_or(0.0);

        if gender.eq_ignore_ascii_case("m") {
            sroc_b.push(sroc);
            iq_b.push(iq);
            ph_b.push(ph);
        } else if gender.eq_ignore_ascii_case("f") {
            sroc_g.push(sroc);
            iq_g.push(iq);
            ph_g.push(ph);
        }
    }

    Ok((iq_b, iq_g, sroc_b, sroc_g, ph_b, ph_g))
}

pub fn load_grades_dataset(
    filepath: &str,
) -> Result<GradesDataset, Box<dyn std::error::Error>> {
    let (iq_b, iq_g, sroc_b, sroc_g, ph_b, ph_g) = load_grades_data(filepath)?;

    let mut iq_all = iq_b.clone();
    iq_all.extend(&iq_g);

    let mut sroc_all = sroc_b.clone();
    sroc_all.extend(&sroc_g);

    let mut ph_all = ph_b.clone();
    ph_all.extend(&ph_g);

    let n_boys = iq_b.len();
    let n_girls = iq_g.len();
    let n_total = n_boys + n_girls;

    Ok(GradesDataset {
        n_total,
        n_boys,
        n_girls,
        variables: vec![
            Variable {
                name: "IQ",
                all: iq_all,
                boys: iq_b,
                girls: iq_g,
            },
            Variable {
                name: "ŚrOc",
                all: sroc_all,
                boys: sroc_b,
                girls: sroc_g,
            },
            Variable {
                name: "PH",
                all: ph_all,
                boys: ph_b,
                girls: ph_g,
            },
        ],
    })
}

#[derive(Debug, Clone)]
pub struct IncomeRecord {
    pub education: usize, // 1..6
    pub sex: usize,       // 1 = mężczyzna, 2 = kobieta
    pub income: f64,
    pub sector: usize,    // 5..7
}
pub fn load_income_data(
    filepath: &str,
) -> Result<Vec<IncomeRecord>, Box<dyn std::error::Error>> {
    let file = File::open(filepath)?;
    let reader = BufReader::new(file);

    let mut records = Vec::new();

    for line_result in reader.lines() {
        let line = line_result?;
        let parts: Vec<&str> = line.split_whitespace().collect();

        if parts.len() < 6 {
            continue;
        }

        // format: id age education sex income sector
        let education: usize = parts[2].parse().unwrap_or(0);
        let sex: usize = parts[3].parse().unwrap_or(0);
        let income: f64 = parts[4].parse().unwrap_or(0.0);
        let sector: usize = parts[5].parse().unwrap_or(0);

        if (1..=6).contains(&education) && (1..=2).contains(&sex) && (5..=7).contains(&sector) {
            records.push(IncomeRecord {
                education,
                sex,
                income,
                sector,
            });
        }
    }

    Ok(records)
}