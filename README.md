# gpredomics

gpredomics is a rewrite of predomics in Rust, with the goal of using GPU, which is not the case right now. For the time being, it is a pure CPU program in _safe_ Rust, single thread. Only a very small subset of predomics is coded:

- only the Genetic Algorithm is coded, the equivalent of gabin in Predomics terms,
- a shortcut is taken as to evaluate only on AUC (a specific care has been taken so as to optimize the computation time of AUC),
- lots of features are not implemented such as feature importance,

At this stage (alpha), the program is simplistic and does minimal things, yet it is already useful in some cases.

## compile

To compile:

- install Rust with [rustup](https://rustup.rs) : `curl –proto '=https' –tlsv1.2 -sSf https://sh.rustup.rs | sh`
- at the root of this repository, compile: `cargo build --release`, which will create the binary in `target/release`


## use

The binary will look for a `param.yaml` file in the current working directory. There is a sample one at the root of this repository. You will of course need some data, in the typical form of a matrix (X) of values with the biological features as lines and the samples in column (in TSV format), and a target vector (y), a two column TSV (which header is ignored) containing a column with the samples (same name as X column names), and a column with the class of each sample, 0 for the first class, 1 for the second class, and 2 when the class is unknown.

You will find two sample sets, in the `samples` folder, one on microbiome and cirrhosis (Qin 2014, a.k.a. PRJEB6337), and one on microbiome and response to treatment in ICI (Immune Checkpoint Inhibitors) (Derosa 2022, a.k.a. PRJNA751792).

## some details about param.yaml

There are three sections, general, data and ga.

### general

- seed: gpredomics is fully determinist, re-running a computation with the same seed bear the same results, this should be a number,
- algo: either `random` (not useful, for tests only), `ga` the basic genetic algorithm, `ga+cv` the same algorithm but with a simple cross val scheme

### data

- The name of the files are pretty obvious: X is the matrix of features, y the target vector.
- Xtest and ytest are optional holdouts (that should not be included in X and y)
- feature_minimal_prevalence is a filter: this is the minimal number of sample non null for the feature in either class for the feature to be kept.
- feature_minimal_value is a filter on the average value of the feature (not implemented yet),
- feature_maximal_pvalue is a filter to remove features which average is not significantly different between the two classes.

### ga

- population_size: the number of model (a.k.a. Individual in the Rust code) per epoch,
- epochs: the target number of epoch (an epoch is a generation in the Genetic Algorithm)
- min_epochs: the minimal number of epoch befor trying to see if AUC are converging
- max_divergence: the criteria of AUC convergence,
- kmin: the minimal number of feature in the initial random population,
- kmax: the maximal number of feature in the initial random population,
- kpenalty: the penalty applied to the AUC per number of feature in the model,
- select_elite_pct: the percentage of best N-1 individual selected as parents,
- select_random_pct: the percentage of random N-1 individual selected as parents,
- mutated_children_pct: the percentage of children mutated,
- mutated_features_pct: the percentage of feature mutated in a mutated child,
- mutation_non_null_chance_pct: the likeliness of "positive" mutation, e.g. a mutation that select a new feature in the model.

