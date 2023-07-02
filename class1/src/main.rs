mod module1;

use module1::module1::module_a_bz;
use module1::module2::module2::module_ba_z;

fn main() {
    module_a_bz::print_a_to_bz();
    println!();
    module_ba_z::print_ba_to_z();
}
