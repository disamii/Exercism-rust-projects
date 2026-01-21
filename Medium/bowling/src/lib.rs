#[derive(Debug, PartialEq, Eq)]
pub enum Error {
    NotEnoughPinsLeft,
    GameComplete,
}

pub struct BowlingGame {
    rolls: Vec<u16>,
}

impl BowlingGame {
    pub fn new() -> Self {
        BowlingGame { rolls: Vec::new() }
    }

    pub fn roll(&mut self, pins: u16) -> Result<(), Error> {
        if pins > 10 {
            return Err(Error::NotEnoughPinsLeft);
        }

        if self.is_complete() {
            return Err(Error::GameComplete);
        }

        let mut rolls = self.rolls.iter().copied();
        let mut frame = 0;

        // Frames 1–9
        while frame < 9 {
            match rolls.next() {
                Some(10) => frame += 1,
                Some(first) => match rolls.next() {
                    Some(second) => {
                        if first + second > 10 {
                            return Err(Error::NotEnoughPinsLeft);
                        }
                        frame += 1;
                    }
                    None => {
                        if first + pins > 10 {
                            return Err(Error::NotEnoughPinsLeft);
                        }
                        break;
                    }
                },
                None => break,
            }
        }

        if frame == 9 {
            let first = rolls.next();
            let second = rolls.next();

            match (first, second) {
                (None, _) => {} // first roll always ok
                (Some(f), None) => {
                    if f < 10 && f + pins > 10 {
                        return Err(Error::NotEnoughPinsLeft);
                    }
                }
                (Some(f), Some(s)) => {
                    if f == 10 {
                        // strike → bonus rolls
                        if s != 10 && s + pins > 10 {
                            return Err(Error::NotEnoughPinsLeft);
                        }
                    } else if f + s == 10 {
                        // spare → one bonus roll allowed
                    } else {
                        // open frame → no third roll
                        return Err(Error::GameComplete);
                    }
                }
            }
        }

        self.rolls.push(pins);
        Ok(())
    }

    pub fn score(&self) -> Option<u16> {
        let mut score: u16 = 0;
        let mut roll_index = 0;
        let mut frames = 0;

        while frames < 10 {
            let first = *self.rolls.get(roll_index)?;
            let second = *self.rolls.get(roll_index + 1).unwrap_or(&0);

            if first == 10 {
                // Strike
                let bonus1 = *self.rolls.get(roll_index + 1)?;
                let bonus2 = *self.rolls.get(roll_index + 2)?;
                score += 10 + bonus1 + bonus2;
                roll_index += 1;
            } else if first + second == 10 {
                // Spare
                let bonus = *self.rolls.get(roll_index + 2)?;
                score += 10 + bonus;
                roll_index += 2;
            } else {
                // Open frame
                score += first + second;
                roll_index += 2;
            }

            frames += 1;
        }

        Some(score)
    }

    pub fn is_complete(&self) -> bool {
        let mut rolls = self.rolls.iter();
        let mut frame = 0;

        while frame < 9 {
            match rolls.next() {
                Some(&10) => frame += 1,
                Some(&_first) => {
                    rolls.next();
                    frame += 1;
                }
                None => return false,
            }
        }

        // 10th frame
        let first = match rolls.next() {
            Some(&v) => v,
            None => return false,
        };
        let second = match rolls.next() {
            Some(&v) => v,
            None => return false,
        };

        if first == 10 || first + second == 10 {
            // Need 3rd roll
            rolls.next().is_some()
        } else {
            true
        }
    }


}
