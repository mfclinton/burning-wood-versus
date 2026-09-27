use turbo::*;

pub trait ClientControlled {
    fn get_last_processed_sequence(&self) -> u64;
    fn set_last_processed_sequence(&mut self, seq: u64);
    
    fn should_process_sequence(&self, seq: u64) -> bool {
        seq > self.get_last_processed_sequence()
    }
}
