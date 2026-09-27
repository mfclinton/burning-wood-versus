use turbo::*;

use crate::{ServerEvent, borsh::BorshSerialize};

const TARGET_BATCH_SIZE_BYTES: usize = 1200;

#[inline]
fn serialized_len<T: BorshSerialize>(x: &T) -> usize {
    borsh::to_vec(x).map(|v| v.len()).unwrap_or(usize::MAX / 2)
}

pub fn create_batches(events: Vec<ServerEvent>) -> Vec<ServerEvent> {
    if events.len() <= 1 {
        return events;
    }

    let mut batches = Vec::new();

    let mut batch_size = 0usize;
    let mut batch = Vec::new();
    for event in events {
        let size = serialized_len(&event);

        // Finalize Batch
        if !batch.is_empty() && batch_size + size > TARGET_BATCH_SIZE_BYTES {
            batches.push(ServerEvent::EventBatch { events: std::mem::take(&mut batch) });
            batch_size = 0;
        }

        // Add Event to Batch
        batch_size += size;
        batch.push(event);
    }

    // Finalize Last Batch
    if !batch.is_empty() {
        batches.push(ServerEvent::EventBatch { events: batch });
    }

    batches
}
