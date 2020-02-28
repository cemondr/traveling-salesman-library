
pub struct City{
    name: String,
    coordinates: (f32,f32),
}

impl City{
    pub fn new(name: String, coordinates: (f32,f32)) -> City{
        City{name,coordinates}
    }
    // Calculate the Eucledian Distance between two points 
    pub fn euclidean_distance(&self, target: &City )-> f32{
        let distance = (self.coordinates.0-target.coordinates.0).powf(2.0) + (self.coordinates.1 - target.coordinates.1).powf(2.0);
        distance.sqrt()
    }

    pub fn print_name(&self, tag_included : bool){
        if tag_included == true{
            println!("City Name: {:?}", self.name);
        }else{
            println!("{:?}",self.name);
        }
    }
    pub fn print_coordinates(&self, tag_included : bool ){
        if tag_included == true{
            println!("{:?} Coordinates: {:?}", self.name, self.coordinates);
        }else{
            println!("{:?}",self.coordinates);
        }
    }
    
}

// Pre calculate all the distances between every city so that we don't have to do on the go
pub fn precalculate_distance(city_coordinates: &Vec<City>) -> Vec<Vec<f32>>{
    let size = city_coordinates.len();
    let mut distance_vector = Vec::with_capacity(size);
    for i in 0..size  {
        let mut temp = Vec::with_capacity(size);
        for j in 0..size{
            temp.push(city_coordinates[i].euclidean_distance(&lcity_coordinates[j]));
        }
        distance_vector.push(temp);
    }
    distance_vector
}

/*
fn create_initial_path(){

}
*/

//pub fn evaluate_current_distance();

//pub 

//pub fn randomize_path();