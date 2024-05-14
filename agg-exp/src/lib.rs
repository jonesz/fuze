//! Prediction with Expert Advice.
//!
//! These procedures allow for the weighing and combination of multiple expert
//! predictions with the specific goal of minimizing loss in respect to the
//! *best* expert prediction. Implementations are taken from
//! 'Prediction, Learning, and Games' (2006) by Cesa-Bianchi and Lugosi.
// #![cfg_attr(not(test), no_std)]
#![warn(missing_docs)]

/// Routines for continual prediction with expert advice.
pub mod forecaster;
pub mod loss;
