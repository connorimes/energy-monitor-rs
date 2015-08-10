use super::EnergyMonitor;

#[derive(Copy, Clone, Debug)]
pub struct DummyEnergyMonitor;

pub const DUMMY_ENERGY_MONITOR: DummyEnergyMonitor = DummyEnergyMonitor;

impl EnergyMonitor for DummyEnergyMonitor {
    fn read_uj(&self) -> Result<u64, &'static str> {
        Ok(0)
    }

    fn source(&self) -> String {
        "Dummy Energy Monitor".to_owned()
    }

    fn interval_us(&self) -> u64 {
        1
    }
}

#[cfg(test)]
mod test {
    use super::DUMMY_ENERGY_MONITOR;
    use super::super::EnergyMonitor;

    #[test]
    fn test() {
        assert!(DUMMY_ENERGY_MONITOR.interval_us() == 1);
        assert!(DUMMY_ENERGY_MONITOR.read_uj().unwrap() == 0);
    }
}
