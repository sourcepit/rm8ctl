#[macro_use]
extern crate clap;
#[macro_use]
extern crate common_failures;
#[macro_use]
extern crate failure;
extern crate libc;
#[macro_use]
extern crate log;

mod gpio;
mod rm8;

use common_failures::prelude::*;

use clap::App;
use clap::Arg;
use clap::SubCommand;
use rm8::Relay::Relay1;
use rm8::Relay::Relay2;
use rm8::Relay::Relay3;
use rm8::Relay::Relay4;
use rm8::Relay::Relay5;
use rm8::Relay::Relay6;
use rm8::Relay::Relay7;
use rm8::Relay::Relay8;
use rm8::RelayState::Off;
use rm8::Rm8Control;

const ARG_GPIO_PINS: &str = "gpio-pins";
const ARG_INVERT_OUTPUTS: &str = "invert-outputs";
const ARG_VERBOSITY: &str = "verbosity";
const ARG_QUIET: &str = "quiet";

quick_main!(run);

fn run() -> Result<()> {
    let args = App::new(crate_name!())
        .version(crate_version!())
        .author(crate_authors!())
        .arg(
            Arg::with_name(ARG_VERBOSITY)
                .long(ARG_VERBOSITY)
                .short("v")
                .multiple(true)
                .takes_value(false)
                .required(false),
        )
        .arg(
            Arg::with_name(ARG_QUIET)
                .long(ARG_QUIET)
                .short("q")
                .multiple(false)
                .takes_value(false)
                .required(false),
        )
        .arg(
            Arg::with_name(ARG_GPIO_PINS)
                .long(ARG_GPIO_PINS)
                .multiple(true)
                .takes_value(true)
                .required(false),
        )
        .arg(
            Arg::with_name(ARG_INVERT_OUTPUTS)
                .long(ARG_INVERT_OUTPUTS)
                .multiple(false)
                .takes_value(true)
                .required(false)
                .default_value("true"),
        )
        .subcommand(SubCommand::with_name("off"))
        .get_matches();

    let verbosity = args.occurrences_of(ARG_VERBOSITY) as usize + 1;
    let quiet = args.is_present(ARG_QUIET);

    stderrlog::new()
        .module(module_path!())
        .timestamp(stderrlog::Timestamp::Second)
        .verbosity(verbosity)
        .quiet(quiet)
        .init()?;

    let gpio_pins = match args.values_of(ARG_GPIO_PINS) {
        Some(values) => values
            .map(|i| i.parse::<usize>().unwrap())
            .collect::<Vec<usize>>(),
        None => vec![6, 13, 19, 26, 12, 16, 20, 21],
    };

    let invert_outputs = value_t!(args, ARG_INVERT_OUTPUTS, bool)?;

    match args.subcommand_name() {
        Some("off") => {
            info!("Set all relais to off");
            let mut rc = Rm8Control::open(gpio_pins, invert_outputs)?;
            rc.send(&Relay1, Off);
            rc.send(&Relay2, Off);
            rc.send(&Relay3, Off);
            rc.send(&Relay4, Off);
            rc.send(&Relay5, Off);
            rc.send(&Relay6, Off);
            rc.send(&Relay7, Off);
            rc.send(&Relay8, Off);
        }
        _ => {}
    };

    Ok(())
}
