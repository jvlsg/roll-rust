use rand::Rng;
use std::str;
use std::fmt;
use std::num;


pub struct DiceRollResult{
    base_value: i64, //Original value
    mod_value: i64, //Modified Value: Original + inc - dec
    is_success: bool
}

pub enum DiceRollType{
    Sum,
    Pool
    //Sum(Option<DiceRollResult>),
    //Pool(Option< {results: Vec<DiceRollResult>, num_success: i64} >) //store Option<{results: Vec<DiceRollResult>, num_success: i64 }>
}

pub struct DiceRoll{
    dice_type: i64,
    dice_qty: i64,
    inc: i64, //Increment
    dec: i64, //Decrement
    tn: i64,   //Target Number
    roll_type: DiceRollType,
    roll_results: Vec<DiceRollResult>
}

impl DiceRoll {

    pub fn new(dice_qty: i64,
               dice_type: i64,
               inc: i64,
               dec: i64,
               tn: i64,
               roll_type: DiceRollType
    ) -> Self {
        DiceRoll{
            dice_qty,
            dice_type,
            inc,
            dec,
            tn,
            roll_type,
            roll_results : vec![]
        }
    }

    pub fn roll_dice(&mut self) {
        match self.roll_type {
            DiceRollType::Sum => {
                let mut base_value:i64 = 0;
                for _i in 0..self.dice_qty {
                    base_value += rand::thread_rng().gen_range(1, self.dice_type+1);
                }
                let mod_value = base_value + self.inc - self.dec;
                let is_success = if mod_value >= self.tn {true} else {false};
                self.roll_results.push(DiceRollResult{
                    base_value,
                    mod_value,
                    is_success
                });
            }
            DiceRollType::Pool => {
            //TODO: reset roll_results if not empty
                for _i in 0..self.dice_qty {
                    let base_value = rand::thread_rng().gen_range(1, self.dice_type+1);
                    let mod_value = base_value + self.inc - self.dec;
                    let is_success = if mod_value + self.inc - self.dec >= self.tn {
                        true
                    } else {false};
                    self.roll_results.push(DiceRollResult{
                        base_value,
                        mod_value,
                        is_success
                    });
                }
            }
        }
    }
}

//Generate a DiceRoll from a single 'XdY+A-B#Z' string
impl str::FromStr for DiceRoll {
    type Err = num::ParseIntError;
    fn from_str(s: &str) -> Result<Self,Self::Err> {
        let v: Vec<&str> = s.split('d').collect();

        let dice_qty = v[0].parse::<i64>()?;
        let mut inc = 0;
        let mut dec = 0;
        let mut tn = 0;
        let dice_type = 0;
        let roll_type = DiceRollType::Sum;
        Ok(DiceRoll::new(
            dice_qty,
            dice_type,
            inc,
            dec,
            tn,
            roll_type
        ))
    }
}

impl fmt::Display for DiceRoll {
    fn fmt(&self,f: &mut fmt::Formatter<'_>) -> fmt::Result {

//        String modifier_info = if self.inc == 0 && self.dec == 0 { 
//            "".to_string() 
//        } else {
//            format!("+{} -{} = {}",self.inc,self.dec)
//        };
        match self.roll_type {
            DiceRollType::Sum => {
                let result=&self.roll_results[0];
                let mod_str = if self.tn > 0 {format!(" ➔ {}",result.mod_value)} else {"".to_string()};
                if self.tn > 0 {
                   let success_str = if result.mod_value > self.tn {" ✓ "} else {" ✕ "};
                    write!(f,"[{}{}{}]",result.base_value,mod_str,success_str)
                }
                else{
                    write!(f,"[{}{}]",
                           self.roll_results[0].base_value,
                           mod_str,)
                }
            }
            DiceRollType::Pool => {
                let mut x = String::from("");
                for result in &self.roll_results{
                    x = format!("[{}] {}" ,result.mod_value,x);
                }
                write!(f,"{}",x)
            }
        }
    }
}

// TODO to use in verbose mode impl fmt::Debug for DiceRoll {}
//impl fmt::Debug for DiceRoll {
//    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
//
//    }
//}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn roll_one_simple(){
        let mut x = DiceRoll::new(1,6,0,0,0,DiceRollType::Sum);
        x.roll_dice();
        println!("{}",x);
    }

    #[test]
    fn roll_one_modded(){
        let mut x = DiceRoll::new(1,6,4,2,11,DiceRollType::Sum);
        x.roll_dice();
        println!("{}",x);
    }


    #[test]
    fn roll_pool(){
        let mut x = DiceRoll::new(3,6,4,2,11,DiceRollType::Pool);
        x.roll_dice();
        println!("{}",x);
    }

    #[test]
    fn roll_from_str(){
        //let x = DiceRoll::from_str("3d6+12-1-5#12");
        let x = "3d6+12-1-5#10".parse::<DiceRoll>();
    }
}
