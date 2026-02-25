#[derive(Debug, Clone)]
pub struct SummaryStats {
    pub n: usize,
    pub mean: f64,
    pub median: f64,
    pub variance_sample: f64,
    pub variance_population: f64,
    pub std_dev_population: f64,
    pub std_dev_sample: f64,
    pub min: f64,
    pub max: f64,
    pub q1: f64,
    pub q2: f64,
    pub q3: f64,
    pub iqr: f64,
    pub cv_sample: f64, 
}

pub fn mean(data: &[f64]) -> f64 {
    if data.is_empty() {
        return f64::NAN;
    }
    data.iter().sum::<f64>() / data.len() as f64
}

pub fn median(data: &mut [f64]) -> f64 {
    if data.is_empty() {
        return f64::NAN;
    }
    data.sort_by(|a, b| a.partial_cmp(b).unwrap());
    let n = data.len();

    if n % 2 != 0 {
        data[n / 2]
    } else {
        (data[n / 2 - 1] + data[n / 2]) / 2.0
    }
}

pub fn variance_population(data: &[f64]) -> f64 {
    if data.len() < 2 {
        return f64::NAN;
    }
    let m = mean(data);
    data.iter().map(|x| (x - m).powi(2)).sum::<f64>() / data.len() as f64
}

pub fn variance_sample(data: &[f64]) -> f64 {
    if data.len() < 2 {
        return f64::NAN;
    }
    let m = mean(data);
    data.iter().map(|x| (x - m).powi(2)).sum::<f64>() / (data.len() as f64 - 1.0)
}

pub fn std_dev_population(data: &[f64]) -> f64 {
    variance_population(data).sqrt()
}

pub fn std_dev_sample(data: &[f64]) -> f64 {
    variance_sample(data).sqrt()
}

pub fn min(data: &[f64]) -> Option<f64> {
    data.iter().cloned().reduce(f64::min)
}

pub fn max(data: &[f64]) -> Option<f64> {
    data.iter().cloned().reduce(f64::max)
}

pub fn quartiles(data: &mut [f64]) -> (f64, f64, f64) {
    if data.is_empty() {
        return (f64::NAN, f64::NAN, f64::NAN);
    }
    data.sort_by(|a, b| a.partial_cmp(b).unwrap());
    let n = data.len();

    let q2 = median(data);

    let (lower_half, upper_half) = if n % 2 == 0 {
        (&data[..n / 2], &data[n / 2..])
    } else {
        (&data[..n / 2], &data[n / 2 + 1..])
    };

    let mut lower_vec = lower_half.to_vec();
    let mut upper_vec = upper_half.to_vec();

    let q1 = median(&mut lower_vec);
    let q3 = median(&mut upper_vec);

    (q1, q2, q3)
}

pub fn iqr(q1: f64, q3: f64) -> f64 {
    q3 - q1
}

pub fn cv_sample(data: &[f64]) -> f64 {
    let m = mean(data);
    if m == 0.0 || m.is_nan() {
        f64::NAN
    } else {
        std_dev_sample(data) / m
    }
}

pub fn boxplot_fences(q1: f64, q3: f64) -> (f64, f64) {
    let iqr_val = iqr(q1, q3);
    let lower_fence = q1 - 1.5 * iqr_val;
    let upper_fence = q3 + 1.5 * iqr_val;
    (lower_fence, upper_fence)
}

pub fn optimal_bins_sturges(n: usize) -> usize {
    if n == 0 {
        return 0;
    }
    (1.0 + (n as f64).log2()).ceil() as usize
}

pub fn optimal_bins_freedman_diaconis(data: &mut [f64]) -> usize {
    let n = data.len();
    if n == 0 { return 0; }

    let (q1, _, q3) = quartiles(data);
    let iqr_val = iqr(q1, q3);

    if iqr_val == 0.0 {
        return optimal_bins_sturges(n);
    }

    let bin_width = 2.0 * iqr_val / (n as f64).cbrt();

    let min_val = min(data).unwrap_or(0.0);
    let max_val = max(data).unwrap_or(0.0);

    ((max_val - min_val) / bin_width).ceil() as usize
}

pub fn summarize(data: &[f64]) -> SummaryStats {
    let mut sorted = data.to_vec();
    let (q1, q2, q3) = quartiles(&mut sorted);

    SummaryStats {
        n: data.len(),
        mean: mean(data),
        median: q2,
        variance_sample: variance_sample(data),
        variance_population: variance_population(data),
        std_dev_sample: std_dev_sample(data),
        std_dev_population: std_dev_population(data),
        min: min(data).unwrap_or(f64::NAN),
        max: max(data).unwrap_or(f64::NAN),
        q1,
        q2,
        q3,
        iqr: iqr(q1, q3),
        cv_sample: cv_sample(data),
    }
}