use nannou::prelude::*;
use nannou::noise::NoiseFn;

const ORDER: usize = 6;

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
    let texture_size = [6_000, 6_000];

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
    let elapsed_frames = app.main_window().elapsed_frames();
    if elapsed_frames > 0 { return }

    let draw = &model.draw;
    draw.reset();

    let [w, _h] = model.texture.size();
    let elapsed_frames = app.main_window().elapsed_frames();

    draw.background().color(BLACK);
    let n = pow(2, ORDER) as usize;
    let total = n * n;
    let mut path = vec![];
    let nn = nannou::noise::OpenSimplex::new();

    let width = w as f32 * 0.8;
    let k1 = 0.0025;
    let k2 = 0.04;

    for i in 0..total {
        path.push(hilbert(i, ORDER));
        let m = width / n as f32;
        path[i] *= m;
        path[i] += vec2(m / 2.0, m / 2.0);
        let x = path[i].x;
        let y = path[i].y;
        let delta_x = k2  * width * nn.get([k1 * x as f64, k1 * y as f64, 0.0]) as f32;
        let delta_y = k2 * width * nn.get([k1 * x as f64, k1 * y as f64, 0.1]) as f32;
        path[i] = pt2(x + delta_x, y + delta_y);
    }
    path = path
        .into_iter()
        .map(|p| p - vec2(width / 2.0, width / 2.0))
        .collect();
    draw.polyline()
        .weight(10.0)
        .join_round()
        .color(WHITE)
        .points(path);

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

    let path = capture_directory(app)
        .join(elapsed_frames.to_string())
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

fn hilbert(k: usize, order: usize) -> Point2 {
    let points = vec![pt2(0.0, 0.0), pt2(0.0, 1.0), pt2(1.0, 1.0), pt2(1.0, 0.0)];
    let mut v = points[k & 3];
    let mut i = k;

    for j in 1..order {
        i >>= 2;
        let index = i & 3;
        let n = pow(2, j) as f32;
        match index {
            0 => {
                let temp = v.x;
                v.x = v.y;
                v.y = temp;
            }
            1 => {
                v.y += n;
            }
            2 => {
                v.x += n;
                v.y += n;
            }
            3 => {
                let temp = n - 1.0 - v.x;
                v.x = n - 1.0 - v.y;
                v.y = temp;
                v.x += n;
            }
            _ => {}
        }
    }
    v
}