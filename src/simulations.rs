use rand::RngExt;
use rand::Rng;
use rand::SeedableRng;
use rand::rngs::SmallRng;
use rand::seq::SliceRandom;
use std::cell::RefCell;

thread_local! {
    static RNG: RefCell<SmallRng> = RefCell::new(SmallRng::seed_from_u64(42));
}

pub fn set_seed(seed: u64) {
    RNG.with(|rng| {
        *rng.borrow_mut() = SmallRng::seed_from_u64(seed);
    });
}

fn with_rng<T>(f: impl FnOnce(&mut SmallRng) -> T) -> T {
    RNG.with(|rng| {
        let mut rng = rng.borrow_mut();
        f(&mut rng)
    })
}

// =========================
// CORE GENERIC HELPERS
// =========================

pub fn sample<T, F>(n: usize, mut generator: F) -> Vec<T>
where
    F: FnMut() -> T,
{
    let mut out = Vec::with_capacity(n);
    for _ in 0..n {
        out.push(generator());
    }
    out
}

pub fn repeat<T, F>(reps: usize, mut experiment: F) -> Vec<T>
where
    F: FnMut() -> T,
{
    let mut out = Vec::with_capacity(reps);
    for _ in 0..reps {
        out.push(experiment());
    }
    out
}

pub fn experiment_sum<F>(n: usize, generator: F) -> f64
where
    F: FnMut() -> f64,
{
    sample(n, generator).into_iter().sum()
}

pub fn experiment_mean<F>(n: usize, generator: F) -> f64
where
    F: FnMut() -> f64,
{
    let values = sample(n, generator);
    values.iter().sum::<f64>() / values.len() as f64
}

// =========================
// SINGLE DRAWS
// =========================

pub fn dice() -> f64 {
    with_rng(|rng| rng.random_range(1..=6) as f64)
}

pub fn coin() -> f64 {
    with_rng(|rng| if rng.random::<f64>() < 0.5 { 1.0 } else { 0.0 })
}

fn uniform01() -> f64 {
    with_rng(|rng| rng.random::<f64>().max(1e-12))
}

pub fn exp(lambda: f64) -> f64 {
    let u = uniform01();
    -u.ln() / lambda
}

pub fn poisson(lambda: f64) -> f64 {
    with_rng(|rng| {
        let l = (-lambda).exp();
        let mut k = 0usize;
        let mut p = 1.0;

        loop {
            k += 1;
            p *= rng.random::<f64>();
            if p <= l {
                return (k - 1) as f64;
            }
        }
    })
}

pub fn normal(mean: f64, stddev: f64) -> f64 {
    with_rng(|rng| {
        let u1 = rng.random::<f64>().max(1e-12);
        let u2 = rng.random::<f64>();

        let z = (-2.0 * u1.ln()).sqrt()
            * (2.0 * std::f64::consts::PI * u2).cos();

        mean + stddev * z
    })
}

pub fn from_population_with_replacement(population: &[f64]) -> f64 {
    with_rng(|rng| {
        let idx = rng.random_range(0..population.len());
        population[idx]
    })
}

pub fn sample_from_population_with_replacement(population: &[f64], n: usize) -> Vec<f64> {
    sample(n, || from_population_with_replacement(population))
}

pub fn sample_from_population_without_replacement(population: &[f64], n: usize) -> Vec<f64> {
    let mut idxs: Vec<usize> = (0..population.len()).collect();
    with_rng(|rng| idxs.shuffle(rng));

    idxs.into_iter()
        .take(n.min(population.len()))
        .map(|i| population[i])
        .collect()
}