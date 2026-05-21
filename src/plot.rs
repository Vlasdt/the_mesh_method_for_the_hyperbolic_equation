use plotly::common::Title;
use plotly::layout::{Axis, Layout};
use plotly::{Plot, Surface};

pub fn visualize_surface_plotly(u: &Vec<Vec<f64>>, x: &Vec<f64>, t: &Vec<f64>) {
    let mut plot = Plot::new();
    let surface = Surface::new(u.clone()).x(x.clone()).y(t.clone());
    plot.add_trace(surface);
    let layout = Layout::new()
        .title(Title::from("Численное решение u(x, t)"))
        .x_axis(Axis::new().title("x"))
        .y_axis(Axis::new().title("t"))
        .z_axis(Axis::new().title("u"));
    plot.set_layout(layout);
    plot.write_html("surface.html");
    println!("3D график сохранён в surface.html");
}
