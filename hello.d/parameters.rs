
#[derive(Debug)]
pub enum Kind{
    Command(String)
        , Value(String)
}


#[derive(Debug)]
pub struct Parameter {
    pub kind: Kind,
}


pub fn check_parameter_at(rank: usize) -> Result<Parameter, String>{
    let p = std::env::args().nth(rank);
    match p {
        Some(parameter) => {
            if parameter.len() > 1 && &parameter[..2] == "--" {
                return Ok(Parameter{
                    kind: Kind::Command(String::from(&parameter[2..])),
                });
            }
            /**/
            return Ok(Parameter {
                kind: Kind::Value(String::from(parameter)),
            });
        }
        None => {
            return Err(format!("no parameter #{rank}"));
        }
    }
}

