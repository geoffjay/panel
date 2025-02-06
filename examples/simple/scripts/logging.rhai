// This is meant to be implemented using a combination of:
// - https://github.com/rhaiscript/rhai/tree/main/examples/event_handler_js
// - https://rhai.rs/book/rust/functions.html
//
// The idea would be that a UI element could bind to these functions as
//
// {
//   "variables": { "log_location": ... },
//   "dashboard": {
//     "widgets": {
//       "btn_start_logging": {
//         "type": "button",
//         "label": "Start",
//         "on_click": {
//           "bind": "on_start_clicked",
//           "args": {
//             "logger_id": "csv_logger",
//             "location": "${log_location}"
//           }
//         }
//       }
//     }
//   }
// }
//

fn on_start_clicked(logger_id, location) {
  start_logging(logger_id, location);
}

fn on_stop_clicked(logger_id) {
  stop_logging(logger_id);
}
