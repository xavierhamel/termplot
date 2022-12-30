use crate::{DrawView, View, ViewCanvas};
use std::ops;

/// A continuous function to be graphed on the figure.
pub struct Graph<F>
where
    F: Fn(f64) -> f64,
{
    function: F,
}

impl<F> Graph<F>
where
    F: Fn(f64) -> f64,
{
    /// Create a new continuous function.
    pub fn new(function: F) -> Self {
        Self { function }
    }
}

impl<F> DrawView for Graph<F>
where
    F: Fn(f64) -> f64,
{
    fn draw(&self, view: &View, canvas: &mut ViewCanvas) {
        view.domain
            .iter(view.size.w)
            .filter_map(|x| {
                let y = (self.function)(x);
                match y.is_finite() {
                    true => Some((x, y)),
                    false => None,
                }
            })
            .collect::<Vec<_>>()
            .windows(2)
            .into_iter()
            .for_each(|line| {
                canvas.line(line[0].0, line[0].1, line[1].0, line[1].1);
            });
    }
}

pub struct Bar {
    x: f64,
    height: f64,
    width: f64,
}

impl Bar {
    pub fn new(x: f64, width: f64, height: f64) -> Self {
        Self { x, height, width }
    }
}

impl DrawView for Bar {
    fn draw(&self, _: &View, canvas: &mut ViewCanvas) {
        canvas.line(self.x, 0.0, self.x, self.height);
        canvas.line(self.x + self.width, 0.0, self.x + self.width, self.height);
        canvas.line(self.x, self.height, self.x + self.width, self.height);
    }
}

pub struct Bars {
    bars: Vec<Bar>,
}

impl Bars {
    pub fn new(data: Vec<f64>) -> Self {
        let bars = data
            .into_iter()
            .enumerate()
            .map(|(x, height)| Bar::new(x as f64, 1.0, height))
            .collect::<Vec<_>>();
        Self { bars }
    }
}

impl DrawView for Bars {
    fn draw(&self, view: &View, canvas: &mut ViewCanvas) {
        self.bars.iter().for_each(|bar| bar.draw(view, canvas));
    }
}

pub struct Historigram {
    buckets: Vec<Bar>,
    items_count: usize,
}

impl Historigram {
    pub fn new(values: Vec<f64>, ranges: Vec<ops::Range<f64>>) -> Self {
        let buckets = ranges
            .into_iter()
            .map(|range| Bar {
                x: range.start,
                width: range.end - range.start,
                height: values.iter().filter(|v| range.contains(v)).count() as f64,
            })
            .collect::<Vec<_>>();
        Self {
            buckets,
            items_count: values.len(),
        }
    }

    pub fn new_with_buckets_count(values: Vec<f64>, count: u32) -> Self {
        let max = values.iter().copied().fold(f64::NEG_INFINITY, f64::max);
        let min = values.iter().copied().fold(f64::INFINITY, f64::min);
        let width = (max - min) / count as f64;
        let buckets = (0..count)
            .into_iter()
            .map(|idx| (min + width * idx as f64)..(min + width * (idx as f64 + 1.0)))
            .collect::<Vec<ops::Range<f64>>>();
        Self::new(values, buckets)
    }
}

impl DrawView for Historigram {
    fn draw(&self, view: &View, canvas: &mut ViewCanvas) {
        self.buckets
            .iter()
            .for_each(|bucket| bucket.draw(view, canvas));
    }
}
