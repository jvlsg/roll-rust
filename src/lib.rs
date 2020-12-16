use rand::Rng;
use std::str;
use std::fmt;
use std::num;
use regex::Regex;


pub enum DiceRollType{
    Sum,
    Pool
}

pub struct DiceRoll{
    dice_qty: i64,
    dice_type: i64,
    inc: i64, //Increment
    dec: i64, //Decrement
    tn: i64,   //Target Number. Defaults to 0
    roll_type: DiceRollType,
    roll_results: Vec<i64>,
    final_result: i64 //
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
            roll_results : vec![],
            final_result : 0 //Either the modified sum or the number of dice pool Successes
        }
    }

    pub fn roll_dice(&mut self) {
        //Reset variables
        self.roll_results.clear();
        self.final_result = 0;

        match self.roll_type {
            DiceRollType::Sum => {
                for _i in 0..self.dice_qty {
                    self.roll_results.push(rand::thread_rng().gen_range(1, self.dice_type+1));
                }
                self.final_result = self.roll_results.iter().sum::<i64>() + self.inc - self.dec;
            }
            DiceRollType::Pool => {
                let mut success_count: i64 = 0;
                for _i in 0..(self.dice_qty + self.inc - self.dec) { //Number of dice is modified
                    let r =rand::thread_rng().gen_range(1, self.dice_type+1);
                    if r  >= self.tn {
                        success_count+=1;
                    }
                    self.roll_results.push(r);
                    self.final_result = success_count;
                }
            }
        }
    }
}

//Generate a DiceRoll from a single 'XdY+A-B#Z' string
impl str::FromStr for DiceRoll {
    type Err = num::ParseIntError;
    fn from_str(s: &str) -> Result<Self,Self::Err> {
        let re = Regex::new(r"^(?P<dice_qty>\d+)[d|D](?P<dice_type>\d+)(?P<mods>(?:\+\d+|-\d+)+)?(?:#(?P<tn>\d+))?$").unwrap();
        for cap in re.captures_iter(s) {
            println!("{:?}",&cap);
        }

        let mut inc = 0;
        let mut dec = 0;
        let mut tn = 0;
        let dice_type = 0;
        let roll_type = DiceRollType::Sum;
        Ok(DiceRoll::new(
            1,
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
        match self.roll_type {
            DiceRollType::Sum => {
                let base_result:i64 = self.roll_results.iter().sum();
                let mod_str = if self.tn > 0 {format!(" ➔ {}",self.final_result)} else {"".to_string()};
                if self.tn > 0 {
                   let success_str = if self.final_result > self.tn {" ✓ "} else {" ✕ "};
                    write!(f,"[{}{}{}]",base_result,mod_str,success_str)
                }
                else{
                    write!(f,"[{}{}]",
                           base_result,
                           mod_str,)
                }
            }
            DiceRollType::Pool => {
                let mut results_str = String::from("");
                let success_str = if self.tn>0 {
                    let word = if self.final_result == 1 { "Success" } else { "Successes" };
                    format!("{} {}",self.final_result,word)
                } else {
                    "".to_string()
                };
                
                for die_result in &self.roll_results{
                    results_str = format!("{} {}" ,die_result,results_str);
                }
                write!(f,"[{}| {}]",results_str,success_str)
            }
        }
    }
}

// TODO to use in verbose mode impl fmt::Debug for DiceRoll {}
//impl fmt::Debug for DiceRoll {
//    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
//        String modifier_info = if self.inc == 0 && self.dec == 0 { 
//            "".to_string() 
//        } else {
//            format!("+{} -{} = {}",self.inc,self.dec)
//        };
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
        let mut x = DiceRoll::new(3,6,4,2,4,DiceRollType::Pool);
        x.roll_dice();
        println!("{}",x);
    }

    #[test]
    fn re_play(){
        let re = Regex::new(r"^.*(a).*$").unwrap();
        for cap in re.captures_iter("daad") {
            println!("{:?}",&cap);
        }
    }

    #[test]
    fn roll_from_str(){
        let x = "3D6-3-3+1#5".parse::<DiceRoll>();
        let x = "31d632+12-3+1#10".parse::<DiceRoll>();
        let y = "15d16".parse::<DiceRoll>();
        let z = "1d3#2".parse::<DiceRoll>();
    }
}
