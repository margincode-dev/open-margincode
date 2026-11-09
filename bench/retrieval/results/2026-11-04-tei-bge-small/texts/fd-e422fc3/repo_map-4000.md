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
  L606: fn scan(&self, paths: &[PathBuf]) -> Result<ExitCode> {
  L647: fn broken_symlink_from_err(mut err: ignore::Error) -> Result<DirEntry, ignore::Error> {
  L688: fn search_str_for_entry<'a>(
  L717: pub fn scan(paths: &[PathBuf], patterns: Vec<Regex>, config: Config) -> Result<ExitCode> {
  L727: fn search_str_for_entry_with_relative_path() {
  L736: fn search_str_for_entry_strips_dot_prefix() {
  L745: fn search_str_for_entry_with_absolute_path() {
  L754: fn search_str_no_base_dir() {
  L762: fn search_str_no_base_dir_with_plain_relative_path() {
  L770: fn search_str_no_base_dir_with_file_in_current_dir() {
tests/tests.rs:
  L255: fn test_and_plus_case_sensitive() {
  L517: fn test_smart_case() {
  L541: fn test_case_sensitive() {
  L556: fn test_case_insensitive() {
  L626: fn test_smart_case_glob_searches() {
  L640: fn test_case_sensitive_glob_searches() {
  L244: fn test_and_plus_ignore_case() {
  L447: fn test_explicit_root_path() {
  L496: fn test_regex_searches() {
  L659: fn test_regex_overrides_glob() {
  L2448: fn test_exact_literal_nonsubstring() {
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
  L265: fn test_and_plus_full_path() {
  L295: fn test_empty_pattern() {
  L316: fn test_multi_file() {
  L340: fn test_multi_file_with_missing() {
  L383: fn test_pattern_with_forward_slash_is_rejected() {
  L410: fn test_pattern_with_forward_slash_allowed_with_full_path() {
  L425: fn test_and_pattern_with_forward_slash_is_rejected() {
  L488: fn test_single_dash_root_path() {
  L574: fn test_glob_searches() {
  L604: fn test_full_path_glob_searches() {
  L648: fn test_glob_searches_with_extension() {
  L667: fn test_full_path() {
  L682: fn test_hidden() {
  L700: fn test_hidden_file_attribute() {
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
  L2486: fn test_invalid_utf8() {
  L2510: fn test_size() {
  L2588: fn create_file_with_modified<P: AsRef<Path>>(path: P, duration_in_secs: u64) {
  L2596: fn remove_symlink<P: AsRef<Path>>(path: P) {
  L2608: fn test_modified_relative() {
  L2641: fn change_file_modified<P: AsRef<Path>>(path: P, iso_date: &str) {
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
tests/testenv/mod.rs:
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
  L169: pub fn global_ignore_file(self, content: &str) -> TestEnv {
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
src/exec/mod.rs:
  L369: fn tokens_with_literal_braces() {
  L379: fn tokens_with_literal_braces_and_placeholder() {
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
  L205: fn exit_code(&self) -> ExitCode {
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
  L386: fn tokens_multiple() {
  L406: fn tokens_single_batch() {
  L422: fn tokens_multiple_batch() {
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
src/exec/command.rs:
  L8: struct Outputs {
  L12: pub struct OutputBuffer {
  L18: pub fn new(null_separator: bool) -> Self {
  L25: fn push(&mut self, stdout: Vec<u8>, stderr: Vec<u8>) {
  L29: fn write(self) {
  L59: pub fn execute_commands<I: Iterator<Item = io::Result<Command>>>(
  L100: pub fn handle_cmd_error(cmd: Option<&Command>, err: io::Error) -> ExitCode {
src/exit_codes.rs:
  L7: pub enum ExitCode {
  L15: fn from(code: ExitCode) -> Self {
  L26: fn is_error(self) -> bool {
