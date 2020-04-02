## Cem Onder (yco@pdx.edu)

## optimization library for traveling salesman problem


### What the project is and does. What is its intended function?

- Implements various optimization algorithms for the traveling salesman problem.  
    - Hill Climbing, 
    - Simulated Annealing, 
    - Genetic Algorithm 
- Provides a modest histogram to see the progress that is done (all solutions that have been reached;

### How to build and run the project

     cargo run --example example <"data directory"> <option_algorithm> <option_time>
     //option_algorithm can be:
                    for hill climbing = h
                    for simulated_annealing = s
                    for genetic algorithm = g
    //option_time can be
                    any time in seconds : 
    
    Examples (copy/paste friendly):

    -> cargo run --example example "data/data10.txt" g 5
    -> cargo run --example example "data/data50.txt" h 1
    -> cargo run --example example "data/data30.txt" s 10


