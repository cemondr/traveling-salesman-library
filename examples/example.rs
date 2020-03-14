use rso4ts::*;
extern crate rand;
use rand:: Rng;
use std::fs::File;
use std::io::{BufRead,BufReader};
use std::env;

fn get_random_city() -> (f32,f32) {
    let mut rng = rand::thread_rng();
    let tempvec = (rng.gen_range(0.0,1000.00),rng.gen_range(0.0,1000.00));
    tempvec
}
fn main(){
    let args: Vec<String> = env::args().collect();
    if args.len() > 1{

        let filename = &args[1];
        //Open the file in read-only mode
        let file = File::open(filename).unwrap();
        let reader = BufReader::new(file);

        //Read the file line by line using the lines() iterator from std::io::BufRead
        let mut data = Vec::new();
        for (index, line) in reader.lines().enumerate(){
            let line = line.unwrap();
            let split = line.split(',');
            let mut city_vec = Vec::new();
            for s in split{
                city_vec.push(s);
            }
            let city_coordinates:(f32,f32) = (city_vec[0].parse().unwrap(),city_vec[1].parse().unwrap());
            let city = City::new(index.to_string(),index,city_coordinates);
            data.push(city);
        }

        let returned_path:(Vec<&City>, f32, u64) = match args[2].as_ref(){
            "h" => optimize_using_hill_climbing(&data,args[3].parse().unwrap(),true),
            "s"=> optimize_using_simulated_annealing(&data,args[3].parse().unwrap()),
            "g"=> optimize_using_genetic_algorithm(&data,100,30,0.01,args[3].parse().unwrap()),
            _ => panic!("no"),
        };
        
        print!("Post optimization path: ");
        display_path(&returned_path.0);        
        println!("{} miles", returned_path.1);
        //println!("Flip #: {:?}", returned_path.2);
    }else{
        let mut vec10 = Vec::new();
        let mut vec30 = Vec::new();
        let mut vec50 = Vec::new();
        let mut vec100 = Vec::new();
        let mut vec200= Vec::new();
        
        for i in 0..200{
            let city_name: String = i.to_string();
            if i <10{    
                let city10 = City::new(city_name.clone(),i,get_random_city());
                vec10.push(city10);
            }
            if i < 30{
                let city30 = City::new(city_name.clone(),i,get_random_city());
                vec30.push(city30);
            }
            if i< 50 {
                let city50 = City::new(city_name.clone(),i,get_random_city());
                vec50.push(city50);    
            }
            if i<100{
                let city100 = City::new(city_name.clone(),i,get_random_city());
                vec100.push(city100);
            }
            let city200 = City::new(city_name,i,get_random_city());
            vec200.push(city200);
        }
        for city in vec10{
            println!("{:?},{:?}" , city.get_coordinates().0, city.get_coordinates().1);
        }
    }
}