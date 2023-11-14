use num_traits::ToPrimitive;

pub fn variance<T>(vec: &[T]) -> f64
where
    T: ToPrimitive,
{
    let n = vec.len() as f64;
    let mean = vec.iter().map(|x| x.to_f64().unwrap()).sum::<f64>() / n;
    vec.iter().map(|x| x.to_f64().unwrap().powi(2)).sum::<f64>() / n - mean * mean
}
