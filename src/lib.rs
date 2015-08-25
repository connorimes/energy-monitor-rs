//! This crate contains the EnergyMonitor trait.
//! A dummy implementation is included in the `dummy` module.

pub mod dummy;

/// An energy monitor typically reads from an internal or external sensor.
/// It could also estimate energy consumption using hardware counters or other heuristics.
pub trait EnergyMonitor {
    /// Read the energy value in microjoules
    fn read_uj(&self) -> Result<u64, &'static str>;

    /// Get the energy monitoring source as a String
    fn source(&self) -> String;

    /// Get the energy monitor's refresh interval in microseconds.
    /// This can be used as a minimum polling period.
    fn interval_us(&self) -> u64;
}
