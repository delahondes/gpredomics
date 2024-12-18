use crate::population::Population;
use crate::data::Data;
use crate::individual::Individual;
use crate::param::Param;
use rand::prelude::SliceRandom;
use rand::Rng;
use rand::seq::index::sample;
use rand::prelude::*;
use rand_chacha::ChaCha8Rng;

pub fn ga(mut data: &mut Data, param: &Param) -> Vec<Population> {
    // generate a random population with a given size  (and evaluate it for selection)
    let mut pop = Population::new();
    let mut epoch:u32 = 0;
    let mut populations: Vec<Population> = Vec::new();
    let mut auc_values: Vec<f64> = Vec::new();
    let mut rng = ChaCha8Rng::seed_from_u64(param.general.seed);

    //println!("Selecting features");
    data.select_features(param);
    print!("Features: {}",data.feature_selection.len());

    //println!("Generate initial population");
    pop.generate(param.ga.population_size,param.ga.kmin, param.ga.kmax, data, &mut rng);
    pop.evaluate_with_k_penalty(data, param.ga.kpenalty);
    
    loop {
        epoch += 1;
        //println!("Starting epoch {}",epoch);


        // we create a new generation
        let mut new_pop = Population::new();

        // select some parents (according to elitiste/random params)
        // these individuals are flagged with the parent attribute
        // this populate half the new generation new_pop
        let sorted_pop = pop.sort();  
        //println!("best AUC so far {} (k={})", &sorted_pop.individuals[0].auc, &sorted_pop.individuals[0].k);
        auc_values.push(sorted_pop.fit[0]);

        new_pop.add(select_parents(&sorted_pop, param, &mut rng));

        let mut children = cross_over(&new_pop,param,data.feature_len, &mut rng);
        mutate(&mut children, param, data.feature_len, &mut rng);
        children.evaluate_with_k_penalty(data, param.ga.kpenalty);
        new_pop.add(children);

        populations.push(sorted_pop);
        pop = new_pop;

        if (epoch>=param.ga.epochs) {
            //println!("The target number of epoch {} has been reached, stopping",epoch);
            break
        }
        if auc_values.len()>10 && auc_values.len()>param.ga.min_epochs {
            let avg:f64=auc_values[auc_values.len()-10..].iter().sum::<f64>()/10.0;
            let divergence = auc_values[auc_values.len()-10..].iter().map(|x| (*x-avg).abs()).sum::<f64>();
            if divergence<avg*param.ga.max_divergence {
                //println!("AUCs stay stable for the last 10 rounds (divergence {} is below {}), stopping",divergence,avg*param.ga.max_divergence);
                break
            }
        }
    }

    populations
    
}

/// pick params.ga_select_elite_pct% of the best individuals and params.ga_select_random_pct%  
fn select_parents(pop: &Population, param: &Param, rng: &mut ChaCha8Rng) -> Population {
    
    // order pop by fit and select params.ga_select_elite_pct
    let (mut parents, n) = pop.select_first_pct(param.ga.select_elite_pct);

    // add a random part of the others
    parents.add(pop.select_random_above_n(param.ga.select_random_pct, n, rng));

    parents

}


/// create children from parents
fn cross_over(parents: &Population, param: &Param, feature_len: usize, rng: &mut ChaCha8Rng) -> Population {
    let mut children=Population::new();

    for i in 0..(param.ga.population_size as usize-parents.individuals.len()) {
        let mut child=Individual::new();        
        let [p1,p2] = parents.individuals.choose_multiple(rng, 2)
                        .collect::<Vec<&Individual>>()
                        .try_into()
                        .expect("Vec must have exactly 2 elements");
        let x=rng.gen_range(1..feature_len-1);
        child.features = p1.features[..x].to_vec();
        child.features.extend_from_slice(&p2.features[x..]);

        child.count_k();

        children.individuals.push(child);
    }

    children

}

/// change a sign, remove a variable, add a new variable
fn mutate(children: &mut Population, param: &Param, feature_len: usize, rng: &mut ChaCha8Rng) {
    let p1 = param.ga.mutation_non_null_chance_pct/200.0;
    let p2= 2.0*p1;

    if param.ga.mutated_children_pct > 0.0 {
        let num_mutated_individuals = (children.individuals.len() as f64 
            * param.ga.mutated_children_pct / 100.0) as usize;

        let num_mutated_features = (feature_len as f64 
            * param.ga.mutated_features_pct / 100.0) as usize;

        // Select indices of the individuals to mutate
        let individuals_to_mutate = sample(rng, children.individuals.len(), num_mutated_individuals);

        for idx in individuals_to_mutate {
            // Mutate features for each selected individual
            let individual = &mut children.individuals[idx]; // Get a mutable reference
            let feature_indices = sample(rng, feature_len, num_mutated_features);

            for i in feature_indices {
                if individual.features[i]!=0 { individual.k-=1 }
                individual.features[i] = match rng.gen::<f64>() {
                    r if r < p1 => {individual.k+=1; 1},
                    r if r < p2 => {individual.k+=1; -1},
                    _ => 0,
                };
            }
        }
    }
}