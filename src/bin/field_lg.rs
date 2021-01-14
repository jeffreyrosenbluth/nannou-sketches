use getopts::Options;
use nannou::prelude::*;
use nannou::noise::NoiseFn;
use std::env;


use sketches::{random_rgba, Grid};

const WIDTH: f32 = 15_000.0;
const HEIGHT: f32 = 12_500.0;
const GRID_SPACING: f32 = 50.0;
const LENGTH: usize = 200;
const STEP: f32 = 250.0;
const K: f64 = 0.0015;

fn scene(draw: &Draw, _w: u32, _h: u32) {
    draw.background().color(CORNSILK);

    let nn = nannou::noise::BasicMulti::new();

    let grid = Grid::new(1.1 * WIDTH, 1.1 * HEIGHT, GRID_SPACING, |x, y| {
        TAU * nn.get([K * x as f64, K * y as f64]) as f32
    });

    for i in 0..(grid.cols() / 4) {
        let mut l1 = pt2(
            -WIDTH / 2.0 +  GRID_SPACING * i as f32,
            0.0,
        );
        let mut l2 = pt2(
            -WIDTH / 2.0 + GRID_SPACING * (i + 1) as f32,
            0.0,
        );
        let mut up = vec![];
        for _i in 0..LENGTH {
            up.push(l1);
            let angle = &grid.get(l1.x, l1.y);
            l1.x += STEP * angle.cos();
            l1.y += STEP * angle.sin();
        }
        let mut dn = vec![];
        for _i in 0..LENGTH {
            dn.push(l2);
            let angle = &grid.get(l2.x, l2.y);
            l2.x += STEP * angle.cos();
            l2.y += STEP * angle.sin();
        }
        dn.reverse();
        up.extend(dn);

        draw.polygon().points(up).color(random_rgba());
    }

}

fn main() {
    nannou::app(model).update(update).exit(exit).run();
}

struct Model {
    texture: wgpu::Texture,
    draw: nannou::Draw,
    renderer: nannou::draw::Renderer,
    texture_capturer: wgpu::TextureCapturer,
    texture_reshaper: wgpu::TextureReshaper,
}

fn model(app: &App) -> Model {
    let texture_size = [WIDTH as u32, HEIGHT as u32];

    let [win_w, win_h] = [texture_size[0] / 10, texture_size[1] / 10];
    let w_id = app
        .new_window()
        .size(win_w, win_h)
        .title("nannou")
        .view(view)
        .build()
        .unwrap();
    let window = app.window(w_id).unwrap();

    let device = window.swap_chain_device();

    let sample_count = window.msaa_samples();
    let texture = wgpu::TextureBuilder::new()
        .size(texture_size)
        .usage(wgpu::TextureUsage::OUTPUT_ATTACHMENT | wgpu::TextureUsage::SAMPLED)
        .sample_count(sample_count)
        .format(wgpu::TextureFormat::Rgba16Float)
        .build(device);

    let draw = nannou::Draw::new();
    let descriptor = texture.descriptor();
    let renderer =
        nannou::draw::RendererBuilder::new().build_from_texture_descriptor(device, descriptor);

    let texture_capturer = wgpu::TextureCapturer::default();

    let texture_view = texture.create_default_view();
    let texture_component_type = texture.component_type();
    let dst_format = Frame::TEXTURE_FORMAT;
    let texture_reshaper = wgpu::TextureReshaper::new(
        device,
        &texture_view,
        sample_count,
        texture_component_type,
        sample_count,
        dst_format,
    );

    std::fs::create_dir_all(&capture_directory(app)).unwrap();

    Model {
        texture,
        draw,
        renderer,
        texture_capturer,
        texture_reshaper,
    }
}

fn update(app: &App, model: &mut Model, _update: Update) {
    let args: Vec<String> = env::args().collect();
    let mut opts = Options::new();
    opts.optflag("p", "png", "save frames to file as png.");
    let matches = match opts.parse(&args[1..]) {
        Ok(m) => m,
        Err(f) => panic!(f.to_string()),
    };
    let png = matches.opt_present("p");

    let elapsed_frames = app.main_window().elapsed_frames();
    if elapsed_frames > 0 {
        return;
    }

    let draw = &model.draw;
    draw.reset();

    let [w, h] = model.texture.size();

    scene(draw, w, h);

    let window = app.main_window();
    let device = window.swap_chain_device();
    let ce_desc = wgpu::CommandEncoderDescriptor {
        label: Some("texture renderer"),
    };

    let mut encoder = device.create_command_encoder(&ce_desc);
    model
        .renderer
        .render_to_texture(device, &mut encoder, draw, &model.texture);

    let snapshot = model
        .texture_capturer
        .capture(device, &mut encoder, &model.texture);

    window.swap_chain_queue().submit(&[encoder.finish()]);

    if png {
        let path = capture_directory(app)
            .join("image")
            .with_extension("png");
        snapshot
            .read(move |result| {
                let image = result.expect("failed to map texture memory");
                image
                    .save(&path)
                    .expect("failed to save texture to png image");
            })
            .unwrap();
    }
    app.set_loop_mode(LoopMode::loop_once());
}

fn view(_app: &App, model: &Model, frame: Frame) {
    let mut encoder = frame.command_encoder();
    model
        .texture_reshaper
        .encode_render_pass(frame.texture_view(), &mut *encoder);
}

fn exit(app: &App, model: Model) {
    println!("Waiting for PNG writing to complete...");
    let window = app.main_window();
    let device = window.swap_chain_device();
    model
        .texture_capturer
        .await_active_snapshots(&device)
        .unwrap();
    println!("Done!");
}

fn capture_directory(app: &App) -> std::path::PathBuf {
    app.project_path()
        .expect("could not locate project_path")
        .join(app.exe_name().unwrap())
}