use std::{
    f32::{self, consts::PI},
    panic,
};

use raylib::prelude::*;
use ultraviolet::{Rotor2, Vec2};

const AIR_INDEX_OF_REFRACTION: f32 = 1.000293;

fn main() {
    let (mut rhandle, mut thread) = raylib::init()
        .size(800, 600)
        .title("Lens simulation")
        .resizable()
        .msaa_4x()
        .vsync()
        .undecorated()
        .build();

    rhandle.set_target_fps(60);
    let mut light = Light {
        ray_count: 1,
        position: Vec2 { x: -10.0, y: 0.0 },
    };

    // Concave lens (diverging)
    let concave = vec![
        Shape::Line(Line {
            end: Vec2 { x: 12.5, y: 4.0 },
            start: Vec2 { x: 13.0, y: 5.0 },
        }),
        Shape::Line(Line {
            end: Vec2 { x: 12.2, y: 3.0 },
            start: Vec2 { x: 12.5, y: 4.0 },
        }),
        Shape::Line(Line {
            end: Vec2 { x: 12.0, y: 2.0 },
            start: Vec2 { x: 12.2, y: 3.0 },
        }),
        Shape::Line(Line {
            end: Vec2 { x: 12.0, y: 0.0 },
            start: Vec2 { x: 12.0, y: 2.0 },
        }),
        Shape::Line(Line {
            end: Vec2 { x: 12.0, y: -2.0 },
            start: Vec2 { x: 12.0, y: 0.0 },
        }),
        Shape::Line(Line {
            end: Vec2 { x: 12.2, y: -3.0 },
            start: Vec2 { x: 12.0, y: -2.0 },
        }),
        Shape::Line(Line {
            end: Vec2 { x: 12.5, y: -4.0 },
            start: Vec2 { x: 12.2, y: -3.0 },
        }),
        Shape::Line(Line {
            end: Vec2 { x: 13.0, y: -5.0 },
            start: Vec2 { x: 12.5, y: -4.0 },
        }),
        Shape::Line(Line {
            end: Vec2 { x: 13.5, y: -4.0 },
            start: Vec2 { x: 13.0, y: -5.0 },
        }),
        Shape::Line(Line {
            end: Vec2 { x: 13.8, y: -3.0 },
            start: Vec2 { x: 13.5, y: -4.0 },
        }),
        Shape::Line(Line {
            end: Vec2 { x: 14.0, y: -2.0 },
            start: Vec2 { x: 13.8, y: -3.0 },
        }),
        Shape::Line(Line {
            end: Vec2 { x: 14.0, y: 0.0 },
            start: Vec2 { x: 14.0, y: -2.0 },
        }),
        Shape::Line(Line {
            end: Vec2 { x: 14.0, y: 2.0 },
            start: Vec2 { x: 14.0, y: 0.0 },
        }),
        Shape::Line(Line {
            end: Vec2 { x: 13.8, y: 3.0 },
            start: Vec2 { x: 14.0, y: 2.0 },
        }),
        Shape::Line(Line {
            end: Vec2 { x: 13.5, y: 4.0 },
            start: Vec2 { x: 13.8, y: 3.0 },
        }),
        Shape::Line(Line {
            end: Vec2 { x: 13.0, y: 5.0 },
            start: Vec2 { x: 13.5, y: 4.0 },
        }),
    ];

    // Triangular prism (classic rainbow maker)
    let triangle_prism = vec![
        Shape::Line(Line {
            end: Vec2 { x: 10.0, y: 5.0 },
            start: Vec2 { x: 14.0, y: 0.0 },
        }),
        Shape::Line(Line {
            end: Vec2 { x: 14.0, y: 0.0 },
            start: Vec2 { x: 10.0, y: -5.0 },
        }),
        Shape::Line(Line {
            end: Vec2 { x: 10.0, y: -5.0 },
            start: Vec2 { x: 10.0, y: 5.0 },
        }),
    ];

    // Flat mirror (for total internal reflection demos)
    let mirror = vec![Shape::Line(Line {
        end: Vec2 { x: 14.0, y: -6.0 },
        start: Vec2 { x: 14.0, y: 6.0 },
    })];

    // Cylinder lens (focuses only in one direction)
    let cylinder = vec![
        Shape::Line(Line {
            start: Vec2 { x: 15.0, y: 5.0 },
            end: Vec2 { x: 14.0, y: 5.0 },
        }),
        Shape::Line(Line {
            start: Vec2 { x: 15.0, y: -5.0 },
            end: Vec2 { x: 15.0, y: 5.0 },
        }),
        Shape::Line(Line {
            start: Vec2 { x: 14.0, y: -5.0 },
            end: Vec2 { x: 15.0, y: -5.0 },
        }),
        Shape::Line(Line {
            start: Vec2 { x: 13.0, y: -4.0 },
            end: Vec2 { x: 14.0, y: -5.0 },
        }),
        Shape::Line(Line {
            start: Vec2 { x: 12.5, y: -2.0 },
            end: Vec2 { x: 13.0, y: -4.0 },
        }),
        Shape::Line(Line {
            start: Vec2 { x: 12.5, y: 2.0 },
            end: Vec2 { x: 12.5, y: -2.0 },
        }),
        Shape::Line(Line {
            start: Vec2 { x: 13.0, y: 4.0 },
            end: Vec2 { x: 12.5, y: 2.0 },
        }),
        Shape::Line(Line {
            start: Vec2 { x: 14.0, y: 5.0 },
            end: Vec2 { x: 13.0, y: 4.0 },
        }),
    ];

    // Diamond (interesting multi-facet refraction)
    let diamond = vec![
        Shape::Line(Line {
            start: Vec2 { x: 16.0, y: 0.0 },
            end: Vec2 { x: 14.0, y: 3.0 },
        }),
        Shape::Line(Line {
            start: Vec2 { x: 14.0, y: -3.0 },
            end: Vec2 { x: 16.0, y: 0.0 },
        }),
        Shape::Line(Line {
            start: Vec2 { x: 12.0, y: 0.0 },
            end: Vec2 { x: 14.0, y: -3.0 },
        }),
        Shape::Line(Line {
            start: Vec2 { x: 14.0, y: 3.0 },
            end: Vec2 { x: 12.0, y: 0.0 },
        }),
    ];

    // Meniscus lens (common in eyeglasses)
    let meniscus = vec![
        Shape::Line(Line {
            start: Vec2 { x: 15.0, y: 4.0 },
            end: Vec2 { x: 14.0, y: 5.0 },
        }),
        Shape::Line(Line {
            start: Vec2 { x: 15.5, y: 2.0 },
            end: Vec2 { x: 15.0, y: 4.0 },
        }),
        Shape::Line(Line {
            start: Vec2 { x: 15.5, y: -2.0 },
            end: Vec2 { x: 15.5, y: 2.0 },
        }),
        Shape::Line(Line {
            start: Vec2 { x: 15.0, y: -4.0 },
            end: Vec2 { x: 15.5, y: -2.0 },
        }),
        Shape::Line(Line {
            start: Vec2 { x: 14.0, y: -5.0 },
            end: Vec2 { x: 15.0, y: -4.0 },
        }),
        Shape::Line(Line {
            start: Vec2 { x: 13.0, y: -3.5 },
            end: Vec2 { x: 14.0, y: -5.0 },
        }),
        Shape::Line(Line {
            start: Vec2 { x: 12.5, y: -2.0 },
            end: Vec2 { x: 13.0, y: -3.5 },
        }),
        Shape::Line(Line {
            start: Vec2 { x: 12.3, y: 0.0 },
            end: Vec2 { x: 12.5, y: -2.0 },
        }),
        Shape::Line(Line {
            start: Vec2 { x: 12.5, y: 2.0 },
            end: Vec2 { x: 12.3, y: 0.0 },
        }),
        Shape::Line(Line {
            start: Vec2 { x: 13.0, y: 3.5 },
            end: Vec2 { x: 12.5, y: 2.0 },
        }),
        Shape::Line(Line {
            start: Vec2 { x: 14.0, y: 5.0 },
            end: Vec2 { x: 13.0, y: 3.5 },
        }),
    ];

    // Asymmetric prism (creates interesting beam steering)
    let asymmetric_prism = vec![
        Shape::Line(Line {
            start: Vec2 { x: 12.0, y: 5.0 },
            end: Vec2 { x: 10.0, y: -5.0 },
        }),
        Shape::Line(Line {
            start: Vec2 { x: 16.0, y: -2.0 },
            end: Vec2 { x: 12.0, y: 5.0 },
        }),
        Shape::Line(Line {
            start: Vec2 { x: 10.0, y: -5.0 },
            end: Vec2 { x: 16.0, y: -2.0 },
        }),
    ];

    // Two line prism/wedge
    let prism = vec![
        Shape::Line(Line {
            start: Vec2 { x: 10.0, y: 5.0 },
            end: Vec2 { x: 15.0, y: -5.0 },
        }),
        Shape::Line(Line {
            start: Vec2 { x: 18.0, y: -5.0 },
            end: Vec2 { x: 25.0, y: 5.0 },
        }),
    ];

    // Octagonal lens
    let octagon = vec![
        Shape::Line(Line {
            start: Vec2 { x: 14.0, y: 5.0 },
            end: Vec2 { x: 13.0, y: 3.0 },
        }),
        Shape::Line(Line {
            start: Vec2 { x: 13.0, y: 3.0 },
            end: Vec2 { x: 13.0, y: -3.0 },
        }),
        Shape::Line(Line {
            start: Vec2 { x: 13.0, y: -3.0 },
            end: Vec2 { x: 14.0, y: -5.0 },
        }),
        Shape::Line(Line {
            start: Vec2 { x: 14.0, y: -5.0 },
            end: Vec2 { x: 15.0, y: -3.0 },
        }),
        Shape::Line(Line {
            start: Vec2 { x: 15.0, y: -3.0 },
            end: Vec2 { x: 15.0, y: 3.0 },
        }),
        Shape::Line(Line {
            start: Vec2 { x: 15.0, y: 3.0 },
            end: Vec2 { x: 14.0, y: 5.0 },
        }),
    ];

    let biconvex = vec![
        Shape::Line(Line {
            start: Vec2 { x: 14.0, y: 5.0 },
            end: Vec2 { x: 13.5, y: 4.0 },
        }),
        Shape::Line(Line {
            start: Vec2 { x: 13.5, y: 4.0 },
            end: Vec2 { x: 13.2, y: 3.0 },
        }),
        Shape::Line(Line {
            start: Vec2 { x: 13.2, y: 3.0 },
            end: Vec2 { x: 13.0, y: 2.0 },
        }),
        Shape::Line(Line {
            start: Vec2 { x: 13.0, y: 2.0 },
            end: Vec2 { x: 13.0, y: 0.0 },
        }),
        Shape::Line(Line {
            start: Vec2 { x: 13.0, y: 0.0 },
            end: Vec2 { x: 13.0, y: -2.0 },
        }),
        Shape::Line(Line {
            start: Vec2 { x: 13.0, y: -2.0 },
            end: Vec2 { x: 13.2, y: -3.0 },
        }),
        Shape::Line(Line {
            start: Vec2 { x: 13.2, y: -3.0 },
            end: Vec2 { x: 13.5, y: -4.0 },
        }),
        Shape::Line(Line {
            start: Vec2 { x: 13.5, y: -4.0 },
            end: Vec2 { x: 14.0, y: -5.0 },
        }),
        Shape::Line(Line {
            start: Vec2 { x: 14.0, y: -5.0 },
            end: Vec2 { x: 14.5, y: -4.0 },
        }),
        Shape::Line(Line {
            start: Vec2 { x: 14.5, y: -4.0 },
            end: Vec2 { x: 14.8, y: -3.0 },
        }),
        Shape::Line(Line {
            start: Vec2 { x: 14.8, y: -3.0 },
            end: Vec2 { x: 15.0, y: -2.0 },
        }),
        Shape::Line(Line {
            start: Vec2 { x: 15.0, y: -2.0 },
            end: Vec2 { x: 15.0, y: 0.0 },
        }),
        Shape::Line(Line {
            start: Vec2 { x: 15.0, y: 0.0 },
            end: Vec2 { x: 15.0, y: 2.0 },
        }),
        Shape::Line(Line {
            start: Vec2 { x: 15.0, y: 2.0 },
            end: Vec2 { x: 14.8, y: 3.0 },
        }),
        Shape::Line(Line {
            start: Vec2 { x: 14.8, y: 3.0 },
            end: Vec2 { x: 14.5, y: 4.0 },
        }),
        Shape::Line(Line {
            start: Vec2 { x: 14.5, y: 4.0 },
            end: Vec2 { x: 14.0, y: 5.0 },
        }),
    ];

    let lense_shapes = vec![
        biconvex,
        concave,
        octagon,
        prism,
        triangle_prism,
        mirror,
        cylinder,
        diamond,
        meniscus,
        asymmetric_prism,
    ];

    let mut current_lense = 0;
    let mut lense = Lense {
        shapes: lense_shapes[current_lense].clone(),
        index_of_refraction: 1.458,
    };
    let mut target = Vector2::zero();

    let mut zoom = 15.0;
    let mut move_speed = 1.0;
    while !rhandle.window_should_close() {
        if rhandle.is_key_down(KeyboardKey::KEY_EQUAL) {
            lense.index_of_refraction *= 1.01;
        }
        if rhandle.is_key_down(KeyboardKey::KEY_MINUS) {
            if lense.index_of_refraction * 0.99 > 1.0 {
                lense.index_of_refraction *= 0.99;
            } else {
                lense.index_of_refraction = AIR_INDEX_OF_REFRACTION
            }
        }
        if rhandle.is_key_pressed(KeyboardKey::KEY_J) {
            if current_lense > 0 {
                current_lense -= 1;
                lense.shapes = lense_shapes[current_lense].clone()
            }
        }
        if rhandle.is_key_pressed(KeyboardKey::KEY_L) {
            if current_lense < lense_shapes.len() - 1 {
                current_lense += 1;
                lense.shapes = lense_shapes[current_lense].clone()
            }
        }
        if rhandle.is_key_pressed(KeyboardKey::KEY_ZERO) {
            light.ray_count *= 2;
        }
        if rhandle.is_key_pressed(KeyboardKey::KEY_NINE) {
            if light.ray_count > 1 {
                light.ray_count /= 2;
            }
        }

        if rhandle.is_key_down(KeyboardKey::KEY_I) {
            zoom *= 1.02;
        }
        if rhandle.is_key_down(KeyboardKey::KEY_K) {
            zoom *= 0.98;
        }
        if rhandle.is_key_down(KeyboardKey::KEY_W) {
            target.y -= 0.05 * move_speed;
        }
        if rhandle.is_key_down(KeyboardKey::KEY_S) {
            target.y += 0.05 * move_speed;
        }
        if rhandle.is_key_down(KeyboardKey::KEY_A) {
            target.x -= 0.05 * move_speed;
        }
        if rhandle.is_key_down(KeyboardKey::KEY_D) {
            target.x += 0.05 * move_speed;
        }

        if rhandle.is_key_down(KeyboardKey::KEY_T) {
            light.position.y -= 0.05 * move_speed;
        }
        if rhandle.is_key_down(KeyboardKey::KEY_G) {
            light.position.y += 0.05 * move_speed;
        }
        if rhandle.is_key_down(KeyboardKey::KEY_F) {
            light.position.x -= 0.05 * move_speed;
        }
        if rhandle.is_key_down(KeyboardKey::KEY_H) {
            light.position.x += 0.05 * move_speed;
        }

        if rhandle.is_key_down(KeyboardKey::KEY_E) {
            move_speed *= 1.05;
        }
        if rhandle.is_key_down(KeyboardKey::KEY_Q) {
            move_speed *= 0.95;
        }

        let screen_width = rhandle.get_screen_width() as f32;
        let screen_height = rhandle.get_screen_height() as f32;

        // recompute offset each frame
        let camera = Camera2D {
            offset: Vector2::new(screen_width / 2.0, screen_height / 2.0),
            target,
            rotation: 0.0,
            zoom: zoom,
        };
        let to_draw = generate_shapes(lense.clone(), light.clone());
        draw(&to_draw, &mut rhandle, &mut thread, camera);
    }
}

#[derive(Debug, Clone)]
struct Light {
    position: Vec2,
    ray_count: u32,
}

#[derive(Debug, Clone)]
struct Lense {
    shapes: Vec<Shape>,
    index_of_refraction: f32,
}

#[derive(Debug, Clone)]
enum Shape {
    Line(Line),
    Circle(Circle),
}

fn generate_shapes(lense: Lense, light: Light) -> Vec<(Shape, Color)> {
    let mut shapes: Vec<(Shape, Color)> = Vec::new();
    let mut rays: Vec<Ray> = Vec::new();

    let max_ray_count = 5_000;
    let mut ray_count = 0;

    for i in 0..light.ray_count {
        let theta: f32 = ((2.0 * PI) / light.ray_count as f32) * i as f32;
        let direction = Vec2 {
            x: theta.cos(),
            y: theta.sin(),
        }
        .normalized();
        rays.push(Ray {
            start: light.position,
            direction,
        });
    }
    for shape in &lense.shapes {
        match shape {
            Shape::Line(line) => {
                shapes.push((Shape::Line(line.clone()), Color::WHITE));
            }
            Shape::Circle(x) => {
                todo!()
            }
        }
    }

    while let Some(ray) = rays.pop() {
        ray_count += 1;
        if ray_count > max_ray_count {
            return shapes;
        }

        let mut intersections = Vec::new();
        for shape in &lense.shapes {
            match shape {
                Shape::Line(line) => {
                    let intersection = ray_line_collision(&ray, &line);
                    match intersection {
                        RayIntersection::Hit(intersection) => {
                            intersections.push((line.clone(), intersection.clone()));
                        }
                        // wait until its missed all shapes before drawing the long ray
                        RayIntersection::Miss => {}
                    }
                }
                Shape::Circle(x) => {
                    todo!()
                }
            }
        }

        if intersections.is_empty() {
            dbg!(&ray);
            dbg!("didnt hit anything");
            shapes.push((
                Shape::Line(Line {
                    start: ray.start,
                    end: ray.start + (ray.direction * 200.0),
                }),
                Color::WHITESMOKE,
            ));
            continue;
        }
        let mut current_closest = f32::INFINITY;
        let mut closest_intersection = None;

        for intersection in intersections {
            let distance = (ray.start - intersection.1.point).mag_sq();
            if distance < current_closest {
                closest_intersection = Some(intersection);
                current_closest = distance;
            }
        }
        let closest_intersection = closest_intersection.unwrap();

        shapes.push((
            Shape::Line(Line {
                start: ray.start,
                end: closest_intersection.1.point,
            }),
            Color::ORANGE,
        ));
        let direction = calculate_refraction_direction(
            ray.direction,
            closest_intersection.0.clone(),
            AIR_INDEX_OF_REFRACTION,
            lense.index_of_refraction,
        );
        dbg!(&direction);
        rays.push(Ray {
            start: closest_intersection.1.point,
            direction: direction,
        });
    }

    shapes
}

#[derive(Debug, Clone)]
struct Ray {
    start: Vec2,
    direction: Vec2,
}
#[derive(Debug, Clone)]
struct Line {
    start: Vec2,
    end: Vec2,
}
#[derive(Debug, Clone)]
struct Circle {
    center: Vec2,
    radius: f32,
}

#[derive(Debug, Clone)]
struct Intersection {
    point: Vec2,
}
#[derive(Debug, Clone)]
enum RayIntersection {
    Miss,
    Hit(Intersection),
}

fn ray_line_collision(ray: &Ray, line: &Line) -> RayIntersection {
    let ray_direction = ray.direction.normalized();

    // Direction vector of the line segment
    let line_direction = line.end - line.start;
    // Vector from ray origin to line start point
    let origin_to_line = line.start - ray.start;

    // wedge product: how far along the line direction we need to go
    // (measures the "signed area" between origin_to_line and line_direction)
    let numerator_t = origin_to_line.wedge(line_direction).xy;

    // wedge product: how far along the ray direction we need to go from the line's perspective
    // (used to find s, the parameter along the line segment)
    let numerator_s = origin_to_line.wedge(ray_direction).xy;

    // wedge product of the two directions - determines if ray and line are parallel
    // Also serves as the common denominator for solving the parametric equations
    let direction_wedge = ray_direction.wedge(line_direction).xy;

    if direction_wedge.abs() < 0.0001 {
        return RayIntersection::Miss;
    }

    // t: distance along the ray (must be >= 0 for a valid ray intersection)
    let t = numerator_t / direction_wedge;

    // s: parameter along the line segment (must be in [0, 1] to be within the segment)
    let s = numerator_s / direction_wedge;

    if s < 0.0001 || s > 1.0 || t < 0.0001 {
        return RayIntersection::Miss;
    }

    let point = ray.start + ray_direction * t;

    let intersection = Intersection { point };
    return RayIntersection::Hit(intersection);
}

fn ray_circle_collision(ray: &Ray, circle: &Circle) -> RayIntersection {
    todo!()
}

fn draw(
    to_draw: &Vec<(Shape, Color)>,
    rhandle: &mut RaylibHandle,
    thread: &mut RaylibThread,
    camera: Camera2D,
) {
    let mut default_drawer = rhandle.begin_drawing(&thread);
    default_drawer.clear_background(Color::SLATEBLUE);

    let mut drawer = default_drawer.begin_mode2D(camera);
    for (shape, color) in to_draw {
        match shape {
            Shape::Circle(x) => {
                drawer.draw_circle_lines_v(
                    Vector2 {
                        x: x.center.x,
                        y: x.center.y,
                    },
                    x.radius,
                    color,
                );
            }
            Shape::Line(x) => {
                drawer.draw_line_v(
                    Vector2 {
                        x: x.start.x,
                        y: x.start.y,
                    },
                    Vector2 {
                        x: x.end.x,
                        y: x.end.y,
                    },
                    color,
                );
            }
        }
    }
}

fn calculate_refraction_direction(
    incoming_ray_direction: Vec2,
    line: Line,
    from_refraction_index: f32,
    to_refraction_index: f32,
) -> Vec2 {
    // the direction from the start to the end
    let line_direction = (line.end - line.start).normalized();
    // the line perpendicular to that, facing left
    let mut line_normal = Vec2 {
        x: -line_direction.y,
        y: line_direction.x,
    };
    // the ratio of the indices of refraction
    let mut eta = from_refraction_index / to_refraction_index;

    // if the incoming ray is looking at the normal instead of in the same direction, swap the line
    // normal
    let normal_flipped = incoming_ray_direction.dot(line_normal) < 0.0;
    if normal_flipped {
        line_normal *= -1.0;
        eta = 1.0 / eta;
    }

    // can be coming from either side of the normal
    let angle_of_incidence = f32::acos(incoming_ray_direction.dot(line_normal).clamp(-1.0, 1.0));
    // we recompute the tangent in case the normal has flipped
    let tangent = Vec2 {
        x: line_normal.y,
        y: -line_normal.x,
    };

    let side = incoming_ray_direction.dot(tangent).signum() * -1.0;

    // the sin of the refraction angle
    let temp = eta * f32::sin(angle_of_incidence);
    // if this value is greater than one, this whole ray should be skipped, not sure how to
    // represent that, so its todo for now
    if temp > 1.0 {
        return line_normal.rotated_by(Rotor2::from_angle(angle_of_incidence * side * -1.0)) * -1.0;
    }

    let refraction_angle = f32::asin(temp);

    // if positive then rays poiting in the same direction, if negative then pointing opposite
    // direction

    dbg!(refraction_angle / PI * 180.0);

    // rotate the line normal by the side, but swapped if the sign is opposite
    line_normal.rotated_by(Rotor2::from_angle(side * refraction_angle))
}
