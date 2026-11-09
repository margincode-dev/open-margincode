src/dir_entry.rs:
  L84: pub fn file_type(&self) -> Option<FileType> {
  L146: fn file_name(&self) -> OsString {
  L162: fn file_type(&self) -> Option<FileType> {
  L12: enum DirEntryInner {
  L20: pub struct DirEntry {
  L28: pub fn normal(e: ignore::DirEntry) -> Self {
  L36: pub fn broken_symlink(path: PathBuf, depth: Option<usize>) -> Self {
  L44: pub fn path(&self) -> &Path {
  L51: pub fn into_path(self) -> PathBuf {
  L61: pub fn stripped_path(&self, config: &Config) -> &Path {
  L76: pub fn into_stripped_path(self, config: &Config) -> PathBuf {
  L91: pub fn metadata(&self) -> Option<&Metadata> {
  L100: pub fn depth(&self) -> Option<usize> {
  L107: pub fn style(&self, ls_colors: &LsColors) -> Option<&Style> {
  L114: fn starts_with_dash(path: &Path) -> bool {
  L120: fn eq(&self, other: &Self) -> bool {
  L129: fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
  L136: fn cmp(&self, other: &Self) -> std::cmp::Ordering {
  L142: fn path(&self) -> PathBuf {
  L166: fn metadata(&self) -> Option<Metadata> {
  L177: fn dash_prefixed_paths_detected() {
  L184: fn safe_paths_not_flagged() {
src/cli.rs:
  L806: pub enum FileType {
  L31: pub struct Opts {
  L697: pub fn search_paths(&self) -> anyhow::Result<Vec<PathBuf>> {
  L726: fn normalize_path(&self, path: &Path) -> PathBuf {
  L741: pub fn no_search_paths(&self) -> bool {
  L746: pub fn rg_alias_ignore(&self) -> bool {
  L750: pub fn max_depth(&self) -> Option<usize> {
  L754: pub fn min_depth(&self) -> Option<usize> {
  L758: pub fn threads(&self) -> NonZeroUsize {
  L762: pub fn max_results(&self) -> Option<usize> {
  L768: pub fn strip_cwd_prefix<P: FnOnce() -> bool>(&self, auto_pred: P) -> bool {
  L779: pub fn gen_completions(&self) -> anyhow::Result<Option<Shell>> {
  L792: fn default_num_threads() -> NonZeroUsize {
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
src/walk.rs:
  L770: fn search_str_no_base_dir_with_file_in_current_dir() {
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
  L606: fn scan(&self, paths: &[PathBuf]) -> Result<ExitCode> {
  L647: fn broken_symlink_from_err(mut err: ignore::Error) -> Result<DirEntry, ignore::Error> {
  L688: fn search_str_for_entry<'a>(
  L717: pub fn scan(paths: &[PathBuf], patterns: Vec<Regex>, config: Config) -> Result<ExitCode> {
  L727: fn search_str_for_entry_with_relative_path() {
  L736: fn search_str_for_entry_strips_dot_prefix() {
  L745: fn search_str_for_entry_with_absolute_path() {
  L754: fn search_str_no_base_dir() {
  L762: fn search_str_no_base_dir_with_plain_relative_path() {
src/filesystem.rs:
  L63: pub fn is_block_device(ft: fs::FileType) -> bool {
  L68: pub fn is_block_device(_: fs::FileType) -> bool {
  L14: pub fn path_absolute_form(path: &Path) -> io::Result<PathBuf> {
  L23: pub fn absolute_path(path: &Path) -> io::Result<PathBuf> {
  L38: pub fn is_existing_directory(path: &Path) -> bool {
  L44: pub fn is_empty(entry: &dir_entry::DirEntry) -> bool {
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
tests/testenv/mod.rs:
  L169: pub fn global_ignore_file(self, content: &str) -> TestEnv {
  L14: pub struct TestEnv {
  L29: fn create_working_directory(
  L65: fn create_config_directory_with_global_ignore(ignore_file_content: &str) -> io::Result<TempDir> {
  L76: fn find_fd_exe() -> PathBuf {
  L82: fn format_exit_error(args: &[&str], output: &process::Output) -> String {
  L92: fn format_output_error(args: &[&str], expected: &str, actual: &str) -> String {
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
  L296: pub fn assert_failure_with_error(&self, args: &[&str], expected: &str) {
  L304: pub fn assert_failure(&self, args: &[&str]) {
  L312: pub fn assert_error(&self, args: &[&str], expected: &str) -> process::ExitStatus {
  L316: fn run_command(&self, path: &Path, args: &[&str]) -> process::Output {
  L336: fn assert_error_subdirectory<P: AsRef<Path>>(
tests/tests.rs:
  L56: fn create_file_with_size<P: AsRef<Path>>(path: P, size_in_bytes: usize) {
  L316: fn test_multi_file() {
  L340: fn test_multi_file_with_missing() {
  L700: fn test_hidden_file_attribute() {
  L1074: fn test_file_system_boundaries() {
  L2588: fn create_file_with_modified<P: AsRef<Path>>(path: P, duration_in_secs: u64) {
  L2641: fn change_file_modified<P: AsRef<Path>>(path: P, iso_date: &str) {
  L17: static DEFAULT_DIRS: &[&str] = &["one/two/three", "one/two/three/directory_foo"];
  L19: static DEFAULT_FILES: &[&str] = &[
  L32: fn get_absolute_root_path(env: &TestEnv) -> String {
  L49: fn get_test_env_with_abs_path(dirs: &[&'static str], files: &[&'static str]) -> (TestEnv, String) {
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
  L244: fn test_and_plus_ignore_case() {
  L255: fn test_and_plus_case_sensitive() {
  L265: fn test_and_plus_full_path() {
  L295: fn test_empty_pattern() {
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
  L721: fn test_no_ignore() {
  L752: fn test_gitignore_and_fdignore() {
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
  L1056: fn test_follow() {
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
  L2596: fn remove_symlink<P: AsRef<Path>>(path: P) {
  L2608: fn test_modified_relative() {
  L2651: fn test_modified_absolute() {
  L2669: fn test_owner_ignore_all() {
  L2677: fn test_owner_current_user() {
  L2688: fn test_owner_current_group() {
  L2699: fn test_owner_root() {
  L2712: fn test_custom_path_separator() {
  L2726: fn test_base_directory() {
  L2775: fn test_max_results() {
  L2818: fn test_exec_invalid_utf8() {
  L2856: fn test_list_details() {
  L2878: fn test_single_and_multithreaded_execution() {
  L2887: fn test_number_parsing_errors() {
  L2913: fn test_opposing(flag: &str, opposing_flags: &[&str]) {
  L2934: fn test_error_if_hidden_not_set_and_pattern_starts_with_dot() {
  L2946: fn test_strip_cwd_prefix() {
  L2969: fn test_invalid_cwd() {
  L2990: fn test_git_dir() {
  L3030: fn test_gitignore_parent() {
  L3043: fn test_hyperlink() {
  L3061: fn test_ignore_contain() {
  L3087: fn test_ignore_contain_precedence_over_depth_check() {
  L3105: fn test_ignore_contain_precedence_over_root_check() {
  L3117: fn test_sanitize_exec_error_msg() {
  L3130: fn test_sanitize_recursive_link_msg() {
src/fmt/mod.rs:
  L18: pub enum Token {
  L28: fn fmt(&self, f: &mut Formatter) -> fmt::Result {
  L46: pub enum FormatTemplate {
  L51: static PLACEHOLDERS: OnceLock<AhoCorasick> = OnceLock::new();
  L54: pub fn has_tokens(&self) -> bool {
  L58: pub fn parse(fmt: &str) -> Self {
  L112: pub fn generate(&self, path: impl AsRef<Path>, path_separator: Option<&str>) -> OsString {
  L147: fn replace_separator<'a>(path: &'a OsStr, path_separator: Option<&str>) -> Cow<'a, OsStr> {
