use std::fs;

use canvas::Canvas;
use color::Color;
use float_cmp::F64Margin;
use light::PointLight;
use material::Material;
use ray::Ray;
use sphere::Sphere;
use tuple::Tuple;

mod canvas;
mod color;
mod light;
mod material;
mod matrix;
mod ray;
mod sphere;
mod tuple;

#[cfg(test)]
mod test;

pub const MARGIN: F64Margin = F64Margin {
    ulps: 2,
    epsilon: 0.00001,
};

fn main() {
    let ray_origin = Tuple::point(0.0, 0.0, -5.0);
    let wall_z = 10.0;
    let wall_size = 7.0;
    let canvas_pixels = 100;
    let pixel_size = wall_size / canvas_pixels as f64;
    let half = wall_size / 2.0;

    let mut c = Canvas::new(canvas_pixels, canvas_pixels);
    let mut shape = Sphere::default();
    shape.material = Material::default();

    let light = PointLight::new(Tuple::point(-10.0, 10.0, -10.0), Color::new(1.0, 1.0, 1.0));

    for y in 0..canvas_pixels - 1 {
        let world_y = half - pixel_size * y as f64;
        for x in 0..canvas_pixels - 1 {
            let world_x = -half + pixel_size * x as f64;

            let position = Tuple::point(world_x, world_y, wall_z);
            let ray = Ray::new(ray_origin, (position - ray_origin).normalize());
            let xs = shape.intersect(ray);
            if let Some(hit) = xs.hit() {
                let point = ray.position(hit.t);
                if let Ok(normal) = hit.object.normal_at(point) {
                    let eye = -ray.direction;
                    let color = hit.object.material.lighting(light, point, eye, normal);
                    c.write_pixel(x, y, color);
                }
            }
        }
    }

    fs::write("canvas.ppm", c.to_ppm()).unwrap();
}
