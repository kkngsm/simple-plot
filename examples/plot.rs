
fn main(){
    let range = 0..100;
    let sin_iter = (0..150).map(|x| (x as f32 / 10.0).sin() * 20.0);
    let parabola_vec:Vec<i32> = (-10..10).map(|x| x*x - 30).collect();
    let cos_vec: Vec<f32> = (0..150).map(|x| (x as f32 / 20.0).cos() * 10.0).collect();
    let cos_slcie: &[f32] = &cos_vec;

    simple_plot::plot!("title", range, sin_iter, parabola_vec, cos_slcie);
}