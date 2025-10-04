use glam::Vec2;

#[derive(Debug)]
pub struct MovementResult2D {
    pub new_position: [f32; 2],
    pub step: [f32; 2],
    pub movement_finished: bool,
}

/// Moves from `current_position` toward `target_position` at `movement_speed` (m/s) for `delta_time_seconds`,
/// stopping on the boundary of `acceptance_radius` around the target (never overlapping).
pub fn calculate_step_2d(
    current_position: [f32; 2],
    target_position: [f32; 2],
    acceptance_radius: f32,
    movement_speed: f32,
    delta_time_seconds: f32,
) -> MovementResult2D {
    // Clamp non-physical inputs
    let clamped_acceptance_radius = acceptance_radius.max(0.05);
    let clamped_speed = movement_speed.max(0.0);
    let clamped_delta_time = delta_time_seconds.max(0.0);

    let current = Vec2::from_array(current_position);
    let target = Vec2::from_array(target_position);
    let vector_to_target = target - current;

    let distance_to_target = vector_to_target.length();

    // 1) Already within the acceptance radius → no movement, finished.
    if distance_to_target <= clamped_acceptance_radius {
        return MovementResult2D {
            new_position: current_position,
            step: [0.0, 0.0],
            movement_finished: true,
        };
    }

    // 2) Compute how far we can move this frame and how far to the boundary.
    let max_distance_this_frame = clamped_speed * clamped_delta_time;
    let distance_to_boundary = distance_to_target - clamped_acceptance_radius;

    // 3) If we can reach (or pass) the boundary this frame → land exactly on boundary and finish.
    if max_distance_this_frame >= distance_to_boundary {
        // Direction is safe here: distance_to_target > acceptance_radius >= 0
        let direction_to_target = vector_to_target / distance_to_target;
        let boundary_point = target - direction_to_target * clamped_acceptance_radius;
        let step_vector = boundary_point - current;

        return MovementResult2D {
            new_position: boundary_point.to_array(),
            step: step_vector.to_array(),
            movement_finished: true,
        };
    }

    // 4) Otherwise take a partial step toward the target and continue next tick.
    let direction_to_target = vector_to_target / distance_to_target; // normalize
    let step_vector = direction_to_target * max_distance_this_frame;

    MovementResult2D {
        new_position: (current + step_vector).to_array(),
        step: step_vector.to_array(),
        movement_finished: false,
    }
}

// 3D version if needed
// // Define the outcome of a movement step
// #[derive(Debug)]
// struct MovementResult3D {
//     pub new_position: [f32; 3],
//     pub movement_finished: bool,
//     pub step: [f32; 3],
// }

// fn calculate_step(
//     current_position: [f32; 3],
//     move_target: [f32; 3],
//     speed: f32,
//     delta_time_secs: f32,
//     // acceptance_radius: f32,
// ) -> MovementResult3D {
//     let position = Vec3A::from_array(current_position);
//     let target = Vec3A::from_array(move_target);
//     let direction_to_target = target - position;
//     let distance_to_target_squared = direction_to_target.length_squared();
//     let max_distance_to_move = speed * delta_time_secs;
//     let max_distance_to_move_squared = max_distance_to_move * max_distance_to_move;

//     if distance_to_target_squared <= max_distance_to_move_squared {
//         // Destination reached
//         MovementResult3D {
//             new_position: move_target,
//             movement_finished: true,
//             step: [0., 0., 0.],
//         }
//     } else {
//         // Take a step
//         let normalized_direction = direction_to_target.normalize();
//         let step_vector = normalized_direction * max_distance_to_move;
//         MovementResult3D {
//             new_position: (position + step_vector).to_array(),
//             movement_finished: false,
//             step: step_vector.to_array(),
//         }
//     }
// }
