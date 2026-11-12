//! The [clash] binary

use yaffa::App as _;

fn main() -> yaffa::Result<()> {
    clash::App::run_main_default()
}
