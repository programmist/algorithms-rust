use rand::Rng;
use std::time::Instant;

mod binarysearch;
use binarysearch::search;

fn get_random_array() -> [i32; 1000] {
  let mut rng = rand::thread_rng();
  let mut nums: [i32; 1000] = [0; 1000];
  for i in 0..1000 {
    nums[i] = rng.gen_range(0..100);
  }
  return nums;
}

fn main() {
  let mut nums: [i32; 1000] = get_random_array();
  nums.sort();

  let mut successes: Vec<core::time::Duration> = Vec::new();
  let mut fails: Vec<core::time::Duration> = Vec::new();
  let mut rng = rand::thread_rng();

  for _ in 0..100 {
    let key = rng.gen_range(0..1000);
    println!("search for {}", key);
    let start_time = Instant::now();
    let result = search(&nums, key);
    let elapsed_time = start_time.elapsed();

    if result == -1 {
      fails.push(elapsed_time);
      println!("{} not found", key);
    } else {
      successes.push(elapsed_time);
      println!("{} found at {}", key, result);
    }
  }

  let success_sum: core::time::Duration = successes.iter().sum();
  let success_size: i32 = successes.len().try_into().unwrap();
  if success_size > 0 {
    println!(
      "Success avg time: {:?}",
      success_sum / successes.len().try_into().unwrap()
    );
  } else {
    println!("no successes")
  }

  let fail_sum: core::time::Duration = fails.iter().sum();
  let fails_size: i32 = fails.len().try_into().unwrap();
  if fails_size > 0 {
    println!(
      "Fail avg time: {:?}",
      fail_sum / fails.len().try_into().unwrap()
    );
  } else {
    println!("no fails")
  }

  println!(
    "Avg total search times: {:?}",
    (success_sum + fail_sum) / (successes.len() + fails.len()).try_into().unwrap()
  );
}
