#[derive(Debug, PartialEq, Eq)]
pub struct Clock{
    hours:i32,
    minutes:i32,
}

impl Clock {

  pub fn new(hours: i32, minutes: i32) -> Self {
        let total_minutes = hours * 60 + minutes;
        let minutes_in_day = 24 * 60;              
        let normalized_minutes = ((total_minutes % minutes_in_day) + minutes_in_day) % minutes_in_day;
        Clock {
            hours: normalized_minutes / 60,
            minutes: normalized_minutes % 60,
        }
    }

    pub fn add_minutes(&self, minutes: i32) -> Self {
        Clock::new(
            self.hours,
            self.minutes+minutes
        )
    }

    pub fn to_string(&self)-> String{
format!("{:02}:{:02}", self.hours, self.minutes)
    }
}
