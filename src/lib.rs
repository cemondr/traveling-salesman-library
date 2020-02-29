use std::time::Instant;
extern crate rand;
use rand:: Rng;

pub struct City{
    name: String,
    assigned_id: usize,
    coordinates: (f32,f32),
}

impl City{
    pub fn new(name: String, assigned_id: usize, coordinates: (f32,f32)) -> City{
        City{name,assigned_id,coordinates}
    }
    // Calculate the Eucledian Distance between two points 
    pub fn euclidean_distance(&self, target: &City )-> f32{
        let distance = (self.coordinates.0-target.coordinates.0).powf(2.0) + (self.coordinates.1 - target.coordinates.1).powf(2.0);
        distance.sqrt()
    }
    pub fn get_name(&self) -> &String{
        &self.name
    }
    pub fn get_coordinates(&self)-> &(f32,f32){
        &self.coordinates
    }
    pub fn get_id(&self)->usize{
        self.assigned_id
    }
}

// Pre calculate all the distances between every city so that we don't have to do on the go
pub fn precalculate_distance(city_coordinates: &Vec<City>) -> Vec<Vec<f32>>{
    let size = city_coordinates.len();
    let mut distance_vector = Vec::with_capacity(size);
    for i in 0..size  {
        assert_eq!(city_coordinates[i].get_id(), i);
        let mut temp = Vec::with_capacity(size);
        for j in 0..size{
            temp.push(city_coordinates[i].euclidean_distance(&city_coordinates[j]));
        }
        distance_vector.push(temp);
    }
    distance_vector
}

fn create_initial_path(data: &Vec<City>)-> Vec<&City>{
    let mut path = Vec::new();
    for city in data{
        path.push(city);
    }
    path
}

pub fn evaluate_current_path_length(distance_table: &Vec<Vec<f32>>, path: &Vec<&City>) -> f32{ 
    let size = path.len();
    let mut total = 0.0;
    for i in 0..size-1{
        let source = path[i].get_id();
        let destination = path[i+1].get_id();
        total += distance_table[source][destination];
    }
    total += path[size-1].euclidean_distance(&path[0]);
    total
}

pub fn optimize_using_hill_climbing(data: &Vec<City>, time: u64) -> (Vec<&City>, f32, u64){
    let distance_table: Vec<Vec<f32>> = precalculate_distance(data);
    let mut current_path = create_initial_path(data);
    let mut path_length = evaluate_current_path_length(&distance_table, &current_path);
    let size = current_path.len();
    let start = Instant::now();
    let mut flip = 0;
    while start.elapsed().as_secs() < time {

        let mut rng = rand::thread_rng();
        let mut candidate_one = 0;
        let mut candidate_two = 0;
        while candidate_one == candidate_two {
            candidate_one = rng.gen_range(0,size);
            candidate_two = rng.gen_range(0,size);
        }
    
        current_path.swap(candidate_one, candidate_two);
        let temp_length = evaluate_current_path_length(&distance_table, &current_path);
        if  temp_length < path_length{
            path_length = temp_length;
            flip+=1;
        }else{
            current_path.swap(candidate_one, candidate_two);
        }
    }
    (current_path, path_length,flip)
}