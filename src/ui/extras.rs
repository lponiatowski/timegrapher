#[derive(Debug, Clone)]
pub struct ProcessError{
    is_error: bool,
    msg: String
}

impl Default for ProcessError {
    fn default() -> Self {
        Self { is_error: false, msg: String::new() }
    }
    
}

impl ProcessError {
    pub fn rais(&mut self, msg: String) {
        self.is_error=true;
        self.msg = msg;
    }

    pub fn close(&mut self){
        self.is_error=false;
        self.msg=String::new();
    }

    pub fn is_error(&self) -> bool{
        self.is_error
    }

    pub fn is_error_mut(&mut self) -> &mut bool{
        &mut self.is_error
    }

    pub fn message(&self) -> &String {
        &self.msg
    }
}

pub struct Settings {
    is_open: bool,
    pub sample_size: u64,
    pub y_limits: f64, 
}

impl Default for Settings {

    fn default() -> Self {
        Self {
            is_open: false,
            sample_size: 3 as u64,
            y_limits: 1.0.into()
        }
    }
}

impl Settings{
    pub fn open(&mut self) {
        self.is_open = true;
    }

    pub fn is_open(&self) -> bool {
        self.is_open
    }

    pub fn is_open_mut(&mut self) -> &mut bool{
        &mut self.is_open
    }

    pub fn parse_i32(str: String) -> i32 {
        // str.retain(|c| c.is_digit(10) || c == '-');
        str.chars().filter(|&c| c.is_ascii_digit() || c == '-').collect::<String>().parse::<i32>().unwrap_or(0)
    }

    pub fn parse_i64(str: String) -> i64 {
        // str.retain(|c| c.is_digit(10) || c == '-');
        str.chars().filter(|&c| c.is_ascii_digit()).collect::<String>().parse::<i64>().unwrap_or(0)
    }

    pub fn parse_u32(str: String) -> u32 {
        // str.retain(|c| c.is_digit(10) || c == '-');
        str.chars().filter(|&c| c.is_ascii_digit()).collect::<String>().parse::<u32>().unwrap_or(0)
    }

    pub fn parse_u64(str: String) -> u64 {
        // str.retain(|c| c.is_digit(10) || c == '-');
        str.chars().filter(|&c| c.is_ascii_digit()).collect::<String>().parse::<u64>().unwrap_or(0)
    }

    pub fn parse_f32(str: String) -> f32 {
        // str.retain(|c| c.is_digit(10) || c == '-');
        str.chars().filter(|&c| c.is_ascii_digit() || c == '-'  || c == '.').collect::<String>().parse::<f32>().unwrap_or(0.0)
    }

    pub fn parse_f64(str: String) -> f64 {
        // str.retain(|c| c.is_digit(10) || c == '-');
        str.chars().filter(|&c| c.is_ascii_digit() || c == '.' || c == '-').collect::<String>().parse::<f64>().unwrap_or(0.0)
        // str.parse::<f64>().unwrap_or(0.0)
    }


}