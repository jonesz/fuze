use agg_exp::forecaster::{exp::EWAF, ExpertForecaster};
use agg_exp::loss::{Loss, L2};

#[test]
fn test_sine() {
    // We're going to sample from some sinusoid with predictions from
    // two dumb, monotonic predictors. The cumulative loss should be
    // less than either of the experts.

    const FREQ: f32 = 2.0f32;
    let environment =
        |t: usize| -> f32 { f32::sin(2.0 * std::f32::consts::PI * FREQ * (t as f32)) };
    let expert_a = |p: &f32| -> f32 { p + 1.0f32 };
    let expert_b = |p: &f32| -> f32 { p - 1.0f32 };

    let mut cumulative_loss = [0.0f32, 0.0f32, 0.0f32];

    // An exponential predictor with two experts and f32 weights.
    let mut ewaf = EWAF::<L2, f32, 2>::default();

    let mut state = 0.0f32; // The previous prediction; start at 0.
    for t in 0..32 {
        let p = [expert_a(&state), expert_b(&state)];
        let p_hat = ewaf.predict(&p);
        state = environment(t);
        ewaf.update(&p, &state); // Update the EWAF.

        // Update the cumulative loss for the EWAF and the experts.
        cumulative_loss[0] += L2::l(&p_hat, &state);
        cumulative_loss[1] += L2::l(&p[0], &state);
        cumulative_loss[2] += L2::l(&p[1], &state);
    }

    // The cumulative loss for the EWAF should be better than either
    // of the experts.
    assert!(cumulative_loss[0] > cumulative_loss[1]);
    assert!(cumulative_loss[0] > cumulative_loss[2]);
}
