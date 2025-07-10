use crate::fast_30::flags_options::Fast30Option;

const DEFAULT_ELUDE_DIAGONAL_STEPS: usize = 10;
const DEFAULT_SAVE_STEPS: usize = 100_000;
const DEFAULT_SAVE_FILE_PATH: &str = "output/diagonal.txt";
const DEFAULT_LOGGING_STEPS: usize = 1_000_000;

pub(crate) struct Options {
    pub(crate) elude_diagonal_steps: usize,

    pub(crate) want_to_save: bool,
    pub(crate) save_steps: usize,
    pub(crate) path_to_save_file: String,
    pub(crate) start_time: std::time::Instant,
    pub(crate) last_save: Option<std::time::Instant>,

    pub(crate) want_log_doubling: bool,

    pub(crate) want_log_steps: bool,
    pub(crate) logging_steps: usize,
}

impl Options {
    pub(crate) fn new() -> Self {
        Self {
            elude_diagonal_steps: DEFAULT_ELUDE_DIAGONAL_STEPS,

            want_to_save: false,
            save_steps: DEFAULT_SAVE_STEPS,
            path_to_save_file: DEFAULT_SAVE_FILE_PATH.to_string(),
            start_time: std::time::Instant::now(),
            last_save: None,

            want_log_doubling: false,

            want_log_steps: false,
            logging_steps: DEFAULT_LOGGING_STEPS,
        }
    }

    pub fn set_options(&mut self, opts: Fast30Option) {
        self.want_to_save = opts.contains(Fast30Option::SAVE_DIAGONALS);
        self.want_log_steps = opts.contains(Fast30Option::LOG_STEPS);
        self.want_log_doubling = opts.contains(Fast30Option::LOG_DOUBLING);
    }
}