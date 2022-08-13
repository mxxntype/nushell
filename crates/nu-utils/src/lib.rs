pub mod locale;
pub mod utils;

pub use locale::get_system_locale;
pub use utils::{
    enable_vt_processing, get_default_config, get_default_env, get_ls_colors,
    stderr_write_all_and_flush, stdout_write_all_and_flush,
};
