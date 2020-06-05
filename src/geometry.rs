use std::option::Option;
use std::f64::INFINITY;
use std::f64::consts::PI;

use crate::rand::prelude::*;

use crate::linalg::Vec3D;
use crate::linalg::dot;

use crate::materials::Material;

pub struct Ray {
    pub origin: Vec3D,
    pub direction: Vec3D,
}

impl Ray {
    pub fn at(&self, t:f64) -> Vec3D {
        self.origin + t * self.direction
    }
}

#[derive(Copy, Clone)]
pub struct Intersection {
    pub point: Vec3D,
    pub distance: f64,
    pub local_normal: Vec3D,
    pub inside: bool,
}

pub trait Intersects {
    fn intersects(&self, ray: &Ray) -> Option<Intersection>;
    fn surface_normal(&self, point: Vec3D) -> Vec3D;
    fn material(&self) -> Material;
}

pub struct Sphere {
    pub origin: Vec3D,
    pub radius: f64,
    pub material: Material,
}

impl Sphere {
    pub fn random_unit_vector() -> Vec3D {
        loop {
            let point = Vec3D::random(-0.5, 0.5, -0.5, 0.5, -0.5, 0.5);
            let norm_squared = point.length().powf(2.0);
            if norm_squared < 1.0 {
                return point.l2_normalize();
            }
        }
    }
}

impl Intersects for Sphere {
    fn surface_normal(&self, point: Vec3D) -> Vec3D {
        (point - self.origin).l2_normalize()
    }

    fn intersects(&self, ray: &Ray) -> Option<Intersection> {
        // using quadratic formula
        let sphere_to_ray = ray.origin - self.origin;
        let a = dot(&ray.direction, &ray.direction);
        let h = dot(&sphere_to_ray, &ray.direction);
        let c = dot(&sphere_to_ray, &sphere_to_ray) - self.radius * self.radius;
        let discriminant = (h * h) - (a * c);
        if discriminant < 0.0 {
            return None;
        }
        let discriminant_sqrt = discriminant.sqrt();
        let t0 = (-h - discriminant_sqrt) / a;
        let t1 = (-h + discriminant_sqrt) / a;
        if t0 < 0.01 && t1 < 0.01 {
            return None;
        }
        let t = if t0 >= 0.01 { t0 }  else { t1 };
        let point = ray.at(t);
        let surface_normal = self.surface_normal(point);
        let mut inside = false;
        if dot(&ray.direction, &surface_normal) > 0.0 {
            inside = true;
        }
        let local_normal = if inside { -surface_normal } else { surface_normal };
        let distance = (point - ray.origin).length();
        let intersection = Intersection {
            point: point,
            distance: distance,
            local_normal: local_normal,
            inside: inside,
        };
        Some(intersection)
    }

    fn material(&self) -> Material {
        self.material
    }
}

pub enum Intersectable {
    Sphere(Sphere),
}

impl Intersects for Intersectable {
    fn surface_normal(&self, point: Vec3D) -> Vec3D {
        match self {
            Intersectable::Sphere(s) => s.surface_normal(point),
        }
    }

    fn intersects(&self, ray: &Ray) -> Option<Intersection> {
        match self {
            Intersectable::Sphere(s) => s.intersects(ray),
        }
    }

    fn material(&self) -> Material {
        match self {
            Intersectable::Sphere(s) => s.material(),
        }
    }
}

pub fn first_intersection<'a>(intersections: Vec<Option<Intersection>>,
                              intersectables: &'a Vec<&Intersectable>)
                              -> Option<(Intersection, &'a Intersectable)> {
    let num_objects = intersectables.len() as usize;
    let mut closest_distance = INFINITY;
    let mut closest_intersectable = intersectables[0];
    let mut closest_intersection = intersections[0];
    for i in 0..num_objects {
        let result = intersections[i];
        match result {
            Some(intersection) => {
                if intersection.distance < closest_distance {
                    closest_distance = intersection.distance;
                    closest_intersectable = intersectables[i];
                    closest_intersection = intersections[i];
                }
            },
            None => {}
        }
    }
    if closest_distance == INFINITY {
        return None;
    }
    Some((closest_intersection?, closest_intersectable))
}
