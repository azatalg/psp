use crate::math;
// =========================
// BASIC RESULT TYPES
// =========================
use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SampleExperimentResult {
    pub n: usize,
    pub data: Vec<f64>,
    pub stats: math::SummaryStats,
}


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConvergenceResult {
    pub xs: Vec<f64>,
    pub ys: Vec<f64>,
    pub reference_y: f64,
}


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CltResult {
    pub n: usize,
    pub reps: usize,
    pub means: Vec<f64>,
    pub normalized: Vec<f64>,
    pub mean_of_means: f64,
    pub stats_of_means: math::SummaryStats,
    pub stats_of_normalized: math::SummaryStats,
}


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MeansForNsResult {
    pub rows: Vec<(usize, f64)>,
}


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PopulationSamplingResult {
    pub n: usize,
    pub sample: Vec<f64>,
    pub sample_stats: math::SummaryStats,
    pub population_mean: f64,
    pub sample_mean: f64,
    pub abs_diff: f64,
}

// =========================
// GENERIC EXPERIMENTS
// =========================

pub fn run_sample_experiment<F>(n: usize, sampler: F) -> SampleExperimentResult
where
    F: Fn(usize) -> Vec<f64>,
{
    let data = sampler(n);
    let stats = math::summarize(&data);

    SampleExperimentResult { n, data, stats }
}

pub fn run_means_for_ns<F>(ns: &[usize], sampler: F) -> MeansForNsResult
where
    F: Fn(usize) -> Vec<f64>,
{
    let rows = math::means_for_ns(ns, sampler);
    MeansForNsResult { rows }
}

pub fn run_convergence_experiment<F>(
    max_n: usize,
    reference_y: f64,
    sampler: F,
) -> ConvergenceResult
where
    F: Fn(usize) -> Vec<f64>,
{
    let series = math::convergence_series(max_n, sampler);

    let xs = series.iter().map(|(x, _)| *x).collect();
    let ys = series.iter().map(|(_, y)| *y).collect();

    ConvergenceResult {
        xs,
        ys,
        reference_y,
    }
}

pub fn run_clt_experiment<F>(
    n: usize,
    reps: usize,
    mu: f64,
    sigma: f64,
    sampler: F,
) -> CltResult
where
    F: Fn(usize) -> Vec<f64>,
{
    let means = math::repeated_sample_means(n, reps, sampler);
    let normalized = math::normalize_means(&means, mu, sigma, n);

    let mean_of_means = math::mean(&means);
    let stats_of_means = math::summarize(&means);
    let stats_of_normalized = math::summarize(&normalized);

    CltResult {
        n,
        reps,
        means,
        normalized,
        mean_of_means,
        stats_of_means,
        stats_of_normalized,
    }
}

// =========================
// POPULATION-BASED
// =========================

pub fn run_population_sampling_experiment<F>(
    population: &[f64],
    n: usize,
    sampler: F,
) -> PopulationSamplingResult
where
    F: Fn(&[f64], usize) -> Vec<f64>,
{
    let sample = sampler(population, n);
    let sample_stats = math::summarize(&sample);

    let population_mean = math::mean(population);
    let sample_mean = sample_stats.mean;
    let abs_diff = (sample_mean - population_mean).abs();

    PopulationSamplingResult {
        n,
        sample,
        sample_stats,
        population_mean,
        sample_mean,
        abs_diff,
    }
}

pub fn run_population_clt_experiment<F>(
    population: &[f64],
    n: usize,
    reps: usize,
    sampler: F,
) -> CltResult
where
    F: Fn(&[f64], usize) -> Vec<f64>,
{
    let mu = math::mean(population);
    let sigma = math::std_dev_population(population);

    let means = (0..reps)
        .map(|_| {
            let sample = sampler(population, n);
            math::mean(&sample)
        })
        .collect::<Vec<_>>();

    let normalized = math::normalize_means(&means, mu, sigma, n);

    let mean_of_means = math::mean(&means);
    let stats_of_means = math::summarize(&means);
    let stats_of_normalized = math::summarize(&normalized);

    CltResult {
        n,
        reps,
        means,
        normalized,
        mean_of_means,
        stats_of_means,
        stats_of_normalized,
    }
}