use std::{f32::consts::PI, time::Duration};

use bevy::prelude::*;

use crate::gun::BulletBlueprint;

const SQRT_3: f32 = 1.73205080757;

#[allow(dead_code)]
/// Generate points on a circle 
pub fn generate_circle(center_x:f32, center_y:f32, radius: f32, num_bullets: usize) -> Vec<(f32,f32)>{
    let mut vec = Vec::new();
    for i in 0..num_bullets {
        let angle = i as f32 / num_bullets as f32 * std::f32::consts::PI * 2.0;
        let bullet_x = center_x + radius * angle.cos();
        let bullet_y = center_y + radius * angle.sin();
        vec.push((bullet_x,bullet_y));
    }

    vec
}

#[allow(dead_code)]
pub fn generate_square(center_x: f32, center_y: f32, side_length: f32, num_bullets_per_side: usize) -> Vec<(f32,f32)> {
    let mut vec = Vec::new();
    
    let half = side_length / 2.0;
    let spacing = side_length / num_bullets_per_side as f32;

    for i in 0..num_bullets_per_side {
        let x = center_x - half + (i as f32 * spacing);
        let y_top = center_y + half;
        let y_bot: f32 = center_y - half;
        vec.push((x,y_top));
        vec.push((x,y_bot));
    }

    for i in 1..num_bullets_per_side {
        let y = center_y - half + (i as f32 * spacing);
        let x_right = center_x + half;
        let x_left: f32 = center_x - half;
        vec.push((x_right,y));
        vec.push((x_left,y));
    }
    vec
}

/// Generate a line providing start, and end points, as well as the total number of points(bullets)
pub fn generate_line(x1: f32, y1: f32, x2: f32, y2: f32, num_bullets: usize) -> Vec<(f32,f32)> {
    let mut vec = Vec::new();
    let dx = (x2 - x1) / num_bullets as f32;
    let dy = (y2-y1) / num_bullets as f32;

    for i in 0..num_bullets {
        let bx = x1 + (i as f32 * dx);
        let by = y1 + (i as f32 * dy);
        vec.push((bx,by));
    }
    vec
}

/// Generate a triangle based on 3  points ()
pub fn generate_triangle(p1: (f32,f32), p2: (f32,f32), p3: (f32, f32), num_bullets: usize) -> Vec<(f32,f32)> {
    let mut vec: Vec<(f32, f32)> = Vec::new();
    
    
    let line_0 = generate_line(p1.0, p1.1, p2.0, p2.1, num_bullets);
    let line_1 = generate_line(p1.0, p1.1, p3.0, p3.1, num_bullets);
    let line_2 = generate_line(p3.0, p3.1, p2.0, p2.1, num_bullets);

    vec.extend(line_0);
    vec.extend(line_1);
    vec.extend(line_2);

    vec
}


pub enum ShapeType {
    Triangle, 
    Square,
    Circle,
    HorizontalLine,
    VerticalLine,
}

pub struct ShapeBloop{ pub offset: (f32,f32), pub num_bullets: usize, pub t: ShapeType, pub size_scale: (f32, f32)}

#[derive(Component)]
pub struct  ShapeGun {
    max_shots: u64,
    shots: u64,
    size: f32,
    pub timer: Timer,
    reload_time: f32,
    bullet_size: usize,
    bloops: Vec<ShapeBloop>,
    pub bullet: BulletBlueprint

}

impl Default for ShapeGun {
    fn default() -> Self {
        ShapeGun {
            max_shots: 10, 
            shots: 10,
            size: 200.,
            timer: Timer::new(Duration::from_secs_f32(10.), TimerMode::Once),
            reload_time: 10.,
            bloops: Vec::new(),
            bullet: BulletBlueprint(1, |y| y*y, |_| 0., 0., true, 50),
            bullet_size: 25
        }

    }
}

impl ShapeGun {
    pub fn new(max_shots: u64, size: f32, reload: f32, bloops: Vec<ShapeBloop>, bullet: BulletBlueprint ) -> ShapeGun {
        ShapeGun {
            max_shots: max_shots,
            shots: max_shots,
            size: size,
            reload_time: reload,
            timer: Timer::new(Duration::from_secs_f32(reload), TimerMode::Once),
            bloops: bloops,
            bullet: bullet,
            bullet_size: 25
        }
    }

    /// Return a vector of all the points this shape will need to make 
    pub fn get_shapes(&self, x: f32, y: f32) -> Vec<(f32, f32)>{
        let mut vec = Vec::new();
        let bullet_count =(( self.size / 100.) as usize * self.bullet_size) as usize;
        for bloop in &self.bloops {
            match bloop.t {
                ShapeType::HorizontalLine => {
                    let x1 = x - (self.size*bloop.size_scale.0) + bloop.offset.0;
                    let x2 = x + (self.size*bloop.size_scale.0) + bloop.offset.0;
                    let y0 = y + bloop.offset.1;

                    let points = generate_line(x1, y0, x2, y0, bullet_count);
                    vec.extend(points);
                },
                ShapeType::VerticalLine => {
                    let y1 = y - (self.size*bloop.size_scale.1) + bloop.offset.1;
                    let y2 = y + (self.size*bloop.size_scale.1) + bloop.offset.1;
                    let x0 = x + bloop.offset.0;

                    let points = generate_line(x0, y1, x0, y2, bullet_count);
                    vec.extend(points);
                },
                ShapeType::Square => {
                    let points = generate_square(x + bloop.offset.0, y + bloop.offset.1, (self.size*bloop.size_scale.0), bullet_count / 2);
                    vec.extend(points);
                },
                ShapeType::Circle => {
                    let points = generate_circle(x + bloop.offset.0, y + bloop.offset.1 , (self.size*bloop.size_scale.0) / 2., bullet_count);
                    vec.extend(points);
                },
                ShapeType::Triangle => { 
                    let p0 = ((x - (self.size*bloop.size_scale.0) / 2.) + bloop.offset.0, y + bloop.offset.1);
                    let p1 = ((x + (self.size*bloop.size_scale.0) / 2.) + bloop.offset.0, y + bloop.offset.1);
                    let p2 = (x + bloop.offset.0, ((y + bloop.offset.1) + (((self.size*bloop.size_scale.1) / 2.) * SQRT_3)));

                    let points = generate_triangle(p0, p1, p2, bullet_count / 3);
                    vec.extend(points);
                },
                _ => {}
            }
        }

        vec
    }
    
    pub fn shoot(&mut self){
        self.shots = if ((self.shots - 1) as i64) < 0 {0} else {self.shots - 1};
    }

    pub fn reload(&mut self){
        self.shots = self.max_shots;
        self.timer.reset();
    }

    pub fn set_max_shots(&mut self, shots: u64){
        self.max_shots = shots;
    }

    pub fn get_max_shots(&self) -> u64 { self.max_shots}
    pub fn get_shots(&self) -> u64 {self.shots}

    pub fn set_reload_time(&mut self, t: f32){
        self.reload_time = t;
        self.timer.set_duration(Duration::from_secs_f32(t)); 
    }

    pub fn add_bloop(&mut self, bloop: ShapeBloop){
        self.bloops.push(bloop);
    }

    pub fn remove_bloop(&mut self, index:usize){
        self.bloops.remove(index % self.bloops.len());
    }

    pub fn set_size(&mut self, size: f32){
        self.size = if size > 1. {size} else {1.};
    }

    pub fn get_size(&self) -> f32 {self.size}

   
}
