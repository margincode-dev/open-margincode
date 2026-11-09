src/walk.rs:
  L27: enum ReceiverMode {
  L39: pub enum WorkerResult {
  L48: struct Batch {
  L53: fn new() -> Self {
  L59: fn lock(&self) -> MutexGuard<'_, Option<Vec<WorkerResult>>> {
  L65: type Item = WorkerResult;
  L66: type IntoIter = std::vec::IntoIter<WorkerResult>;
  L68: fn into_iter(self) -> Self::IntoIter {
  L74: struct BatchSender {
  L81: fn new(tx: Sender<Batch>, limit: usize) -> Self {
  L90: fn needs_flush(&self, batch: Option<&Vec<WorkerResult>>) -> bool {
  L100: fn send(&mut self, item: WorkerResult) -> Result<(), SendError<()>> {
  L124: const MAX_BUFFER_LENGTH: usize = 1000;
  L126: const DEFAULT_MAX_BUFFER_TIME: Duration = Duration::from_millis(100);
  L129: struct ReceiverBuffer<'a, W> {
  L152: fn new(state: &'a WorkerState, rx: Receiver<Batch>, stdout: W) -> Self {
  L173: fn process(&mut self) -> ExitCode {
  L183: fn recv(&self) -> Result<Batch, RecvTimeoutError> {
  L197: fn poll(&mut self) -> Result<(), ExitCode> {
  L251: fn print(&mut self, entry: &DirEntry) -> Result<(), ExitCode> {
  L269: fn stream(&mut self) -> Result<(), ExitCode> {
  L281: fn stop(&mut self) -> Result<(), ExitCode> {
  L295: fn flush(&mut self) -> Result<(), ExitCode> {
  L305: struct WorkerState {
  L317: fn new(patterns: Vec<Regex>, config: Config) -> Self {
  L329: fn build_overrides(&self, paths: &[PathBuf]) -> Result<Override> {
  L346: fn build_walker(&self, paths: &[PathBuf]) -> Result<WalkParallel> {
  L407: fn receive(&self, rx: Receiver<Batch>) -> ExitCode {
  L442: fn spawn_senders(&self, walker: WalkParallel, tx: Sender<Batch>) {
  L616: fn scan(&self, paths: &[PathBuf]) -> Result<ExitCode> {
  L661: fn is_broken_symlink(path: &Path, err: &ignore::Error) -> bool {
  L670: fn search_str_for_entry<'a>(
  L699: pub fn scan(paths: &[PathBuf], patterns: Vec<Regex>, config: Config) -> Result<ExitCode> {
  L709: fn search_str_for_entry_with_relative_path() {
  L718: fn search_str_for_entry_strips_dot_prefix() {
  L727: fn search_str_for_entry_with_absolute_path() {
  L736: fn search_str_no_base_dir() {
  L744: fn search_str_no_base_dir_with_plain_relative_path() {
  L752: fn search_str_no_base_dir_with_file_in_current_dir() {
src/exit_codes.rs:
  L26: fn is_error(self) -> bool {
  L63: fn general_error_if_at_least_one_error() {
  L7: pub enum ExitCode {
  L31: pub fn exit(self) -> ! {
  L87: fn success_if_no_error() {
  L15: fn from(code: ExitCode) -> Self {
  L46: pub fn merge_exitcodes(results: impl IntoIterator<Item = ExitCode>) -> ExitCode {
  L58: fn success_when_no_results() {
tests/testenv/mod.rs:
  L65: fn create_config_directory_with_global_ignore(ignore_file_content: &str) -> io::Result<TempDir> {
  L82: fn format_exit_error(args: &[&str], output: &process::Output) -> String {
  L92: fn format_output_error(args: &[&str], expected: &str, actual: &str) -> String {
  L169: pub fn global_ignore_file(self, content: &str) -> TestEnv {
  L296: pub fn assert_failure_with_error(&self, args: &[&str], expected: &str) {
  L312: pub fn assert_error(&self, args: &[&str], expected: &str) -> process::ExitStatus {
  L336: fn assert_error_subdirectory<P: AsRef<Path>>(
  L14: pub struct TestEnv {
  L29: fn create_working_directory(
  L76: fn find_fd_exe() -> PathBuf {
  L115: fn normalize_output(s: &str, trim_start: bool, normalize_line: bool) -> String {
  L137: fn trim_lines(s: &str) -> String {
  L148: pub fn new(directories: &[&'static str], files: &[&'static str]) -> TestEnv {
  L160: pub fn normalize_line(self, normalize: bool) -> TestEnv {
  L179: pub fn create_broken_symlink<P: AsRef<Path>>(
  L199: pub fn create_symlink<P: AsRef<Path>>(
  L216: pub fn test_root(&self) -> PathBuf {
  L222: pub fn test_exe(&self) -> &PathBuf {
  L227: pub fn system_root(&self) -> PathBuf {
  L234: pub fn assert_success_and_get_output<P: AsRef<Path>>(
  L250: pub fn assert_success_and_get_normalized_output<P: AsRef<Path>>(
  L264: pub fn assert_output(&self, args: &[&str], expected: &str) {
  L270: pub fn assert_output_raw(&self, args: &[&str], expected: &[u8]) {
  L278: pub fn assert_output_subdirectory<P: AsRef<Path>>(
  L304: pub fn assert_failure(&self, args: &[&str]) {
  L316: fn run_command(&self, path: &Path, args: &[&str]) -> process::Output {
tests/tests.rs:
  L244: fn test_and_plus_ignore_case() {
  L721: fn test_no_ignore() {
  L790: fn test_no_ignore_parent() {
  L822: fn test_no_ignore_parent_inner_git() {
  L862: fn test_custom_ignore_precedence() {
  L888: fn test_respect_ignore_files() {
  L944: fn test_no_ignore_vcs() {
  L961: fn test_no_ignore_vcs_child_dir() {
  L977: fn test_custom_ignore_files() {
  L996: fn test_no_ignore_aliases() {
  L1015: fn test_global_ignore() {
  L1049: fn test_no_global_ignore(flag: &str, expected_output: &str) {
  L2669: fn test_owner_ignore_all() {
  L2887: fn test_number_parsing_errors() {
  L2934: fn test_error_if_hidden_not_set_and_pattern_starts_with_dot() {
  L3061: fn test_ignore_contain() {
  L3087: fn test_ignore_contain_precedence_over_depth_check() {
  L3105: fn test_ignore_contain_precedence_over_root_check() {
  L3117: fn test_sanitize_exec_error_msg() {
  L17: static DEFAULT_DIRS: &[&str] = &["one/two/three", "one/two/three/directory_foo"];
  L19: static DEFAULT_FILES: &[&str] = &[
  L32: fn get_absolute_root_path(env: &TestEnv) -> String {
  L49: fn get_test_env_with_abs_path(dirs: &[&'static str], files: &[&'static str]) -> (TestEnv, String) {
  L56: fn create_file_with_size<P: AsRef<Path>>(path: P, size_in_bytes: usize) {
  L64: fn test_simple() {
  L82: static AND_EXTRA_FILES: &[&str] = &[
  L104: fn test_and_basic() {
  L131: fn test_and_empty_pattern() {
  L137: fn test_and_bad_pattern() {
  L147: fn test_and_pattern_starts_with_dash() {
  L167: fn test_and_plus_extension() {
  L200: fn test_and_plus_type() {
  L216: fn test_and_plus_glob() {
  L223: fn test_and_plus_fixed_strings() {
  L255: fn test_and_plus_case_sensitive() {
  L265: fn test_and_plus_full_path() {
  L295: fn test_empty_pattern() {
  L316: fn test_multi_file() {
  L340: fn test_multi_file_with_missing() {
  L383: fn test_pattern_with_forward_slash_is_rejected() {
  L410: fn test_pattern_with_forward_slash_allowed_with_full_path() {
  L425: fn test_and_pattern_with_forward_slash_is_rejected() {
  L447: fn test_explicit_root_path() {
  L488: fn test_single_dash_root_path() {
  L496: fn test_regex_searches() {
  L517: fn test_smart_case() {
  L541: fn test_case_sensitive() {
  L556: fn test_case_insensitive() {
  L574: fn test_glob_searches() {
  L604: fn test_full_path_glob_searches() {
  L626: fn test_smart_case_glob_searches() {
  L640: fn test_case_sensitive_glob_searches() {
  L648: fn test_glob_searches_with_extension() {
  L659: fn test_regex_overrides_glob() {
  L667: fn test_full_path() {
  L682: fn test_hidden() {
  L700: fn test_hidden_file_attribute() {
  L752: fn test_gitignore_and_fdignore() {
  L1056: fn test_follow() {
  L1074: fn test_file_system_boundaries() {
  L1112: fn test_follow_broken_symlink() {
  L1139: fn test_print0() {
  L1155: fn test_max_depth() {
  L1192: fn test_min_depth() {
  L1216: fn test_min_depth_broken_symlink() {
  L1254: fn test_min_depth_broken_symlink_absolute_path() {
  L1297: fn test_min_depth_broken_symlink_overlapping_roots() {
  L1340: fn test_max_depth_broken_symlink() {
  L1379: fn test_exact_depth_broken_symlink() {
  L1414: fn test_exact_depth() {
  L1427: fn test_prune() {
  L1461: fn test_absolute_path() {
  L1498: fn test_implicit_absolute_path() {
  L1517: fn test_normalized_absolute_path() {
  L1537: fn test_type() {
  L1575: fn test_type_executable() {
  L1617: fn test_type_empty() {
  L1644: fn test_extension() {
  L1701: fn test_no_extension() {
  L1729: fn test_symlink_as_root() {
  L1770: fn test_symlink_and_absolute_path() {
  L1791: fn test_symlink_as_absolute_root() {
  L1808: fn test_symlink_and_full_path() {
  L1833: fn test_symlink_and_full_path_abs_path() {
  L1853: fn test_excludes() {
  L1902: fn format() {
  L1974: fn test_exec() {
  L2058: fn test_exec_multi() {
  L2118: fn test_exec_nulls() {
  L2132: fn test_exec_batch() {
  L2209: fn test_exec_batch_multi() {
  L2277: fn test_exec_batch_with_limit() {
  L2315: fn test_exec_with_separator() {
  L2399: fn test_quiet() {
  L2412: fn test_fixed_strings() {
  L2448: fn test_exact_literal_nonsubstring() {
  L2486: fn test_invalid_utf8() {
  L2510: fn test_size() {
  L2588: fn create_file_with_modified<P: AsRef<Path>>(path: P, duration_in_secs: u64) {
  L2596: fn remove_symlink<P: AsRef<Path>>(path: P) {
  L2608: fn test_modified_relative() {
  L2641: fn change_file_modified<P: AsRef<Path>>(path: P, iso_date: &str) {
  L2651: fn test_modified_absolute() {
  L2677: fn test_owner_current_user() {
  L2688: fn test_owner_current_group() {
  L2699: fn test_owner_root() {
  L2712: fn test_custom_path_separator() {
  L2726: fn test_base_directory() {
  L2775: fn test_max_results() {
  L2818: fn test_exec_invalid_utf8() {
  L2856: fn test_list_details() {
  L2878: fn test_single_and_multithreaded_execution() {
  L2913: fn test_opposing(flag: &str, opposing_flags: &[&str]) {
  L2946: fn test_strip_cwd_prefix() {
  L2969: fn test_invalid_cwd() {
  L2990: fn test_git_dir() {
  L3030: fn test_gitignore_parent() {
  L3043: fn test_hyperlink() {
  L3130: fn test_sanitize_recursive_link_msg() {
src/dir_entry.rs:
  L12: enum DirEntryInner {
  L20: pub struct DirEntry {
  L28: pub fn normal(e: ignore::DirEntry) -> Self {
  L36: pub fn broken_symlink(path: PathBuf, depth: Option<usize>) -> Self {
  L44: pub fn path(&self) -> &Path {
  L51: pub fn into_path(self) -> PathBuf {
  L61: pub fn stripped_path(&self, config: &Config) -> &Path {
  L76: pub fn into_stripped_path(self, config: &Config) -> PathBuf {
  L84: pub fn file_type(&self) -> Option<FileType> {
  L91: pub fn metadata(&self) -> Option<&Metadata> {
  L100: pub fn depth(&self) -> Option<usize> {
  L107: pub fn style(&self, ls_colors: &LsColors) -> Option<&Style> {
  L114: fn starts_with_dash(path: &Path) -> bool {
  L120: fn eq(&self, other: &Self) -> bool {
  L129: fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
  L136: fn cmp(&self, other: &Self) -> std::cmp::Ordering {
  L142: fn path(&self) -> PathBuf {
  L146: fn file_name(&self) -> OsString {
  L162: fn file_type(&self) -> Option<FileType> {
  L166: fn metadata(&self) -> Option<Metadata> {
  L177: fn dash_prefixed_paths_detected() {
  L184: fn safe_paths_not_flagged() {
src/exec/command.rs:
  L100: pub fn handle_cmd_error(cmd: Option<&Command>, err: io::Error) -> ExitCode {
  L8: struct Outputs {
  L12: pub struct OutputBuffer {
  L18: pub fn new(null_separator: bool) -> Self {
  L25: fn push(&mut self, stdout: Vec<u8>, stderr: Vec<u8>) {
  L29: fn write(self) {
  L59: pub fn execute_commands<I: Iterator<Item = io::Result<Command>>>(
src/exec/mod.rs:
  L205: fn exit_code(&self) -> ExitCode {
  L386: fn tokens_multiple() {
  L422: fn tokens_multiple_batch() {
  L22: pub enum ExecutionMode {
  L30: pub struct CommandSet {
  L36: pub fn new<I, T, S>(input: I) -> Result<CommandSet>
  L51: pub fn new_batch<I, T, S>(input: I) -> Result<CommandSet>
  L72: pub fn in_batch_mode(&self) -> bool {
  L76: pub fn execute(
  L90: pub fn execute_batch<I>(&self, paths: I, limit: usize, path_separator: Option<&str>) -> ExitCode
  L125: struct CommandBuilder {
  L136: fn new(template: &CommandTemplate, limit: usize) -> io::Result<Self> {
  L164: fn new_command(pre_args: &[OsString]) -> io::Result<Command> {
  L173: fn push(&mut self, path: &Path, separator: Option<&str>) -> io::Result<()> {
  L191: fn finish(&mut self) -> io::Result<()> {
  L215: struct CommandTemplate {
  L220: fn new<I, S>(input: I, mode: ExecutionMode) -> Result<CommandTemplate>
  L258: fn number_of_tokens(&self) -> usize {
  L266: fn generate(&self, input: &Path, path_separator: Option<&str>) -> io::Result<Command> {
  L279: fn generate_str(template: &CommandTemplate, input: &str) -> Vec<String> {
  L288: fn tokens_with_placeholder() {
  L305: fn tokens_with_no_extension() {
  L321: fn tokens_with_basename() {
  L337: fn tokens_with_parent() {
  L353: fn tokens_with_basename_no_extension() {
  L369: fn tokens_with_literal_braces() {
  L379: fn tokens_with_literal_braces_and_placeholder() {
  L406: fn tokens_single_batch() {
  L427: fn template_no_args() {
  L434: fn command_set_no_args() {
  L439: fn placeholder_as_executable_rejected() {
  L446: fn generate_custom_path_separator() {
  L461: fn generate_custom_path_separator_windows() {
src/fmt/mod.rs:
  L18: pub enum Token {
  L28: fn fmt(&self, f: &mut Formatter) -> fmt::Result {
  L46: pub enum FormatTemplate {
  L51: static PLACEHOLDERS: OnceLock<AhoCorasick> = OnceLock::new();
  L54: pub fn has_tokens(&self) -> bool {
  L58: pub fn parse(fmt: &str) -> Self {
  L112: pub fn generate(&self, path: impl AsRef<Path>, path_separator: Option<&str>) -> OsString {
  L147: fn replace_separator<'a>(path: &'a OsStr, path_separator: Option<&str>) -> Cow<'a, OsStr> {
  L201: fn token_from_pattern_id(id: u32) -> Token {
  L219: fn parse_no_placeholders() {
  L228: fn parse_only_brace_escapes() {
  L237: fn all_placeholders() {
src/cli.rs:
  L746: pub fn rg_alias_ignore(&self) -> bool {
  L31: pub struct Opts {
  L697: pub fn search_paths(&self) -> anyhow::Result<Vec<PathBuf>> {
  L726: fn normalize_path(&self, path: &Path) -> PathBuf {
  L741: pub fn no_search_paths(&self) -> bool {
  L750: pub fn max_depth(&self) -> Option<usize> {
  L754: pub fn min_depth(&self) -> Option<usize> {
  L758: pub fn threads(&self) -> NonZeroUsize {
  L762: pub fn max_results(&self) -> Option<usize> {
  L768: pub fn strip_cwd_prefix<P: FnOnce() -> bool>(&self, auto_pred: P) -> bool {
  L779: pub fn gen_completions(&self) -> anyhow::Result<Option<Shell>> {
  L792: fn default_num_threads() -> NonZeroUsize {
  L806: pub enum FileType {
  L829: pub enum ColorWhen {
  L839: pub enum StripCwdWhen {
  L849: pub enum HyperlinkWhen {
  L860: pub struct Exec {
  L865: fn from_arg_matches(matches: &ArgMatches) -> clap::error::Result<Self> {
  L879: fn update_from_arg_matches(&mut self, matches: &ArgMatches) -> clap::error::Result<()> {
  L886: fn augment_args(cmd: Command) -> Command {
  L962: fn augment_args_for_update(cmd: Command) -> Command {
  L967: fn parse_millis(arg: &str) -> Result<Duration, std::num::ParseIntError> {
  L971: fn ensure_current_directory_exists(current_directory: &Path) -> anyhow::Result<()> {
src/hyperlink.rs:
  L5: pub(crate) struct PathUrl(PathBuf);
  L8: pub(crate) fn new(path: &Path) -> Option<PathUrl> {
  L14: fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
  L24: fn encode(f: &mut Formatter, byte: u8) -> fmt::Result {
  L44: fn host() -> &'static str {
  L60: const fn host() -> &'static str {
  L69: struct Encoded(&'static str);
  L72: fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
  L81: fn test_unicode_encoding() {
src/filter/time.rs:
  L7: pub enum TimeFilter {
  L13: fn now() -> Zoned {
  L24: fn now() -> Zoned {
  L29: fn from_str(s: &str) -> Option<SystemTime> {
  L49: pub fn before(s: &str) -> Option<TimeFilter> {
  L53: pub fn after(s: &str) -> Option<TimeFilter> {
  L57: pub fn applies_to(&self, t: &SystemTime) -> bool {
  L70: struct TestTime(SystemTime);
  L73: fn new(time: Zoned) -> Self {
  L78: fn set(&mut self, time: Zoned) {
  L83: fn timestamp(&self) -> SystemTime {
  L89: fn drop(&mut self) {
  L96: fn is_time_filter_applicable() {
  L204: fn out_of_range_unix_timestamp_is_rejected() {
src/filter/owner.rs:
  L19: const IGNORE: Self = OwnerFilter {
  L61: pub fn filter_ignore(self) -> Option<Self> {
  L6: pub struct OwnerFilter {
  L12: enum Check<T> {
  L27: pub fn from_string(input: &str) -> Result<Self> {
  L69: pub fn matches(&self, md: &fs::Metadata) -> bool {
  L77: fn check(&self, v: T) -> bool {
  L85: fn parse<F>(s: Option<&str>, f: F) -> Result<Self>
src/error.rs:
  L29: type Sanitized = SanitizedError<io::Error>;
  L31: fn sanitize(self) -> Self::Sanitized {
  L37: type Sanitized = SanitizedError<ignore::Error>;
  L39: fn sanitize(self) -> Self::Sanitized {
  L44: pub struct SanitizedError<E>(E);
  L47: fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
  L14: pub(crate) trait SanitizeErr {
  L17: fn sanitize(self) -> Self::Sanitized;
  L21: type Sanitized = Self;
  L23: fn sanitize(self) -> Self {
src/sanitize.rs:
  L8: fn needs_escape(c: char) -> bool {
  L26: pub fn write_sanitized(f: &mut std::fmt::Formatter<'_>, raw: &str) -> std::fmt::Result {
  L47: pub(crate) struct SanitizedStr<'a> {
  L55: fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
  L66: pub fn sanitize_for_term(raw: &str, is_terminal: bool) -> SanitizedStr<'_> {
  L77: fn sanitize_string(s: &str) -> String {
  L86: fn preserves_safe_content() {
  L100: fn strips_osc52_clipboard_payload() {
  L108: fn strips_cr_output_forgery() {
  L113: fn strips_osc8_hyperlink_injection() {
  L119: fn strips_del() {
  L124: fn strips_bel_and_null() {
  L130: fn escape_preserves_information() {
  L136: fn strips_c1_csi_and_osc_initiators() {
  L143: fn strips_bidi_overrides_and_zero_width() {
  L155: fn keeps_legitimate_unicode_features() {
  L168: fn maybe_sanitize_passthrough_when_not_terminal() {
src/filesystem.rs:
  L14: pub fn path_absolute_form(path: &Path) -> io::Result<PathBuf> {
  L23: pub fn absolute_path(path: &Path) -> io::Result<PathBuf> {
  L38: pub fn is_existing_directory(path: &Path) -> bool {
  L44: pub fn is_empty(entry: &dir_entry::DirEntry) -> bool {
  L63: pub fn is_block_device(ft: fs::FileType) -> bool {
  L68: pub fn is_block_device(_: fs::FileType) -> bool {
  L73: pub fn is_char_device(ft: fs::FileType) -> bool {
  L78: pub fn is_char_device(_: fs::FileType) -> bool {
  L83: pub fn is_socket(ft: fs::FileType) -> bool {
  L88: pub fn is_socket(_: fs::FileType) -> bool {
  L93: pub fn is_pipe(ft: fs::FileType) -> bool {
  L98: pub fn is_pipe(_: fs::FileType) -> bool {
  L103: pub fn osstr_to_bytes(input: &OsStr) -> Cow<'_, [u8]> {
  L109: pub fn osstr_to_bytes(input: &OsStr) -> Cow<'_, [u8]> {
  L119: pub fn strip_current_dir(path: &Path) -> &Path {
  L128: pub fn default_path_separator() -> Option<String> {
  L144: fn strip_current_dir_basic() {
src/filetypes.rs:
  L21: pub fn should_ignore(&self, entry: &dir_entry::DirEntry) -> bool {
  L8: pub struct FileTypes {
src/config.rs:
  L14: pub struct Config {
  L140: pub fn is_printing(&self) -> bool {
src/exec/job.rs:
  L10: pub fn job(
  L45: pub fn batch(
src/fmt/input.rs:
  L7: pub fn basename(path: &Path) -> &OsStr {
  L12: pub fn remove_extension(path: &Path) -> OsString {
  L22: pub fn dirname(path: &Path) -> OsString {
  L39: fn correct(input: &str) -> String {
  L77: fn dirname_root() {
  L84: fn dirname_root() {
src/filter/size.rs:
  L6: static SIZE_CAPTURES: OnceLock<Regex> = OnceLock::new();
  L9: pub enum SizeFilter {
  L16: const KILO: u64 = 1000;
  L17: const MEGA: u64 = KILO * 1000;
  L18: const GIGA: u64 = MEGA * 1000;
  L19: const TERA: u64 = GIGA * 1000;
  L22: const KIBI: u64 = 1024;
  L23: const MEBI: u64 = KIBI * 1024;
  L24: const GIBI: u64 = MEBI * 1024;
  L25: const TEBI: u64 = GIBI * 1024;
  L28: pub fn from_string(s: &str) -> anyhow::Result<Self> {
  L33: fn parse_opt(s: &str) -> Option<Self> {
  L68: pub fn is_within(&self, size: u64) -> bool {
  L197: fn is_within_less_than() {
  L203: fn is_within_less_than_equal() {
  L209: fn is_within_greater_than() {
  L215: fn is_within_greater_than_equal() {
src/main.rs:
  L57: static ALLOC: tikv_jemallocator::Jemalloc = tikv_jemallocator::Jemalloc;
  L60: const DEFAULT_LS_COLORS: &str = "
  L65: fn main() {
  L80: fn run() -> Result<ExitCode> {
  L121: fn print_completions(shell: clap_complete::Shell) -> Result<ExitCode> {
  L136: fn set_working_dir(opts: &Opts) -> Result<()> {
  L174: fn ensure_search_pattern_is_not_a_path(opts: &Opts) -> Result<()> {
  L190: fn ensure_single_search_pattern_is_not_a_path(pattern: &str) -> Result<()> {
  L223: fn build_pattern_regex(pattern: &str, opts: &Opts) -> Result<String> {
  L239: fn check_path_separator_length(path_separator: Option<&str>) -> Result<()> {
  L253: fn construct_config(mut opts: Opts, pattern_regexps: &[String]) -> Result<Config> {
  L400: fn extract_command(opts: &mut Opts, colored_output: bool) -> Result<Option<CommandSet>> {
  L417: fn determine_ls_command(colored_output: bool) -> Result<Vec<&'static str>> {
  L501: fn extract_time_constraints(opts: &Opts) -> Result<Vec<TimeFilter>> {
  L526: fn ensure_use_hidden_option_for_leading_dot_pattern(
  L546: fn build_regex(pattern_regex: String, config: &Config) -> Result<regex::bytes::Regex> {
src/output.rs:
  L12: fn replace_path_separator(path: &str, new_path_separator: &str) -> String {
  L17: pub fn print_entry<W: Write>(stdout: &mut W, entry: &DirEntry, config: &Config) -> io::Result<()> {
  L49: fn print_trailing_slash<W: Write>(
  L69: fn print_entry_format<W: Write>(
  L89: fn print_entry_colorized<W: Write>(
  L156: fn print_entry_uncolorized_base<W: Write>(
  L173: fn print_entry_uncolorized<W: Write>(
  L182: fn print_entry_uncolorized<W: Write>(
src/regex_helper.rs:
  L5: pub fn pattern_has_uppercase_char(pattern: &str) -> bool {
  L15: fn hir_has_uppercase_char(hir: &Hir) -> bool {
  L40: pub fn pattern_matches_strings_with_leading_dot(pattern: &str) -> bool {
  L50: fn hir_matches_strings_with_leading_dot(hir: &Hir) -> bool {
  L82: fn pattern_has_uppercase_char_simple() {
  L91: fn pattern_has_uppercase_char_advanced() {
  L99: fn matches_strings_with_leading_dot_simple() {
