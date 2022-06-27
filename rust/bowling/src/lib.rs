#[derive(Debug, PartialEq)]
pub enum Error {
    NotEnoughPinsLeft,
    GameComplete,
}

pub struct BowlingGame {
    rolls: Vec<u16>,
    frames: Vec<BowlingFrames>,
}
enum FrameResult {
    Strike,
    Spare,
    Open
}

struct WorkingFrame {

}


impl WorkingFrame {
    fn from(roll: u16) -> Self {

    }
}

impl BowlingGame {
    pub fn new() -> Self {
        todo!()
    }

    pub fn roll(&mut self, pins: u16) -> Result<(), Error> {
        self.rolls.push(pins);
        self._reconstruct_game_frames();
        // let current_frame = self.frames.len() + 1;
        // if pins == 10 {
        //     self.frames.push(BowlingFrame::strike())
        // } else {
        //     self.working_frame.rolls.push(pins);
        //     if self.working_frame.is_done(current_frame)
        // }
        return Result::Ok(());
    }


    fn _reconstruct_game_frames(&mut self) {
        let mut completed_frames = Vec::new();
        let working_frame = WorkingFrame::new();
        for roll in self.rolls {
            working_frame.pins -= roll;
            if working_frame.pins <= 0 {
                completed_frames.push(working_frame);
            }
        }
    }

    pub fn score(&self) -> Option<u16> {
        let mut running_total = 0;

        unimplemented!("Return the score if the game is complete, or None if not.");
    }
}
