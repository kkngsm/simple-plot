//! # simple-plot
//! Provides a macro plot!() that plots a one-dimensional vector (impl IntoIterator<Item={number}>) using Plotly. 
//! 
//! There is no need for complicated settings; simply pass to plot!() the title of the graph and the vectors you wish to plot, and the graph will be displayed. 
//! 
//! The passed vector is plotted with index on the x-axis and elements as values on the y-axis.
//! 
//! # What this library cannot do 
//! - Changing the value of the x-axis
//! - Changing the color of a graph
//! etc.
//! 
//! If you need these functions, use plotly 
//! 
//! # Examples
//! ```
//! let range = 0..100;
//! let sin_iter = (0..150).map(|x| (x as f32 / 10.0).sin() * 20.0);
//! let parabola_vec:Vec<i32> = (-10..10).map(|x| x*x - 30).collect();
//! let cos_vec: Vec<f32> = (0..150).map(|x| (x as f32 / 20.0).cos() * 10.0).collect();
//! let cos_slcie: &[f32] = &cos_vec;
//!
//! simple_plotly::plot!("title", range, sin_iter, parabola_vec, cos_slcie);
//! ```


pub use plotly;
/// Plot a one-dimensional vector using Plotly. 
/// 
/// The passed vector is plotted with index on the x-axis and elements as values on the y-axis.
/// 
/// First argument : `&str` Title of the graph
/// 
/// Second and subsequent arguments: `impl IntoIterator<Item={number}>` Vector(s) to plot.
///
/// # Examples 
/// ```
/// let range = 0..100;
/// let sin_iter = (0..150).map(|x| (x as f32 / 10.0).sin() * 20.0);
/// simple_plotly::plot!("title", range, sin_iter);
/// ```
#[macro_export]
macro_rules! plot {
    ($title:expr, $($ys:expr), +) => {
        use simple_plot::plotly::common::{Mode, Title};
        use simple_plot::plotly::layout::Layout;
        use simple_plot::plotly::{Plot, Scatter};
        let layout = Layout::new().title(Title::new($title));
        let mut plot = Plot::new();
        $(
            let name = stringify!($ys);
            let (xs, ys):(Vec<_>, Vec<_>) = $ys.into_iter().enumerate().map(|(i, x)| (i as f32, x.clone() as f32)).unzip();
            let trace = Scatter::new(xs, ys)
                .mode(Mode::Lines)
                .name(name);
            plot.add_trace(trace);
        )+
        plot.set_layout(layout);
        plot.show();
    }
}