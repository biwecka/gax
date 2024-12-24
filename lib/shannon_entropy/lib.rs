// Imports /////////////////////////////////////////////////////////////////////
use hashbrown::HashMap;

// Functions ///////////////////////////////////////////////////////////////////
/// Function to calculate the normalized Shannon entropy for the given iterator.
/// The calculation is thereby based on the commonly known Shannon entropy
/// (see <https://en.wikipedia.org/wiki/Entropy_(information_theory)>), and
/// complemented by a normalization step which divides the Shannon entropy
/// by the maximum possible entropy of the given data.
pub fn normalized_shannon_entropy<T>(data: impl Iterator<Item = T>) -> f64
where
    T: std::cmp::Eq + std::hash::Hash,
{
    // Initialize a hash map for counting data items.
    let mut counts = HashMap::<T, usize>::new();

    // Count data items.
    for d in data {
        *counts.entry(d).or_insert(0) += 1;
    }

    // Calculate total number of items.
    let total_items: usize = counts.values().sum();

    // If the total number of items is 0 (no data was supplied), then return
    // the maximum entropy of "0.0".
    // It's important to return early here, because the `log2` function in the
    // next step would PANIC if total_items is zero!
    if total_items == 0 {
        return 0.;
    }

    // Calculate maximal entropy
    let max_entropy = (total_items as f64).log2();

    // Calculate Shannon entropy
    let entropy: f64 = -counts // <- the minus "-" is very important !!!
        .values()
        .filter(|num| **num > 0)
        .map(|num| {
            // Calculate probability
            let p: f64 = *num as f64 / total_items as f64;

            // Inner formula
            p * p.log2()
        })
        .sum::<f64>();

    // Normalize entropy and return
    entropy / max_entropy
}

// Tests ///////////////////////////////////////////////////////////////////////
#[cfg(test)]
mod test {
    use crate::normalized_shannon_entropy;

    #[test]
    fn basic_0() {
        let values = [1, 1, 1];
        let entropy = normalized_shannon_entropy(values.iter());

        assert_eq!(format!("{:.2}", entropy), "-0.00");
    }

    #[test]
    fn basic_1() {
        let values = [1, 2, 3];
        let entropy = normalized_shannon_entropy(values.iter());

        assert_eq!(format!("{:.2}", entropy), "1.00");
    }

    #[test]
    fn basic_2() {
        let values = [1, 1, 2];
        let entropy = normalized_shannon_entropy(values.iter());

        assert_eq!(format!("{:.4}", entropy), "0.5794");
    }

    #[test]
    fn basic_3() {
        let values: [u8; 0] = [];
        let entropy = normalized_shannon_entropy(values.iter());

        assert_eq!(format!("{:.2}", entropy), "0.00")
    }
}

////////////////////////////////////////////////////////////////////////////////
