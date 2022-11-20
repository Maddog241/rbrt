mod camera;
mod geometry;
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

use camera::{film::Film, perspective::PerspectiveCamera};
use cgmath::{Point2, Vector3};
use geometry::transform::Transform;
use indicatif::{ProgressBar, MultiProgress, ProgressStyle};

use crate::integrator::path_integrator::PathIntegrator;
use crate::scene::Scene;
use crate::spectrum::Spectrum;
use crate::camera::{Camera, CameraSample};
use crate::integrator::Integrator;

use rand::random;

use clap::Parser;

use std::path::Path;
use std::sync::Arc;
use std::thread;

const WIDTH: usize = 800;
const HEIGHT: usize = 500;
const FRAME: f64 = (WIDTH as f64) / (HEIGHT as f64);

#[derive(Parser)]
#[command(name = "rbrt")]
#[command(author = "Maddog241 <Maddog5XZ@gmail.com")]
#[command(version = "1.0")]
struct Cli {
    /// number of threads to run
    #[arg(short, long)]
    thread: Option<usize>,

    /// number of samples per pixel per thread
    #[arg(short, long)]
    sample: Option<usize>,

    /// maximum recursion depth
    #[arg(short, long)]
    depth: Option<usize>,

    /// relative path for storing the rendered image
    #[arg(short, long)]
    filename: Option<String>,
}

struct Arguments<'a> {
    n_thread: usize,
    n_sample: usize,
    max_depth: usize,
    filename: &'a Path,
}

impl<'a> Arguments<'a> {
    fn new() -> Self {
        // set the default values for the arguments
        Arguments { 
            n_thread: 10,
            n_sample: 20,
            max_depth: 20,
            filename: Path::new("./images/result.ppm"),
        }
    }

    fn process_arguments(&mut self, cli: &'a Cli) {
        if let Some(n_thread) =  &cli.thread {
            self.n_thread = *n_thread;
        }

        if let Some(n_sample) = &cli.sample {
            self.n_sample = *n_sample;
        }

        if let Some(max_depth) = &cli.depth {
            self.max_depth = *max_depth;
        }

        if let Some(filename) = &cli.filename {
            self.filename = Path::new(filename);
        }


        eprintln!("CURRENT CONFIG:");
        eprintln!("\tn_thread: {}", self.n_thread);
        eprintln!("\tn_sample: {}", self.n_sample);
        eprintln!("\tmax_depth: {}", self.max_depth);
        eprintln!("\tfilename: {:?}", self.filename);
    }
}


fn main() {
    // receiving command line arguments
    let cli = Cli::parse();
    let mut args = Arguments::new();
    args.process_arguments(&cli);

    // create camera
    let pos = Vector3::new(0.0, 0.0, 0.0);
    let look = Vector3::new(0.0, 0.0, 1.0);
    let up = Vector3::new(0.0, 1.0, 0.0);
    let camera_to_world = Transform::look_at(pos, look, up).inverse();

    let camera = PerspectiveCamera::new(
        camera_to_world,
        (Point2::new(-FRAME, -1.0), Point2::new(FRAME, 1.0)),
        0.0,
        1.0,
        60.0,
        Film::new(WIDTH, HEIGHT),
    );

    let scene = Scene::test_obj();

    // render
    let now = std::time::Instant::now();

    let integrator = PathIntegrator::new(camera, args.max_depth, args.n_sample, args.n_thread);
    render(integrator, scene, &args.filename);

    let cost = now.elapsed().as_millis();
    println!("RENDER COST: {} secs", (cost as f64) / 1000.0);
}



fn render(integrator: PathIntegrator, scene: Scene, filename: &Path) {
    let res = integrator.camera.film.resolution;
    let (width, height) = (res.x, res.y);

    let integrator = Arc::new(integrator);
    let scene = Arc::new(scene);

    let mut handlers = Vec::new();

    let multi_bar = MultiProgress::new();

    multi_bar.println(format!("{} threads running...", integrator.n_thread)).unwrap();

    for tid in 0..integrator.n_thread {
        let int = Arc::clone(&integrator);
        let scene = Arc::clone(&scene);
        // set the progress bar
        let bar = multi_bar.add(ProgressBar::new(height as u64));
        bar.set_message(format!("t{}", tid));
        bar.set_style(ProgressStyle::with_template("{msg}  {bar:40.cyan/blue} {pos:>7}/{len:7} [{elapsed_precise}]")
            .unwrap()
            .progress_chars("=>-"));

        // create process
        let handler = thread::spawn(move || {
            for i in 0..height {
                for j in 0..width {
                    // first render the upper left pixel, then go rightwards and downwards
                    let mut radiance = Spectrum::new(0.0, 0.0, 0.0);
                    
                    for _ in 0..int.n_sample {
                        let sample = CameraSample::new(Point2::new(j as f64 + random::<f64>(), i as f64 + random::<f64>()), 0.0);
                        let mut r = int.camera.generate_ray(sample);

                        radiance += int.li(&mut r, &scene);
                    }

                    radiance /= int.n_sample as f64 * int.n_thread as f64;

                    int.camera.film.record(i, j, radiance);
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

    integrator.camera.film.write_to_image(filename);
}
