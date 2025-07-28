use super::math_helper::MathHelper;
use crate::spriter_model::{
    SpriterCurveType, SpriterKey, SpriterKeyTrait, SpriterMainlineKey, SpriterObject,
    SpriterSpatial, SpriterVarValue,
};

/// Helper functions for Spriter animation processing
pub struct SpriterHelper;

impl SpriterHelper {
    /// Checks if the given Mainline keys are compatible for animation blending.
    /// Even if this method returns true there is no guarantee that the animations are really compatible.
    pub fn will_it_blend(first_key: &SpriterMainlineKey, second_key: &SpriterMainlineKey) -> bool {
        if !first_key.bone_refs.is_empty() {
            if first_key.bone_refs.len() != second_key.bone_refs.len() {
                return false;
            }
        } else if !second_key.bone_refs.is_empty() {
            return false;
        }

        if !first_key.object_refs.is_empty() {
            if first_key.object_refs.len() != second_key.object_refs.len() {
                return false;
            }
        } else if !second_key.object_refs.is_empty() {
            return false;
        }

        true
    }

    /// Gets the next animation key coming after the given one.
    pub fn get_next_key<'a, T: SpriterKeyTrait>(
        keys: &'a [T],
        first_key: &T,
        looping: bool,
    ) -> Option<&'a T> {
        if keys.len() == 1 {
            return None;
        }

        let mut key_b_id = first_key.id() + 1;
        if key_b_id >= keys.len() as i32 {
            if !looping {
                return None;
            }
            key_b_id = 0;
        }

        let next_key = &keys[key_b_id as usize];
        if first_key.time() == next_key.time() {
            key_b_id += 1;
            if key_b_id >= keys.len() as i32 {
                if !looping {
                    return None;
                }
                key_b_id = 0;
            }
            Some(&keys[key_b_id as usize])
        } else {
            Some(next_key)
        }
    }

    /// Finds the last key before the given time.
    pub fn get_last_key<T: SpriterKeyTrait>(keys: &[T], target_time: f32) -> Option<&T> {
        let mut current = None;
        for key in keys {
            if key.time() > target_time {
                break;
            }
            current = Some(key);
        }
        current
    }

    /// Applies parent transforms to the given child spatial.
    pub fn apply_parent_transform(child: &mut SpriterSpatial, parent: &SpriterSpatial) {
        let px = parent.scale_x * child.x;
        let py = parent.scale_y * child.y;
        let angle_rad = parent.angle * std::f32::consts::PI / 180.0;
        let s = angle_rad.sin();
        let c = angle_rad.cos();

        child.x = px * c - py * s + parent.x;
        child.y = px * s + py * c + parent.y;
        child.scale_x *= parent.scale_x;
        child.scale_y *= parent.scale_y;
        child.angle = parent.angle + (parent.scale_x * parent.scale_y).signum() * child.angle;
        child.angle %= 360.0;
        child.alpha *= parent.alpha;
    }

    /// Fills all the values of target object from the given source object.
    pub fn fill_object_from(target: &mut SpriterObject, source: &SpriterObject) {
        target.animation_id = source.animation_id;
        target.entity_id = source.entity_id;
        target.file_id = source.file_id;
        target.folder_id = source.folder_id;
        target.pivot_x = source.pivot_x;
        target.pivot_y = source.pivot_y;
        target.t = source.t;
        target.x = source.x;
        target.y = source.y;
        target.angle = source.angle;
        target.scale_x = source.scale_x;
        target.scale_y = source.scale_y;
        target.alpha = source.alpha;
    }

    /// Fills all the values of target spatial from the given source spatial.
    pub fn fill_spatial_from(target: &mut SpriterSpatial, source: &SpriterSpatial) {
        target.alpha = source.alpha;
        target.angle = source.angle;
        target.scale_x = source.scale_x;
        target.scale_y = source.scale_y;
        target.x = source.x;
        target.y = source.y;
    }

    /// Fills all the values of target varvalue by interpolating values from the two given varvalues with the given factor.
    pub fn interpolate_var_value(
        target: &mut SpriterVarValue,
        val_a: &SpriterVarValue,
        val_b: &SpriterVarValue,
        factor: f32,
    ) {
        target.var_type = val_a.var_type;
        target.string_value = val_a.string_value.clone();
        target.float_value = MathHelper::linear(val_a.float_value, val_b.float_value, factor);
        target.int_value =
            MathHelper::linear(val_a.int_value as f32, val_b.int_value as f32, factor) as i32;
    }

    /// Fills all the values of target spatial by interpolating values from the two given spatials with the given factor and spin.
    pub fn interpolate_spatial(
        target: &mut SpriterSpatial,
        a: &SpriterSpatial,
        b: &SpriterSpatial,
        factor: f32,
        spin: i32,
    ) {
        target.angle = MathHelper::angle_linear(a.angle, b.angle, spin, factor);
        target.x = MathHelper::linear(a.x, b.x, factor);
        target.y = MathHelper::linear(a.y, b.y, factor);
        target.scale_x = MathHelper::linear(a.scale_x, b.scale_x, factor);
        target.scale_y = MathHelper::linear(a.scale_y, b.scale_y, factor);
        target.alpha = MathHelper::linear(a.alpha, b.alpha, factor);
    }

    /// Fills all the values of target object by interpolating values from the two given object with the given factor and spin.
    pub fn interpolate_object(
        target: &mut SpriterObject,
        a: &SpriterObject,
        b: &SpriterObject,
        factor: f32,
        spin: i32,
    ) {
        target.angle = MathHelper::angle_linear(a.angle, b.angle, spin, factor);
        target.alpha = MathHelper::linear(a.alpha, b.alpha, factor);
        target.x = MathHelper::linear(a.x, b.x, factor);
        target.y = MathHelper::linear(a.y, b.y, factor);
        target.scale_x = MathHelper::linear(a.scale_x, b.scale_x, factor);
        target.scale_y = MathHelper::linear(a.scale_y, b.scale_y, factor);
        target.pivot_x = a.pivot_x;
        target.pivot_y = a.pivot_y;
        target.file_id = a.file_id;
        target.folder_id = a.folder_id;
        target.entity_id = a.entity_id;
        target.animation_id = a.animation_id;
        target.t = MathHelper::linear(a.t, b.t, factor);
    }

    /// Adjusts the factor based on the curve type from the given key.
    pub fn adjust_factor(factor: f32, key: &SpriterKey) -> f32 {
        match key.curve_type {
            SpriterCurveType::Instant => 0.0,
            SpriterCurveType::Linear => factor,
            SpriterCurveType::Quadratic => MathHelper::bezier_3(0.0, key.c1, 1.0, factor),
            SpriterCurveType::Cubic => MathHelper::bezier_4(0.0, key.c1, key.c2, 1.0, factor),
            SpriterCurveType::Quartic => {
                MathHelper::bezier_5(0.0, key.c1, key.c2, key.c3, 1.0, factor)
            }
            SpriterCurveType::Quintic => {
                MathHelper::bezier_6(0.0, key.c1, key.c2, key.c3, key.c4, 1.0, factor)
            }
            SpriterCurveType::Bezier => {
                MathHelper::bezier_2d(key.c1, key.c2, key.c3, key.c4, factor)
            }
        }
    }

    /// Adjusts the animation time taking into account key curve types.
    pub fn adjust_time(
        target_time: f32,
        key_a: &SpriterKey,
        key_b: &SpriterKey,
        animation_length: f32,
    ) -> f32 {
        let next_time = Self::get_next_time(key_a, key_b, animation_length);
        let factor = Self::get_factor(key_a, key_b, animation_length, target_time);
        MathHelper::linear(key_a.time, next_time, factor)
    }

    /// Gets the interpolation factor for the two keys. Takes into account key curve types.
    pub fn get_factor(
        key_a: &SpriterKey,
        key_b: &SpriterKey,
        animation_length: f32,
        target_time: f32,
    ) -> f32 {
        let time_a = key_a.time;
        let mut time_b = key_b.time;
        let mut target_time = target_time;

        if time_a > time_b {
            time_b += animation_length;
            if time_a == time_b {
                time_b += animation_length;
            }
            if target_time < time_a {
                target_time += animation_length;
            }
        }

        let factor = MathHelper::get_factor(time_a, time_b, target_time);
        Self::adjust_factor(factor, key_a)
    }

    /// Gets next time for the two keys based on relative keys timeline position.
    fn get_next_time(key_a: &SpriterKey, key_b: &SpriterKey, animation_length: f32) -> f32 {
        if key_a.time < key_b.time {
            key_b.time
        } else if key_b.time + animation_length == key_a.time {
            key_a.time + animation_length
        } else {
            animation_length
        }
    }
}
