use crate::*;

pub struct Eye {
    fov_range: f32,
    fov_angle: f32,
    cells: usize,
}

impl Eye {
    pub(crate) fn new(config: &Config) -> Self {
        let (fov_range, fov_angle, cells) = (config.eye_range, config.eye_fov, config.eye_cells);

        assert!(fov_range > 0.0);
        assert!(fov_angle > 0.0);
        assert!(cells > 0);

        Self {
            fov_range,
            fov_angle,
            cells,
        }
    }

    pub fn cells(&self) -> usize {
        self.cells
    }

    pub fn process_vision(
        &self,
        pos: Point2<f32>,
        rot: Rotation2<f32>,
        food: &[Food],
    ) -> DVector<f32> {
        let mut cells = vec![0.0; self.cells];

        for food in food {
            let vec = food.pos - pos;
            let dist = vec.norm();

            if dist >= self.fov_range {
                continue;
            }

            let angle = Rotation2::rotation_between(&Vector2::x(), &vec).angle();
            let angle = angle - rot.angle();
            let angle = wrap(angle, -PI, PI);

            if angle < -self.fov_angle / 2.0 || angle > self.fov_angle / 2.0 {
                continue;
            }

            let angle = angle + self.fov_angle / 2.0;

            let cell = angle / self.fov_angle;
            let cell = cell * (self.cells as f32);
            let cell = (cell as usize).min(cells.len() - 1);

            let energy = (self.fov_range - dist) / self.fov_range;

            cells[cell] += energy;
        }

        DVector::from_vec(cells)
    }
}

#[cfg(test)]
mod tests {
    use std::f32::consts::FRAC_PI_2;

    use super::*;
    use test_case::test_case;

    const TEST_EYE_CELLS: usize = 13;

    struct TestCase {
        food: Vec<Food>,
        fov_range: f32,
        fov_angle: f32,
        x: f32,
        y: f32,
        rot: f32,
        expected_vision: &'static str,
    }

    impl TestCase {
        fn run(self) {
            let eye = Eye { fov_range: self.fov_range, fov_angle: self.fov_angle, cells: TEST_EYE_CELLS };

            let actual_vision = eye.process_vision(
                Point2::new(self.x, self.y),
                Rotation2::new(self.rot),
                &self.food,
            );
            let actual_vision = actual_vision
                .into_iter()
                .map(|&cell| {
                    if cell >= 0.7 {
                        "#"
                    } else if cell >= 0.3 {
                        "+"
                    } else if cell > 0.0 {
                        "."
                    } else {
                        " "
                    }
                })
                .collect::<Vec<_>>()
                .join("");

            assert_eq!(actual_vision, self.expected_vision);
        }
    }

    fn food(x: f32, y: f32) -> Food {
        Food {
            pos: Point2::new(x, y),
        }
    }

    #[test_case(1.0, "      +      ")]
    #[test_case(0.9, "      +      ")]
    #[test_case(0.8, "      +      ")]
    #[test_case(0.7, "      .      ")]
    #[test_case(0.6, "      .      ")]
    #[test_case(0.5, "             ")]
    #[test_case(0.4, "             ")]
    #[test_case(0.3, "             ")]
    #[test_case(0.2, "             ")]
    #[test_case(0.1, "             ")]
    fn test_ranges(fov_range: f32, expected_vision: &'static str) {
        TestCase {
            food: vec![food(1.0, 0.5)],
            fov_range,
            fov_angle: FRAC_PI_2,
            x: 0.5,
            y: 0.5,
            rot: 0.0,
            expected_vision,
        }
        .run()
    }

    #[test_case(0.00 * PI, "         +   ")]
    #[test_case(0.25 * PI, "        +    ")]
    #[test_case(0.50 * PI, "      +      ")]
    #[test_case(0.75 * PI, "    +        ")]
    #[test_case(1.00 * PI, "   +         ")]
    #[test_case(1.25 * PI, " +           ")]
    #[test_case(1.50 * PI, "            +")]
    #[test_case(1.75 * PI, "           + ")]
    #[test_case(2.00 * PI, "         +   ")]
    #[test_case(2.25 * PI, "        +    ")]
    #[test_case(2.50 * PI, "      +      ")]
    fn test_rot(rot: f32, expected_vision: &'static str) {
        TestCase {
            food: vec![food(0.5, 1.0)],
            fov_range: 1.0,
            fov_angle: 2.0 * PI,
            x: 0.5,
            y: 0.5,
            rot,
            expected_vision,
        }
        .run()
    }

    #[test_case(0.9, 0.5, "#           #")]
    #[test_case(0.8, 0.5, "  #       #  ")]
    #[test_case(0.7, 0.5, "   +     +   ")]
    #[test_case(0.6, 0.5, "    +   +    ")]
    #[test_case(0.5, 0.5, "    +   +    ")]
    #[test_case(0.4, 0.5, "     + +     ")]
    #[test_case(0.3, 0.5, "     . .     ")]
    #[test_case(0.2, 0.5, "     . .     ")]
    #[test_case(0.1, 0.5, "     . .     ")]
    #[test_case(0.0, 0.5, "             ")]
    #[test_case(0.5, 0.0, "            +")]
    #[test_case(0.5, 0.1, "          + .")]
    #[test_case(0.5, 0.2, "         +  +")]
    #[test_case(0.5, 0.3, "        + +  ")]
    #[test_case(0.5, 0.4, "      +  +   ")]
    #[test_case(0.5, 0.6, "   +  +      ")]
    #[test_case(0.5, 0.7, "  + +        ")]
    #[test_case(0.5, 0.8, "+  +         ")]
    #[test_case(0.5, 0.9, ". +          ")]
    #[test_case(0.5, 1.0, "+            ")]
    fn test_positions(x: f32, y: f32, expected_vision: &'static str) {
        TestCase {
            food: vec![food(1.0, 0.4), food(1.0, 0.6)],
            fov_range: 1.0,
            fov_angle: FRAC_PI_2,
            rot: 0.0,
            x,
            y,
            expected_vision,
        }
        .run()
    }

    #[test_case(0.25 * PI, " +         + ")]
    #[test_case(0.50 * PI, ".  +     +  .")]
    #[test_case(0.75 * PI, "  . +   + .  ")]
    #[test_case(1.00 * PI, "   . + + .   ")]
    #[test_case(1.25 * PI, "   . + + .   ")]
    #[test_case(1.50 * PI, ".   .+ +.   .")]
    #[test_case(1.75 * PI, ".   .+ +.   .")]
    #[test_case(2.00 * PI, "+.  .+ +.  .+")]
    fn test_angles(fov_angle: f32, expected_vision: &'static str) {
        TestCase {
            food: vec![
                food(0.0, 0.0),
                food(0.0, 0.33),
                food(0.0, 0.66),
                food(0.0, 1.0),
                food(1.0, 0.0),
                food(1.0, 0.33),
                food(1.0, 0.66),
                food(1.0, 1.0),
            ],
            fov_range: 1.0,
            x: 0.5,
            y: 0.5,
            rot: 0.0,
            fov_angle,
            expected_vision,
        }
        .run()
    }
}
