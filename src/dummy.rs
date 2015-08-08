use super::EnergyMonitor;

#[derive(Copy, Clone, Debug)]
pub struct DummyEnergyMonitor;

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

impl Default for DummyEnergyMonitor {
    fn default() -> DummyEnergyMonitor {
        DummyEnergyMonitor
    }
}

#[cfg(test)]
mod test {
    use super::DummyEnergyMonitor;
    use super::super::EnergyMonitor;

    #[test]
    fn test() {
        let em = DummyEnergyMonitor::default();
        assert!(em.interval_us() == 1);
        assert!(em.read_uj().unwrap() == 0);
    }
}
