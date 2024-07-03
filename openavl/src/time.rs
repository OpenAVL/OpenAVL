pub struct Time {
    string_time: String,
    int_time_sec: Int
}

impl Time {
    pub fn set_time(&mut self, string_time: &str){
        self.string_time = string_time;
        let separated_time = string_time.split(":");
        let hour  = separated_time[0] as u8;
        let minutes = separated_time[1] as u8;
        let seconds = separated_time[2] as u8;

        self.int_time_sec = (hour * 24 * 60) + (minutes * 60) + (seconds)
    }
}