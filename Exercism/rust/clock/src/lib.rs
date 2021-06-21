use std::fmt;

#[derive(Debug)]
#[derive(PartialEq)]
pub struct Clock{
    hours: i32,
    minutes: i32,
}

impl Clock {
    pub fn new(hours: i32, minutes: i32) -> Self {
        Clock{
            hours: (hours.rem_euclid(24)+(minutes as f64 /60f64).floor() as i32).rem_euclid(24), 
            minutes: minutes.rem_euclid(60)
        }
    }

    pub fn add_minutes(&self, minutes: i32) -> Self {
        let mut new_hours = minutes/60;
        let new_minutes = self.minutes + (minutes%60);

        if new_minutes < 0{
            new_hours -= 1;
        }

        if new_minutes >= 60{
            new_hours += 1;
        }

        Clock{
            hours: (self.hours + new_hours).rem_euclid(24), 
            minutes: new_minutes.rem_euclid(60)
        }
    }
}

impl fmt::Display for Clock{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:0>2}:{:0>2}", self.hours, self.minutes)
    }
}