use input_fsm::{InputFSM};
use table::StatsTable;

pub mod input_fsm;
pub mod table;

const LINE_CAPACITY: usize = 50;
const TOP: usize = 5;

fn main() {
    let mut fsm = InputFSM::new();
    let mut tab: StatsTable = StatsTable::new();

    loop {
        if let Some(command) = fsm.get() {
            tab.execute_command(command);
            table::redraw_top(&tab, TOP);
        }
    }
}