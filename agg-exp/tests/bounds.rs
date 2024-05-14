use agg_exp::forecaster::{exp::EWAF, ExpertForecaster};
use agg_exp::loss::{Loss, L2};

#[test]
fn test_round_dependent_bound_zero_loss() {
    // By "2.4 An Improvement for Small Losses", a sequence with a
    // perfect predictor should have a bound of \ln{N}.

    // Duplicate of the sin test.
    const FREQ: f32 = 0.05f32;
    let environment =
        |t: usize| -> f32 { f32::sin(2.0 * std::f32::consts::PI * FREQ * (t as f32)) };
    let expert_a = |p: &f32| -> f32 { p + 0.5f32 };
    let expert_b = |p: &f32| -> f32 { p - 0.5f32 };

    // An exponential predictor with three experts and f32 weights.
    let mut ewaf = EWAF::<L2, f32, 3>::default();
    let mut cumulative_loss = 0.0f32;

    let mut state = environment(0);
    for t in 0..100 {
        // Note: the third predictor is perfect and thus is the revealed value.
        let p = [expert_a(&state), expert_b(&state), environment(t)];
        let p_hat = ewaf.predict(&p);

        state = environment(t);
        ewaf.update(&p, &state); // Update the EWAF.

        cumulative_loss += L2::l(&p_hat, &state);
    }

    // We shouldn't have eclipsed the upper bound for a perfect predictor: ln(N).
    assert!(cumulative_loss <= f32::ln(3f32));
}
