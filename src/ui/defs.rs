use std::default::Default;
use std::fmt::Debug;
use std::str::FromStr;

// Define Trear of Parsible

#[allow(dead_code)]
pub trait ParseFilter: FromStr + Default {
    fn filter(c: char) -> bool;
}

impl ParseFilter for u32 {
    fn filter(c: char) -> bool {
        c.to_ascii_lowercase().is_ascii_digit()
    }
}

impl ParseFilter for u64 {
    fn filter(c: char) -> bool {
        c.to_ascii_lowercase().is_ascii_digit()
    }
}

impl ParseFilter for i32 {
    fn filter(c: char) -> bool {
        c.to_ascii_lowercase().is_ascii_digit() || c == '-'
    }
}

impl ParseFilter for i64 {
    fn filter(c: char) -> bool {
        c.to_ascii_lowercase().is_ascii_digit() || c == '-'
    }
}

impl ParseFilter for f32 {
    fn filter(c: char) -> bool {
        c.to_ascii_lowercase().is_ascii_digit() || c == '-' || c == '.'
    }
}

impl ParseFilter for f64 {
    fn filter(c: char) -> bool {
        c.to_ascii_lowercase().is_ascii_digit() || c == '-' || c == '.'
    }
}

impl ParseFilter for bool {
    fn filter(c: char) -> bool {
        c == '0' || c == '1'
    }
    
}

// Define trait for individual Add Setting
#[allow(dead_code)]
pub trait AppSetting<T>: Default + Debug + Clone where
T: ParseFilter + Debug + Clone,
 {
    fn new(value: T) -> Self;
    fn parse(&mut self, str: String);
    fn get_value(&self) -> &T;
    fn get_value_mut(&mut self) -> &mut T;
    fn update_value(&mut self, value: T);
}

// create structure of type Setting that implements AppSetting trait
#[derive(Default, Debug, Clone)]
pub struct Setting<T>
where T: ParseFilter + Default
{
    value: T
}


impl<T> AppSetting<T> for Setting<T>
where T: ParseFilter + Debug + Clone,
{
    fn new(value: T) -> Self{
        Self{
            value
        }
    }

    fn parse(&mut self, str: String) {
        let parsed_value = str.chars()
            .filter(|c: &char| T::filter(*c))
            .collect::<String>()
            .parse::<T>()
            .unwrap_or(T::default());
        
        self.value = parsed_value;
    }

    fn get_value(&self) -> &T {
        &self.value
    }

    fn get_value_mut(&mut self) -> &mut T {
        &mut self.value
    }

    fn update_value(&mut self, value: T) {
        self.value = value;
    }
}


// Define traits for the Collection Off app settings

#[allow(dead_code)]
pub trait AppSettingCollection: Default + Debug + Clone {
    fn is_open(&self) -> &bool;

    fn is_open_mut(&mut self) -> &mut bool;

    fn open(&mut self) {
        *self.is_open_mut() = true;
    }
}


pub trait ProcessError: Default + Debug + Clone {
    fn is_error(&self) -> &bool;

    fn is_error_mut(&mut self) -> &mut bool;

    fn get_message(&self) -> &String;

    fn get_message_mut(&mut self) -> &mut String;

    fn rais(&mut self, msg: String) {
        *self.is_error_mut() = true;
        *self. get_message_mut() = msg;
    }

    fn close(&mut self){
        *self.is_error_mut() = false;
        *self.get_message_mut() = String::new();
    }
}
