use rso4ts::*;
extern crate rand;
use rand:: Rng;
use std::fs::File;
use std::io::{BufRead,BufReader};

fn get_random_city() -> (f32,f32) {
    let mut rng = rand::thread_rng();
    let tempvec = (rng.gen_range(0.0,100.00),rng.gen_range(0.0,100.00));
    tempvec
}
fn main(){
    
    
    let filename = "data/data100.txt";
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
        //println!("City: {:?}", city_coordinates);
    }

    /*
    let returned_path = optimize_using_simulated_annealing(&data,60);


    println!("Flip #: {:?}", returned_path.2);
    println!("Path length: {}", returned_path.1);
    display_path(&returned_path.0);
    */

    /*
    let this = match plot_histogram(){
        Ok(num) => num,
        Err(_) => print!("Bish it's an error"),
    };
    */

    /*
    for city in data{
        println!("{:?}, {:?}, {:?}", city.get_name(), city.get_id(), city.get_coordinates());
    }
    */

    

/*
    
    let mut vec10 = Vec::new();
    let mut vec30 = Vec::new();
    let mut vec50 = Vec::new();
    let mut vec100 = Vec::new();
    let mut vec1000= Vec::new();
    

    
    for i in 0..1000{
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
        let city1000 = City::new(city_name,i,get_random_city());
        vec1000.push(city1000);
    }
    
    for city in vec50{
        println!("{:?}, {:?}" , city.get_coordinates().0, city.get_coordinates().1);

    }
   */ 

    optimize_using_genetic_algorithm(&data, 100, 20, 0.02, 1500);

    /*
    let distance_table: Vec<Vec<f32>> = precalculate_distance(&vec10);
    
    let mut current_path = create_initial_path(&vec10);
    for x in current_path{
        print!{"{:?} -> ", x.get_name()}
    }
    
    

    let mut population: Vec<Vec<&City>> = create_initial_population(&vec10,10);

    let mut ordered_pops: Vec<(u32,f32)> = determine_fitness(&population, &distance_table, true);

    
    for x in ordered_pops{
        println!{"route_id-> {:?}, route-> {:?}", x.0, x.1};
    }
    */
    


    /*
    
    println!("The path");

    let optimized_path: (Vec<&City>, f32, u64) = optimize_using_simulated_annealing(&vec50,300);
    for city in optimized_path.0{
        print!("{:?} ->", city.get_name());
    }

    println!("{:?}", "");
    println!("length: {:?} |||| flips: {:?}", optimized_path.1,optimized_path.2);

    println!("The path");

    let optimized_path: (Vec<&City>, f32, u64) = optimize_using_hill_climbing(&vec50,30);
    for city in optimized_path.0{
        print!("{:?} ->", city.get_name());
    }

    println!("{:?}", "");
    println!("length: {:?} |||| flips: {:?}", optimized_path.1,optimized_path.2);

    */

    

//    let distance_vector = precalculate_distance(&vec10);

    

/*
    let size = distance_table.len();



    for i in 0..size{
        for j in 0..size{
            print!("{:?} , ", distance_table[i][j]);
        }
        println!()
    }
*/
    
}