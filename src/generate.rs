/**
 * Generate signals
 */

pub trait Generator {
    fn output(&self, time: f64) -> f64;
}

/**
 * Takes an iterable of floats and a generator and applies
 * the generator to the floats, returning an iterator
 */
pub fn trace<I, G>(gen: G, times: I) -> impl Iterator<Item=f64>
    where I: IntoIterator<Item=f64>,
          G: Generator,
{
    times.into_iter().map(move |value| gen.output(value))
}

