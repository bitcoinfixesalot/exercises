#[derive(Debug, PartialEq)]
pub enum Error {
    NotEnoughPinsLeft,
    GameComplete,
}

pub struct BowlingGame {
    frame_number: u16,
    rolls: Vec<u16>,
    frame_rolls: u16,
    standing_pins: u16,
    last_frame_bonus: bool,
}

impl BowlingGame {
    const NUMBER_OF_FRAMES: u16 = 10;
    const NUMBER_OF_PINS: u16 = 10;

    pub fn new() -> Self {
        Self {
            frame_number: 1,
            rolls: Vec::new(),
            frame_rolls: 0,
            standing_pins: Self::NUMBER_OF_PINS,
            last_frame_bonus: false,
        }
    }

    pub fn roll(&mut self, pins: u16) -> Result<(), Error> {
        if self.standing_pins < pins {
            Err(Error::NotEnoughPinsLeft)
        } else if self.frame_number < Self::NUMBER_OF_FRAMES {
            if self.frame_rolls == 0 && pins == Self::NUMBER_OF_PINS {
                //strike
                self.frame_rolls = 2;
                self.standing_pins = Self::NUMBER_OF_PINS; //reset pins
            } else {
                self.standing_pins -= pins;
                self.frame_rolls += 1;
            }

            self.rolls.push(pins);
            if self.frame_rolls == 2 {
                //last frame completed
                self.frame_number += 1; //next frame
                self.frame_rolls = 0; //reset frame rolls
                self.standing_pins = Self::NUMBER_OF_PINS; //reset pins
            }
            Ok(())
        } else {
            //Last frame
            if self.frame_rolls < 2 {
                self.standing_pins -= pins;
                self.frame_rolls += 1;
                if self.standing_pins == 0 {
                    //strike or spare
                    self.last_frame_bonus = true;
                    self.standing_pins = Self::NUMBER_OF_PINS;
                }
                self.rolls.push(pins);
                Ok(())
            } else if self.frame_rolls == 2 && self.last_frame_bonus {
                //fill roll
                self.last_frame_bonus = false;
                self.rolls.push(pins);
                Ok(())
            } else {
                Err(Error::GameComplete)
            }
        }
    }

    pub fn score(&self) -> Option<u16> {
        if self.frame_number != Self::NUMBER_OF_FRAMES
            || self.last_frame_bonus
            || self.frame_rolls < 2
        {
            None
        } else {
            let mut score = 0;
            let mut index = 0;
            while index + 1 < self.rolls.len() {
                if self.rolls[index] + self.rolls[index + 1] < 10 {
                    score += self.rolls[index] + self.rolls[index + 1];
                    index += 2; //next frame
                    continue;
                }

                if index + 2 >= self.rolls.len() {
                    break;
                }

                score += self.rolls[index] + self.rolls[index + 1] + self.rolls[index + 2];
                if self.rolls[index] == 10 && self.rolls.len() > index + 3 {
                    //in case of strike but not the last, advance only by one
                    index += 1;
                } else {
                    index += 2;
                }
            }
            Some(score)
        }
    }
}
