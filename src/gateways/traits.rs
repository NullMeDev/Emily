use crate::models::{Card, PaymentResponse};
use async_trait::async_trait;
use anyhow::Result;

/// Gateway information
#[derive(Debug, Clone)]
pub struct GatewayInfo {
    pub name: &'static str,
    pub site: &'static str,
    pub amount: &'static str,
    pub currency: &'static str,
}

/// Trait that all payment gateways must implement
#[async_trait]
pub trait PaymentGateway: Send + Sync {
    /// Get gateway information
    fn info(&self) -> GatewayInfo;

    /// Process a payment with the given card
    async fn process_payment(&self, card: &Card) -> Result<PaymentResponse>;

    /// Get the gateway name
    fn name(&self) -> &'static str {
        self.info().name
    }

    /// Get the target site
    fn site(&self) -> &'static str {
        self.info().site
    }

    /// Get the charge amount
    fn amount(&self) -> &'static str {
        self.info().amount
    }
}

/// Gateway registry for managing multiple gateways
pub struct GatewayRegistry {
    gateways: Vec<Box<dyn PaymentGateway>>,
}

impl GatewayRegistry {
    pub fn new() -> Self {
        Self {
            gateways: Vec::new(),
        }
    }

    /// Register a new gateway
    pub fn register(&mut self, gateway: Box<dyn PaymentGateway>) {
        self.gateways.push(gateway);
    }

    /// Get a gateway by name
    pub fn get(&self, name: &str) -> Option<&dyn PaymentGateway> {
        self.gateways
            .iter()
            .find(|g| g.name() == name)
            .map(|g| g.as_ref())
    }

    /// Get all registered gateways
    pub fn all(&self) -> &[Box<dyn PaymentGateway>] {
        &self.gateways
    }

    /// Get gateway count
    pub fn count(&self) -> usize {
        self.gateways.len()
    }
}

impl Default for GatewayRegistry {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct MockGateway;

    #[async_trait]
    impl PaymentGateway for MockGateway {
        fn info(&self) -> GatewayInfo {
            GatewayInfo {
                name: "Mock Gateway",
                site: "mock.example.com",
                amount: "$1.00",
                currency: "USD",
            }
        }

        async fn process_payment(&self, _card: &Card) -> Result<PaymentResponse> {
            Ok(PaymentResponse::charged("Mock payment successful".to_string()))
        }
    }

    #[test]
    fn test_gateway_registry() {
        let mut registry = GatewayRegistry::new();
        registry.register(Box::new(MockGateway));
        
        assert_eq!(registry.count(), 1);
        assert!(registry.get("Mock Gateway").is_some());
        assert!(registry.get("Nonexistent").is_none());
    }
}
