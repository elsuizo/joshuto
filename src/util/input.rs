use signal_hook::consts::signal;
use termion::event::{MouseButton, MouseEvent};
use tui::layout::{Constraint, Direction, Layout};

use crate::commands::{cursor_move, parent_cursor_move, JoshutoRunnable, KeyCommand};
use crate::context::JoshutoContext;
use crate::history::DirectoryHistory;
use crate::io::{FileOp, IoWorkerProgress};
use crate::ui;
use crate::util::event::JoshutoEvent;
use crate::util::format;

pub fn process_mouse(
    event: MouseEvent,
    context: &mut JoshutoContext,
    backend: &mut ui::TuiBackend,
) {
    let f_size = backend.terminal.as_ref().unwrap().size().unwrap();

    let constraints: &[Constraint; 3] = &context.config_ref().default_layout;
    let layout_rect = Layout::default()
        .direction(Direction::Horizontal)
        .vertical_margin(1)
        .constraints(constraints.as_ref())
        .split(f_size);

    match event {
        MouseEvent::Press(MouseButton::WheelUp, x, _) => {
            if x < layout_rect[1].x {
                let command = KeyCommand::ParentCursorMoveUp(1);
                if let Err(e) = command.execute(context, backend) {
                    context.push_msg(e.to_string());
                }
            } else if x < layout_rect[2].x {
                let command = KeyCommand::CursorMoveUp(1);
                if let Err(e) = command.execute(context, backend) {
                    context.push_msg(e.to_string());
                }
            } else {
                // TODO: scroll in child list
                let command = KeyCommand::CursorMoveUp(1);
                if let Err(e) = command.execute(context, backend) {
                    context.push_msg(e.to_string());
                }
            }
        }
        MouseEvent::Press(MouseButton::WheelDown, x, _) => {
            if x < layout_rect[1].x {
                let command = KeyCommand::ParentCursorMoveDown(1);
                if let Err(e) = command.execute(context, backend) {
                    context.push_msg(e.to_string());
                }
            } else if x < layout_rect[2].x {
                let command = KeyCommand::CursorMoveDown(1);
                if let Err(e) = command.execute(context, backend) {
                    context.push_msg(e.to_string());
                }
            } else {
                // TODO: scroll in child list
                let command = KeyCommand::CursorMoveDown(1);
                if let Err(e) = command.execute(context, backend) {
                    context.push_msg(e.to_string());
                }
            }
        }
        MouseEvent::Press(MouseButton::Left, x, y)
            if y > layout_rect[1].y && y <= layout_rect[1].y + layout_rect[1].height =>
        {
            if x < layout_rect[1].x {
                if let Some(dirlist) = context.tab_context_ref().curr_tab_ref().curr_list_ref() {
                    if let Some(curr_index) = dirlist.index {
                        let skip_dist = curr_index / layout_rect[1].height as usize
                            * layout_rect[1].height as usize;

                        let new_index = skip_dist + (y - layout_rect[1].y - 1) as usize;
                        if let Err(e) = parent_cursor_move::parent_cursor_move(new_index, context) {
                            context.push_msg(e.to_string());
                        }
                    }
                }
            } else if x < layout_rect[2].x {
                if let Some(dirlist) = context.tab_context_ref().curr_tab_ref().curr_list_ref() {
                    if let Some(curr_index) = dirlist.index {
                        let skip_dist = curr_index / layout_rect[1].height as usize
                            * layout_rect[1].height as usize;

                        let new_index = skip_dist + (y - layout_rect[1].y - 1) as usize;
                        if let Err(e) = cursor_move::cursor_move(new_index, context) {
                            context.push_msg(e.to_string());
                        }
                    }
                }
            } else {
            }
        }
        MouseEvent::Press(MouseButton::Left, x, y)
            if y > layout_rect[1].y && y <= layout_rect[1].y + layout_rect[1].height => {}
        _ => {}
    }
    context.flush_event();
}

pub fn process_noninteractive(event: JoshutoEvent, context: &mut JoshutoContext) {
    match event {
        JoshutoEvent::IoWorkerProgress(res) => process_worker_progress(context, res),
        JoshutoEvent::IoWorkerResult(res) => process_finished_worker(context, res),
        JoshutoEvent::Signal(signal::SIGWINCH) => {}
        _ => {}
    }
}

pub fn process_worker_progress(context: &mut JoshutoContext, res: IoWorkerProgress) {
    context.set_worker_progress(res);
    context.update_worker_msg();
}

pub fn process_finished_worker(
    context: &mut JoshutoContext,
    res: std::io::Result<IoWorkerProgress>,
) {
    let observer = context.remove_job().unwrap();
    let options = context.config_ref().sort_option.clone();
    for tab in context.tab_context_mut().iter_mut() {
        let _ = tab.history_mut().reload(observer.dest_path(), &options);
        let _ = tab.history_mut().reload(observer.src_path(), &options);
    }
    observer.join();
    match res {
        Ok(progress) => {
            let op = match progress.kind() {
                FileOp::Copy => "copied",
                FileOp::Cut => "moved",
            };
            let size_str = format::file_size_to_string(progress.bytes_processed());
            let msg = format!(
                "successfully {} {} items ({})",
                op,
                progress.len(),
                size_str
            );
            context.push_msg(msg);
        }
        Err(e) => {
            let msg = format!("{}", e);
            context.push_msg(msg);
        }
    }
}
