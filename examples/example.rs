//#[derive(Clone)]
use rso4ts::*;
extern crate rand;
use rand:: Rng;


fn get_random_city() -> (f32,f32) {
    let mut rng = rand::thread_rng();
    let tempvec = (rng.gen_range(0.0,30.00),rng.gen_range(0.0,30.00));
    tempvec
}
fn main(){
    let mut vec10 = Vec::new();
    let mut vec30 = Vec::new();
    let mut vec50 = Vec::new();
    

    for i in 0..50{
        let city_name: String = i.to_string();
        if i <10{    
            let city10 = City::new(city_name.clone(), get_random_city());
            vec10.push(city10);
        }
        if i < 30{
            let city30 = City::new(city_name.clone(), get_random_city());
            vec30.push(city30);
        }
        let city50 = City::new(city_name, get_random_city());
        vec50.push(city50);
    }
    for i in 0..10{
        vec10[i].print_name(true);
        vec10[i].print_coordinates(true);
    }

    let mut optimized_path = optimize_using_hill_climbing(&vec10,10);
    for city in optimized_path{
        print!("{:?} ->", city.print_name(false))
    }

    /*

    let distance_vector = precalculate_distance(&vec10);

    
    let size = distance_vector.len();


    for i in 0..size{
        for j in 0..size{
            print!("{:?} , ", distance_vector[i][j]);
        }
        println!()
    }
    */
    
}