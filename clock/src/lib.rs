
pub struct Clock {
    hours: i32,
    minutes: i32
}

impl Clock {
    pub fn new(hours: i32, minutes: i32) -> Self {
        let mut remainder = hours % 24;
        let mut bonusHours = 0;
        let mut actualMinutes = 0;
        if minutes > 60 {
            bonusHours = minutes/60;
            actualMinutes = minutes % 60

        }
        remainder += bonusHours;
        Self { hours: remainder, minutes: actualMinutes }
    }

    pub fn to_string(&self) -> String {
        if self.minutes < 10  && self.hours < 10 {
            return format!(
                "0{}:0{}",
                &self.hours, &self.minutes
            )
        } else if self.minutes < 10 {
            return format!(
                "{}:0{}",
                &self.hours, &self.minutes
            )
        } else if self.hours < 10 {
            return format!(
                "0{}:{}",
                &self.hours, &self.minutes
            )
        }
        return format!(
            "{}:{}",
            &self.hours, &self.minutes
        )
    }

    pub fn add_minutes(&mut self, minutes: i32) {
        self.minutes += minutes;
    }


    
}
