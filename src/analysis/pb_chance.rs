//! Calculates chance of beating the Personal Best with the current attempt. The
//! value is in the range 0 to 1.

use comparison::{average_segments, best_segments, personal_best};
use Timer;

/// Calculates chance of beating the Personal Best with the current attempt. The
/// value is in the range 0 to 1.
pub fn calculate(timer: &Timer) -> Option<f64> {
    let run = timer.run();
    let timing_method = timer.current_timing_method();

    // We need to extract information from the most recent split.
    let recent_split_index = timer.current_split_index()?.checked_sub(1)?;
    let recent_split = run.segment(recent_split_index);

    let final_segment = run.segment(run.len() - 1);

    // Extract all the constants needed for the calculation.
    let number_of_remaining_splits = run.len() - recent_split_index - 1;

    let remaining_average = final_segment.comparison(average_segments::NAME)[timing_method]?
        - recent_split.comparison(average_segments::NAME)[timing_method]?;
    let remaining_average = remaining_average.total_seconds();

    let remaining_pb_time = final_segment.comparison(personal_best::NAME)[timing_method]?
        - recent_split.split_time()[timing_method]?;
    let remaining_pb_time = remaining_pb_time.total_seconds();

    let remaining_best_segments = final_segment.comparison(best_segments::NAME)[timing_method]?
        - recent_split.comparison(best_segments::NAME)[timing_method]?;
    let remaining_best_segments = remaining_best_segments.total_seconds();

    // Calculate the actual PB Chance now.

    let number_of_remaining_splits = (number_of_remaining_splits as f64).min(150.0);

    let l1 = number_of_remaining_splits * (remaining_pb_time - remaining_best_segments)
        / (remaining_average - remaining_best_segments);
    let p1 = (-l1).exp();

    let mut s1 = 1.0;
    let mut h = 1.0;
    let mut i = 1.0;
    let k = number_of_remaining_splits;
    while i < k {
        h *= l1 / i;
        s1 += h;
        i += 1.0;
    }

    let p1 = p1 * s1;

    Some(1.0 - p1).filter(|n| !n.is_nan())
}
