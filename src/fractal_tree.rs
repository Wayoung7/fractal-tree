use std::f32::consts::PI;

use bevy::prelude::*;
use rpds::Stack;

use crate::sketch::gizmos_segment_2d_angle;

const ITERATIONS: usize = 9;
const BRANCH_LENGTH: f32 = 0.7;

pub struct FractalTreePlugin;

impl Plugin for FractalTreePlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<TreeData>()
            .add_systems(Startup, setup)
            .add_systems(Update, update);
    }
}

fn setup(mut tree_data: ResMut<TreeData>) {
    for _ in 0..ITERATIONS {
        tree_data.current = apply_rules(&tree_data.current, &tree_data.rules);
    }
    // info!(tree_data.current);
}

fn update(gizmos: Gizmos, tree_data: ResMut<TreeData>) {
    render_tree(gizmos, &tree_data.current, tree_data.angle);
}

fn apply_rules(string: &String, rules: &Vec<(char, String)>) -> String {
    let mut res = String::new();
    for c in string.chars() {
        let mut has_available_rule = false;
        for rule in rules.iter() {
            if c == rule.0 {
                res.push_str(&rule.1);
                has_available_rule = true;
                break;
            }
        }
        if !has_available_rule {
            res.push(c);
        }
    }

    res
}

fn render_tree(mut gizmos: Gizmos, data: &str, angle_step: f32) {
    let mut angle = PI / 2.;
    let mut current_pos: Vec2 = Vec2::new(0., -500.);
    let mut stack: Stack<(Vec2, f32)> = Stack::new();
    for c in data.chars() {
        match c {
            'F' => {
                current_pos = gizmos_segment_2d_angle(
                    &mut gizmos,
                    current_pos,
                    angle,
                    BRANCH_LENGTH,
                    Color::WHITE,
                );
            }
            '-' => {
                angle += angle_step;
            }
            '+' => {
                angle -= angle_step;
            }
            '[' => {
                stack = stack.push((current_pos, angle));
            }
            ']' => {
                (current_pos, angle) = stack.peek().unwrap_or(&(current_pos, angle)).to_owned();
                stack = stack.pop().unwrap_or(stack);
            }
            _ => {}
        }
    }
}

#[derive(Resource, Debug)]
struct TreeData {
    rules: Vec<(char, String)>,
    angle: f32,
    current: String,
}

impl Default for TreeData {
    fn default() -> Self {
        TreeData {
            rules: vec![
                ('F', "FF".to_string()),
                ('X', "F-[[X]+X]+F[+FX]-X".to_string()),
            ],
            angle: PI / 8.,
            current: String::from("X"),
        }
    }
}
