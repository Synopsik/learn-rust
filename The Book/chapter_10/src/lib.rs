mod function_definitions;
mod method_definitions;
mod traits;

pub fn run_all() {
    function_definitions::run_function_definitions();
    method_definitions::run_method_definitions();
    traits::run_traits();
}





