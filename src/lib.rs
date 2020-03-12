use std::time::Instant;
extern crate rand;
use rand::Rng;
use rand::seq;
use plotters::prelude::*;
use ordered_float::NotNan;
use std::convert::TryInto;
use std::collections::HashSet;
use std::cmp;
use rand::SeedableRng;
use std::collections::vec_deque::VecDeque;

const FPS: u32 = 10;
const LENGTH: u32 = 20;
const N_DATA_POINTS: usize = (FPS * LENGTH) as usize;

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

pub fn create_initial_path(data: &Vec<City>)-> Vec<&City>{
    use rand::thread_rng;
    use::rand::seq::SliceRandom;
    let mut rng = thread_rng();
    let mut path = Vec::new();
    for city in data{
        path.push(city);
    }
    path.shuffle(&mut rng);
    path
}

pub fn create_initial_population(data: &Vec<City>, size:usize) -> Vec<Vec<&City>>{
    let mut population = Vec::new();
    let mut counter = 0;
    while counter < size{
        population.push(create_initial_path(data));
        counter += 1;
    }
    return population
}

//fn create_initial_path_random

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

pub fn display_path(path: &Vec<&City>){
    for each in path {
        print!("{:?} ->", each.get_name());
    }
}

pub fn plot_histogram() -> Result<(), Box<dyn std::error::Error>>{
    let root =
        BitMapBackend::new("data/plotters-doc-data:histogram.png", (640, 480)).into_drawing_area();

    root.fill(&WHITE)?;

    let mut chart = ChartBuilder::on(&root)
        .x_label_area_size(35)
        .y_label_area_size(40)
        .margin(5)
        .caption("Histogram Test", ("sans-serif", 50.0).into_font())
        .build_ranged(0u32..1100u32, 0u32..1100u32)?;

    chart
        .configure_mesh()
        .disable_x_mesh()
        .line_style_1(&WHITE.mix(0.3))
        .x_label_offset(30)
        .y_desc("Count")
        .x_desc("Bucket")
        .axis_desc_style(("sans-serif", 15).into_font())
        .draw()?;

    let data = [
        0u32, 1, 1, 1, 4, 2, 5, 7, 8, 6, 4, 2, 1, 8, 3, 3, 3, 4, 4, 3, 3, 10009,
    ];

    let data1:Vec<(u32,u32)>= [(1,2),(2,3),(3,4),(4,1),(1000,10)].to_vec();

    chart.draw_series(
        Histogram::vertical(&chart)
            .style(RED.mix(0.5).filled())
            //data.iter().enumerate().map(|x: &u32| (*x, 1))
            .data(data1.iter().map(|x: &(u32,u32)| (x.0, x.1))),
    )?;

    Ok(())
}

pub fn optimize_using_hill_climbing(data: &Vec<City>, time: u64) -> (Vec<&City>, f32, u64){
    let distance_table: Vec<Vec<f32>> = precalculate_distance(data);
    let mut current_path = create_initial_path(data);
    let mut path_length = evaluate_current_path_length(&distance_table, &current_path);
    println!{"Path Length: {:?}", &path_length};
    print!{"Pre-optimized path: "};
    display_path(&current_path);
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

fn acceptance_probability(newscore:f32, oldscore:f32, temperature:f32) -> f32{
    if newscore == oldscore{
        return -1.0;
    }else if newscore<oldscore{
        return 1.0;
    }
    let loss = newscore-oldscore;
    println!("newscore: {:?}, oldscore: {:?}, temperature: {:?}, probability: {:?}", newscore, oldscore, temperature, 1.0-(loss*temperature));
    
    return 1.0-(loss*temperature);
}


pub fn optimize_using_simulated_annealing(data: &Vec<City>, time: u64) -> (Vec<&City>, f32, u64){
    let distance_table: Vec<Vec<f32>> = precalculate_distance(data);
    let mut current_path = create_initial_path(data);
    let mut path_length = evaluate_current_path_length(&distance_table, &current_path);
    let size = current_path.len();
    let start = Instant::now();
    let mut flip = 0;
    let mut temperature = 0.000001;
    let cooling_rate = 0.0001; 
    let mut loopcount = 0;
    while start.elapsed().as_secs() < time {
        loopcount+=1;
        let mut rng = rand::thread_rng();
        let mut candidate_one = 0;
        let mut candidate_two = 0;
        while candidate_one == candidate_two {
            candidate_one = rng.gen_range(0,size);
            candidate_two = rng.gen_range(0,size);
        }
        current_path.swap(candidate_one, candidate_two);
        let temp_length = evaluate_current_path_length(&distance_table, &current_path);
        let mut something= rng.gen_range(0.0,1.0);
        if acceptance_probability(temp_length, path_length, temperature) > something {
            path_length = temp_length;
            flip+=1;
            temperature += cooling_rate;

        }else{
            current_path.swap(candidate_one, candidate_two);
        }
    }
    println!("LOOP COUNT: {:?}",loopcount);
    (current_path, path_length,flip)
}

// this is for rank based selection returns --> (route_id, fitness)
pub fn determine_fitness(population:&Vec<Vec<&City>>, distance_table: &Vec<Vec<f32>>, is_sorted:bool) -> Vec<(u32,f32)> {
    let mut fitness_scores = Vec::new();
    let size = population.len();
    for i in 0..size{
        let fitness = evaluate_current_path_length(distance_table, &population[i]);
        let route_id: u32 = i.try_into().unwrap();
        fitness_scores.push((route_id,fitness));
    }
    if is_sorted{
        fitness_scores.sort_by_key(|k| NotNan::new(k.1 as f32).unwrap());
        return fitness_scores;
    }
    fitness_scores
}

pub fn select_for_pool(fitness_scores: &Vec<(u32,f32)>, elite_size:u32)-> HashSet<u32> {
    let mut rng = rand::thread_rng();
    let length = fitness_scores.len();
    //let mut pool = Vec::new();
    let mut routes_to_breed = HashSet::new();

    // put the route(id, score) for elite size
    for i in (0..elite_size as usize){
        routes_to_breed.insert(fitness_scores[i].0);
    }
    // for the remainder get a random value
    let random_remainder:u32 = rng.gen_range(elite_size, length as u32);
    // Inversely associate that with the probability of picking it up
    let fixed_point = 1.00/&fitness_scores[random_remainder as usize].1;
    
    for j in (elite_size as usize ..length as usize){
        //println!{"fs: {:?}, fixed_point{:?}",1.00/fitness_scores[j].1, fixed_point};
        if 1.00/fitness_scores[j].1>= fixed_point{
            routes_to_breed.insert(fitness_scores[j].0);
        }else{
            break;
        }
    }
    
    /*
    println!{"routes_to_breed LENGTH: {:?}", &routes_to_breed.len()};
    for x in &routes_to_breed{
        println!("{:?}", x);
    }
    */
    
    routes_to_breed
}

pub fn create_mating_pool<'a>(mut population:Vec<Vec<&'a City>>, pool:HashSet<u32>) -> Vec<Vec<&'a City>>{
    let mut mating_pool: Vec<Vec<&City>> = Vec::new();
    //println!("population length: {:?}", &population.len());
    //println!("pool length: {:?}", pool.len());
    let mut k = 0;
    for i in 0..population.len(){
        if (pool.contains(&(i as u32))){
            mating_pool.push(population.remove(i-k));
            k+=1;
        }
    }
    //println!{"mp len: {:?}", &mating_pool.len()};
    mating_pool
}

pub fn breed_and_mutate<'a>(parent1: &Vec<&'a City>, parent2:&Vec<&'a City>, mutation_rate:f32) -> Vec<&'a City>{
    let mut rng = rand::thread_rng();
    let length = parent1.len();
    let mut partner_genes = HashSet::new();
    let mut child = Vec::new();

    let gene1 = rng.gen_range(0,length);
    let gene2 = rng.gen_range(0,length);
    let starting_gene = cmp::min(gene1,gene2);
    let ending_gene = cmp::max(gene1,gene2);

    for i in starting_gene..ending_gene{
        partner_genes.insert(parent1[i].get_id());
        child.push(parent1[i]);
    }
    for j in 0..length{
        if !partner_genes.contains(&parent2[j].get_id()){
            child.push(parent2[j]);
        }
    }
    assert_eq!(length,child.len());

    rng = rand::thread_rng();
    let child_length = child.len();
    for i in 0..child_length{
        if rng.gen_range(0.000,1.000) < mutation_rate{
            let mut mutation_partner = child_length+1;
            while mutation_partner!=i {
                mutation_partner = rng.gen_range(0,child_length);
            }
            child.swap(i, mutation_partner);
        }
    }
    child
}

pub fn population_crossover<'a>(mating_pool: Vec<Vec<&'a City>>, elite_size:u32, mutation_rate:f32)->Vec<Vec<&'a City>>{
    //use rand::{thread_rng, seq};
    let mut rng = rand::thread_rng();
    assert!(true, mating_pool.len() > elite_size.try_into().unwrap());
    let length = mating_pool.len()-elite_size as usize;
    let mut children = Vec::new();
    // keep the elite members as children already
    for i in 0..elite_size{
        children.push(mating_pool[i as usize].to_vec());
    }
    // randomly select from the pool
    /*
    #[derive(Debug)] 
    let mut sample = match seq::sample_iter(&mut rng, &mating_pool, mating_pool.len()){
        Ok(sample) => sample,
        Err(sample) => sample
    };
    */
    //
    while children.len()< 100{
        let parent1 = rng.gen_range(0, mating_pool.len());
        let parent2 = rng.gen_range(0, mating_pool.len());
        let mut child = breed_and_mutate(&mating_pool[parent1], &mating_pool[parent2], mutation_rate);
        //let mut child = breed_and_mutate(&sample[i as usize], &sample[mating_pool.len()-1-i as usize], mutation_rate);
        children.push(child);
    }
    children
}

// This runs for every generation meaning every population
pub fn get_next_generation<'a>(population:Vec<Vec<&'a City>>,distance_table: &Vec<Vec<f32>>, elite_size:u32, mutation_rate:f32) -> Vec<Vec<&'a City>>{
    // If this method doesn't accept it, think about cloning it
    // Get A vector of (u32,f32) indicating route(id,score)
    let fitness_table = determine_fitness(&population, distance_table, true);
    //println!("CHECCCKPOINT 1");
    // Create which ones are going to be in mating pool (elite size and random stuff);
    let selection = select_for_pool(&fitness_table, elite_size);
    //println!("CHECCCKPOINT 2");

    //create the mating pool and pick them from pool. at this point current population vector is gone forever
    let mating_pool:Vec<Vec<&'a City>> = create_mating_pool(population,selection);
    //println!("CHECCCKPOINT 3");

    //let children = population_crossover(mating_pool,elite_size);
    let next_generation:Vec<Vec<&'a City>> = population_crossover(mating_pool, elite_size,mutation_rate);
    //println!("ng len: {:?}", next_generation.len());
    next_generation
}


pub fn optimize_using_genetic_algorithm(data: &Vec<City>, size:usize, elite_size:u32,mutation_rate:f32, generations: u32){
    // *create distance table between all the cities (these locations will not change)
    let distance_table = precalculate_distance(data);
    // *create bunch of (size times) paths
    let mut population = create_initial_population(data,size);
    // * Call the 


    let initial = determine_fitness(&population, &distance_table, true);
    for route in &initial{
        println!("{:?}", route);
    }

    //Print the first route !!! YOU CAN USE DATA HERE INSTEAD OF POPULATION. POPULATION IS TEMPORARY!
    for city in &population[0]{
        print!("{:?} -> ", city.get_name());
    }
    println!("Value: {:?}",evaluate_current_path_length(&distance_table, &population[0]));


    for i in 0..generations{
        let mut temp = get_next_generation(population,&distance_table,elite_size,mutation_rate);
        population = temp
    }

    let fitness_table = determine_fitness(&population, &distance_table, true);

    let prime_index = fitness_table[0].0;
    let prime_val = fitness_table[0].1;

    println!("After return");
    for city in &population[prime_index as usize]{
        print!("{:?} ->", city.get_name());
    }
    println!("VALUE:");
    println!("{:?}", prime_val);

    //let mut temp = get_next_generation(population,&distance_table,elite_size,mutation_rate);
    //population = temp




   // let mut new_gen = get_next_generation(&temp.clone(),&distance_table,elite_size,mutation_rate);
    //let mut final_population_rank = determine_fitness(&temp, &distance_table, true);
    //let mut index:u32 = temp[0]

    /*
    for city in &temp[0]{
        print!("{:?} ->",city.get_name());
    }
    println!("Value: {:?}",evaluate_current_path_length(&distance_table, &temp[0]));
    */
}
