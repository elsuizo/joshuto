use globset::Glob;

use crate::context::JoshutoContext;
use crate::error::{JoshutoError, JoshutoErrorKind, JoshutoResult};
use crate::util::select::SelectOption;

use super::cursor_move;

pub fn select_files(
    context: &mut JoshutoContext,
    pattern: &str,
    options: &SelectOption,
) -> JoshutoResult<()> {
    if pattern.is_empty() {
        select_without_pattern(context, options)
    } else {
        select_with_pattern(context, pattern, options)
    }
}

fn select_without_pattern(
    context: &mut JoshutoContext,
    options: &SelectOption,
) -> JoshutoResult<()> {
    if options.all {
        if let Some(curr_list) = context.tab_context_mut().curr_tab_mut().curr_list_mut() {
            curr_list.iter_mut().for_each(|e| {
                if options.reverse {
                    e.set_selected(false);
                } else if options.toggle {
                    e.set_selected(!e.is_selected());
                } else {
                    e.set_selected(true);
                }
            });
        }
    } else if let Some(entry) = context
        .tab_context_mut()
        .curr_tab_mut()
        .curr_list_mut()
        .and_then(|s| s.curr_entry_mut())
    {
        if options.reverse {
            entry.set_selected(false);
        } else if options.toggle {
            entry.set_selected(!entry.is_selected());
        } else {
            entry.set_selected(true);
        }
        cursor_move::down(context, 1)?;
    }
    Ok(())
}

fn select_with_pattern(
    context: &mut JoshutoContext,
    pattern: &str,
    options: &SelectOption,
) -> JoshutoResult<()> {
    let glob = match Glob::new(pattern) {
        Ok(s) => s.compile_matcher(),
        Err(e) => {
            return Err(JoshutoError::new(
                JoshutoErrorKind::IoInvalidData,
                "Invalid glob input".to_string(),
            ));
        }
    };
    if let Some(curr_list) = context.tab_context_mut().curr_tab_mut().curr_list_mut() {
        curr_list
            .iter_mut()
            .filter(|e| glob.is_match(e.file_name()))
            .for_each(|e| {
                if options.reverse {
                    e.set_selected(false);
                } else if options.toggle {
                    e.set_selected(!e.is_selected());
                } else {
                    e.set_selected(true);
                }
            });
    }
    Ok(())
}
