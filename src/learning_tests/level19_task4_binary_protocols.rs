// Level 19 Task 4: Binary Protocol Implementation
// Learn to design and implement efficient binary communication protocols for robots

use std::fmt;
use std::collections::HashMap;

/// Robot communication protocol header
/// Packed into 32 bits for efficient transmission
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ProtocolHeader {
    data: u32,
}

impl ProtocolHeader {
    // Bit field layout for protocol header:
    // Bits 31-28: Protocol version (4 bits, supports versions 0-15)
    // Bits 27-24: Message type (4 bits, 16 different message types)
    // Bits 23-22: Priority (2 bits: Low=0, Normal=1, High=2, Critical=3)
    // Bits 21-20: Encryption type (2 bits: None=0, AES=1, Custom=2, Reserved=3)
    // Bits 19-16: Source robot ID (4 bits, supports 16 robots in local network)
    // Bits 15-12: Destination robot ID (4 bits, 0 = broadcast)
    // Bits 11-8:  Sequence number (4 bits, for message ordering)
    // Bits 7-4:   Flags (4 bits: ACK_REQ, COMPRESSED, FRAGMENTED, EMERGENCY)
    // Bits 3-0:   Header checksum (4 bits, simple error detection)

    const VERSION_MASK: u32 = 0xF0000000;
    const VERSION_SHIFT: u8 = 28;

    const MSG_TYPE_MASK: u32 = 0x0F000000;
    const MSG_TYPE_SHIFT: u8 = 24;

    const PRIORITY_MASK: u32 = 0x00C00000;
    const PRIORITY_SHIFT: u8 = 22;

    const ENCRYPTION_MASK: u32 = 0x00300000;
    const ENCRYPTION_SHIFT: u8 = 20;

    const SOURCE_MASK: u32 = 0x000F0000;
    const SOURCE_SHIFT: u8 = 16;

    const DEST_MASK: u32 = 0x0000F000;
    const DEST_SHIFT: u8 = 12;

    const SEQ_MASK: u32 = 0x00000F00;
    const SEQ_SHIFT: u8 = 8;

    const FLAGS_MASK: u32 = 0x000000F0;
    const FLAGS_SHIFT: u8 = 4;

    const CHECKSUM_MASK: u32 = 0x0000000F;
    const CHECKSUM_SHIFT: u8 = 0;

    // Flag bits
    pub const FLAG_ACK_REQUIRED: u8 = 1 << 0;
    pub const FLAG_COMPRESSED: u8 = 1 << 1;
    pub const FLAG_FRAGMENTED: u8 = 1 << 2;
    pub const FLAG_EMERGENCY: u8 = 1 << 3;

    /// Create new protocol header
    pub fn new() -> Self {
        Self { data: 0 }
    }

    /// Create from raw data
    pub fn from_raw(data: u32) -> Self {
        Self { data }
    }

    /// Get raw header data
    pub fn raw(&self) -> u32 {
        self.data
    }

    /// Extract field from header
    fn extract_field(&self, mask: u32, shift: u8) -> u8 {
        ((self.data & mask) >> shift) as u8
    }

    /// Set field in header
    fn set_field(&mut self, mask: u32, shift: u8, value: u8) {
        self.data = (self.data & !mask) | (((value as u32) << shift) & mask);
    }

    /// Set protocol version (0-15)
    pub fn set_version(&mut self, version: u8) {
        self.set_field(Self::VERSION_MASK, Self::VERSION_SHIFT, version & 0xF);
    }

    /// Get protocol version
    pub fn version(&self) -> u8 {
        self.extract_field(Self::VERSION_MASK, Self::VERSION_SHIFT)
    }

    /// Set message type
    pub fn set_message_type(&mut self, msg_type: MessageType) {
        self.set_field(Self::MSG_TYPE_MASK, Self::MSG_TYPE_SHIFT, msg_type as u8);
    }

    /// Get message type
    pub fn message_type(&self) -> MessageType {
        MessageType::from_u8(self.extract_field(Self::MSG_TYPE_MASK, Self::MSG_TYPE_SHIFT))
    }

    /// Set priority
    pub fn set_priority(&mut self, priority: Priority) {
        self.set_field(Self::PRIORITY_MASK, Self::PRIORITY_SHIFT, priority as u8);
    }

    /// Get priority
    pub fn priority(&self) -> Priority {
        Priority::from_u8(self.extract_field(Self::PRIORITY_MASK, Self::PRIORITY_SHIFT))
    }

    /// Set encryption type
    pub fn set_encryption(&mut self, encryption: EncryptionType) {
        self.set_field(Self::ENCRYPTION_MASK, Self::ENCRYPTION_SHIFT, encryption as u8);
    }

    /// Get encryption type
    pub fn encryption(&self) -> EncryptionType {
        EncryptionType::from_u8(self.extract_field(Self::ENCRYPTION_MASK, Self::ENCRYPTION_SHIFT))
    }

    /// Set source robot ID (0-15)
    pub fn set_source(&mut self, source: u8) {
        self.set_field(Self::SOURCE_MASK, Self::SOURCE_SHIFT, source & 0xF);
    }

    /// Get source robot ID
    pub fn source(&self) -> u8 {
        self.extract_field(Self::SOURCE_MASK, Self::SOURCE_SHIFT)
    }

    /// Set destination robot ID (0-15, 0 = broadcast)
    pub fn set_destination(&mut self, dest: u8) {
        self.set_field(Self::DEST_MASK, Self::DEST_SHIFT, dest & 0xF);
    }

    /// Get destination robot ID
    pub fn destination(&self) -> u8 {
        self.extract_field(Self::DEST_MASK, Self::DEST_SHIFT)
    }

    /// Set sequence number (0-15)
    pub fn set_sequence(&mut self, seq: u8) {
        self.set_field(Self::SEQ_MASK, Self::SEQ_SHIFT, seq & 0xF);
    }

    /// Get sequence number
    pub fn sequence(&self) -> u8 {
        self.extract_field(Self::SEQ_MASK, Self::SEQ_SHIFT)
    }

    /// Set flags
    pub fn set_flags(&mut self, flags: u8) {
        self.set_field(Self::FLAGS_MASK, Self::FLAGS_SHIFT, flags & 0xF);
    }

    /// Get flags
    pub fn flags(&self) -> u8 {
        self.extract_field(Self::FLAGS_MASK, Self::FLAGS_SHIFT)
    }

    /// Set specific flag
    pub fn set_flag(&mut self, flag: u8, enabled: bool) {
        let mut flags = self.flags();
        if enabled {
            flags |= flag;
        } else {
            flags &= !flag;
        }
        self.set_flags(flags);
    }

    /// Check if flag is set
    pub fn has_flag(&self, flag: u8) -> bool {
        (self.flags() & flag) != 0
    }

    /// Calculate and set header checksum
    pub fn set_checksum(&mut self) {
        // Simple checksum: XOR of all other fields
        let temp_data = self.data & !Self::CHECKSUM_MASK;
        let checksum = ((temp_data >> 28) ^ (temp_data >> 24) ^
                       (temp_data >> 20) ^ (temp_data >> 16) ^
                       (temp_data >> 12) ^ (temp_data >> 8) ^
                       (temp_data >> 4)) & 0xF;
        self.set_field(Self::CHECKSUM_MASK, Self::CHECKSUM_SHIFT, checksum as u8);
    }

    /// Verify header checksum
    pub fn verify_checksum(&self) -> bool {
        let stored_checksum = self.extract_field(Self::CHECKSUM_MASK, Self::CHECKSUM_SHIFT);
        let temp_data = self.data & !Self::CHECKSUM_MASK;
        let calculated_checksum = ((temp_data >> 28) ^ (temp_data >> 24) ^
                                  (temp_data >> 20) ^ (temp_data >> 16) ^
                                  (temp_data >> 12) ^ (temp_data >> 8) ^
                                  (temp_data >> 4)) & 0xF;
        stored_checksum == (calculated_checksum as u8)
    }

    /// Check if this is a broadcast message
    pub fn is_broadcast(&self) -> bool {
        self.destination() == 0
    }

    /// Check if acknowledgment is required
    pub fn requires_ack(&self) -> bool {
        self.has_flag(Self::FLAG_ACK_REQUIRED)
    }
}

impl fmt::Display for ProtocolHeader {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Header[V{} {:?}->{} {:?} P:{:?} E:{:?} S:{} F:0x{:X}]",
               self.version(), self.source(), self.destination(),
               self.message_type(), self.priority(), self.encryption(),
               self.sequence(), self.flags())
    }
}

/// Message types in the robot protocol
#[derive(Debug, Clone, Copy, PartialEq)]
#[repr(u8)]
pub enum MessageType {
    Heartbeat = 0,
    Status = 1,
    Command = 2,
    Data = 3,
    Emergency = 4,
    Discovery = 5,
    Configuration = 6,
    Diagnostic = 7,
    FileTransfer = 8,
    Acknowledgment = 9,
    Error = 10,
    Reserved1 = 11,
    Reserved2 = 12,
    Reserved3 = 13,
    Reserved4 = 14,
    Custom = 15,
}

impl MessageType {
    fn from_u8(value: u8) -> Self {
        match value {
            0 => Self::Heartbeat,
            1 => Self::Status,
            2 => Self::Command,
            3 => Self::Data,
            4 => Self::Emergency,
            5 => Self::Discovery,
            6 => Self::Configuration,
            7 => Self::Diagnostic,
            8 => Self::FileTransfer,
            9 => Self::Acknowledgment,
            10 => Self::Error,
            11 => Self::Reserved1,
            12 => Self::Reserved2,
            13 => Self::Reserved3,
            14 => Self::Reserved4,
            _ => Self::Custom,
        }
    }
}

/// Priority levels for messages
#[derive(Debug, Clone, Copy, PartialEq)]
#[repr(u8)]
pub enum Priority {
    Low = 0,
    Normal = 1,
    High = 2,
    Critical = 3,
}

impl Priority {
    fn from_u8(value: u8) -> Self {
        match value {
            0 => Self::Low,
            1 => Self::Normal,
            2 => Self::High,
            _ => Self::Critical,
        }
    }
}

/// Encryption types supported
#[derive(Debug, Clone, Copy, PartialEq)]
#[repr(u8)]
pub enum EncryptionType {
    None = 0,
    AES = 1,
    Custom = 2,
    Reserved = 3,
}

impl EncryptionType {
    fn from_u8(value: u8) -> Self {
        match value {
            0 => Self::None,
            1 => Self::AES,
            2 => Self::Custom,
            _ => Self::Reserved,
        }
    }
}

/// Complete robot message with header and payload
#[derive(Debug, Clone)]
pub struct RobotMessage {
    pub header: ProtocolHeader,
    pub payload: Vec<u8>,
}

impl RobotMessage {
    /// Create new message
    pub fn new(message_type: MessageType, source: u8, destination: u8) -> Self {
        let mut header = ProtocolHeader::new();
        header.set_version(1);
        header.set_message_type(message_type);
        header.set_source(source);
        header.set_destination(destination);
        header.set_priority(Priority::Normal);
        header.set_encryption(EncryptionType::None);

        Self {
            header,
            payload: Vec::new(),
        }
    }

    /// Set payload data
    pub fn set_payload(&mut self, data: Vec<u8>) {
        self.payload = data;
    }

    /// Add data to payload
    pub fn append_payload(&mut self, data: &[u8]) {
        self.payload.extend_from_slice(data);
    }

    /// Serialize message to bytes for transmission
    pub fn serialize(&mut self) -> Vec<u8> {
        // Update header with current state
        self.header.set_checksum();

        let mut bytes = Vec::new();
        bytes.extend_from_slice(&self.header.raw().to_be_bytes());
        bytes.extend_from_slice(&self.payload);
        bytes
    }

    /// Deserialize message from bytes
    pub fn deserialize(bytes: &[u8]) -> Result<Self, String> {
        if bytes.len() < 4 {
            return Err("Message too short".to_string());
        }

        let header_bytes = [bytes[0], bytes[1], bytes[2], bytes[3]];
        let header_data = u32::from_be_bytes(header_bytes);
        let header = ProtocolHeader::from_raw(header_data);

        if !header.verify_checksum() {
            return Err("Header checksum verification failed".to_string());
        }

        let payload = if bytes.len() > 4 {
            bytes[4..].to_vec()
        } else {
            Vec::new()
        };

        Ok(Self { header, payload })
    }

    /// Get total message size
    pub fn size(&self) -> usize {
        4 + self.payload.len() // Header is always 4 bytes
    }

    /// Check if message is valid
    pub fn is_valid(&self) -> bool {
        self.header.verify_checksum()
    }
}

/// Robot communication protocol manager
pub struct ProtocolManager {
    robot_id: u8,
    sequence_counter: u8,
    pending_acks: HashMap<u8, RobotMessage>, // Sequence -> Message
}

impl ProtocolManager {
    /// Create new protocol manager
    pub fn new(robot_id: u8) -> Self {
        Self {
            robot_id,
            sequence_counter: 0,
            pending_acks: HashMap::new(),
        }
    }

    /// Create a new outgoing message
    pub fn create_message(&mut self, message_type: MessageType, destination: u8) -> RobotMessage {
        let mut message = RobotMessage::new(message_type, self.robot_id, destination);
        message.header.set_sequence(self.sequence_counter);
        self.sequence_counter = (self.sequence_counter + 1) & 0xF; // Wrap at 16
        message
    }

    /// Send message (in real implementation, this would transmit over network)
    pub fn send_message(&mut self, mut message: RobotMessage) -> Vec<u8> {
        if message.header.requires_ack() {
            self.pending_acks.insert(message.header.sequence(), message.clone());
        }
        message.serialize()
    }

    /// Process received message
    pub fn receive_message(&mut self, bytes: &[u8]) -> Result<RobotMessage, String> {
        let message = RobotMessage::deserialize(bytes)?;

        // Check if message is for us or broadcast
        if message.header.destination() != self.robot_id && message.header.destination() != 0 {
            return Err("Message not for this robot".to_string());
        }

        // Handle acknowledgment messages
        if message.header.message_type() == MessageType::Acknowledgment {
            self.pending_acks.remove(&message.header.sequence());
        }

        // Send acknowledgment if required
        if message.header.requires_ack() {
            let _ack = self.create_acknowledgment(&message);
            // In real implementation, would send this ACK
        }

        Ok(message)
    }

    /// Create acknowledgment message
    fn create_acknowledgment(&mut self, original: &RobotMessage) -> RobotMessage {
        let mut ack = self.create_message(MessageType::Acknowledgment, original.header.source());
        ack.set_payload(vec![original.header.sequence()]); // ACK contains original sequence
        ack
    }

    /// Get pending acknowledgments
    pub fn pending_acknowledgments(&self) -> Vec<u8> {
        self.pending_acks.keys().copied().collect()
    }

    /// Clean up old pending acknowledgments (timeout handling)
    pub fn cleanup_old_acks(&mut self, max_age_sequences: u8) {
        let current_seq = self.sequence_counter;
        self.pending_acks.retain(|&seq, _| {
            let age = if current_seq >= seq {
                current_seq - seq
            } else {
                (16 + current_seq) - seq
            };
            age <= max_age_sequences
        });
    }
}

/// Utility functions for protocol encoding/decoding
pub struct ProtocolUtils;

impl ProtocolUtils {
    /// Encode a 32-bit value as variable-length bytes (little-endian)
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

    /// Decode variable-length bytes to 32-bit value
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

    /// Create a simple XOR checksum for payload data
    pub fn xor_checksum(data: &[u8]) -> u8 {
        data.iter().fold(0, |acc, &byte| acc ^ byte)
    }

    /// Create CRC-8 checksum for robust error detection
    pub fn crc8_checksum(data: &[u8]) -> u8 {
        const CRC8_POLY: u8 = 0xD5; // CRC-8-Dallas/Maxim polynomial

        let mut crc = 0u8;
        for &byte in data {
            crc ^= byte;
            for _ in 0..8 {
                if (crc & 0x80) != 0 {
                    crc = (crc << 1) ^ CRC8_POLY;
                } else {
                    crc <<= 1;
                }
            }
        }
        crc
    }

    /// Compress data using simple run-length encoding
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

    /// Decompress run-length encoded data
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
}

/// Demonstrate binary protocol operations
pub fn demonstrate_protocol_operations() {
    println!("=== Robot Binary Protocol Demonstration ===");

    // Create protocol managers for two robots
    let mut robot1 = ProtocolManager::new(1);
    let mut robot2 = ProtocolManager::new(2);

    println!("Created robots with IDs 1 and 2");

    // Robot 1 sends a status message to Robot 2
    println!("\n--- Status Message ---");
    let mut status_msg = robot1.create_message(MessageType::Status, 2);
    status_msg.header.set_priority(Priority::Normal);
    status_msg.header.set_flag(ProtocolHeader::FLAG_ACK_REQUIRED, true);

    // Add some status data
    let status_data = vec![0x01, 0x42, 0x3C, 0x75]; // Battery: 66%, Temp: 60°C, Status: 0x75
    status_msg.set_payload(status_data);

    println!("Status message: {}", status_msg.header);

    let transmitted_bytes = robot1.send_message(status_msg);
    println!("Transmitted {} bytes: {:02X?}", transmitted_bytes.len(), transmitted_bytes);

    // Robot 2 receives the message
    match robot2.receive_message(&transmitted_bytes) {
        Ok(received_msg) => {
            println!("Robot 2 received: {}", received_msg.header);
            println!("Payload: {:02X?}", received_msg.payload);
            println!("Message valid: {}", received_msg.is_valid());
        }
        Err(e) => println!("Error receiving message: {}", e),
    }

    // Demonstrate emergency broadcast
    println!("\n--- Emergency Broadcast ---");
    let mut emergency_msg = robot2.create_message(MessageType::Emergency, 0); // Broadcast
    emergency_msg.header.set_priority(Priority::Critical);
    emergency_msg.header.set_flag(ProtocolHeader::FLAG_EMERGENCY, true);

    let emergency_data = b"FIRE_DETECTED_SECTOR_7".to_vec();
    emergency_msg.set_payload(emergency_data);

    println!("Emergency broadcast: {}", emergency_msg.header);
    let emergency_bytes = robot2.send_message(emergency_msg);
    println!("Broadcast {} bytes: {:02X?}", emergency_bytes.len(), emergency_bytes);

    // Demonstrate protocol utilities
    println!("\n--- Protocol Utilities ---");

    // Variable-length integer encoding
    let test_values = [0, 127, 128, 16383, 16384, 2097151];
    for value in test_values {
        let encoded = ProtocolUtils::encode_varint(value);
        match ProtocolUtils::decode_varint(&encoded) {
            Ok((decoded, bytes_read)) => {
                println!("Value {} -> {:02X?} -> {} ({} bytes)",
                         value, encoded, decoded, bytes_read);
            }
            Err(e) => println!("Error decoding {}: {}", value, e),
        }
    }

    // Checksum demonstration
    let test_data = b"Robot sensor data: temp=25C, humidity=60%";
    let xor_sum = ProtocolUtils::xor_checksum(test_data);
    let crc8_sum = ProtocolUtils::crc8_checksum(test_data);
    println!("Test data: {:?}", std::str::from_utf8(test_data).unwrap());
    println!("XOR checksum: 0x{:02X}", xor_sum);
    println!("CRC-8 checksum: 0x{:02X}", crc8_sum);

    // Compression demonstration
    let repetitive_data = vec![0xAA; 10]; // 10 bytes of 0xAA
    let mut test_data = repetitive_data.clone();
    test_data.extend(vec![0xBB; 5]); // 5 bytes of 0xBB
    test_data.extend(vec![0xCC; 8]); // 8 bytes of 0xCC

    println!("\n--- Data Compression ---");
    println!("Original data ({} bytes): {:02X?}", test_data.len(), test_data);

    let compressed = ProtocolUtils::rle_compress(&test_data);
    println!("RLE compressed ({} bytes): {:02X?}", compressed.len(), compressed);

    match ProtocolUtils::rle_decompress(&compressed) {
        Ok(decompressed) => {
            println!("Decompressed ({} bytes): {:02X?}", decompressed.len(), decompressed);
            println!("Compression successful: {}", test_data == decompressed);
        }
        Err(e) => println!("Decompression error: {}", e),
    }

    let compression_ratio = (1.0 - compressed.len() as f32 / test_data.len() as f32) * 100.0;
    println!("Compression ratio: {:.1}%", compression_ratio);
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::learning_tests::test_utils::*;

    #[test]
    fn test_protocol_header_fields() {
        let mut header = ProtocolHeader::new();

        // Test version
        header.set_version(5);
        assert_eq!(header.version(), 5);

        // Test message type
        header.set_message_type(MessageType::Command);
        assert_eq!(header.message_type(), MessageType::Command);

        // Test priority
        header.set_priority(Priority::High);
        assert_eq!(header.priority(), Priority::High);

        // Test source and destination
        header.set_source(7);
        header.set_destination(12);
        assert_eq!(header.source(), 7);
        assert_eq!(header.destination(), 12);

        // Test sequence
        header.set_sequence(9);
        assert_eq!(header.sequence(), 9);

        // Test flags
        header.set_flag(ProtocolHeader::FLAG_ACK_REQUIRED, true);
        header.set_flag(ProtocolHeader::FLAG_COMPRESSED, true);
        assert!(header.has_flag(ProtocolHeader::FLAG_ACK_REQUIRED));
        assert!(header.has_flag(ProtocolHeader::FLAG_COMPRESSED));
        assert!(!header.has_flag(ProtocolHeader::FLAG_EMERGENCY));
    }

    #[test]
    fn test_header_checksum() {
        let mut header = ProtocolHeader::new();
        header.set_version(1);
        header.set_message_type(MessageType::Status);
        header.set_source(3);
        header.set_destination(7);

        // Before setting checksum, verification should fail
        assert!(!header.verify_checksum());

        // After setting checksum, verification should pass
        header.set_checksum();
        assert!(header.verify_checksum());

        // Modifying any field should break checksum
        header.set_source(4);
        assert!(!header.verify_checksum());
    }

    #[test]
    fn test_message_serialization() {
        let mut message = RobotMessage::new(MessageType::Data, 1, 2);
        message.set_payload(vec![0x12, 0x34, 0x56, 0x78]);

        let serialized = message.serialize();
        assert_eq!(serialized.len(), 8); // 4 bytes header + 4 bytes payload

        let deserialized = RobotMessage::deserialize(&serialized).unwrap();
        assert_eq!(deserialized.header.message_type(), MessageType::Data);
        assert_eq!(deserialized.header.source(), 1);
        assert_eq!(deserialized.header.destination(), 2);
        assert_eq!(deserialized.payload, vec![0x12, 0x34, 0x56, 0x78]);
        assert!(deserialized.is_valid());
    }

    #[test]
    fn test_protocol_manager() {
        let mut manager = ProtocolManager::new(5);

        let message = manager.create_message(MessageType::Heartbeat, 3);
        assert_eq!(message.header.source(), 5);
        assert_eq!(message.header.destination(), 3);
        assert_eq!(message.header.sequence(), 0);

        let message2 = manager.create_message(MessageType::Status, 7);
        assert_eq!(message2.header.sequence(), 1);

        // Test sequence wrapping
        for _ in 0..20 {
            manager.create_message(MessageType::Heartbeat, 1);
        }
        assert!(manager.sequence_counter < 16); // Should have wrapped
    }

    #[test]
    fn test_varint_encoding() {
        let test_cases = [
            (0, vec![0x00]),
            (127, vec![0x7F]),
            (128, vec![0x80, 0x01]),
            (16383, vec![0xFF, 0x7F]),
            (16384, vec![0x80, 0x80, 0x01]),
        ];

        for (value, expected) in test_cases {
            let encoded = ProtocolUtils::encode_varint(value);
            assert_eq!(encoded, expected);

            let (decoded, bytes_read) = ProtocolUtils::decode_varint(&encoded).unwrap();
            assert_eq!(decoded, value);
            assert_eq!(bytes_read, expected.len());
        }
    }

    #[test]
    fn test_checksums() {
        let test_data = b"Hello, Robot!";

        let xor_sum = ProtocolUtils::xor_checksum(test_data);
        assert_ne!(xor_sum, 0); // Should not be zero for this data

        let crc8_sum = ProtocolUtils::crc8_checksum(test_data);
        assert_ne!(crc8_sum, 0); // Should not be zero for this data

        // Verify that different data produces different checksums
        let different_data = b"Hello, Human!";
        assert_ne!(ProtocolUtils::xor_checksum(different_data), xor_sum);
        assert_ne!(ProtocolUtils::crc8_checksum(different_data), crc8_sum);
    }

    #[test]
    fn test_rle_compression() {
        let test_data = vec![0xAA, 0xAA, 0xAA, 0xBB, 0xCC, 0xCC];
        let compressed = ProtocolUtils::rle_compress(&test_data);
        let expected = vec![3, 0xAA, 1, 0xBB, 2, 0xCC];
        assert_eq!(compressed, expected);

        let decompressed = ProtocolUtils::rle_decompress(&compressed).unwrap();
        assert_eq!(decompressed, test_data);

        // Test empty data
        let empty_compressed = ProtocolUtils::rle_compress(&[]);
        assert!(empty_compressed.is_empty());
        let empty_decompressed = ProtocolUtils::rle_decompress(&[]).unwrap();
        assert!(empty_decompressed.is_empty());
    }

    #[test]
    fn test_protocol_error_handling() {
        // Test deserialization with invalid data
        let short_data = vec![0x12, 0x34]; // Too short for header
        assert!(RobotMessage::deserialize(&short_data).is_err());

        // Test invalid varint
        let invalid_varint = vec![0x80, 0x80, 0x80, 0x80, 0x80]; // Too long
        assert!(ProtocolUtils::decode_varint(&invalid_varint).is_err());

        // Test invalid RLE data
        let invalid_rle = vec![0x12]; // Odd length
        assert!(ProtocolUtils::rle_decompress(&invalid_rle).is_err());
    }

    #[test]
    fn test_user_code_implementation() {
        let analyzer = create_analyzer().expect("Failed to load user code");

        // Check for binary protocol structures
        assert!(
            analyzer.contains_struct("Protocol") ||
            analyzer.contains_struct("Header") ||
            analyzer.contains_struct("Message") ||
            analyzer.code.contains("protocol"),
            "❌ You should implement protocol-related structures"
        );

        // Check for bit manipulation in protocol
        assert!(
            analyzer.code.contains("<<") || analyzer.code.contains(">>") ||
            analyzer.code.contains("&") || analyzer.code.contains("|"),
            "❌ You should use bitwise operations for protocol field manipulation"
        );

        // Check for serialization/deserialization
        assert!(
            analyzer.contains_function("serialize") ||
            analyzer.contains_function("deserialize") ||
            analyzer.contains_function("encode") ||
            analyzer.contains_function("decode"),
            "❌ You should implement serialization/deserialization functions"
        );

        // Check for message types or enums
        assert!(
            analyzer.code.contains("enum") ||
            analyzer.code.contains("MessageType") ||
            analyzer.code.contains("Command") ||
            analyzer.code.contains("Status"),
            "❌ You should define message types using enums"
        );

        // Check for checksum or validation
        assert!(
            analyzer.contains_function("checksum") ||
            analyzer.contains_function("verify") ||
            analyzer.contains_function("validate") ||
            analyzer.code.contains("crc") ||
            analyzer.code.contains("xor"),
            "❌ You should implement checksum or validation functionality"
        );

        // Check for output demonstration
        assert!(
            analyzer.contains_println(),
            "❌ You should demonstrate binary protocol operations with output"
        );
    }
}

/// Student exercises for practicing binary protocol implementation
pub mod exercises {
    use super::*;

    /// Exercise 1: Design a robot sensor protocol
    pub fn exercise_sensor_protocol() {
        println!("Exercise 1: Robot Sensor Protocol");

        // TODO: Design a binary protocol for sensor data transmission:
        // - 16-bit header: sensor type (4 bits), data format (2 bits),
        //   sequence (4 bits), flags (2 bits), checksum (4 bits)
        // - Variable payload: different formats for different sensors
        // - Implement encoding/decoding with proper error checking

        println!("Design a sensor data protocol:");
        println!("Sensor types: Temperature, Humidity, Pressure, Light, Sound, Motion");
        println!("Data formats: Raw, Calibrated, Compressed, Delta");

        let sensor_readings = [
            ("Temperature", 23.5f32, "Calibrated"),
            ("Humidity", 65.2f32, "Raw"),
            ("Light", 180u8, "Compressed"),
            ("Motion", 1u8, "Delta"),
        ];

        for (sensor, value, format) in sensor_readings {
            println!("Sensor: {}, Value: {:?}, Format: {}", sensor, value, format);
            println!("  Encoded protocol: [YOUR IMPLEMENTATION HERE]");
        }
    }

    /// Exercise 2: Implement a command protocol with acknowledgments
    pub fn exercise_command_protocol() {
        println!("\nExercise 2: Command Protocol with ACK");

        // TODO: Create a command protocol that supports:
        // - Command ID (8 bits), parameters (variable), checksum
        // - Reliable delivery with acknowledgments and retransmission
        // - Sequence numbers for ordering
        // - Timeout handling for failed commands

        println!("Commands to implement:");
        let commands = [
            ("MOVE", vec![10u8, 20, 5]), // x, y, speed
            ("ROTATE", vec![90]), // degrees
            ("SCAN", vec![360, 1]), // degrees, resolution
            ("PICKUP", vec![3]), // item_id
            ("REPORT", vec![]), // no parameters
        ];

        for (cmd, params) in commands {
            println!("Command: {}, Parameters: {:?}", cmd, params);
            println!("  Protocol packet: [YOUR IMPLEMENTATION HERE]");
            println!("  Expected ACK: [YOUR IMPLEMENTATION HERE]");
        }
    }

    /// Exercise 3: Create a file transfer protocol
    pub fn exercise_file_transfer() {
        println!("\nExercise 3: File Transfer Protocol");

        // TODO: Implement a protocol for transferring files between robots:
        // - File header with metadata (name, size, checksum)
        // - Chunked transfer with sequence numbers
        // - Error detection and recovery
        // - Progress tracking and flow control

        println!("File transfer protocol requirements:");
        println!("- Support files up to 64KB");
        println!("- Chunk size: 256 bytes");
        println!("- Include filename, total size, and CRC");
        println!("- Handle missing/corrupted chunks");

        let test_files = [
            ("robot_config.json", 1024),
            ("navigation_map.dat", 8192),
            ("sensor_log.csv", 32768),
        ];

        for (filename, size) in test_files {
            println!("File: {}, Size: {} bytes", filename, size);
            println!("  Chunks needed: [YOUR CALCULATION HERE]");
            println!("  Header packet: [YOUR IMPLEMENTATION HERE]");
        }
    }

    /// Exercise 4: Implement a discovery and pairing protocol
    pub fn exercise_discovery_protocol() {
        println!("\nExercise 4: Discovery and Pairing Protocol");

        // TODO: Create a protocol for robots to discover and pair with each other:
        // - Broadcast discovery messages
        // - Capability advertisement
        // - Secure pairing with challenge-response
        // - Maintain neighbor tables

        println!("Discovery protocol features:");
        println!("- Periodic broadcasts with robot capabilities");
        println!("- Response with available services");
        println!("- Authentication for trusted pairing");
        println!("- Heartbeat to maintain connections");

        let robot_types = [
            ("SecurityBot", vec!["patrol", "surveillance", "alert"]),
            ("WorkerBot", vec!["transport", "assembly", "maintenance"]),
            ("ScoutBot", vec!["exploration", "mapping", "reconnaissance"]),
        ];

        for (robot_type, capabilities) in robot_types {
            println!("Robot: {}, Capabilities: {:?}", robot_type, capabilities);
            println!("  Discovery broadcast: [YOUR IMPLEMENTATION HERE]");
            println!("  Pairing challenge: [YOUR IMPLEMENTATION HERE]");
        }
    }

    /// Exercise 5: Create a mesh network routing protocol
    pub fn exercise_routing_protocol() {
        println!("\nExercise 5: Mesh Network Routing Protocol");

        // TODO: Implement a routing protocol for robot mesh networks:
        // - Routing table maintenance
        // - Path discovery and optimization
        // - Message forwarding with hop limits
        // - Network topology changes handling

        println!("Routing protocol components:");
        println!("- Route advertisement messages");
        println!("- Multi-hop message forwarding");
        println!("- Loop prevention and hop counting");
        println!("- Dynamic route updates");

        // Example network topology
        println!("\nNetwork topology:");
        println!("Robot 1 <-> Robot 2 <-> Robot 3");
        println!("   |                        |");
        println!("Robot 4 <-> Robot 5 <-> Robot 6");

        let routing_scenarios = [
            ("Robot 1 to Robot 6", "Multi-hop route"),
            ("Robot 3 to Robot 4", "Alternative paths"),
            ("Robot 2 offline", "Route recalculation"),
        ];

        for (scenario, description) in routing_scenarios {
            println!("Scenario: {}, Description: {}", scenario, description);
            println!("  Routing solution: [YOUR IMPLEMENTATION HERE]");
        }
    }
}

fn main() {
    demonstrate_protocol_operations();

    println!("\n{}", "=".repeat(50));
    println!("STUDENT EXERCISES");
    println!("{}", "=".repeat(50));

    exercises::exercise_sensor_protocol();
    exercises::exercise_command_protocol();
    exercises::exercise_file_transfer();
    exercises::exercise_discovery_protocol();
    exercises::exercise_routing_protocol();
}