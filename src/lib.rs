
pub mod windows {
    winrt::import! {
        dependencies
            os
        types
            windows::devices::bluetooth::*
    }
    pub use windows::*;
}
#[cfg(test)]
mod tests {
    #[test]
    fn init_test() -> Result<(), winrt::Error> {
        super::windows::devices::bluetooth::BluetoothAdapter::get_default_async()?.get_results()?;
        Ok(())
    }
}