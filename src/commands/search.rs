use globset::{GlobBuilder, GlobMatcher};

use crate::context::JoshutoContext;
use crate::error::{JoshutoError, JoshutoErrorKind, JoshutoResult};
use crate::tab::JoshutoTab;

use super::cursor_move;
use super::search_glob;
use super::search_string;

#[derive(Clone, Debug)]
pub enum SearchPattern {
    Glob(GlobMatcher),
    String(String),
}

pub fn search_next(context: &mut JoshutoContext) -> JoshutoResult<()> {
    if let Some(s) = context.get_search_state() {
        let index = match s {
            SearchPattern::Glob(s) => {
                search_glob::search_glob_fwd(context.tab_context_ref().curr_tab_ref(), s)
            }
            SearchPattern::String(s) => {
                search_string::search_string_fwd(context.tab_context_ref().curr_tab_ref(), s)
            }
        };
        if let Some(index) = index {
            let _ = cursor_move::cursor_move(index, context);
        }
    }
    Ok(())
}

pub fn search_prev(context: &mut JoshutoContext) -> JoshutoResult<()> {
    if let Some(s) = context.get_search_state() {
        let index = match s {
            SearchPattern::Glob(s) => {
                search_glob::search_glob_fwd(context.tab_context_ref().curr_tab_ref(), s)
            }
            SearchPattern::String(s) => {
                search_string::search_string_rev(context.tab_context_ref().curr_tab_ref(), s)
            }
        };
        if let Some(index) = index {
            let _ = cursor_move::cursor_move(index, context);
        }
    }
    Ok(())
}
