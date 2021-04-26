use bevy::prelude::*;
use glam::DVec2;

pub mod systems {
    mod orientation_text;
    pub use orientation_text::orientation_text;
    mod panel_setup;
    pub use panel_setup::panel_setup;
    mod panel_text;
    pub use panel_text::panel_text;
    mod play_area_movement;
    pub use play_area_movement::play_area_movement;
}

pub struct Panel {
    pub origin_direction: OriginDirection,
}
pub struct PanelText {
    pub origin_direction: OriginDirection,
}
impl PanelText {
    fn format_panel_values(panel: &PlayAreaPanel) -> String {
        let pos_x = panel.position.x;
        let pos_y = panel.position.y;
        let size_x = panel.size.x.abs();
        let size_y = panel.size.y.abs();
        format!(
            "Pos: [{:.2}, {:.2}]  Size: [{:.2}, {:.2}]",
            pos_x, pos_y, size_x, size_y
        )
    }
}
pub struct OrientationText;
impl OrientationText {
    fn format_orientation(o: DVec2) -> String {
        format!("[{:.3}, {:.3}]", o.x, o.y)
    }
}

#[derive(Clone, Copy, Debug)]
pub enum OriginDirection {
    Up,
    Right,
    Down,
    Left,
}
impl OriginDirection {
    fn left_corner(&self) -> DVec2 {
        const VEC: [[f64; 2]; 4] = [[-1., 1.], [1., 1.], [1., -1.], [-1., -1.]];
        VEC[*self as usize].into()
    }
    fn origin(&self) -> DVec2 {
        const VEC: [[f64; 2]; 4] = [[0., 1.], [1., 0.], [0., -1.], [-1., 0.]];
        VEC[*self as usize].into()
    }
    fn right_corner(&self) -> DVec2 {
        const VEC: [[f64; 2]; 4] = [[1., 1.], [1., -1.], [-1., -1.], [-1., 1.]];
        VEC[*self as usize].into()
    }
    fn closest(vector: DVec2) -> Self {
        let mut returned = Self::Up;
        let mut closest_diff = 46.;
        for orientation in [Self::Up, Self::Right, Self::Down, Self::Left].iter() {
            let diff = vector.angle_between(orientation.origin()).abs();
            if diff < closest_diff {
                returned = *orientation;
                closest_diff = diff;
            }
        }
        returned
    }
}

pub struct PlayArea {
    max_play_area: DVec2,
    horizontal_main_area: DVec2,
    horizontal_side_area: DVec2,
    vertical_main_area: DVec2,
    vertical_side_area: DVec2,
    orientation: DVec2,
    panels: [PlayAreaPanel; 4],
}
impl Default for PlayArea {
    fn default() -> Self {
        PlayArea {
            max_play_area: DVec2::ZERO,
            horizontal_main_area: DVec2::ZERO,
            horizontal_side_area: DVec2::ZERO,
            vertical_main_area: DVec2::ZERO,
            vertical_side_area: DVec2::ZERO,
            orientation: DVec2::ZERO,
            panels: [PlayAreaPanel::default(); 4],
        }
    }
}
impl PlayArea {
    pub fn new(max_width: f64, max_height: f64) -> PlayArea {
        let (horizontal_main_area, horizontal_side_area) =
            Self::bounds(max_width, max_height, false);
        let (vertical_main_area, vertical_side_area) = Self::bounds(max_height, max_width, true);
        let max_play_area = DVec2::new(horizontal_main_area.y, vertical_main_area.x);

        println!("horizontal_main_area: {}", horizontal_main_area);
        println!("horizontal_side_area: {}", horizontal_side_area);
        println!("vertical_main_area: {}", vertical_main_area);
        println!("vertical_side_area: {}", vertical_side_area);

        let mut play_area = PlayArea {
            max_play_area,
            horizontal_main_area,
            horizontal_side_area,
            vertical_main_area,
            vertical_side_area,
            panels: PlayAreaPanel::init_panel_spread(),
            ..Default::default()
        };
        println!("max_play_area: {}", play_area.max_play_area);

        play_area.set_orientation(DVec2::new(0.5, 0.5).normalize());
        play_area
    }
    fn bounds(width: f64, height: f64, swap_xy: bool) -> (DVec2, DVec2) {
        let ratio = 2_f64.sqrt();

        let floor_to_divisible_by_two = |n: f64| -> f64 { (n / 2.).floor() * 2. };

        let mut main_width = floor_to_divisible_by_two(height / ratio);
        let mut main_height = floor_to_divisible_by_two(height);
        if main_width * 2. > width {
            main_width = floor_to_divisible_by_two(width / 2_f64);
            main_height = floor_to_divisible_by_two(main_width * ratio);
        }
        let side_width = main_width;
        let side_height = floor_to_divisible_by_two(main_height / 2_f64);

        if swap_xy {
            (
                DVec2::new(main_height, main_width),
                DVec2::new(side_height, side_width),
            )
        } else {
            (
                DVec2::new(main_width, main_height),
                DVec2::new(side_width, side_height),
            )
        }
    }
    pub fn set_orientation(&mut self, orientation: DVec2) {
        self.orientation = orientation;
        self.update_panels();
    }

    fn scale_panel_size(main_size: DVec2, side_size: DVec2, s: f64) -> DVec2 {
        if s >= 0. {
            side_size.lerp(main_size, s)
        } else {
            side_size.lerp(DVec2::ZERO, -s)
        }
    }

    fn lerp_panel_position(&self, panel_size: DVec2, origin_direction: OriginDirection) -> DVec2 {
        let half_area = panel_size / 2.;
        let s = (self.orientation.angle_between(origin_direction.origin()) * 2.
            / std::f64::consts::PI)
            .clamp(-1., 1.);
        // println!("origin_direction, s = {:?}, {}", origin_direction, s);
        if s >= 0. {
            let corner_point = half_area * origin_direction.left_corner();
            let direction_point = half_area * origin_direction.origin();
            direction_point.lerp(corner_point, s)
            // println!(
            //     "panel_size {:?} - corner_point {:?} - direction_point {:?} - edge_point {:?}",
            //     panel_size, corner_point, direction_point, edge_point
            // );
        } else {
            let corner_point = half_area * origin_direction.right_corner();
            let direction_point = half_area * origin_direction.origin();
            direction_point.lerp(corner_point, -s)
            // println!(
            //     "panel_size {:?} - corner_point {:?} - direction_point {:?} - edge_point {:?}",
            //     panel_size, corner_point, direction_point, edge_point
            // );
        }
    }

    fn update_panels(&mut self) {
        self.panels[0].size = Self::scale_panel_size(
            self.vertical_main_area,
            self.horizontal_side_area,
            self.orientation.y,
        );
        self.panels[1].size = Self::scale_panel_size(
            self.horizontal_main_area,
            self.vertical_side_area,
            self.orientation.x,
        );
        self.panels[2].size = Self::scale_panel_size(
            self.vertical_main_area,
            self.horizontal_side_area,
            -self.orientation.y,
        );
        self.panels[3].size = Self::scale_panel_size(
            self.horizontal_main_area,
            self.vertical_side_area,
            -self.orientation.x,
        );

        self.panels[0].position =
            self.lerp_panel_position(self.panels[0].size, self.panels[0].origin_direction);
        self.panels[1].position =
            self.lerp_panel_position(self.panels[1].size, self.panels[1].origin_direction);
        self.panels[2].position =
            self.lerp_panel_position(self.panels[2].size, self.panels[2].origin_direction);
        self.panels[3].position =
            self.lerp_panel_position(self.panels[3].size, self.panels[3].origin_direction);
    }
    fn orientation(&self) -> DVec2 {
        self.orientation
    }
    fn panels(&self) -> &[PlayAreaPanel; 4] {
        &self.panels
    }
}

#[derive(Clone, Copy)]
struct PlayAreaPanel {
    pub size: DVec2,
    pub position: DVec2,
    pub origin_direction: OriginDirection,
}
impl Default for PlayAreaPanel {
    fn default() -> Self {
        PlayAreaPanel {
            size: DVec2::ZERO,
            position: DVec2::ZERO,
            origin_direction: OriginDirection::Up,
        }
    }
}
impl PlayAreaPanel {
    fn init_panel_spread() -> [Self; 4] {
        [
            PlayAreaPanel {
                origin_direction: OriginDirection::Up,
                ..Default::default()
            },
            PlayAreaPanel {
                origin_direction: OriginDirection::Right,
                ..Default::default()
            },
            PlayAreaPanel {
                origin_direction: OriginDirection::Down,
                ..Default::default()
            },
            PlayAreaPanel {
                origin_direction: OriginDirection::Left,
                ..Default::default()
            },
        ]
    }
    fn as_transform(&self) -> Transform {
        Transform::from_xyz(self.position.x as f32, self.position.y as f32, 0_f32)
    }
    fn as_sprite(&self) -> Sprite {
        Sprite::new(self.size.as_f32())
    }
}
