use crate::utils::generate_random_vector;
use crate::data::Data;

pub struct Individual {
    pub features: Vec<i8>, /// a vector of feature indices with their corresponding signs
    //pub feature_names: Vec<string>, /// a vector of feature indices
    pub fit_method: String, // AUC, accuracy, etc.
    pub accuracy: f64, // accuracy of the model
}

impl Individual {
    /// Provides a help message describing the `Individual` struct and its fields.
    pub fn help() -> &'static str {
        "
        Individual Struct:
        -----------------
        Represents an individual entity with a set of attributes or features.

        Fields:
        - feature_indices: HashMap<u32,u8>
            A map between feature indices (u32) and their corresponding signs (u8).
            This represents the features present in the individual, with their signs indicating 
            the direction of the relationship with the target variable.

        - feature_names: Vec<String>
            A vector containing the names of the features present in the individual.
            This provides a human-readable representation of the features.

        - fit_method: String
            A string representing the method used to evaluate the fitness of the individual.
            This could be 'AUC', 'accuracy', or any other evaluation metric.

        - accuracy: f64
            A floating-point number representing the accuracy of the model represented by the individual.
            This value indicates how well the model performs on the given data.
        "
    }

    pub fn new() -> Individual {
        Individual {
            features: Vec::new(),
            fit_method: String::from("AUC"),
            accuracy: 0.0,
        }
    }

    pub fn evaluate(&self, d: &Data) -> Vec<f64> {
        let mut value=vec![0.0; d.samples.len()];
        for (i,row) in d.X.iter().enumerate() {
            for (j,x) in row.iter().enumerate() {
                value[j]+=self.features[i] as f64*x;
            }
        }
        value
    }

    pub fn random(d: &Data) -> Individual {
        Individual {
            features: generate_random_vector(d.features.len()),
            fit_method: String::from("AUC"),
            accuracy: 0.0,
        }
    }

    // write a function fit_model that takes in the data and computes all the following fields

    // write a function evaluate_contingency_table that takes in the data and evaluates the contingency table of the model

    // write a function evaluate_accuracy that takes in the data and evaluates the accuracy of the model

    // write a function evaluate_auc that takes in the data and evaluates the AUC of the model


}
