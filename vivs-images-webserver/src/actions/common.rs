use std::rc::Rc;

use crate::actions::action_indicator::IActionIndicator;
use crate::actions::sql_db_action_indicators;


pub fn get_all_action_indicators() -> Vec<Rc<dyn IActionIndicator>> {
    sql_db_action_indicators::get_sql_db_action_indicators()
}