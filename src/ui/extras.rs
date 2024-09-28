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