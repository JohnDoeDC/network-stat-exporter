use std::fs;
pub struct Collector{
    pub device_name : String
}

pub enum DataType{
    Rx,
    Tx
}

impl Collector{
    pub fn new(device_name : String) -> Self{
        Self{
            device_name
        }
    }

    fn get_path(&self) -> String{
        format!("/sys/class/net/{}/statistics", &self.device_name)
    }

    fn get_data(path : &String, data_type : DataType) -> Option<u64>{ // I hope u64::MAX = 18446744073709551615 would be enough and we dont need i64 since stat I HOPE cant go negative
        let endpoint = match data_type{
            DataType::Rx => "rx_bytes".to_string(),
            DataType::Tx => "tx_bytes".to_string()
        };
        let path = format!("{}/{}",path,endpoint);
        if let Ok(data) = fs::read_to_string(path){
            let number = data.chars().filter(|char| char.is_digit(10)).collect::<String>();
            if let Ok(number) = number.parse::<u64>(){
                return Some(number)
            }
        }
        None
    }
    pub fn get_rx(&self) -> Option<u64>{ 
        Self::get_data(&self.get_path(), DataType::Rx)
    }

    pub fn get_tx(&self) -> Option<u64>{
        Self::get_data(&self.get_path(), DataType::Tx)
    }
}