use core::fmt::Display;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct MacAddress([u8; 6]);

impl Display for MacAddress {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        for x in self.0 {
            write!(f, "{:02X}", x)?;

            // Print a : between each component.
            if self.0.last().map(|it| *it != x).unwrap_or(false) {
                write!(f, ":")?;
            }
        }

        Ok(())
    }
}
