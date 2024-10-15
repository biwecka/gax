// Objective Value /////////////////////////////////////////////////////////////
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Cost(usize);

impl ga::encoding::ObjectiveValue for Cost {
    fn calc_average(values: &[Self]) -> f32 {
        let sum: usize = values.iter().map(|x| x.0).sum();
        sum as f32 / values.len() as f32
    }

    fn calc_distribution(values: &[Self]) -> Vec<usize> {
        // Calculate worst objective value
        let max = values.iter().map(|x| x.0).max().unwrap();

        // Initialize array
        let mut arr = vec![0; max + 1];

        // Evaluate distribution
        for val in values {
            assert!(val.0 < arr.len());
            arr[val.0] += 1;
        }

        // Return
        arr
    }

    fn to_usize(&self) -> usize {
        self.0
    }
}

impl From<usize> for Cost {
    fn from(value: usize) -> Self {
        Self(value)
    }
}

impl From<Cost> for usize {
    fn from(value: Cost) -> Self {
        value.0
    }
}

////////////////////////////////////////////////////////////////////////////////
