mod camera;
mod geometry;
mod mesh;
mod utils;
mod bxdf;
mod spectrum;
mod primitive;
mod material;
mod light;
mod integrator;
mod sampler;
mod texture;
mod accelerator;
mod scene;
mod parser;

use camera::perspective::PerspectiveCamera;
use cgmath::Point2;
use indicatif::{ProgressBar, MultiProgress, ProgressStyle};
use sampler::Sampler;

use crate::parser::parse_scene;
use crate::scene::Scene;
use crate::spectrum::Spectrum;
use crate::camera::{Camera, CameraSample};
use crate::integrator::Integrator;

use rand::random;

use std::sync::Arc;
use std::thread;
use std::env;

pub struct WorldSetting {
    pub n_sample: usize,
    pub n_thread: usize, 
    pub integrator: Arc<Box<dyn Integrator>>,
    pub sampler: Arc<dyn Sampler>,
}

impl WorldSetting {
    pub fn new(n_sample: usize, n_thread: usize, integrator: Arc<Box<dyn Integrator>>, sampler: Arc<dyn Sampler>) -> WorldSetting {
        Self {
            n_sample,
            n_thread,
            integrator,
            sampler
        }
    }
}

fn main() {
    // parse scene filename
    let args: Vec<String> = env::args().collect();
    let scene_filename = if args.len() != 2 {
        eprintln!("Usage: rbrt.exe [scene_filename]");
        return ;
    } else {
        args[1].clone()
    };

    // scene configuration
    let s_configure = std::time::Instant::now();

    let (setting, camera, scene) = parse_scene(&scene_filename);

    let configure_cost = s_configure.elapsed().as_millis();
    println!("CONFIGURATION COST: {} secs", (configure_cost as f64) / 1000.0);

    // render
    let s_render = std::time::Instant::now();
    render(setting.integrator.clone(), camera, scene, &setting);

    let render_cost = s_render.elapsed().as_millis();
    println!("RENDER COST: {} secs", (render_cost as f64) / 1000.0);
}



fn render(integrator: Arc<Box<dyn Integrator>>, camera: PerspectiveCamera, scene: Scene, setting: &WorldSetting) {
    let n_thread = setting.n_thread;
    let n_sample = setting.n_sample;
    // let n_sample = setting.n_sample;
    let camera = Arc::new(camera);

    let res = camera.film.resolution;
    let (width, height) = (res.x, res.y);

    let scene = Arc::new(scene);

    let mut handlers = Vec::new();

    let multi_bar = MultiProgress::new();

    multi_bar.println(format!("{} threads running...", n_thread)).unwrap();

    for tid in 0..n_thread {
        let int = Arc::clone(&integrator);
        let scene = Arc::clone(&scene);
        let camera = Arc::clone(&camera);
        let sampler = Arc::clone(&setting.sampler);
        // set the progress bar
        let bar = multi_bar.add(ProgressBar::new(height as u64));
        bar.set_message(format!("t{}", tid));
        bar.set_style(ProgressStyle::with_template("{msg}  {bar:40.cyan/blue} {pos:>7}/{len:7} [{elapsed_precise}]")
            .unwrap()
            .progress_chars("=>-"));

        // create thread
        let handler = thread::spawn(move || {
            for i in 0..height {
                for j in 0..width {
                    // first render the upper left pixel, then go rightwards and downwards
                    let mut radiance = Spectrum::black();

                    for _count in 0..n_sample {
                        let sample = CameraSample::new(Point2::new(j as f64 + random::<f64>(), i as f64 + random::<f64>()), 0.0);
                        
                        let mut r = camera.generate_ray(sample);

                        radiance += int.li(&mut r, &scene, &sampler);
                    }

                    radiance /= n_sample as f64 * n_thread as f64;
                    camera.film.record(i, j, radiance.tone_mapping());
                }
                bar.inc(1);
            }

            bar.finish();
        });

        handlers.push(handler);
    }

    for handler in handlers {
        handler.join().unwrap();
    }

    camera.film.write_to_image();
}
