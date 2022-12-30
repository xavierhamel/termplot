//! Different types of plots and graphs that can be plotted or graphed onto the view.
//!
//! If a given type of plot is not present, creat it with [`DrawView`].
use crate::{DrawView, View, ViewCanvas};
use std::ops;

/// A continuous function to be graphed on the figure.
///
/// Use this struct to plot continuous functions on the graph.
///
/// # Examples
///
/// ```rust
/// use termplot::*;
///
/// let graph = plot::Graph::new(|x| x.sin() / x);
///
/// let mut plot = Plot::default();
/// plot.set_domain(Domain(-10.0..10.0))
///     .set_codomain(Domain(-0.3..1.2))
///     .set_title("Graph title")
///     .set_x_label("X axis")
///     .set_y_label("Y axis")
///     .set_size(Size::new(50, 25))
///     .add_plot(Box::new(graph));
///
/// println!("{plot}");
/// ```
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
    /// Create a new continuous function to be added to the plot.
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

/// A bar in a bar graph or a histogram.
///
/// See [`Bars`] or [`Histogram`] for more informations.
pub(crate) struct Bar {
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

/// A bars graph.
///
/// All bars are 1 unit wide.
///
/// # Examples
///
/// ```rust
/// use termplot::*;
///
/// let mut plot = Plot::default();
///
/// plot.set_domain(Domain(0.0..6.0))
///     .set_codomain(Domain(0.0..10.0))
///     .set_title("Graph title")
///     .set_x_label("X axis")
///     .set_y_label("Y axis")
///     .set_size(Size::new(50, 25))
///     .add_plot(Box::new(plot::Bars::new(
///         vec![2.0, 5.0, 1.0, 8.0, 9.0, 1.0],
///     )));
/// ```
pub struct Bars {
    bars: Vec<Bar>,
}

impl Bars {
    /// Create a new bars graph.
    ///
    /// Each value inside `bars_height` represent a bar of the graph. Each value is the height of
    /// the corresponding bar.
    pub fn new(bars_height: Vec<f64>) -> Self {
        let bars = bars_height
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

/// An [histogram](https://en.wikipedia.org/wiki/Histogram) graph. An approximation of the
/// distribution of data.
///
/// # Examples
///
/// ```rust
/// use termplot::*;
/// use rand::Rng;
///
/// let mut rng = rand::thread_rng();
/// let values: Vec<f64> = (0..100).map(|_| rng.gen_range(0.0f64..10.0f64)).collect();
///
/// let mut plot = Plot::default();
///
/// plot.set_domain(Domain(0.0..11.0))
///     .set_codomain(Domain(0.0..45.0))
///     .set_title("Graph title")
///     .set_x_label("X axis")
///     .set_y_label("Y axis")
///     .set_size(Size::new(50, 25))
///     .add_plot(Box::new(plot::Histogram::new(
///         values,
///         vec![0.0..2.0, 2.0..4.0, 4.0..6.0, 6.0..8.0, 8.0..10.0],
///     )));
///
/// println!("{plot}");
/// ```
pub struct Histogram {
    buckets: Vec<Bar>,
}

impl Histogram {
    /// Create an histogram from data and buckets in which the data will be sorted.
    ///
    /// For each given value, the value will increment the count of the bucket in which it resides
    /// inside.
    pub fn new(values: Vec<f64>, buckets_range: Vec<ops::Range<f64>>) -> Self {
        let buckets = buckets_range
            .into_iter()
            .map(|range| Bar {
                x: range.start,
                width: range.end - range.start,
                height: values.iter().filter(|v| range.contains(v)).count() as f64,
            })
            .collect::<Vec<_>>();
        Self { buckets }
    }

    /// Create an histogram from data and a number of buckets.
    ///
    /// All buckets will have the same width, depending on the range of the min and max value and
    /// the number of buckets.
    ///
    /// For each given value, the value will increment the count of the bucket in which it resides
    /// inside.
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

impl DrawView for Histogram {
    fn draw(&self, view: &View, canvas: &mut ViewCanvas) {
        self.buckets
            .iter()
            .for_each(|bucket| bucket.draw(view, canvas));
    }
}
