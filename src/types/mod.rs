pub mod payload;
pub mod webhook;

// Re-export dos tipos principais para conveniência
pub use payload::{BotContext, ChatGuruPayload, EventData, EventTypePayload, GenericPayload};

pub use webhook::WebhookPayload;
