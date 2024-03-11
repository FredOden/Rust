mod parameters;
mod fact;


enum WichError {
    ParseError(std::num::ParseIntError)
        ,StringError(String)
}

impl From<std::num::ParseIntError> for WichError {
    fn from(error: std::num::ParseIntError) -> Self {
        WichError::ParseError(error)
    }
}
impl From<String> for WichError {
    fn from(error: String) -> Self {
        WichError::StringError(error)
    }
}

/*
fn print_type_of<T>(_: &T) {
    println!("{}", std::any::type_name::<T>())
}
*/

fn hello() -> Result<String, WichError>
{

    let name =  parameters::check_parameter_at(1)?; 
    match name.kind {
        parameters::Kind::Command(val) => {
            match val.as_str() {
                "fact" => {
                    let number = parameters::check_parameter_at(2)?; 
                    match number.kind {
                        parameters::Kind::Command(val) => {
                            Err(format!("facr doesnot accept command '{}' as parameter", val).into())
                        }
                        parameters::Kind::Value(val) => {
                            let n:usize = val.parse::<usize>()?; //.context(|| "--fact #argument")?; 
                            Ok(format!("fact({}) = {}", n, fact::fact(n)))
                        }
                    }
                }
                "help" => {
                    println!("Usage: --fact #unsigned\n--help");
                    Ok(String::from("help ok"))
                }
                _ => {
                    Err(format!("{} is invalid command", val).into())
                }
            }
        },
        parameters::Kind::Value(val) => {
            Err(format!("{} is not a command (valid are --fact #n)", val).into())
        }
    }
}

fn main() {
    match hello() {
        Err(e) => {
            match e {
                WichError::ParseError(error) => {
                    println!("ParseError:: {}", error)
                }
                WichError::StringError(error) => {
                    println!("StringError:: {}", error)
                }
            }
        }
        Ok(s) => {
            println!("Hello: all is well {}", s);
        }
    }
}
