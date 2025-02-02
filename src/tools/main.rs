use structopt::StructOpt;

use task_maker_rust::error::NiceError;
use task_maker_rust::tools::add_solution_checks::main_add_solution_checks;
use task_maker_rust::tools::booklet::main_booklet;
use task_maker_rust::tools::clear::main_clear;
use task_maker_rust::tools::fuzz_checker::main_fuzz_checker;
use task_maker_rust::tools::gen_autocompletion::main_get_autocompletion;
use task_maker_rust::tools::opt::{Opt, Tool};
use task_maker_rust::tools::reset::main_reset;
use task_maker_rust::tools::sandbox::main_sandbox;
use task_maker_rust::tools::server::main_server;
use task_maker_rust::tools::task_info::main_task_info;
use task_maker_rust::tools::typescriptify::main_typescriptify;
use task_maker_rust::tools::worker::main_worker;

fn main() {
    let base_opt = Opt::from_args();
    base_opt.logger.enable_log();

    match base_opt.tool {
        Tool::Clear(opt) => main_clear(opt),
        Tool::GenAutocompletion(opt) => main_get_autocompletion(opt),
        Tool::Server(opt) => main_server(opt),
        Tool::Worker(opt) => main_worker(opt),
        Tool::Typescriptify => main_typescriptify(),
        Tool::Reset(opt) => main_reset(opt),
        Tool::Sandbox(opt) => main_sandbox(opt),
        Tool::TaskInfo(opt) => main_task_info(opt),
        Tool::Booklet(opt) => main_booklet(opt, base_opt.logger),
        Tool::FuzzChecker(opt) => main_fuzz_checker(opt),
        Tool::AddSolutionChecks(opt) => main_add_solution_checks(opt, base_opt.logger),
        Tool::InternalSandbox => return task_maker_rust::main_sandbox(),
    }
    .nice_unwrap()
}
