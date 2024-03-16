mod parameters;
mod fact;

fn main() {
    match  parameters::check_parameter_at(1) {
        Ok(name) => {
            match name.kind {
                parameters::Kind::Command(val) => {
                    match val.as_str() {
                        "fact" => {
                            match parameters::check_parameter_at(2) {
                                Ok(number) => {
                                    match number.kind {
                                        parameters::Kind::Command(val) => {
                                            println!("fact doesnot accept command '{}' as parameter", val);
                                        }
                                        parameters::Kind::Value(val) => {
                                            match val.parse::<usize>() {
                                                Ok(n) => {
                                                    println!("fact({}) = {}", n, fact::fact(n));
                                                }
                                                Err(e) => {
                                                    println!("number expected for fact:{}", e);
                                                }
                                            }
                                        }
                                    }
                                }
                                Err(e) => {
                                    println!("No number for fact:{}", e);
                                }
                            }
                        }
                        "help" => {
                            println!("Usage: --fact #unsigned\n--help");
                        }
                        _ => {
                            println!("{} is invalid command", val);
                        }
                    }
                }
                parameters::Kind::Value(val) => {
                    println!("{} is not a command (valid are --fact #n)", val);
                }
            }
        }
        Err(e) => {
            println!("ERROR NO name:{}", e);
        }
    }
}
