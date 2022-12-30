///! # termplot
///! An _extensible_ plotting library for CLI applications.
///!
///! <img src="./assets/logo.png" width="300" alt="termplot.rs logo" align="right">
///!
///!  - [Quick Start](#quick-start)
///!  - [Documentation](#documentation)
///!  - [Examples](#examples)
///!     + [Plotting a function](#plotting-a-function)
///!     + [Historigram](#historigram)
///!     + [Composing multiple plots](#composing-multiple-plots)
///!
///! ## Documentation
///! Find the full documentation on [doc.rs](https://docs.rs/termplot/latest/termplot)
///!
///! ## Quick start
///! To use `termplot`, add the crate to your `Cargo.toml`.
///! ```toml
///! [dependencies]
///! termplot = "0.1.0"
///! ```
///!
///! ## Examples
///!
///! ### Plotting a function
///! Here is a quick example of what plotting `sin(x) / x` looks like.
///!
///! ```rust
///! use termplot::*;
///!
///! let mut plot = Plot::default();
///! plot.set_domain(Domain(-10.0..10.0))
///!     .set_codomain(Domain(-0.3..1.2))
///!     .set_title("Graph title")
///!     .set_x_label("X axis")
///!     .set_y_label("Y axis")
///!     .set_size(Size::new(50, 25))
///!     .add_plot(Box::new(plot::Graph::new(|x| x.sin() / x)));
///!
///! println!("{plot}");
///! ```
///! Output of the previous example:
///!
///! <img src="./assets/example-simple.png" width="500" alt="Simple example (plotting)">
///!
///! ### Historigram
///! ```rust
///! use termplot::*;
///! use rand::Rng;
///!
///! let mut rng = rand::thread_rng();
///! let values: Vec<f64> = (0..100).map(|_| rng.gen_range(0.0f64..10.0f64)).collect();
///!
///! let mut plot = Plot::default();
///!
///! plot.set_domain(Domain(0.0..11.0))
///!     .set_codomain(Domain(0.0..45.0))
///!     .set_title("Graph title")
///!     .set_x_label("X axis")
///!     .set_y_label("Y axis")
///!     .set_size(Size::new(50, 25))
///!     .add_plot(Box::new(plot::Historigram::new(
///!         values,
///!         vec![0.0..2.0, 2.0..4.0, 4.0..6.0, 6.0..8.0, 8.0..10.0],
///!     )));
///!
///! println!("{plot}");
///! ```
///! Output of the previous example:
///!
///! <img src="./assets/example-hist.png" width="500" alt="Historigram example">
///!
///! ### Composing multiple plots
///! It is also possible to compose multiple plots:
///!
///! ```rust
///! use termplot::*;
///! use rand::Rng;
///!
///! let mut rng = rand::thread_rng();
///! let values: Vec<f64> = (0..100).map(|_| rng.gen_range(0.0f64..10.0f64)).collect();
///!
///! let mut plot = Plot::default();
///!
///! plot.set_domain(Domain(0.0..11.0))
///!     .set_codomain(Domain(0.0..45.0))
///!     .set_title("Graph title")
///!     .set_x_label("X axis")
///!     .set_y_label("Y axis")
///!     .set_size(Size::new(50, 25))
///!     .add_plot(Box::new(plot::Historigram::new(
///!         values,
///!         vec![0.0..2.0, 2.0..4.0, 4.0..6.0, 6.0..8.0, 8.0..10.0],
///!     )))
///!     .add_plot(Box::new(plot::Graph::new(|x| {
///!         -2.0 * (x - 5.0).powf(2.0) + 40.0
///!     })));
///!
///! println!("{plot}");
///! ```
///!
///! Output of the previous example:
///!
///! <img src="./assets/example-composed.png" width="500" alt="Composed plot example">
///!
///! ## License
///! MIT - Enjoy!
use std::fmt;
use std::ops;

pub mod plot;
mod ticks;

/// A drawable component on the view.
pub trait DrawView {
    /// Draw the component on the given canvas.
    ///
    /// Simply draw on the given canvas lines and/or points. See [`ViewCanvas`] for more
    /// informations.
    ///
    /// `view` is provided only to give context if needed. It gives access to the domain, codomain,
    /// size of the view, etc... See [`View`] for more informations.
    ///
    /// # Examples
    ///
    /// This draws a rectangle centered on both the x and y axis.
    ///
    /// ```rust
    /// use termplot::{ViewCanvas, DrawView, Domain, Size, Plot, View};
    ///
    /// struct Rect;
    ///
    /// impl DrawView for Rect {
    ///     fn draw(&self, _: &View, canvas: &mut ViewCanvas) {
    ///         canvas.line(-2.0, 2.0, 2.0, 2.0);
    ///         canvas.line(2.0, 2.0, 2.0, -2.0);
    ///         canvas.line(-2.0, -2.0, 2.0, -2.0);
    ///         canvas.line(-2.0, 2.0, -2.0, -2.0);
    ///     }
    /// }
    ///
    /// let mut plot = Plot::default();
    /// plot.set_domain(Domain(-5.0..5.0))
    ///     .set_codomain(Domain(-5.0..5.0))
    ///     .set_size(Size::new(50, 50))
    ///     .add_plot(Box::new(Rect));
    ///
    /// println!("{plot}");
    /// ```
    fn draw(&self, view: &View, canvas: &mut ViewCanvas);
}

/// A size.
pub struct Size {
    /// The width.
    w: usize,
    /// The height.
    h: usize,
}

impl Size {
    pub fn new(w: usize, h: usize) -> Self {
        Self { w, h }
    }
}

impl Default for Size {
    fn default() -> Self {
        Self { w: 100, h: 100 }
    }
}

/// A container used for adding plots.
///
/// # Examples
///
/// ```rust
/// use termplot::*;
///
/// let mut plot = Plot::default();
/// plot.set_domain(Domain(-10.0..10.0))
///     .set_codomain(Domain(-0.3..1.2))
///     .set_title("Graph title")
///     .set_x_label("X axis")
///     .set_y_label("Y axis")
///     .set_size(Size::new(50, 25))
///     .add_plot(Box::new(plot::Graph::new(|x| x.sin() / x)));
///
/// println!("{plot}");
/// ```
/// Output:
///
/// <img src="./assets/example-simple.png" width="500" alt="Simple example (plotting)">
pub struct Plot {
    title: String,
    x_label: String,
    y_label: String,
    view: View,
    with_decoration: bool,
}

impl Default for Plot {
    fn default() -> Self {
        Self {
            title: String::new(),
            x_label: String::new(),
            y_label: String::new(),
            view: View::default(),
            with_decoration: true,
        }
    }
}

impl Plot {
    /// Add a plot or graph to the view.
    ///
    /// Multiple types of plots and graphs are already implemented. See [`plot`] for all the types
    /// of available plots and graphs.
    ///
    /// To create a new type of plot, see [`DrawView`].
    ///
    /// # Examples
    ///
    /// ```rust
    /// use termplot::{Plot, plot};
    ///
    /// let mut plot = Plot::default();
    /// plot.add_plot(Box::new(plot::Graph::new(|x| x.sin() / x)));
    ///
    /// println!("{plot}");
    /// ```
    pub fn add_plot(&mut self, plot: Box<dyn DrawView>) -> &mut Self {
        self.view.plots.push(plot);
        self
    }

    /// Set the domain (range of the x axis) of the plot.
    ///
    /// By default the domain is from -10 to 10.
    ///
    /// This function sets the minimum and maximum x values in the graph.
    pub fn set_domain(&mut self, domain: Domain) -> &mut Self {
        self.view.domain = domain;
        self
    }

    /// Set the codomain (range of the y axis) of the plot.
    ///
    /// By default the codomain is from -10 to 10.
    ///
    /// This function sets the minimum and maximum y values in the graph.
    pub fn set_codomain(&mut self, codomain: Domain) -> &mut Self {
        self.view.codomain = codomain;
        self
    }

    /// Set the title of the plot.
    pub fn set_title(&mut self, title: &str) -> &mut Self {
        self.title = String::from(title);
        self
    }

    /// Set the label of the x axis.
    ///
    /// The label is shown at the bottom of the figure.
    pub fn set_x_label(&mut self, label: &str) -> &mut Self {
        self.x_label = String::from(label);
        self
    }

    /// Set the label of the y axis.
    ///
    /// The label is shown at the bottom of the figure.
    pub fn set_y_label(&mut self, label: &str) -> &mut Self {
        self.y_label = String::from(label);
        self
    }

    /// Set the size of the view. This does not include decorations around the plot.
    ///
    /// The size is not the number of chars but the number of pixels. Pixels are smaller than
    /// chars. A char in the terminal is 2 by 4 pixels.
    pub fn set_size(&mut self, size: Size) -> &mut Self {
        self.view.size = size;
        self
    }
}

impl fmt::Display for Plot {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let rows = self.view.drawing(self.with_decoration);
        if !self.with_decoration {
            return write!(f, "{}", rows.join("\n"));
        }
        let width = rows[0].chars().count();
        writeln!(f, "╭{:─^width$}╮", self.title)?;
        for row in rows.iter() {
            writeln!(f, "│{row}│")?;
        }
        writeln!(f, "╰{:─<width$}╯", "")?;
        writeln!(f, " {: ^width$} ", self.x_label)?;
        writeln!(f, " {: ^width$} ", self.y_label)
    }
}

/// A `View` is where the graph and plots are drawn. The view does not includes decorations around
/// the plot (labels, title, border, etc..).
#[derive(Default)]
pub struct View {
    /// Domain (range of the x axis) of the plot or graph.
    pub domain: Domain,
    /// Codomain (range of the y axis) of the plot or graph.
    pub codomain: Domain,

    /// The size of the view. This does not include decorations around the plot.
    ///
    /// The size is not the number of chars but the number of pixels. Pixels are smaller than
    /// chars. A char in the terminal is 2 by 4 pixels.
    pub size: Size,
    plots: Vec<Box<dyn DrawView>>,
}

impl View {
    /// Draw x and y axis onto the view.
    fn draw_axis(&self, canvas: &mut ViewCanvas) {
        canvas.line(self.domain.min(), 0.0, self.domain.max(), 0.0);
        canvas.line(0.0, self.codomain.min(), 0.0, self.codomain.max());
    }

    /// Draw the plots and graphs that were added.
    fn draw_plots(&self, canvas: &mut ViewCanvas) {
        for plot in self.plots.iter() {
            plot.draw(&self, canvas);
        }
    }

    /// Return the plot with labels as a vector of strings.
    ///
    /// This function create a [`ViewCanvas`] and draw elements (like axis and plots) onto the
    /// canvas. It also generates and add the label of the axis.
    pub(crate) fn drawing(&self, with_decoration: bool) -> Vec<String> {
        let mut canvas = ViewCanvas::new(&self);
        self.draw_axis(&mut canvas);
        self.draw_plots(&mut canvas);
        let rows = canvas.rows();
        if !with_decoration {
            return rows;
        }
        let width = rows[0].chars().count();
        let mut out = Vec::new();
        let y_ticks = ticks::YTicks::new(&self.codomain, rows.len(), 2);
        let offset = y_ticks.display_width();
        let x_ticks = ticks::XTicks::new(&self.domain, width, 2);
        for (index, row) in rows.iter().enumerate() {
            out.push(format!("{: >offset$}{row}", y_ticks.get(index)));
        }
        out.push(format!("{: >offset$}{x_ticks}", ""));
        out
    }
}

/// Domain or codomain of a graph.
///
/// [`Domain`] is needed for implementing [`Draw`] for a new type of plot.
pub struct Domain(pub std::ops::Range<f64>);

impl Default for Domain {
    fn default() -> Self {
        Self(-10.0..10.0)
    }
}

impl Domain {
    /// The smallest value of the domain.
    ///
    /// # Examples
    /// ```rust
    /// use termplot::Domain;
    ///
    /// let domain = Domain(-10.0..10.0);
    /// assert_eq!(domain.min(), -10.0);
    pub fn min(&self) -> f64 {
        self.0.start
    }

    /// The largest value of the domain.
    ///
    /// # Examples
    /// ```rust
    /// use termplot::Domain;
    ///
    /// let domain = Domain(-10.0..10.0);
    /// assert_eq!(domain.max(), 10.0);
    /// ```
    pub fn max(&self) -> f64 {
        self.0.end
    }

    /// The range of the domain as an absolute value.
    ///
    /// # Examples
    /// ```rust
    /// use termplot::Domain;
    ///
    /// let domain = Domain(-10.0..10.0);
    /// assert_eq!(domain.range(), 20.0);
    ///
    /// let domain = Domain(8.0..-8.0);
    /// assert_eq!(domain.range(), 16.0);
    /// ```
    pub fn range(&self) -> f64 {
        (self.0.end - self.0.start).abs()
    }

    /// An iterator over the range where a number of steps.
    ///
    /// For example, use this if it's needed to compute the value of each pixel for a plot.
    ///
    /// # Examples
    /// ```rust
    /// use termplot::{Domain};
    ///
    /// let domain = Domain(-10.0..10.0);
    /// let view_width = 100;
    ///
    /// domain
    ///     .iter(view_width)
    ///     .for_each(|x| {
    ///         // draw into the canvas
    ///     });
    /// ```
    /// See how [`plot::Graph`] is implemented for an in depth example.
    pub fn iter(&self, steps: usize) -> DomainIterator {
        DomainIterator::new(self.0.clone(), self.range() / steps as f64)
    }
}

/// An iterator over the domain with a number of steps.
///
/// This is used to only compute specific points of a continuous graph.
///
/// See [`Domain`] for more informations.
pub struct DomainIterator {
    current: f64,
    domain: ops::Range<f64>,
    step_by: f64,
}

impl DomainIterator {
    pub fn new(domain: ops::Range<f64>, step_by: f64) -> Self {
        Self {
            current: domain.start,
            domain,
            step_by,
        }
    }
}

impl Iterator for DomainIterator {
    type Item = f64;

    fn next(&mut self) -> Option<Self::Item> {
        if self.current >= self.domain.end {
            return None;
        }
        let result = self.current;
        self.current += self.step_by;
        Some(result)
    }
}

/// The view where graphs are graphed and plots are plotted.
///
/// Braille characters are use to draw on the canvas. `termplot` uses [`drawille::Canvas`] for
/// darwing.
pub struct ViewCanvas<'view> {
    canvas: drawille::Canvas,
    view: &'view View,
}

impl<'view> ViewCanvas<'view> {
    pub(crate) fn new(view: &'view View) -> Self {
        Self {
            canvas: drawille::Canvas::new(view.size.w as u32, view.size.h as u32),
            view,
        }
    }

    pub(crate) fn rows(&self) -> Vec<String> {
        let rows = self.canvas.rows();
        // println!("{:?}", rows[0].chars().count());
        rows
    }

    fn project_on_canvas(&self, x: f64, y: f64) -> (u32, u32) {
        let height = self.view.size.h as f64;
        let y_tmp = (y - self.view.codomain.min()) / self.view.codomain.range();
        let y = (height - y_tmp * height).round().clamp(0.0, height - 1.0);

        let width = self.view.size.w as f64;
        let x_tmp = (x - self.view.domain.min()) / self.view.domain.range();
        let x = (x_tmp * width).round().clamp(0.0, width - 1.0);

        (x as u32, y as u32)
    }

    /// Draw a line from the point (`x0`, `y0`) to (`x1`, `y1`).
    ///
    /// The coordinate are of the plotting space, and **not the actual pixel's coordinate.**
    ///
    /// This function uses the domain and codomain of the [`View`] to determine which pixels should
    /// be drawn. Therefor the drawn shape is relative to the position of the domain and codomain
    /// of the plotting space.
    pub fn line(&mut self, x0: f64, y0: f64, x1: f64, y1: f64) {
        let (x0, y0) = self.project_on_canvas(x0, y0);
        let (x1, y1) = self.project_on_canvas(x1, y1);
        self.canvas.line(x0, y0, x1, y1);
    }

    /// Draw a point at (`x`, `y`).
    ///
    /// The coordinate are of the plotting space, and **not the actual pixel's coordinate.**
    ///
    /// This function uses the domain and codomain of the [`View`] to determine which pixels should
    /// be drawn. Therefor the drawn shape is relative to the position of the domain and codomain
    /// of the plotting space.
    pub fn point(&mut self, x: f64, y: f64) {
        let (x, y) = self.project_on_canvas(x, y);
        self.canvas.set(x, y);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rand::Rng;

    #[test]
    fn simple() {
        let mut plot = Plot::default();
        plot.set_domain(Domain(-10.0..10.0))
            .set_codomain(Domain(-0.3..1.2))
            .set_title("Graph title")
            .set_x_label("X axis")
            .set_y_label("Y axis")
            .set_size(Size::new(100, 25))
            .add_plot(Box::new(plot::Graph::new(|x| x.sin() / x)));

        println!("{plot}");
    }

    #[test]
    fn historigram() {
        let mut rng = rand::thread_rng();
        let values: Vec<f64> = (0..100).map(|_| rng.gen_range(0.0f64..10.0f64)).collect();
        let mut plot = Plot::default();
        plot.set_domain(Domain(0.0..11.0))
            .set_codomain(Domain(0.0..45.0))
            .set_title("Graph title")
            .set_x_label("X axis")
            .set_y_label("Y axis")
            .set_size(Size::new(100, 25))
            .add_plot(Box::new(plot::Historigram::new(
                values,
                vec![0.0..2.0, 2.0..4.0, 4.0..6.0, 6.0..8.0, 8.0..10.0],
            )));
        println!("{plot}");
    }

    #[test]
    fn composition() {
        let mut rng = rand::thread_rng();
        let values: Vec<f64> = (0..100).map(|_| rng.gen_range(0.0f64..10.0f64)).collect();
        let mut plot = Plot::default();
        plot.set_domain(Domain(0.0..11.0))
            .set_codomain(Domain(0.0..45.0))
            .set_title("Graph title")
            .set_x_label("X axis")
            .set_y_label("Y axis")
            .set_size(Size::new(100, 25))
            .add_plot(Box::new(plot::Historigram::new(
                values,
                vec![0.0..2.0, 2.0..4.0, 4.0..6.0, 6.0..8.0, 8.0..10.0],
            )))
            .add_plot(Box::new(plot::Graph::new(|x| {
                -2.0 * (x - 5.0).powf(2.0) + 40.0
            })));
        println!("{plot}");
    }
}
