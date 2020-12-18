use rand::Rng;
use std::str;
use std::fmt;
use regex::Regex;
#[derive(Debug)]
pub struct DiceRollError{
    message: String,
}

#[derive(Debug)]
pub enum DiceRollType{
    Sum,
    Pool
}
pub struct DiceRoll{
    dice_qty: isize,
    dice_type: isize,
    inc: isize, //Increment
    dec: isize, //Decrement
    tn: isize,   //Target Number. Defaults to 0
    roll_type: DiceRollType,
    roll_results: Vec<isize>,
    final_result: isize //
}

pub fn run(roll_str: &str, default_tn: isize , is_pool: bool, is_verbose: bool) -> Result<(),DiceRollError> {
    let mut d = roll_str.parse::<DiceRoll>()?;
    if is_pool {d.roll_type = DiceRollType::Pool};
    if default_tn > 0 && d.tn == 0 { d.tn = default_tn };
    d.roll_dice();
    if is_verbose {println!("{:#?}",d)} else {println!("{}",d)};
    Ok(())
}

fn parse_modifiers(mods_str: String) -> (isize,isize) {
    let mut inc = 0;
    let mut dec = 0;
    let mut aux_str = "".to_string();

    let char_vec: Vec<char> = mods_str.chars().collect();
    let mut i = 0;
    for j in 0..char_vec.len() {
        if char_vec[j] == '+' || char_vec[j] == '-' || j==char_vec.len()-1 {
            if j==char_vec.len()-1 {
                aux_str.push(char_vec[j]);
            }
            if i<j{
                let aux_int = aux_str.parse::<isize>().unwrap_or_default();
                if aux_int > 0 {inc += aux_int} else {dec += aux_int};
                i=j;
                aux_str.clear();
            }
        }
        aux_str.push(char_vec[j]);
    }
    (inc,dec)
}

impl DiceRoll {
    fn new(dice_qty: isize,
               dice_type: isize,
               inc: isize,
               dec: isize,
               tn: isize,
               roll_type: DiceRollType
    ) -> Self {

        if dice_qty <= 0 {panic!("Invalid dice quantity")};
        if dice_type <= 0 {panic!("Invalid dice type")};
        if inc < 0 {panic!("Invalid increment - must be positive or zero")};
        if dec > 0 {panic!("Invalid decrement - must be negative or zero")};
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
            DiceRollType::Sum => self.roll_sum(),
            DiceRollType::Pool => self.roll_pool()
        }
    }

    fn roll_sum(&mut self) {
        for _i in 0..self.dice_qty {
            self.roll_results.push(rand::thread_rng().gen_range(1, self.dice_type+1));
        }
        self.final_result = self.roll_results.iter().sum::<isize>() + self.inc + self.dec;
    }

    fn roll_pool(&mut self){
        for _i in 0..(self.dice_qty + self.inc + self.dec) { //Number of dice is modified
            let r =rand::thread_rng().gen_range(1, self.dice_type+1);
            if r  >= self.tn {self.final_result+=1;}
            self.roll_results.push(r);
        }
    }
}

impl str::FromStr for DiceRoll {
    type Err = DiceRollError;
    fn from_str(s: &str) -> Result<Self,Self::Err> {
        let re = Regex::new(r"^(?P<dice_qty>\d+)[d|D](?P<dice_type>\d+)(?P<mods>(?:\+\d+|-\d+)+)?(?:#(?P<tn>\d+))?$").unwrap();

        let caps  = re.captures(s).ok_or::<DiceRollError>(DiceRollError{message:"Regex Fail".to_string()})?;

        let dice_qty= match caps.name("dice_qty").unwrap().as_str().parse::<isize>() {
            Ok(i) => i,
            Err(e) =>  return Err(DiceRollError{message:e.to_string()})
        };
        let dice_type= match caps.name("dice_type").unwrap().as_str().parse::<isize>() {
            Ok(i) => i,
            Err(e) =>  return Err(DiceRollError{message:e.to_string()})
        };

        let tn = match caps.name("tn") {
            Some(m) => m.as_str().parse::<isize>().unwrap_or_default(),
            None => 0
        };

        let mods: (isize,isize) = match caps.name("mods") { //inc,dec
            Some(m) => {
                parse_modifiers(m.as_str().to_string())   
            }
            None => (0,0)
        };

        let roll_type = DiceRollType::Sum;
        Ok(DiceRoll::new(
            dice_qty,
            dice_type,
            mods.0,
            mods.1,
            tn,
            roll_type
        ))
    }
}

impl fmt::Display for DiceRoll {
    fn fmt(&self,f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.roll_type {
            DiceRollType::Sum => {
                let base_result:isize = self.roll_results.iter().sum();
                let mod_str = if self.inc > 0 || self.dec < 0 {format!(" ➔ {}",self.final_result)} else {"".to_string()};
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
                    format!("| {} {}",self.final_result,word)
                } else {
                    "".to_string()
                };
                for die_result in &self.roll_results{
                    results_str += &die_result.to_string()[..];
                    results_str += ",";
                }
                results_str.pop();
                write!(f,"[{}{}]",results_str,success_str)
            }
        }
    }
}

impl fmt::Debug for DiceRoll {
   fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Dice Roll")
        .field("Dice quantity", &self.dice_qty)
        .field("Dice type", &self.dice_type)
        .field("Increment", &self.inc)
        .field("Decrement", &self.dec)
        .field("Target number", &self.tn)
        .field("Type", &self.roll_type)
        .field("Results", &self.roll_results)
        .field("Final Result", &self.final_result)
        .finish()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    #[should_panic(expected = "Invalid dice quantity")]
    fn invalid_dice_qty(){
        let _x = DiceRoll::new(-21,6,4,-2,11,DiceRollType::Sum);
    }

    #[test]
    #[should_panic(expected = "Invalid dice type")]
    fn invalid_dice_type(){
        let _x = DiceRoll::new(1,-6,4,-2,11,DiceRollType::Sum);
    }

    #[test]
    fn force_one_sum(){
        let mut x = DiceRoll::new(1,1,2,0,0,DiceRollType::Sum);
        x.roll_dice();
        assert_eq!(x.final_result,3)
    }

    #[test]
    fn force_one_pool(){
        let mut x = DiceRoll::new(2,1,0,-1,1,DiceRollType::Pool);
        x.roll_dice();
        assert_eq!(x.final_result,1)
    }


    #[test]
    fn roll_from_str(){
        let x = "3D6-3-3+1#5".parse::<DiceRoll>();
        println!("{:?}",x.unwrap());
        let x = "2d20+12-3+1-1-2+4#10".parse::<DiceRoll>();
        println!("{:?}",x.unwrap());
        let x = "15d16".parse::<DiceRoll>();
        println!("{:?}",x.unwrap());
        let x = "1d3#2".parse::<DiceRoll>();
        println!("{:?}",x.unwrap());
    }
}
