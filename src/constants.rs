//! Constants definitions module

/// Display-related constants
pub mod display {
    /// Maximum number of rows to show in inline diff
    pub const MAX_INLINE_DIFF_ROWS: usize = 15;

    /// Maximum number of history entries to keep
    pub const MAX_HISTORY_SIZE: usize = 100;

    /// Number of spaces to clear prompt line
    pub const PROMPT_CLEAR_WIDTH: usize = 60;
}

/// Database-related constants
pub mod db {
    /// Maximum number of rows to fetch per table
    pub const MAX_ROWS_PER_TABLE: usize = 1000;

    /// Maximum number of debounce iterations
    pub const DEBOUNCE_MAX_ITERATIONS: usize = 5;

    /// Debounce wait interval (milliseconds)
    pub const DEBOUNCE_INTERVAL_MS: u64 = 100;
}

/// Default values
pub mod defaults {
    /// Default polling interval (milliseconds)
    pub const POLLING_INTERVAL_MS: u64 = 1000;

    /// Default host
    pub const HOST: &str = "localhost";

    /// Default port
    pub const PORT: u16 = 5432;

    /// Default user
    pub const USER: &str = "postgres";

    /// Default schema
    pub const SCHEMA: &str = "public";
}

/// Preset configurations
pub mod presets {
    /// Supabase local port
    pub const SUPABASE_PORT: u16 = 54322;

    /// Supabase local database name
    pub const SUPABASE_DATABASE: &str = "postgres";

    /// Supabase local user
    pub const SUPABASE_USER: &str = "postgres";

    /// Supabase local password
    pub const SUPABASE_PASSWORD: &str = "postgres";
}

/// Input channel buffer size
pub const INPUT_CHANNEL_BUFFER: usize = 10;
