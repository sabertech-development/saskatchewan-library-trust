//! Deterministic visual QA: rasterize egui's actual meshes without a browser.
//! This inspects the same Rust UI used in WebAssembly. Browser QA is separate.
use eframe::egui::{epaint::Primitive, *};
use saskatchewan_library_funding::app::ProposalApp;
use std::collections::HashMap;

fn main() {
    std::fs::create_dir_all("target/review").unwrap();
    for (name, w, h, route) in [
        ("overview-desktop", 1440, 1100, 0),
        ("atlas-desktop", 1600, 1100, 1),
        ("lab-desktop", 1440, 1200, 4),
        ("allocation-desktop", 1440, 1200, 5),
        ("overview-mobile", 390, 844, 0),
        ("atlas-mobile", 390, 1100, 1),
        ("lab-mobile", 390, 1100, 4),
        ("evidence-mobile", 390, 1100, 7),
        ("menu-mobile", 390, 1100, 0),
    ] {
        let ctx = Context::default();
        let mut app = ProposalApp::with_context(&ctx);
        app.navigate(route);
        let mut textures: HashMap<TextureId, ColorImage> = HashMap::new();
        let mut shapes = Vec::new();
        for frame in 0..5 {
            let events = if name == "menu-mobile" && (frame == 2 || frame == 3) {
                let pos = pos2(339., 41.);
                vec![
                    Event::PointerMoved(pos),
                    Event::PointerButton {
                        pos,
                        button: PointerButton::Primary,
                        pressed: frame == 2,
                        modifiers: Modifiers::NONE,
                    },
                ]
            } else {
                vec![]
            };
            let output = ctx.run(
                RawInput {
                    screen_rect: Some(Rect::from_min_size(Pos2::ZERO, vec2(w as f32, h as f32))),
                    events,
                    ..Default::default()
                },
                |ctx| app.render(ctx),
            );
            for (id, delta) in output.textures_delta.set {
                let ImageData::Color(img) = delta.image;
                if let Some([x, y]) = delta.pos {
                    let tex = textures.get_mut(&id).unwrap();
                    for j in 0..img.size[1] {
                        for i in 0..img.size[0] {
                            tex.pixels[(y + j) * tex.size[0] + x + i] =
                                img.pixels[j * img.size[0] + i];
                        }
                    }
                } else {
                    textures.insert(id, (*img).clone());
                }
            }
            shapes = ctx.tessellate(output.shapes, 1.);
        }
        let mut pixels = vec![0u8; w * h * 4];
        for p in pixels.chunks_exact_mut(4) {
            p.copy_from_slice(&[7, 15, 25, 255]);
        }
        for clipped in shapes {
            let Primitive::Mesh(mesh) = clipped.primitive else {
                continue;
            };
            let Some(tex) = textures.get(&mesh.texture_id) else {
                continue;
            };
            for triangle in mesh.indices.chunks_exact(3) {
                let a = mesh.vertices[triangle[0] as usize];
                let b = mesh.vertices[triangle[1] as usize];
                let c = mesh.vertices[triangle[2] as usize];
                let area = edge(a.pos, b.pos, c.pos);
                if area.abs() < 0.001 {
                    continue;
                }
                let min_x = a
                    .pos
                    .x
                    .min(b.pos.x)
                    .min(c.pos.x)
                    .max(clipped.clip_rect.min.x)
                    .max(0.)
                    .floor() as usize;
                let max_x = a
                    .pos
                    .x
                    .max(b.pos.x)
                    .max(c.pos.x)
                    .min(clipped.clip_rect.max.x)
                    .min(w as f32)
                    .ceil() as usize;
                let min_y = a
                    .pos
                    .y
                    .min(b.pos.y)
                    .min(c.pos.y)
                    .max(clipped.clip_rect.min.y)
                    .max(0.)
                    .floor() as usize;
                let max_y = a
                    .pos
                    .y
                    .max(b.pos.y)
                    .max(c.pos.y)
                    .min(clipped.clip_rect.max.y)
                    .min(h as f32)
                    .ceil() as usize;
                for y in min_y..max_y {
                    for x in min_x..max_x {
                        let pt = pos2(x as f32 + 0.5, y as f32 + 0.5);
                        let u = edge(b.pos, c.pos, pt) / area;
                        let v = edge(c.pos, a.pos, pt) / area;
                        let z = 1. - u - v;
                        if u < 0. || v < 0. || z < 0. {
                            continue;
                        }
                        let uv = a.uv.to_vec2() * u + b.uv.to_vec2() * v + c.uv.to_vec2() * z;
                        let sample = sample(tex, uv);
                        let ca = a.color.to_array();
                        let cb = b.color.to_array();
                        let cc = c.color.to_array();
                        let mut col = [0.; 4];
                        for k in 0..4 {
                            col[k] = (ca[k] as f32 * u + cb[k] as f32 * v + cc[k] as f32 * z)
                                * sample[k]
                                / 255.;
                        }
                        let offset = (y * w + x) * 4;
                        let alpha = col[3] / 255.;
                        for k in 0..3 {
                            pixels[offset + k] = (col[k] + pixels[offset + k] as f32 * (1. - alpha))
                                .clamp(0., 255.)
                                as u8;
                        }
                    }
                }
            }
        }
        image::save_buffer(
            format!("target/review/{name}.png"),
            &pixels,
            w as u32,
            h as u32,
            image::ColorType::Rgba8,
        )
        .unwrap();
        println!("Rendered {name}");
    }
}
fn edge(a: Pos2, b: Pos2, c: Pos2) -> f32 {
    (c.x - a.x) * (b.y - a.y) - (c.y - a.y) * (b.x - a.x)
}
fn sample(t: &ColorImage, uv: Vec2) -> [f32; 4] {
    let x = (uv.x * t.size[0] as f32 - 0.5).clamp(0., (t.size[0] - 1) as f32);
    let y = (uv.y * t.size[1] as f32 - 0.5).clamp(0., (t.size[1] - 1) as f32);
    let ix = x.floor() as usize;
    let iy = y.floor() as usize;
    let fx = x - ix as f32;
    let fy = y - iy as f32;
    let mut out = [0.; 4];
    for (dx, dy, weight) in [
        (0, 0, (1. - fx) * (1. - fy)),
        (1, 0, fx * (1. - fy)),
        (0, 1, (1. - fx) * fy),
        (1, 1, fx * fy),
    ] {
        let p = t.pixels[(iy + dy).min(t.size[1] - 1) * t.size[0] + (ix + dx).min(t.size[0] - 1)]
            .to_array();
        for k in 0..4 {
            out[k] += p[k] as f32 * weight;
        }
    }
    out
}
