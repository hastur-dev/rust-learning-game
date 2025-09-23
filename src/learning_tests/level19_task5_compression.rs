// Level 19 Task 5: Simple Compression Using Bitwise Operations
// Learn compression techniques through bitwise manipulation for robot data optimization

use std::fmt;
use std::collections::HashMap;

/// Robot data compression utilities using bitwise operations
pub struct RobotCompressor;

impl RobotCompressor {
    /// Run-Length Encoding (RLE) for sparse robot map data
    /// Efficient for data with many repeated values (like empty grid cells)
    pub fn rle_compress(data: &[u8]) -> Vec<u8> {
        if data.is_empty() {
            return Vec::new();
        }

        let mut compressed = Vec::new();
        let mut current_byte = data[0];
        let mut count = 1u8;

        for &byte in &data[1..] {
            if byte == current_byte && count < 255 {
                count += 1;
            } else {
                // Store count and value
                compressed.push(count);
                compressed.push(current_byte);
                current_byte = byte;
                count = 1;
            }
        }

        // Add the last run
        compressed.push(count);
        compressed.push(current_byte);
        compressed
    }

    /// Decompress RLE data
    pub fn rle_decompress(compressed: &[u8]) -> Result<Vec<u8>, String> {
        if compressed.len() % 2 != 0 {
            return Err("Invalid RLE data length".to_string());
        }

        let mut decompressed = Vec::new();
        for chunk in compressed.chunks(2) {
            let count = chunk[0];
            let byte = chunk[1];
            for _ in 0..count {
                decompressed.push(byte);
            }
        }

        Ok(decompressed)
    }

    /// Delta compression for sensor time series data
    /// Stores differences between consecutive values instead of absolute values
    pub fn delta_compress(data: &[i16]) -> Vec<u8> {
        if data.is_empty() {
            return Vec::new();
        }

        let mut compressed = Vec::new();

        // Store first value as-is (16 bits)
        compressed.extend_from_slice(&data[0].to_le_bytes());

        // Store deltas
        for i in 1..data.len() {
            let delta = data[i] - data[i - 1];

            // Use variable-length encoding for deltas
            if delta >= -64 && delta <= 63 {
                // 7-bit delta (1 byte): 0XXXXXXX
                compressed.push((delta as u8) & 0x7F);
            } else if delta >= -8192 && delta <= 8191 {
                // 14-bit delta (2 bytes): 10XXXXXX XXXXXXXX
                let encoded = (delta as u16) & 0x3FFF;
                compressed.push(0x80 | ((encoded >> 8) as u8));
                compressed.push(encoded as u8);
            } else {
                // 16-bit delta (3 bytes): 11XXXXXX XXXXXXXX XXXXXXXX
                compressed.push(0xC0);
                compressed.extend_from_slice(&delta.to_le_bytes());
            }
        }

        compressed
    }

    /// Decompress delta-compressed data
    pub fn delta_decompress(compressed: &[u8]) -> Result<Vec<i16>, String> {
        if compressed.len() < 2 {
            return Err("Insufficient data for delta decompression".to_string());
        }

        let mut decompressed = Vec::new();
        let mut pos = 0;

        // Read first value
        if compressed.len() < 2 {
            return Err("Missing initial value".to_string());
        }
        let first_value = i16::from_le_bytes([compressed[0], compressed[1]]);
        decompressed.push(first_value);
        pos += 2;

        let mut current_value = first_value;

        while pos < compressed.len() {
            let control_byte = compressed[pos];
            pos += 1;

            let delta = if (control_byte & 0x80) == 0 {
                // 7-bit delta
                let delta = control_byte as i8;
                delta as i16
            } else if (control_byte & 0x40) == 0 {
                // 14-bit delta
                if pos >= compressed.len() {
                    return Err("Incomplete 14-bit delta".to_string());
                }
                let high = (control_byte & 0x3F) as u16;
                let low = compressed[pos] as u16;
                pos += 1;
                let combined = (high << 8) | low;
                // Sign extend 14-bit to 16-bit
                if (combined & 0x2000) != 0 {
                    (combined | 0xC000) as i16
                } else {
                    combined as i16
                }
            } else {
                // 16-bit delta
                if pos + 1 >= compressed.len() {
                    return Err("Incomplete 16-bit delta".to_string());
                }
                let delta = i16::from_le_bytes([compressed[pos], compressed[pos + 1]]);
                pos += 2;
                delta
            };

            current_value = current_value.wrapping_add(delta);
            decompressed.push(current_value);
        }

        Ok(decompressed)
    }

    /// Bit packing for robot configuration data
    /// Pack multiple small values into fewer bytes
    pub fn pack_robot_config(
        speed: u8,        // 0-15 (4 bits)
        sensitivity: u8,  // 0-7 (3 bits)
        mode: u8,         // 0-3 (2 bits)
        flags: u8,        // 8 boolean flags
        battery_level: u8, // 0-100 (7 bits)
    ) -> u32 {
        let mut packed = 0u32;

        // Pack values using bit shifts and masks
        packed |= (speed as u32 & 0xF) << 28;        // Bits 31-28
        packed |= (sensitivity as u32 & 0x7) << 25;  // Bits 27-25
        packed |= (mode as u32 & 0x3) << 23;         // Bits 24-23
        packed |= (flags as u32) << 15;              // Bits 22-15
        packed |= (battery_level as u32 & 0x7F) << 8; // Bits 14-8
        // Bits 7-0 reserved for future use

        packed
    }

    /// Unpack robot configuration data
    pub fn unpack_robot_config(packed: u32) -> (u8, u8, u8, u8, u8) {
        let speed = ((packed >> 28) & 0xF) as u8;
        let sensitivity = ((packed >> 25) & 0x7) as u8;
        let mode = ((packed >> 23) & 0x3) as u8;
        let flags = ((packed >> 15) & 0xFF) as u8;
        let battery_level = ((packed >> 8) & 0x7F) as u8;

        (speed, sensitivity, mode, flags, battery_level)
    }

    /// Huffman-like encoding for frequent robot commands
    /// Creates a simple frequency-based bit encoding
    pub fn create_command_encoding(commands: &[&str]) -> HashMap<String, Vec<bool>> {
        let mut frequency = HashMap::new();

        // Count frequencies
        for &cmd in commands {
            *frequency.entry(cmd.to_string()).or_insert(0) += 1;
        }

        // Sort by frequency (most frequent first)
        let mut sorted_commands: Vec<_> = frequency.into_iter().collect();
        sorted_commands.sort_by(|a, b| b.1.cmp(&a.1));

        let mut encoding = HashMap::new();

        // Assign shorter bit patterns to more frequent commands
        for (i, (command, _)) in sorted_commands.iter().enumerate() {
            let bit_pattern = if i == 0 {
                vec![false] // Most frequent: 0
            } else if i == 1 {
                vec![true, false] // Second: 10
            } else if i == 2 {
                vec![true, true, false] // Third: 110
            } else if i == 3 {
                vec![true, true, true, false] // Fourth: 1110
            } else {
                // Others: 1111 + 4-bit index
                let mut pattern = vec![true, true, true, true];
                for bit in 0..4 {
                    pattern.push((i & (1 << bit)) != 0);
                }
                pattern
            };

            encoding.insert(command.clone(), bit_pattern);
        }

        encoding
    }

    /// Encode commands using the bit encoding
    pub fn encode_commands(commands: &[&str], encoding: &HashMap<String, Vec<bool>>) -> Vec<u8> {
        let mut bits = Vec::new();

        for &cmd in commands {
            if let Some(pattern) = encoding.get(cmd) {
                bits.extend(pattern);
            }
        }

        // Pack bits into bytes
        let mut bytes = Vec::new();
        for chunk in bits.chunks(8) {
            let mut byte = 0u8;
            for (i, &bit) in chunk.iter().enumerate() {
                if bit {
                    byte |= 1 << (7 - i);
                }
            }
            bytes.push(byte);
        }

        bytes
    }

    /// Sparse matrix compression for robot grid maps
    /// Only stores non-zero values with their coordinates
    pub fn compress_sparse_matrix(matrix: &[Vec<u8>]) -> Vec<u8> {
        let mut compressed = Vec::new();

        if matrix.is_empty() {
            return compressed;
        }

        let rows = matrix.len() as u8;
        let cols = matrix[0].len() as u8;

        // Store dimensions
        compressed.push(rows);
        compressed.push(cols);

        // Store non-zero elements as (row, col, value) triplets
        for (row, row_data) in matrix.iter().enumerate() {
            for (col, &value) in row_data.iter().enumerate() {
                if value != 0 {
                    compressed.push(row as u8);
                    compressed.push(col as u8);
                    compressed.push(value);
                }
            }
        }

        compressed
    }

    /// Decompress sparse matrix
    pub fn decompress_sparse_matrix(compressed: &[u8]) -> Result<Vec<Vec<u8>>, String> {
        if compressed.len() < 2 {
            return Err("Invalid sparse matrix data".to_string());
        }

        let rows = compressed[0] as usize;
        let cols = compressed[1] as usize;

        if (compressed.len() - 2) % 3 != 0 {
            return Err("Invalid triplet data".to_string());
        }

        let mut matrix = vec![vec![0u8; cols]; rows];

        // Process triplets
        for chunk in compressed[2..].chunks(3) {
            let row = chunk[0] as usize;
            let col = chunk[1] as usize;
            let value = chunk[2];

            if row >= rows || col >= cols {
                return Err("Invalid coordinates in sparse matrix".to_string());
            }

            matrix[row][col] = value;
        }

        Ok(matrix)
    }

    /// LZ77-style compression for robot path data
    /// Finds repeating patterns and replaces with back-references
    pub fn lz77_compress(data: &[u8], window_size: usize, lookahead_size: usize) -> Vec<u8> {
        let mut compressed = Vec::new();
        let mut pos = 0;

        while pos < data.len() {
            let mut best_length = 0;
            let mut best_distance = 0;

            // Look for matches in the sliding window
            let window_start = if pos < window_size { 0 } else { pos - window_size };
            let lookahead_end = (pos + lookahead_size).min(data.len());

            for start in window_start..pos {
                let mut length = 0;
                while start + length < pos &&
                      pos + length < lookahead_end &&
                      data[start + length] == data[pos + length] {
                    length += 1;
                }

                if length > best_length {
                    best_length = length;
                    best_distance = pos - start;
                }
            }

            if best_length >= 3 {
                // Store back-reference: distance (2 bytes) + length (1 byte)
                compressed.push(0xFF); // Marker for back-reference
                compressed.extend_from_slice(&(best_distance as u16).to_le_bytes());
                compressed.push(best_length as u8);
                pos += best_length;
            } else {
                // Store literal byte
                compressed.push(data[pos]);
                pos += 1;
            }
        }

        compressed
    }

    /// Calculate compression ratio
    pub fn compression_ratio(original_size: usize, compressed_size: usize) -> f32 {
        if original_size == 0 {
            return 0.0;
        }
        (1.0 - (compressed_size as f32 / original_size as f32)) * 100.0
    }
}

/// Specialized compression for different types of robot data
pub struct RobotDataCompressor;

impl RobotDataCompressor {
    /// Compress robot sensor readings using multiple techniques
    pub fn compress_sensor_data(readings: &[(f32, f32, u8)]) -> Vec<u8> {
        let mut compressed = Vec::new();

        if readings.is_empty() {
            return compressed;
        }

        // Convert to integers for better compression
        let mut temp_data = Vec::new();
        let mut humidity_data = Vec::new();
        let mut light_data = Vec::new();

        for &(temp, humidity, light) in readings {
            temp_data.push((temp * 10.0) as i16); // 0.1 degree precision
            humidity_data.push((humidity * 10.0) as i16); // 0.1% precision
            light_data.push(light);
        }

        // Use delta compression for temperature and humidity
        let temp_compressed = RobotCompressor::delta_compress(&temp_data);
        let humidity_compressed = RobotCompressor::delta_compress(&humidity_data);

        // Use RLE for light data (often has repeated values)
        let light_compressed = RobotCompressor::rle_compress(&light_data);

        // Pack lengths and data
        compressed.extend_from_slice(&(temp_compressed.len() as u16).to_le_bytes());
        compressed.extend_from_slice(&(humidity_compressed.len() as u16).to_le_bytes());
        compressed.extend_from_slice(&(light_compressed.len() as u16).to_le_bytes());

        compressed.extend(temp_compressed);
        compressed.extend(humidity_compressed);
        compressed.extend(light_compressed);

        compressed
    }

    /// Decompress sensor data
    pub fn decompress_sensor_data(compressed: &[u8]) -> Result<Vec<(f32, f32, u8)>, String> {
        if compressed.len() < 6 {
            return Err("Invalid compressed sensor data".to_string());
        }

        let temp_len = u16::from_le_bytes([compressed[0], compressed[1]]) as usize;
        let humidity_len = u16::from_le_bytes([compressed[2], compressed[3]]) as usize;
        let light_len = u16::from_le_bytes([compressed[4], compressed[5]]) as usize;

        let mut pos = 6;

        if pos + temp_len + humidity_len + light_len > compressed.len() {
            return Err("Insufficient data for decompression".to_string());
        }

        // Decompress each data stream
        let temp_data = RobotCompressor::delta_decompress(&compressed[pos..pos + temp_len])?;
        pos += temp_len;

        let humidity_data = RobotCompressor::delta_decompress(&compressed[pos..pos + humidity_len])?;
        pos += humidity_len;

        let light_data = RobotCompressor::rle_decompress(&compressed[pos..pos + light_len])?;

        // Verify all streams have the same length
        if temp_data.len() != humidity_data.len() || humidity_data.len() != light_data.len() {
            return Err("Inconsistent data stream lengths".to_string());
        }

        // Convert back to original format
        let mut result = Vec::new();
        for i in 0..temp_data.len() {
            let temp = temp_data[i] as f32 / 10.0;
            let humidity = humidity_data[i] as f32 / 10.0;
            let light = light_data[i];
            result.push((temp, humidity, light));
        }

        Ok(result)
    }

    /// Compress robot movement patterns
    pub fn compress_movement_pattern(moves: &[(i8, i8)]) -> Vec<u8> {
        let mut compressed = Vec::new();

        // Convert 2D moves to single values for better pattern detection
        let mut move_values = Vec::new();
        for &(dx, dy) in moves {
            // Encode direction as single value: 0-8 (including no movement)
            let encoded = match (dx.signum(), dy.signum()) {
                (0, 0) => 0,   // No movement
                (0, 1) => 1,   // North
                (1, 1) => 2,   // Northeast
                (1, 0) => 3,   // East
                (1, -1) => 4,  // Southeast
                (0, -1) => 5,  // South
                (-1, -1) => 6, // Southwest
                (-1, 0) => 7,  // West
                (-1, 1) => 8,  // Northwest
                _ => 0,
            };
            move_values.push(encoded);
        }

        // Use RLE compression for movement patterns
        RobotCompressor::rle_compress(&move_values)
    }
}

/// Bit-level compression utilities
pub struct BitCompression;

impl BitCompression {
    /// Pack boolean flags into minimal bits
    pub fn pack_flags(flags: &[bool]) -> Vec<u8> {
        let mut packed = Vec::new();

        for chunk in flags.chunks(8) {
            let mut byte = 0u8;
            for (i, &flag) in chunk.iter().enumerate() {
                if flag {
                    byte |= 1 << i;
                }
            }
            packed.push(byte);
        }

        packed
    }

    /// Unpack boolean flags from bits
    pub fn unpack_flags(packed: &[u8], flag_count: usize) -> Vec<bool> {
        let mut flags = Vec::new();

        for (byte_idx, &byte) in packed.iter().enumerate() {
            for bit_idx in 0..8 {
                if byte_idx * 8 + bit_idx >= flag_count {
                    break;
                }
                flags.push((byte & (1 << bit_idx)) != 0);
            }
        }

        flags
    }

    /// Variable-length integer encoding
    pub fn encode_varint(value: u32) -> Vec<u8> {
        let mut result = Vec::new();
        let mut val = value;

        loop {
            let mut byte = (val & 0x7F) as u8;
            val >>= 7;
            if val != 0 {
                byte |= 0x80; // More bytes follow
            }
            result.push(byte);
            if val == 0 {
                break;
            }
        }

        result
    }

    /// Decode variable-length integer
    pub fn decode_varint(bytes: &[u8]) -> Result<(u32, usize), String> {
        let mut result = 0u32;
        let mut shift = 0;
        let mut bytes_read = 0;

        for &byte in bytes {
            bytes_read += 1;
            result |= ((byte & 0x7F) as u32) << shift;
            shift += 7;

            if shift >= 32 {
                return Err("Varint too long".to_string());
            }

            if (byte & 0x80) == 0 {
                return Ok((result, bytes_read));
            }
        }

        Err("Incomplete varint".to_string())
    }
}

/// Demonstrate compression techniques
pub fn demonstrate_compression_techniques() {
    println!("=== Robot Data Compression Techniques ===");

    // RLE Compression demonstration
    println!("\n--- Run-Length Encoding (RLE) ---");
    let sparse_map = vec![0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 2, 2, 2, 0, 0, 0];
    println!("Original map ({} bytes): {:?}", sparse_map.len(), sparse_map);

    let rle_compressed = RobotCompressor::rle_compress(&sparse_map);
    println!("RLE compressed ({} bytes): {:?}", rle_compressed.len(), rle_compressed);

    match RobotCompressor::rle_decompress(&rle_compressed) {
        Ok(decompressed) => {
            println!("Decompressed: {:?}", decompressed);
            println!("RLE ratio: {:.1}%",
                     RobotCompressor::compression_ratio(sparse_map.len(), rle_compressed.len()));
        }
        Err(e) => println!("RLE decompression error: {}", e),
    }

    // Delta compression demonstration
    println!("\n--- Delta Compression ---");
    let sensor_readings = vec![100, 102, 101, 103, 102, 104, 103, 105, 104, 106];
    println!("Original readings ({} values): {:?}", sensor_readings.len(), sensor_readings);

    let delta_compressed = RobotCompressor::delta_compress(&sensor_readings);
    println!("Delta compressed ({} bytes): {:02X?}", delta_compressed.len(), delta_compressed);

    match RobotCompressor::delta_decompress(&delta_compressed) {
        Ok(decompressed) => {
            println!("Decompressed: {:?}", decompressed);
            println!("Delta ratio: {:.1}%",
                     RobotCompressor::compression_ratio(sensor_readings.len() * 2, delta_compressed.len()));
        }
        Err(e) => println!("Delta decompression error: {}", e),
    }

    // Bit packing demonstration
    println!("\n--- Bit Packing ---");
    let speed = 12;       // 4 bits
    let sensitivity = 5;  // 3 bits
    let mode = 2;         // 2 bits
    let flags = 0b10110101; // 8 bits
    let battery = 85;     // 7 bits

    println!("Original config: speed={}, sens={}, mode={}, flags=0b{:08b}, battery={}",
             speed, sensitivity, mode, flags, battery);

    let packed = RobotCompressor::pack_robot_config(speed, sensitivity, mode, flags, battery);
    println!("Packed config: 0x{:08X} (4 bytes)", packed);

    let (s, se, m, f, b) = RobotCompressor::unpack_robot_config(packed);
    println!("Unpacked: speed={}, sens={}, mode={}, flags=0b{:08b}, battery={}", s, se, m, f, b);

    println!("Original size: 5 bytes, Packed size: 4 bytes, Savings: 20%");

    // Command encoding demonstration
    println!("\n--- Command Frequency Encoding ---");
    let commands = vec![
        "MOVE", "MOVE", "MOVE", "MOVE", "MOVE", "MOVE", // 6 times
        "SCAN", "SCAN", "SCAN", // 3 times
        "TURN", "TURN", // 2 times
        "STOP", // 1 time
    ];

    let encoding = RobotCompressor::create_command_encoding(&commands);
    println!("Command encoding:");
    for (cmd, bits) in &encoding {
        let bit_str: String = bits.iter().map(|&b| if b { '1' } else { '0' }).collect();
        println!("  {} -> {}", cmd, bit_str);
    }

    let encoded_bytes = RobotCompressor::encode_commands(&commands, &encoding);
    println!("Encoded commands ({} bytes): {:02X?}", encoded_bytes.len(), encoded_bytes);

    let original_size = commands.len() * 4; // Assume 4 bytes per command string
    println!("Compression ratio: {:.1}%",
             RobotCompressor::compression_ratio(original_size, encoded_bytes.len()));

    // Sparse matrix demonstration
    println!("\n--- Sparse Matrix Compression ---");
    let matrix = vec![
        vec![0, 0, 3, 0, 0],
        vec![0, 0, 0, 0, 0],
        vec![1, 0, 0, 0, 2],
        vec![0, 0, 0, 0, 0],
    ];

    println!("Original matrix ({}x{}):", matrix.len(), matrix[0].len());
    for row in &matrix {
        println!("  {:?}", row);
    }

    let sparse_compressed = RobotCompressor::compress_sparse_matrix(&matrix);
    println!("Sparse compressed ({} bytes): {:?}", sparse_compressed.len(), sparse_compressed);

    match RobotCompressor::decompress_sparse_matrix(&sparse_compressed) {
        Ok(decompressed) => {
            println!("Decompressed matrix:");
            for row in &decompressed {
                println!("  {:?}", row);
            }
            let original_size = matrix.len() * matrix[0].len();
            println!("Compression ratio: {:.1}%",
                     RobotCompressor::compression_ratio(original_size, sparse_compressed.len()));
        }
        Err(e) => println!("Sparse decompression error: {}", e),
    }

    // Sensor data compression
    println!("\n--- Multi-Stream Sensor Compression ---");
    let sensor_data = vec![
        (23.5, 65.2, 180), // temp, humidity, light
        (23.6, 65.1, 180),
        (23.4, 65.3, 180),
        (23.7, 65.0, 182),
        (23.5, 65.2, 182),
    ];

    println!("Original sensor data ({} readings):", sensor_data.len());
    for (i, (t, h, l)) in sensor_data.iter().enumerate() {
        println!("  {}: {:.1}°C, {:.1}%, {}", i, t, h, l);
    }

    let multi_compressed = RobotDataCompressor::compress_sensor_data(&sensor_data);
    println!("Multi-stream compressed ({} bytes): {:02X?}", multi_compressed.len(), multi_compressed);

    match RobotDataCompressor::decompress_sensor_data(&multi_compressed) {
        Ok(decompressed) => {
            println!("Decompressed sensor data:");
            for (i, (t, h, l)) in decompressed.iter().enumerate() {
                println!("  {}: {:.1}°C, {:.1}%, {}", i, t, h, l);
            }
            let original_size = sensor_data.len() * (4 + 4 + 1); // f32 + f32 + u8
            println!("Compression ratio: {:.1}%",
                     RobotCompressor::compression_ratio(original_size, multi_compressed.len()));
        }
        Err(e) => println!("Multi-stream decompression error: {}", e),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::learning_tests::test_utils::*;

    #[test]
    fn test_rle_compression() {
        let data = vec![1, 1, 1, 2, 3, 3, 3, 3, 4];
        let compressed = RobotCompressor::rle_compress(&data);
        let expected = vec![3, 1, 1, 2, 4, 3, 1, 4];
        assert_eq!(compressed, expected);

        let decompressed = RobotCompressor::rle_decompress(&compressed).unwrap();
        assert_eq!(decompressed, data);

        // Test empty data
        let empty_compressed = RobotCompressor::rle_compress(&[]);
        assert!(empty_compressed.is_empty());
    }

    #[test]
    fn test_delta_compression() {
        let data = vec![100, 102, 101, 105, 98];
        let compressed = RobotCompressor::delta_compress(&data);
        let decompressed = RobotCompressor::delta_decompress(&compressed).unwrap();
        assert_eq!(decompressed, data);

        // Test large deltas
        let large_data = vec![0, 10000, -5000, 15000];
        let large_compressed = RobotCompressor::delta_compress(&large_data);
        let large_decompressed = RobotCompressor::delta_decompress(&large_compressed).unwrap();
        assert_eq!(large_decompressed, large_data);
    }

    #[test]
    fn test_bit_packing() {
        let speed = 15;
        let sensitivity = 7;
        let mode = 3;
        let flags = 0xFF;
        let battery = 100;

        let packed = RobotCompressor::pack_robot_config(speed, sensitivity, mode, flags, battery);
        let (s, se, m, f, b) = RobotCompressor::unpack_robot_config(packed);

        assert_eq!(s, speed);
        assert_eq!(se, sensitivity);
        assert_eq!(m, mode);
        assert_eq!(f, flags);
        assert_eq!(b, battery);
    }

    #[test]
    fn test_command_encoding() {
        let commands = vec!["MOVE", "MOVE", "SCAN", "TURN"];
        let encoding = RobotCompressor::create_command_encoding(&commands);

        // Most frequent command should have shortest encoding
        assert!(encoding["MOVE"].len() <= encoding["SCAN"].len());
        assert!(encoding["SCAN"].len() <= encoding["TURN"].len());

        let encoded = RobotCompressor::encode_commands(&commands, &encoding);
        assert!(!encoded.is_empty());
    }

    #[test]
    fn test_sparse_matrix() {
        let matrix = vec![
            vec![0, 1, 0],
            vec![0, 0, 0],
            vec![2, 0, 3],
        ];

        let compressed = RobotCompressor::compress_sparse_matrix(&matrix);
        let decompressed = RobotCompressor::decompress_sparse_matrix(&compressed).unwrap();
        assert_eq!(decompressed, matrix);

        // Test empty matrix
        let empty_matrix: Vec<Vec<u8>> = vec![];
        let empty_compressed = RobotCompressor::compress_sparse_matrix(&empty_matrix);
        assert!(empty_compressed.is_empty());
    }

    #[test]
    fn test_varint_encoding() {
        let test_values = [0, 127, 128, 16383, 16384, 2097151];

        for value in test_values {
            let encoded = BitCompression::encode_varint(value);
            let (decoded, bytes_read) = BitCompression::decode_varint(&encoded).unwrap();
            assert_eq!(decoded, value);
            assert_eq!(bytes_read, encoded.len());
        }
    }

    #[test]
    fn test_flag_packing() {
        let flags = vec![true, false, true, true, false, false, true, false, true];
        let packed = BitCompression::pack_flags(&flags);
        let unpacked = BitCompression::unpack_flags(&packed, flags.len());
        assert_eq!(unpacked, flags);

        // Test exact 8 flags
        let eight_flags = vec![true; 8];
        let eight_packed = BitCompression::pack_flags(&eight_flags);
        assert_eq!(eight_packed.len(), 1);
        assert_eq!(eight_packed[0], 0xFF);
    }

    #[test]
    fn test_sensor_data_compression() {
        let sensor_data = vec![
            (20.0, 50.0, 100),
            (20.1, 50.2, 102),
            (19.9, 49.8, 98),
        ];

        let compressed = RobotDataCompressor::compress_sensor_data(&sensor_data);
        let decompressed = RobotDataCompressor::decompress_sensor_data(&compressed).unwrap();

        assert_eq!(decompressed.len(), sensor_data.len());

        // Check values are approximately equal (allowing for precision loss)
        for (orig, decomp) in sensor_data.iter().zip(decompressed.iter()) {
            assert!((orig.0 - decomp.0).abs() < 0.2); // Temperature
            assert!((orig.1 - decomp.1).abs() < 0.2); // Humidity
            assert_eq!(orig.2, decomp.2); // Light (exact)
        }
    }

    #[test]
    fn test_compression_ratios() {
        // Test that compression actually reduces size for appropriate data
        let repetitive_data = vec![5u8; 100];
        let rle_compressed = RobotCompressor::rle_compress(&repetitive_data);
        assert!(rle_compressed.len() < repetitive_data.len());

        let sequential_data: Vec<i16> = (0..100).collect();
        let delta_compressed = RobotCompressor::delta_compress(&sequential_data);
        assert!(delta_compressed.len() < sequential_data.len() * 2);
    }

    #[test]
    fn test_error_handling() {
        // Test invalid RLE data
        let invalid_rle = vec![1]; // Odd length
        assert!(RobotCompressor::rle_decompress(&invalid_rle).is_err());

        // Test invalid delta data
        let invalid_delta = vec![0]; // Too short
        assert!(RobotCompressor::delta_decompress(&invalid_delta).is_err());

        // Test invalid sparse matrix
        let invalid_sparse = vec![2, 2, 0, 0]; // Incomplete triplet
        assert!(RobotCompressor::decompress_sparse_matrix(&invalid_sparse).is_err());

        // Test invalid varint
        let invalid_varint = vec![0x80, 0x80, 0x80, 0x80, 0x80]; // Too long
        assert!(BitCompression::decode_varint(&invalid_varint).is_err());
    }

    #[test]
    fn test_user_code_implementation() {
        let analyzer = create_analyzer().expect("Failed to load user code");

        // Check for compression-related structures
        assert!(
            analyzer.contains_struct("Compress") ||
            analyzer.contains_struct("RLE") ||
            analyzer.contains_struct("Delta") ||
            analyzer.code.contains("compress") ||
            analyzer.code.contains("pack"),
            "❌ You should implement compression-related structures"
        );

        // Check for bitwise operations used in compression
        assert!(
            analyzer.code.contains("<<") || analyzer.code.contains(">>") ||
            analyzer.code.contains("&") || analyzer.code.contains("|"),
            "❌ You should use bitwise operations for data packing and compression"
        );

        // Check for compression algorithms
        assert!(
            analyzer.contains_function("compress") ||
            analyzer.contains_function("decompress") ||
            analyzer.contains_function("encode") ||
            analyzer.contains_function("decode") ||
            analyzer.contains_function("pack") ||
            analyzer.contains_function("unpack"),
            "❌ You should implement compression/decompression functions"
        );

        // Check for different compression techniques
        assert!(
            analyzer.code.contains("rle") ||
            analyzer.code.contains("delta") ||
            analyzer.code.contains("run") ||
            analyzer.code.contains("length") ||
            analyzer.code.contains("varint") ||
            analyzer.code.contains("sparse"),
            "❌ You should implement specific compression techniques (RLE, delta, etc.)"
        );

        // Check for bit manipulation in packing
        assert!(
            analyzer.code.contains("0x") || analyzer.code.contains("0b") ||
            analyzer.code.contains("mask") || analyzer.code.contains("shift"),
            "❌ You should use bit masks and shifts for data packing"
        );

        // Check for output demonstration
        assert!(
            analyzer.contains_println(),
            "❌ You should demonstrate compression techniques with output"
        );
    }
}

/// Student exercises for practicing compression with bitwise operations
pub mod exercises {
    use super::*;

    /// Exercise 1: Implement image compression for robot vision
    pub fn exercise_image_compression() {
        println!("Exercise 1: Robot Vision Image Compression");

        // TODO: Implement simple image compression for robot cameras:
        // - Grayscale image representation (8-bit per pixel)
        // - RLE compression for background areas
        // - Delta compression for gradual changes
        // - Bit packing for reduced color depth

        println!("Implement compression for robot vision data:");
        println!("- 64x64 grayscale images (4096 bytes uncompressed)");
        println!("- Use RLE for uniform background areas");
        println!("- Use delta encoding for smooth gradients");
        println!("- Pack 4-bit pixels for low-detail areas");

        // Sample image data (simplified)
        let image_scenarios = [
            "Empty room (mostly uniform background)",
            "Outdoor scene (sky gradient + ground texture)",
            "Industrial setting (regular patterns + details)",
        ];

        for scenario in image_scenarios {
            println!("Scenario: {}", scenario);
            println!("  Compression strategy: [YOUR IMPLEMENTATION HERE]");
            println!("  Expected ratio: [YOUR CALCULATION HERE]");
        }
    }

    /// Exercise 2: Create a robot log compression system
    pub fn exercise_log_compression() {
        println!("\nExercise 2: Robot Log Compression System");

        // TODO: Design compression for robot operational logs:
        // - Timestamp delta compression
        // - Event type frequency encoding
        // - Coordinate delta compression for position logs
        // - String table for repeated messages

        println!("Log compression requirements:");
        println!("- Timestamps: millisecond precision, delta encoded");
        println!("- Event types: ~20 types, frequency-based encoding");
        println!("- Positions: (x,y) coordinates, delta compressed");
        println!("- Messages: String table for common messages");

        let log_entries = [
            ("2024-01-01T10:00:00.000", "MOVE", (10, 20), "Moving to waypoint"),
            ("2024-01-01T10:00:01.250", "SCAN", (11, 21), "Obstacle detected"),
            ("2024-01-01T10:00:02.100", "MOVE", (12, 22), "Moving to waypoint"),
            ("2024-01-01T10:00:03.500", "ALERT", (12, 22), "Low battery warning"),
        ];

        for (timestamp, event, pos, message) in log_entries {
            println!("Log: {} {} {:?} '{}'", timestamp, event, pos, message);
            println!("  Compressed: [YOUR IMPLEMENTATION HERE]");
        }
    }

    /// Exercise 3: Implement path compression for navigation
    pub fn exercise_path_compression() {
        println!("\nExercise 3: Navigation Path Compression");

        // TODO: Compress robot navigation paths efficiently:
        // - Direction encoding (8 directions = 3 bits)
        // - Distance run-length encoding
        // - Path optimization (remove redundant waypoints)
        // - Bezier curve approximation for smooth paths

        println!("Path compression techniques:");
        println!("- Encode directions as 3-bit values");
        println!("- Use RLE for straight-line segments");
        println!("- Remove redundant intermediate points");
        println!("- Approximate curves with control points");

        let sample_paths = [
            vec![(0,0), (1,0), (2,0), (3,0), (4,0), (5,0)], // Straight line
            vec![(0,0), (1,1), (2,2), (3,3), (4,4), (5,5)], // Diagonal
            vec![(0,0), (1,0), (1,1), (2,1), (2,2), (3,2)], // L-shaped segments
        ];

        for (i, path) in sample_paths.iter().enumerate() {
            println!("Path {}: {:?}", i + 1, path);
            println!("  Original size: {} waypoints", path.len());
            println!("  Compressed: [YOUR IMPLEMENTATION HERE]");
            println!("  Optimized: [YOUR IMPLEMENTATION HERE]");
        }
    }

    /// Exercise 4: Create a sensor calibration data compressor
    pub fn exercise_calibration_compression() {
        println!("\nExercise 4: Sensor Calibration Data Compression");

        // TODO: Compress calibration tables for robot sensors:
        // - Lookup tables with sparse entries
        // - Polynomial coefficient encoding
        // - Interpolation-based compression
        // - Quantization for acceptable precision loss

        println!("Calibration compression methods:");
        println!("- Sparse lookup tables (store only non-linear regions)");
        println!("- Polynomial approximation (store coefficients)");
        println!("- Linear interpolation between key points");
        println!("- Quantization to reduce precision requirements");

        // Example: temperature sensor calibration (ADC value -> actual temperature)
        let calibration_points = [
            (0, -40.0),     // ADC 0 = -40°C
            (512, 0.0),     // ADC 512 = 0°C
            (1024, 40.0),   // ADC 1024 = 40°C
            (1536, 80.0),   // ADC 1536 = 80°C
            (2048, 120.0),  // ADC 2048 = 120°C
        ];

        println!("Temperature calibration points:");
        for (adc, temp) in calibration_points {
            println!("  ADC {} -> {:.1}°C", adc, temp);
        }
        println!("Full table: 2048 entries × 4 bytes = 8KB");
        println!("Compressed representation: [YOUR IMPLEMENTATION HERE]");
    }

    /// Exercise 5: Design a multi-robot data synchronization compressor
    pub fn exercise_sync_compression() {
        println!("\nExercise 5: Multi-Robot Data Synchronization");

        // TODO: Implement compression for synchronizing data between robots:
        // - Delta synchronization (only send changes)
        // - Bloom filters for change detection
        // - Priority-based compression (critical vs. non-critical data)
        // - Incremental updates with checksums

        println!("Synchronization compression features:");
        println!("- Track data versions for delta sync");
        println!("- Use Bloom filters to detect changes efficiently");
        println!("- Prioritize critical data over informational data");
        println!("- Verify integrity with incremental checksums");

        let robot_states = [
            ("Robot1", vec![("position", "10,20"), ("battery", "85"), ("status", "active")]),
            ("Robot2", vec![("position", "15,25"), ("battery", "72"), ("status", "charging")]),
            ("Robot3", vec![("position", "5,30"), ("battery", "91"), ("status", "active")]),
        ];

        println!("Robot state synchronization:");
        for (robot, state) in robot_states {
            println!("{}: {:?}", robot, state);
        }

        println!("Sync scenarios:");
        println!("1. Robot1 moves to (11,21) - position update only");
        println!("2. Robot2 battery drops to 15% - critical update");
        println!("3. Robot3 goes offline - status change");

        for i in 1..=3 {
            println!("Scenario {}: [YOUR COMPRESSION STRATEGY HERE]", i);
        }
    }
}

fn main() {
    demonstrate_compression_techniques();

    println!("\n{}", "=".repeat(50));
    println!("STUDENT EXERCISES");
    println!("{}", "=".repeat(50));

    exercises::exercise_image_compression();
    exercises::exercise_log_compression();
    exercises::exercise_path_compression();
    exercises::exercise_calibration_compression();
    exercises::exercise_sync_compression();
}