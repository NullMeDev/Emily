use payment_processor::{Card, PaymentStatus};
use tracing::{info, Level};
use tracing_subscriber;

#[tokio::main]
async fn main() {
    // Initialize logging
    tracing_subscriber::fmt()
        .with_max_level(Level::INFO)
        .init();

    info!("Payment Processor - Rust Edition");
    info!("Phase 1: Core Infrastructure - Complete ✅");
    
    // Test card parsing
    let test_card = "4532123456789012|12|25|123";
    match Card::from_string(test_card) {
        Ok(card) => {
            info!("✅ Card parsed successfully:");
            info!("   BIN: {}", card.bin());
            info!("   Masked: {}", card.masked());
            info!("   Last 4: {}", card.last4());
        }
        Err(e) => {
            info!("❌ Card parsing failed: {}", e);
        }
    }

    // Test payment status
    info!("\n📊 Payment Status Examples:");
    let statuses = vec![
        PaymentStatus::Charged,
        PaymentStatus::CcnLive,
        PaymentStatus::ThreeDsRequired,
        PaymentStatus::InsufficientFunds,
        PaymentStatus::Declined,
    ];

    for status in statuses {
        let notify = if status.should_notify() { "✅ NOTIFY" } else { "❌ SKIP" };
        info!("   {} {} - {}", status.icon(), status.title(), notify);
    }

    info!("\n🎯 Next Steps:");
    info!("   Phase 2: Gateway Core (traits, token manager, response parser)");
    info!("   Phase 3: Gateway Implementations (7 gateways)");
    info!("   Phase 4: Telegram Bot Integration");
}
