//! Direct LLM backend — multi-agent (Agent Mode) entry point — M4.1 stub.
//!
//! M1-M3.5 wire single-LLM-call paths (chat panel, search, workflow descriptions,
//! drive AI assist) against the user's BYO API key. Agent Mode uses a different
//! tool-calling protocol that ships in M4 (M4.1 stub here, M4.2.1+ real tool
//! support layered on top in a follow-up commit).
//!
//! When Agent Mode is invoked while only the M1-M3.5 paths are wired, return a
//! sanitized `Finished{InternalError}` event so the UI surfaces a clear "not
//! yet supported" message rather than hanging on `StreamInit`.

use std::sync::Arc;

use futures::StreamExt;
use uuid::Uuid;
use warp_multi_agent_api as api;

use crate::server::server_api::{AIApiError, AIOutputStream};

const STUB_MESSAGE: &str = "Direct LLM mode is enabled (warp-cn fork). Agent Mode tool calling \
     isn't yet wired against your provider in this build — use the AI \
     Chat side panel or `#` natural-language commands while M4 follow-up \
     phases ship real tool support.";

pub fn run(request: &api::Request) -> AIOutputStream<api::ResponseEvent> {
    let conversation_id = request
        .metadata
        .as_ref()
        .map(|m| m.conversation_id.clone())
        .filter(|s| !s.is_empty())
        .unwrap_or_else(|| Uuid::new_v4().to_string());
    error_stream(conversation_id, STUB_MESSAGE.into())
}

fn error_stream(conversation_id: String, message: String) -> AIOutputStream<api::ResponseEvent> {
    let request_id = Uuid::new_v4().to_string();
    let init = api::ResponseEvent {
        r#type: Some(api::response_event::Type::Init(
            api::response_event::StreamInit {
                conversation_id,
                request_id,
                run_id: String::new(),
            },
        )),
    };
    let finished = api::ResponseEvent {
        r#type: Some(api::response_event::Type::Finished(
            api::response_event::StreamFinished {
                reason: Some(api::response_event::stream_finished::Reason::InternalError(
                    api::response_event::stream_finished::InternalError { message },
                )),
                ..Default::default()
            },
        )),
    };
    let stream = futures::stream::iter(
        vec![init, finished]
            .into_iter()
            .map(Ok::<_, Arc<AIApiError>>),
    );
    cfg_if::cfg_if! {
        if #[cfg(target_family = "wasm")] {
            stream.boxed_local()
        } else {
            stream.boxed()
        }
    }
}
