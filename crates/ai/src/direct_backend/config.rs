//! User-facing configuration for the warp-cn Direct LLM backend.
//!
//! Decouples per-provider endpoint + model overrides from `ApiKeyManager`
//! (which only stores the secret key) so the secure-storage payload of API
//! keys is unchanged from upstream.

use serde::{Deserialize, Serialize};
use warpui::{Entity, ModelContext, SingletonEntity};
use warpui_extras::secure_storage::{self, AppContextExt};

const SECURE_STORAGE_KEY: &str = "DirectBackendConfig";

/// Supported provider protocols. Names match the upstream `LLMProvider`
/// variants where possible so future merges of provider-specific UI hooks stay
/// trivial.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
pub enum DirectProviderKind {
    #[default]
    OpenAi,
    Anthropic,
    Gemini,
    /// OpenAI-wire-compatible gateways (vLLM, LiteLLM, DeepSeek, Qwen, Ollama,
    /// OpenRouter, …). Routed through the OpenAI client with a different
    /// `base_url`.
    OpenAiCompatible,
}

/// Per-provider override of base URL and the model id sent on the wire.
///
/// Empty `base_url` means "use the provider default"; empty `model_id` means
/// "fall back to whatever the request layer already chose".
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct ProviderOverrides {
    #[serde(default)]
    pub base_url: String,
    #[serde(default)]
    pub model_id: String,
}

/// Persisted Direct-backend state.
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct DirectBackendState {
    #[serde(default)]
    pub enabled: bool,
    #[serde(default)]
    pub active: DirectProviderKind,
    #[serde(default)]
    pub openai: ProviderOverrides,
    #[serde(default)]
    pub anthropic: ProviderOverrides,
    #[serde(default)]
    pub gemini: ProviderOverrides,
    #[serde(default)]
    pub openai_compatible: ProviderOverrides,
}

/// Emitted when overrides change so dependent views can refresh.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DirectBackendConfigEvent {
    Updated,
}

/// Singleton holding the persisted Direct-backend state.
pub struct DirectBackendConfig {
    state: DirectBackendState,
}

impl DirectBackendConfig {
    pub fn new(ctx: &mut ModelContext<Self>) -> Self {
        Self {
            state: load_from_secure_storage(ctx),
        }
    }

    pub fn state(&self) -> &DirectBackendState {
        &self.state
    }

    pub fn is_enabled(&self) -> bool {
        self.state.enabled
    }

    pub fn active_provider(&self) -> DirectProviderKind {
        self.state.active
    }

    pub fn overrides_for(&self, kind: DirectProviderKind) -> &ProviderOverrides {
        match kind {
            DirectProviderKind::OpenAi => &self.state.openai,
            DirectProviderKind::Anthropic => &self.state.anthropic,
            DirectProviderKind::Gemini => &self.state.gemini,
            DirectProviderKind::OpenAiCompatible => &self.state.openai_compatible,
        }
    }

    pub fn set_enabled(&mut self, enabled: bool, ctx: &mut ModelContext<Self>) {
        if self.state.enabled == enabled {
            return;
        }
        self.state.enabled = enabled;
        self.persist(ctx);
    }

    pub fn set_active(&mut self, kind: DirectProviderKind, ctx: &mut ModelContext<Self>) {
        if self.state.active == kind {
            return;
        }
        self.state.active = kind;
        self.persist(ctx);
    }

    pub fn set_overrides(
        &mut self,
        kind: DirectProviderKind,
        overrides: ProviderOverrides,
        ctx: &mut ModelContext<Self>,
    ) {
        let slot = match kind {
            DirectProviderKind::OpenAi => &mut self.state.openai,
            DirectProviderKind::Anthropic => &mut self.state.anthropic,
            DirectProviderKind::Gemini => &mut self.state.gemini,
            DirectProviderKind::OpenAiCompatible => &mut self.state.openai_compatible,
        };
        if *slot == overrides {
            return;
        }
        *slot = overrides;
        self.persist(ctx);
    }

    fn persist(&self, ctx: &mut ModelContext<Self>) {
        let serialized = match serde_json::to_string(&self.state) {
            Ok(s) => s,
            Err(e) => {
                log::error!("Failed to serialize DirectBackendConfig: {e:#}");
                return;
            }
        };
        if let Err(e) = ctx
            .secure_storage()
            .write_value(SECURE_STORAGE_KEY, &serialized)
        {
            log::error!("Failed to persist DirectBackendConfig: {e:#}");
        }
        ctx.emit(DirectBackendConfigEvent::Updated);
    }
}

fn load_from_secure_storage(ctx: &mut ModelContext<DirectBackendConfig>) -> DirectBackendState {
    let json = match ctx.secure_storage().read_value(SECURE_STORAGE_KEY) {
        Ok(json) => json,
        Err(secure_storage::Error::NotFound) => return DirectBackendState::default(),
        Err(e) => {
            log::error!("Failed to read DirectBackendConfig: {e:#}");
            return DirectBackendState::default();
        }
    };
    serde_json::from_str(&json).unwrap_or_else(|e| {
        log::error!("Failed to deserialize DirectBackendConfig: {e:#}");
        DirectBackendState::default()
    })
}

impl Entity for DirectBackendConfig {
    type Event = DirectBackendConfigEvent;
}

impl SingletonEntity for DirectBackendConfig {}
