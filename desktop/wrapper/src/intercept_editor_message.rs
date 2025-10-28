use graphite_editor::messages::prelude::InputPreprocessorMessage;

use super::DesktopWrapperMessageDispatcher;
use super::messages::{DesktopFrontendMessage, EditorMessage};

pub(super) fn intercept_editor_message(dispatcher: &mut DesktopWrapperMessageDispatcher, message: EditorMessage) -> Option<EditorMessage> {
	match message {
		EditorMessage::InputPreprocessor(InputPreprocessorMessage::UpdateViewportInfo { bounds, scale }) => {
			let top_left = bounds.top_left;
			let bottom_right = bounds.bottom_right;
			dispatcher.respond(DesktopFrontendMessage::UpdateViewportInfo {
				x: top_left.x,
				y: top_left.y,
				width: (bottom_right.x - top_left.x),
				height: (bottom_right.y - top_left.y),
				scale,
			});
			None
		}
		m => Some(m),
	}
}
