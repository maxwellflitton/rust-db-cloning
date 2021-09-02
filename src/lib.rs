use pyo3::prelude::*;
use pyo3::wrap_pyfunction;
use pyo3::types::PyDict;

mod database;
mod schema;
mod models;

use database::establish_connection;
use models::FibEntry;
use schema::fib_entries;


pub fn return_state(user_id: &i32) -> ToDoItems {
    let connection = establish_connection();

    let items = to_do::table
        .order(to_do::columns::id.asc())
        .filter(to_do::columns::user_id.eq(&user_id))
        .load::<Item>(&connection)
        .unwrap();

    let mut array_buffer = Vec::new();

    for item in items {
        let item = to_do_factory(&item.status, item.title).unwrap();
        array_buffer.push(item);
    }
    return ToDoItems::new(array_buffer);
}


#[pyfunction]
fn get_fib_enteries(py: Python) -> Vec<&PyDict> {

   let connection = establish_connection();  

   let fib_enteries = fib_entries::table
       .order(fib_entries::columns::input_number.asc())
       .load::<FibEntry>(&connection)
       .unwrap();
   
   let mut buffer = Vec::new();

   for i in fib_enteries {
       let placeholder = PyDict::new(py);
       placeholder.set_item("input number", i.input_number);
       placeholder.set_item("fib number", i.calculated_number);
       buffer.push(placeholder);
   }
   return buffer
}

#[pymodule]
fn flitton_oasis_risk_modelling(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_wrapped(wrap_pyfunction!(get_fib_enteries));
    Ok(())
}
