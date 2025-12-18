pub mod traits;

pub use traits::{PaymentGateway, GatewayInfo, GatewayRegistry};

// Gateway implementations will be added in Phase 3
// This module will contain:
// - blemart.rs (Charge1)
// - district_people.rs (Charge2)
// - saint_vinson.rs (Charge3)
// - bgd_fresh_milk.rs (Charge4)
// - staleks_florida.rs (Charge5)
// - braintree.rs
// - cc_foundation.rs (Sky.py)
