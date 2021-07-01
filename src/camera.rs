use crate::{
    canvas::Canvas,
    ray::Ray,
    transformations::{self, Transform},
    tuple::Tuple,
    world::World,
};

const MAX_RECURSIVE_DEPTH: u32 = 3;

pub struct Camera {
    hsize: usize,
    vsize: usize,
    field_of_view: f32,
    transform: Transform,
}

impl Camera {
    pub fn new(hsize: usize, vsize: usize, field_of_view: f32) -> Self {
        Self {
            hsize,
            vsize,
            field_of_view,
            transform: transformations::IDENTITY,
        }
    }

    pub fn transform(self, transform: Transform) -> Self {
        Self { transform, ..self }
    }

    pub fn pixel_size(&self) -> f32 {
        (self.half_width() * 2.0) / self.hsize as f32
    }

    pub fn ray_for_pixel(&self, px: usize, py: usize) -> Ray {
        let xoffset = (px as f32 + 0.5) * self.pixel_size();
        let yoffset = (py as f32 + 0.5) * self.pixel_size();

        let world_x = self.half_width() - xoffset;
        let world_y = self.half_height() - yoffset;

        let pixel = self.transform.inverse() * Tuple::point(world_x, world_y, -1.0);
        let origin = self.transform.inverse() * Tuple::point(0.0, 0.0, 0.0);
        let direction = (pixel - origin).normalize();

        Ray::new(origin, direction)
    }

    pub fn render(&self, world: &World) -> Canvas {
        let mut image = Canvas::new(self.hsize, self.vsize);

        for y in 0..self.vsize {
            for x in 0..self.hsize {
                let ray = self.ray_for_pixel(x, y);
                let color = world.color_at(ray, MAX_RECURSIVE_DEPTH);
                image.write_pixel(x, y, color);
            }
        }

        image
    }

    fn aspect(&self) -> f32 {
        self.hsize as f32 / self.vsize as f32
    }

    fn half_view(&self) -> f32 {
        (self.field_of_view / 2.0).tan()
    }

    fn half_width(&self) -> f32 {
        let aspect = self.aspect();
        if aspect >= 1.0 {
            self.half_view()
        } else {
            self.half_view() * self.aspect()
        }
    }

    fn half_height(&self) -> f32 {
        let aspect = self.aspect();
        if aspect >= 1.0 {
            self.half_view() / aspect
        } else {
            self.half_view()
        }
    }
}

#[cfg(test)]
mod tests {
    use std::f32::consts::{PI, SQRT_2};

    use crate::{color::Color, float_eq};

    use super::*;

    #[test]
    fn constructing_a_camera() {
        let hsize = 160;
        let vsize = 120;
        let field_ov_view = PI / 2.0;

        let c = Camera::new(hsize, vsize, field_ov_view);

        assert_eq!(c.hsize, 160);
        assert_eq!(c.vsize, 120);
        assert!(float_eq(c.field_of_view, PI / 2.0));
        assert_eq!(c.transform, transformations::IDENTITY);
    }

    #[test]
    fn pixel_size_for_a_horizontal_canvas() {
        let c = Camera::new(200, 125, PI / 2.0);

        assert!(float_eq(c.pixel_size(), 0.01));
    }

    #[test]
    fn pixel_size_for_a_vertical_canvas() {
        let c = Camera::new(125, 200, PI / 2.0);

        assert!(float_eq(c.pixel_size(), 0.01));
    }

    #[test]
    fn ray_through_center_of_canvas() {
        let c = Camera::new(201, 101, PI / 2.0);

        let r = c.ray_for_pixel(100, 50);

        assert_eq!(r.origin, Tuple::point(0.0, 0.0, 0.0));
        assert_eq!(r.direction, Tuple::vector(0.0, 0.0, -1.0));
    }

    #[test]
    fn ray_through_corner_of_canvas() {
        let c = Camera::new(201, 101, PI / 2.0);

        let r = c.ray_for_pixel(0, 0);

        assert_eq!(r.origin, Tuple::point(0.0, 0.0, 0.0));
        assert_eq!(r.direction, Tuple::vector(0.66519, 0.33259, -0.66851));
    }

    #[test]
    fn ray_when_camera_is_transformed() {
        let mut c = Camera::new(201, 101, PI / 2.0);

        c.transform = Transform::rotation_y(PI / 4.0) * Transform::translation(0.0, -2.0, 5.0);
        let r = c.ray_for_pixel(100, 50);

        assert_eq!(r.origin, Tuple::point(0.0, 2.0, -5.0));
        assert_eq!(r.direction, Tuple::vector(SQRT_2 / 2.0, 0.0, -SQRT_2 / 2.0));
    }

    #[test]
    fn rendering_world_with_camera() {
        let w = World::default();
        let mut c = Camera::new(11, 11, PI / 2.0);
        let from = Tuple::point(0.0, 0.0, -5.0);
        let to = Tuple::point(0.0, 0.0, 0.0);
        let up = Tuple::vector(0.0, 1.0, 0.0);
        c.transform = Transform::view_transform(from, to, up);

        let image = c.render(&w);

        assert_eq!(image.pixel_at(5, 5), Color::new(0.38066, 0.47583, 0.2855));
    }
}
