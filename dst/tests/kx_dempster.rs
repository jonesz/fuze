// use dst::approx::KX;
// use dst::comb::{CombRule, Dempster};
// use dst::dst::bel;
//
// #[test]
// fn films_high_conflict() {
//     // https://en.wikipedia.org/wiki/Dempster%E2%80%93Shafer_theory#Example_producing_correct_results_in_case_of_high_conflict
//     const FILM_X: usize = 0b001;
//     const FILM_Y: usize = 0b010;
//     const FILM_Z: usize = 0b100;
//
//     const FILMS_HIGH_CONFLICT: [&[(usize, f32)]; 2] = [
//         &[(FILM_X, 0.99f32), (FILM_Y, 0.01f32)],
//         &[(FILM_Z, 0.99f32), (FILM_Y, 0.01f32)],
//     ];
//     const EPS: f32 = 0.001f32;
//
//     let bba = Dempster::<usize, f32>::comb::<2usize, 4usize, KX>(FILMS_HIGH_CONFLICT);
//
//     assert!((bel(&bba, &FILM_Y) - 1.0f32).abs() < EPS);
//     assert!(bel(&bba, &FILM_X) < EPS);
//     assert!(bel(&bba, &FILM_Z) < EPS);
// }
