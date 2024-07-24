

fn main() {
    let values = vec![4, 9, 12, 11, 6, 5];
    let weights = vec![1, 2, 10, 4, 3, 5];


    println!("total value = {}", fractional_knapsack(values, weights, 12));
}


fn fractional_knapsack(values: Vec<i32>, weights: Vec<i32>, knapsack_weight: i32) -> f64 {
    let ratios: Vec<f64> = values.iter()
    .zip(weights.iter())
    .map(|(&v, &w)| v as f64 / w as f64)
    .collect();

    let mut items: Vec<(i32, i32, f64)> = values.iter()
    .zip(weights.iter())
    .zip(ratios.iter())
    .map(|((&v,&w),&r)| (v,w,r))
    .collect();

    items.sort_by(|a,b| b.2.partial_cmp(&a.2).unwrap_or(std::cmp::Ordering::Equal));

    let mut curr_weight = 0;
    let mut total_value = 0.0;

    for item in items {
        if knapsack_weight == curr_weight {
            break;
        }

        let diff_weight = knapsack_weight - curr_weight;

        if item.1 <= diff_weight {
            total_value += item.0 as f64;
            curr_weight += item.1;
        }else {
            total_value += diff_weight as f64 * item.2;
            curr_weight += diff_weight;
        }

    }

    total_value
}
