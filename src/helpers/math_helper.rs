/// Mathematical helper functions for Spriter animations
pub struct MathHelper;

impl MathHelper {
    /// Does a linear angle interpolation taking into account the spin.
    pub fn angle_linear(a: f32, b: f32, spin: i32, f: f32) -> f32 {
        if spin == 0 {
            return a;
        }

        let mut b = b;
        if spin > 0 && (b - a) < 0.0 {
            b += 360.0;
        }
        if spin < 0 && (b - a) > 0.0 {
            b -= 360.0;
        }
        Self::linear(a, b, f)
    }

    /// Does a linear angle interpolation towards the closest direction.
    pub fn closer_angle_linear(a: f32, b: f32, f: f32) -> f32 {
        if (b - a).abs() < 180.0 {
            return Self::linear(a, b, f);
        }

        let (mut a, mut b) = (a, b);
        if a < b {
            a += 360.0;
        } else {
            b += 360.0;
        }
        Self::linear(a, b, f)
    }

    /// Calculates the interpolation factor of the given value.
    pub fn get_factor(a: f32, b: f32, v: f32) -> f32 {
        (v - a) / (b - a)
    }

    /// Does a linear interpolation of the two values for the given factor.
    pub fn linear(a: f32, b: f32, f: f32) -> f32 {
        a + (b - a) * f
    }

    /// Calculates the value of the 1-Dimensional Bezier curve defined with control points c
    /// for the given parameter f [0...1] using De Casteljau's algorithm.
    pub fn bezier_3(c0: f32, c1: f32, c2: f32, f: f32) -> f32 {
        Self::linear(Self::linear(c0, c1, f), Self::linear(c1, c2, f), f)
    }

    /// Calculates the value of the 1-Dimensional Bezier curve defined with control points c
    /// for the given parameter f [0...1] using De Casteljau's algorithm.
    pub fn bezier_4(c0: f32, c1: f32, c2: f32, c3: f32, f: f32) -> f32 {
        Self::linear(
            Self::bezier_3(c0, c1, c2, f),
            Self::bezier_3(c1, c2, c3, f),
            f,
        )
    }

    /// Calculates the value of the 1-Dimensional Bezier curve defined with control points c
    /// for the given parameter f [0...1] using De Casteljau's algorithm.
    pub fn bezier_5(c0: f32, c1: f32, c2: f32, c3: f32, c4: f32, f: f32) -> f32 {
        Self::linear(
            Self::bezier_4(c0, c1, c2, c3, f),
            Self::bezier_4(c1, c2, c3, c4, f),
            f,
        )
    }

    /// Calculates the value of the 1-Dimensional Bezier curve defined with control points c
    /// for the given parameter f [0...1] using De Casteljau's algorithm.
    pub fn bezier_6(c0: f32, c1: f32, c2: f32, c3: f32, c4: f32, c5: f32, f: f32) -> f32 {
        Self::linear(
            Self::bezier_5(c0, c1, c2, c3, c4, f),
            Self::bezier_5(c1, c2, c3, c4, c5, f),
            f,
        )
    }

    /// 2D Bezier curve calculation
    pub fn bezier_2d(x1: f32, y1: f32, x2: f32, y2: f32, t: f32) -> f32 {
        let duration = 1.0;
        let cx = 3.0 * x1;
        let bx = 3.0 * (x2 - x1) - cx;
        let ax = 1.0 - cx - bx;
        let cy = 3.0 * y1;
        let by = 3.0 * (y2 - y1) - cy;
        let ay = 1.0 - cy - by;

        Self::solve(ax, bx, cx, ay, by, cy, t, Self::solve_epsilon(duration))
    }

    fn sample_curve(a: f32, b: f32, c: f32, t: f32) -> f32 {
        ((a * t + b) * t + c) * t
    }

    fn sample_curve_derivative_x(ax: f32, bx: f32, cx: f32, t: f32) -> f32 {
        (3.0 * ax * t + 2.0 * bx) * t + cx
    }

    fn solve_epsilon(duration: f32) -> f32 {
        1.0 / (200.0 * duration)
    }

    #[allow(clippy::too_many_arguments)]
    fn solve(ax: f32, bx: f32, cx: f32, ay: f32, by: f32, cy: f32, x: f32, epsilon: f32) -> f32 {
        Self::sample_curve(ay, by, cy, Self::solve_curve_x(ax, bx, cx, x, epsilon))
    }

    fn solve_curve_x(ax: f32, bx: f32, cx: f32, x: f32, epsilon: f32) -> f32 {
        let mut t0: f32;
        let mut t1: f32;
        let mut t2: f32;
        let mut x2: f32;
        let mut d2: f32;

        t2 = x;
        for _ in 0..8 {
            x2 = Self::sample_curve(ax, bx, cx, t2) - x;
            if x2.abs() < epsilon {
                return t2;
            }

            d2 = Self::sample_curve_derivative_x(ax, bx, cx, t2);
            if d2.abs() < 1e-6 {
                break;
            }

            t2 -= x2 / d2;
        }

        t0 = 0.0;
        t1 = 1.0;
        t2 = x;

        if t2 < t0 {
            return t0;
        }
        if t2 > t1 {
            return t1;
        }

        while t0 < t1 {
            x2 = Self::sample_curve(ax, bx, cx, t2);
            if (x2 - x).abs() < epsilon {
                return t2;
            }
            if x > x2 {
                t0 = t2;
            } else {
                t1 = t2;
            }
            t2 = (t1 - t0) * 0.5 + t0;
        }

        t2
    }
}
