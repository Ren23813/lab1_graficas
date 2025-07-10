use raylib::prelude::*;
use crate::framebuffer::Framebuffer;

fn line(framebuffer: &mut Framebuffer, start: Vector2, end: Vector2) {
    let mut x0 = start.x.round() as i32;
    let mut y0 = start.y.round() as i32;
    let x1 = end.x.round() as i32;
    let y1 = end.y.round() as i32;

    let dx = (x1 - x0).abs();
    let dy = (y1 - y0).abs();
    let sx = if x0 < x1 { 1 } else { -1 };
    let sy = if y0 < y1 { 1 } else { -1 };

    let mut err = dx - dy;

    loop {
        framebuffer.set_pixel(x0, y0);
        if x0 == x1 && y0 == y1 {
            break;
        }
        let e2 = err * 2;
        if e2 > -dy {
            err -= dy;
            x0 += sx;
        }
        if e2 < dx {
            err += dx;
            y0 += sy;
        }
    }
}


pub fn draw_polygon(framebuffer: &mut Framebuffer, points:  &[Vector2]) {
    let num_points = points.len();
    
    for i in 0..num_points {
        let next_point = if i + 1 < num_points {
            points[i + 1]
        } else {
            points[0] // Regresa al primer punto para cerrar el polígono
        };
        line(framebuffer, points[i], next_point);
    }
}


pub fn paint_polygon(framebuffer: &mut Framebuffer, original_points: &[Vector2]) {
    println!("🔍 Entrando a paint_polygon");
    let mut points = original_points.to_vec();

    let (min_x, max_x) = points.iter().fold((f32::INFINITY, f32::NEG_INFINITY),
        |(mi, ma), p| (mi.min(p.x), ma.max(p.x)));
    let (min_y, max_y) = points.iter().fold((f32::INFINITY, f32::NEG_INFINITY),
        |(mi, ma), p| (mi.min(p.y), ma.max(p.y)));
    let center = Vector2 { x: (min_x + max_x)/2.0, y: (min_y + max_y)/2.0 };

    let threshold = 0.1;
    let max_steps = 100000;
    let mut step = 0;

    loop {
        draw_polygon(framebuffer, &points);
        step += 1;

        if step % 10 == 0 {
            println!("– paso {}", step);
        }
        if step > max_steps {
            eprintln!("❌ Excedido max_steps={} sin converger", max_steps);
            break;
        }

        let mut all_converged = true;
        for p in &mut points {
            let dx = center.x - p.x;
            let dy = center.y - p.y;
            let dist = (dx*dx + dy*dy).sqrt();

            if dist > threshold {
                all_converged = false;
                let delta = dist.min(0.01);
                p.x += dx/dist * delta;
                p.y += dy/dist * delta;
            } else {
                *p = center;
            }
        }

        if all_converged {
            println!("✅ Convergió en {} pasos", step);
            break;
        }
    }
    println!("🔚 Saliendo de paint_polygon");
}



//se puede probar en main, para probar únicamente el funcionamiento de la linea (por si lo quiere probar, dennis/aux :D)
    // let start = Vector2 { x: 100.0, y: 100.0 };
    // let end = Vector2 { x: 500.0, y: 300.0 };

    // // Dibujar la línea
    // line(&mut framebuffer, start, end);