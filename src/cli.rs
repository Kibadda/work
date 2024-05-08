use crate::slot;

#[derive(Clone, Debug)]
pub struct Time {
    pub value: String,
}

impl std::fmt::Display for Time {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.value)
    }
}

impl std::str::FromStr for Time {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let reg = regex::Regex::new("^[0-9]{2}:[0-9]{2}$").unwrap();
        match s == "now" || reg.is_match(s) {
            true => Ok(Self {
                value: s.parse().map_err(|e| format!("{e}"))?,
            }),
            false => Err("wrong format".to_owned()),
        }
    }
}

#[derive(Clone)]
pub struct Slot {
    pub value: Option<String>,
}

impl std::fmt::Display for Slot {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match &self.value {
                Some(value) => value,
                None => "",
            }
        )
    }
}

impl std::str::FromStr for Slot {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.is_empty() || s == "start1" || s == "end1" || s == "start2" || s == "end2" {
            Ok(Self {
                value: Some(s.to_owned()),
            })
        } else {
            Err("invalid slot".to_owned())
        }
    }
}

#[derive(clap::Subcommand)]
pub enum Action {
    /// Print current times
    Show,

    /// Set next missing timeslot
    Track {
        /// Overwrite time in H:M format
        #[arg(
            short,
            long,
            value_name = "TIME",
            value_parser = clap::value_parser!(Time),
            default_value_t = Time { value: "now".to_owned() }
        )]
        time: Time,
    },

    /// Update timeslot
    Update {
        /// Overwrite time in H:M format
        #[arg(
            short,
            long,
            value_name = "TIME",
            value_parser = clap::value_parser!(Time),
            default_value_t = Time { value: "now".to_owned() }
        )]
        time: Time,

        /// Overwrite slot
        #[arg(
            short,
            long,
            value_name = "SLOT",
            value_parser = clap::value_parser!(Slot),
            default_value_t = Slot { value: slot::last() }
        )]
        slot: Slot,
    },
}

#[derive(clap::Parser)]
#[command(arg_required_else_help = true)]
pub struct Cli {
    #[command(subcommand)]
    pub action: Option<Action>,
}
