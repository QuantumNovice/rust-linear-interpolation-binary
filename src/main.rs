    use std::env;

    fn main() {
        let args: Vec<String> = env::args().collect();

        let l_args = args.len();

        if l_args == 1 {
            println!("lerp.exe x0 x1 y0 y1 x");
        }
        else if l_args == 6 {
            
            let x0 = args[1].parse::<f64>().unwrap();
            let x1 = args[2].parse::<f64>().unwrap();
            let y0 = args[3].parse::<f64>().unwrap();
            let y1 = args[4].parse::<f64>().unwrap();
            let x = args[5].parse::<f64>().unwrap();

            let y = (y0 *(x1-x) + y1*(x-x0))/(x1 - x0);
            println!("All Arguments");
            println!("{}",y);
        }
        
        else {
            println!("Incorrect arguments passed");
        }
    }
