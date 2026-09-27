use std::collections::VecDeque;

use turbo::*;

use crate::UserMessage;

const BUFFER_CUTOFF_TIME_MS: f32 = 5000.0;

// --- Buffered Input ---

#[turbo::serialize]
pub struct BufferedInput {
    pub sequence: u64,
    pub timestamp: f32,
    pub message: UserMessage,
}

// --- Input State ---

#[turbo::serialize]
pub struct InputState {
    pub client_sequence: u64,
    pub last_acknowledged_sequence: u64,
    pub input_buffer: VecDeque<BufferedInput>,
}

impl InputState {
    pub fn new() -> Self {
        Self {
            client_sequence: 0,
            last_acknowledged_sequence: 0,
            input_buffer: VecDeque::new(),
        }
    }
    
    pub fn next_sequence(&mut self) -> u64 {
        self.client_sequence += 1;
        self.client_sequence
    }
    
    pub fn buffer_input(&mut self, message: UserMessage, timestamp: f32) {
        let sequence = match &message {
            UserMessage::PlayerJoin { seq, .. } => *seq,
            UserMessage::Move { seq, .. } => *seq,
            UserMessage::UseItem { seq, .. } => *seq,
            UserMessage::UseAnyItem { seq, .. } => *seq,

            // Entity Recovery
            UserMessage::RequestPlayerLookup { seq, .. } => *seq,
            UserMessage::RequestProjectileLookup { seq, .. } => *seq,
            UserMessage::RequestEffectLookup { seq, .. } => *seq,

            // Debug
            #[cfg(feature = "debug")]
            UserMessage::DebugGetItem { seq, .. } => *seq,
        };
        
        self.input_buffer.push_back(BufferedInput {
            sequence,
            timestamp,
            message,
        });
        
        // Cull Old Inputs
        let cutoff_time = timestamp - BUFFER_CUTOFF_TIME_MS;
        while let Some(front) = self.input_buffer.front() {
            if front.timestamp < cutoff_time {
                self.input_buffer.pop_front();
            } else {
                break;
            }
        }
    }
    
    pub fn acknowledge_sequence(&mut self, server_ack: u64) {
        if server_ack > self.last_acknowledged_sequence {
            self.last_acknowledged_sequence = server_ack;

            // Cull Acknowledged Inputs
            while let Some(front) = self.input_buffer.front() {
                if front.sequence <= server_ack {
                    self.input_buffer.pop_front();
                } else {
                    break;
                }
            }
        }
    }
    
    pub fn get_inputs_since(&self, sequence: u64) -> Vec<&BufferedInput> {
        self.input_buffer
            .iter()
            .filter(|input| input.sequence > sequence)
            .collect()
    }
}