use w4on_sys::*;

#[derive(Debug)]
pub struct W4on {
    seq: w4on_seq_t,
}
unsafe impl Send for W4on {}
unsafe impl Sync for W4on {}

impl W4on {
    pub fn new(data: &[u8]) -> Self {
        let mut seq = w4on_seq_t {
            data: std::ptr::null(),
            tracks: [w4on_track_t {
                channel: 0,
                volume: 0,
                pulseMode: 0,
                a: 0,
                d: 0,
                r: 0,
                s: 0,
                pan: 0,
                arpSpeed: 0,
                segmentsLeft: 0,
                eventValue: 0,
                dataI: 0,
                dataEndI: 0,
                arpNoteDataI: 0,
                eventTick: 0,
                eventTicksLeft: 0,
            }; 8],
        };
        unsafe {
            w4on_seq_init(&mut seq, data.as_ptr());
        }
        Self { seq }
    }
    pub fn tick(&mut self) {
        unsafe {
            w4on_seq_tick(&mut self.seq);
        }
    }
}
