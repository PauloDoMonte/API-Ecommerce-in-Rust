// src/models/shipping_management/tracking.rs

use chrono::{DateTime, Utc};

/// Representa o rastreamento de um envio.
#[derive(Debug, Clone)]
pub struct Tracking {
    pub tracking_number: String,
    pub status: TrackingStatus,
    pub updates: Vec<TrackingUpdate>, // Lista de atualizações de rastreamento
}

#[derive(Debug, Clone)]
pub enum TrackingStatus {
    InTransit,
    Delivered,
    OutForDelivery,
    Delayed,
    Returned,
}

#[derive(Debug, Clone)]
pub struct TrackingUpdate {
    pub timestamp: DateTime<Utc>,
    pub location: String,
    pub message: String,
}

impl Tracking {
    /// Cria um novo rastreamento.
    pub fn new(tracking_number: String, status: TrackingStatus) -> Self {
        Self {
            tracking_number,
            status,
            updates: Vec::new(),
        }
    }

    /// Adiciona uma atualização ao rastreamento.
    pub fn add_update(&mut self, location: String, message: String) {
        let update = TrackingUpdate {
            timestamp: Utc::now(),
            location,
            message,
        };
        self.updates.push(update);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tracking_creation() {
        let mut tracking = Tracking::new("TRACK123".to_string(), TrackingStatus::InTransit);
        assert_eq!(tracking.tracking_number, "TRACK123");
        assert_eq!(tracking.status, TrackingStatus::InTransit);
        assert!(tracking.updates.is_empty());

        tracking.add_update("Centro de Distribuição".to_string(), "Pacote saiu para entrega.".to_string());
        assert_eq!(tracking.updates.len(), 1);
        assert_eq!(tracking.updates[0].message, "Pacote saiu para entrega.");
    }
}