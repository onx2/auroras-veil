//! Utilities for encoding 2D chunk coordinates into a single `u32` ID.
//!
//! - Chunk size: 20.0 meters per side
//! - World size: ~655 km x 655 km (at 20 m per chunk)
//! - Coordinate range: [-32768, 32767] chunks per axis
//! - Storage format: lower 16 bits = X, upper 16 bits = Z

/// Chunk size in world units (meters).
const CHUNK_SIZE: f32 = 20.0;

/// Number of bits per axis (16 + 16 = 32 bits total).
const HALF_BITS: i32 = 16;

/// Offset used to shift negative chunk coordinates into unsigned range.
/// Example: chunk_x = -1 → shifted_x = 32767 (valid u16).
const OFFSET_CHUNKS: i32 = 1 << (HALF_BITS - 1); // 32768

/// Encode world coordinates `(x, z)` (in meters) into a packed chunk ID.
///
/// Each axis is divided into 20 m "chunks".
/// The chunk coordinate is floored so negatives are consistent:
///   - Example: `x = -0.1` → `chunk_x = -1`
///
/// Range checks (debug only) ensure that the packed ID does not overflow
/// the 16-bit per-axis storage.
#[inline]
pub fn encode(x: f32, z: f32) -> u32 {
    // Convert from world units into chunk indices (signed).
    let chunk_x = (x / CHUNK_SIZE).floor() as i32;
    let chunk_z = (z / CHUNK_SIZE).floor() as i32;

    // Sanity check: ensure we are inside the valid range.
    debug_assert!(
        (-OFFSET_CHUNKS..OFFSET_CHUNKS).contains(&chunk_x),
        "chunk_x {} out of range",
        chunk_x
    );
    debug_assert!(
        (-OFFSET_CHUNKS..OFFSET_CHUNKS).contains(&chunk_z),
        "chunk_z {} out of range",
        chunk_z
    );

    // Shift into unsigned space [0..65535].
    let shifted_x = (chunk_x + OFFSET_CHUNKS) as u32;
    let shifted_z = (chunk_z + OFFSET_CHUNKS) as u32;

    // Pack into 32 bits: Z in the upper 16, X in the lower 16.
    (shifted_z << HALF_BITS) | shifted_x
}

/// Decode a packed chunk_id into signed (chunk_x, chunk_z).
#[inline]
pub fn decode(chunk_id: u32) -> (i32, i32) {
    let x = (chunk_id & 0xFFFF) as i32 - OFFSET_CHUNKS;
    let z = ((chunk_id >> HALF_BITS) & 0xFFFF) as i32 - OFFSET_CHUNKS;
    (x, z)
}

/// Chebyshev (box) radius check in chunk space.
/// Returns true if `other_id` is within `radius` chunks of `center_id`.
#[inline]
pub fn within_radius(center_id: u32, other_id: u32, radius: i32) -> bool {
    let (cx, cz) = decode(center_id);
    let (ox, oz) = decode(other_id);
    (ox - cx).abs() <= radius && (oz - cz).abs() <= radius
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn encode_origin() {
        let id = encode(0.0, 0.0);
        // Chunk (0,0) should encode into something valid
        // We don’t test exact u32, just consistency
        assert!(id > 0);
    }

    #[test]
    fn encode_positive_coords() {
        let id1 = encode(45.0, 45.0); // (2,2)
        let id2 = encode(46.0, 46.0); // still (2,2), same chunk
        assert_eq!(id1, id2);
    }

    #[test]
    fn encode_negative_coords() {
        let id1 = encode(-0.1, -0.1); // (-1,-1)
        let id2 = encode(-19.9, -19.9); // still (-1,-1)
        assert_eq!(id1, id2);
    }

    #[test]
    fn encode_chunk_boundary() {
        let id1 = encode(19.9, 19.9); // (0,0)
        let id2 = encode(20.0, 20.0); // (1,1)
        assert_ne!(id1, id2);
    }

    #[test]
    fn encode_max_range() {
        // Edge of the supported world (655,340m)
        let id = encode(32767.0 * CHUNK_SIZE, -32768.0 * CHUNK_SIZE);
        assert!(id > 0);
    }

    #[test]
    #[should_panic]
    fn encode_out_of_range_panics() {
        // Should trigger debug_assert in debug builds (release skips)
        let _ = encode((OFFSET_CHUNKS as f32) * CHUNK_SIZE, 0.0);
    }
}
