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


### A description of testing done to make sure the project works. Include output from test runs somewhere if you are doing something fancier than cargo test

- All testing is at manual level:  by computing the results on smaller data and comparing the results to generally accepted performance levels.
- There are currently no cargo tests (due to project being a heuristic heavy one), but the heuristics behave the way they are supposed to. It is a weakness that I am hoping to change in the near future.
 

### What worked? What didn't? How satisfied are you with the result? What would you like to improve in the future?

- **What worked?** Heuristics behave the way they are supposed to, traveling salesman problem gets optimized, general structure is there.
- **What didn't work?** Here, we go:
    - I was hoping to be able to write something that would integrate well with the geo crate, that turned out to be harder than I thought. This remains as something I'd like to try in the near future.
    - I planned a better visualization option, but I couldn't make it on time. This will need to be improved to make this library a little more usable.
    - I generally had real tough time reading documentation during this project. Part of it is because I am still new to the syntax, and not everything is immediately clear to me. But also, I feel like I encountered a lot of bad documentation, especially for visualization libraries. They were outdated(plotlib). Sometimes an example they provided did not match the documentation. At times the documentation was correct, the other times the example was correct. I spent so much time on basically trying to guess stuff and it really demotivated me at some point.  


