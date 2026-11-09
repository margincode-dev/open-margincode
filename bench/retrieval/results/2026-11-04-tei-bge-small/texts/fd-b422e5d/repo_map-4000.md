tests/tests.rs:
  L147: fn test_and_pattern_starts_with_dash() {
  L1701: fn format() {
  L2639: fn test_list_details() {
  L2703: fn test_error_if_hidden_not_set_and_pattern_starts_with_dot() {
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
  L167: fn test_and_plus_extension() {
  L200: fn test_and_plus_type() {
  L216: fn test_and_plus_glob() {
  L223: fn test_and_plus_fixed_strings() {
  L244: fn test_and_plus_ignore_case() {
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
  L1213: fn test_exact_depth() {
  L1226: fn test_prune() {
  L1260: fn test_absolute_path() {
  L1297: fn test_implicit_absolute_path() {
  L1316: fn test_normalized_absolute_path() {
  L1336: fn test_type() {
  L1374: fn test_type_executable() {
  L1416: fn test_type_empty() {
  L1443: fn test_extension() {
  L1500: fn test_no_extension() {
  L1528: fn test_symlink_as_root() {
  L1569: fn test_symlink_and_absolute_path() {
  L1590: fn test_symlink_as_absolute_root() {
  L1607: fn test_symlink_and_full_path() {
  L1632: fn test_symlink_and_full_path_abs_path() {
  L1652: fn test_excludes() {
  L1762: fn test_exec() {
  L1846: fn test_exec_multi() {
  L1906: fn test_exec_nulls() {
  L1920: fn test_exec_batch() {
  L1997: fn test_exec_batch_multi() {
  L2065: fn test_exec_batch_with_limit() {
  L2103: fn test_exec_with_separator() {
  L2182: fn test_quiet() {
  L2195: fn test_fixed_strings() {
  L2231: fn test_exact_literal_nonsubstring() {
  L2269: fn test_invalid_utf8() {
  L2293: fn test_size() {
  L2371: fn create_file_with_modified<P: AsRef<Path>>(path: P, duration_in_secs: u64) {
  L2379: fn remove_symlink<P: AsRef<Path>>(path: P) {
  L2391: fn test_modified_relative() {
  L2424: fn change_file_modified<P: AsRef<Path>>(path: P, iso_date: &str) {
  L2434: fn test_modified_absolute() {
  L2452: fn test_owner_ignore_all() {
  L2460: fn test_owner_current_user() {
  L2471: fn test_owner_current_group() {
  L2482: fn test_owner_root() {
  L2495: fn test_custom_path_separator() {
  L2509: fn test_base_directory() {
  L2558: fn test_max_results() {
  L2601: fn test_exec_invalid_utf8() {
  L2647: fn test_single_and_multithreaded_execution() {
  L2656: fn test_number_parsing_errors() {
  L2682: fn test_opposing(flag: &str, opposing_flags: &[&str]) {
  L2715: fn test_strip_cwd_prefix() {
  L2738: fn test_invalid_cwd() {
  L2759: fn test_git_dir() {
  L2799: fn test_gitignore_parent() {
  L2812: fn test_hyperlink() {
  L2830: fn test_ignore_contain() {
  L2856: fn test_ignore_contain_precedence_over_depth_check() {
  L2874: fn test_ignore_contain_precedence_over_root_check() {
tests/testenv/mod.rs:
  L82: fn format_exit_error(args: &[&str], output: &process::Output) -> String {
  L92: fn format_output_error(args: &[&str], expected: &str, actual: &str) -> String {
  L14: pub struct TestEnv {
  L29: fn create_working_directory(
  L65: fn create_config_directory_with_global_ignore(ignore_file_content: &str) -> io::Result<TempDir> {
  L76: fn find_fd_exe() -> PathBuf {
  L115: fn normalize_output(s: &str, trim_start: bool, normalize_line: bool) -> String {
  L137: fn trim_lines(s: &str) -> String {
  L148: pub fn new(directories: &[&'static str], files: &[&'static str]) -> TestEnv {
  L160: pub fn normalize_line(self, normalize: bool) -> TestEnv {
  L169: pub fn global_ignore_file(self, content: &str) -> TestEnv {
  L179: pub fn create_broken_symlink<P: AsRef<Path>>(
  L200: pub fn test_root(&self) -> PathBuf {
  L206: pub fn test_exe(&self) -> &PathBuf {
  L211: pub fn system_root(&self) -> PathBuf {
  L218: pub fn assert_success_and_get_output<P: AsRef<Path>>(
  L234: pub fn assert_success_and_get_normalized_output<P: AsRef<Path>>(
  L248: pub fn assert_output(&self, args: &[&str], expected: &str) {
  L254: pub fn assert_output_raw(&self, args: &[&str], expected: &[u8]) {
  L262: pub fn assert_output_subdirectory<P: AsRef<Path>>(
  L280: pub fn assert_failure_with_error(&self, args: &[&str], expected: &str) {
  L288: pub fn assert_failure(&self, args: &[&str]) {
  L296: pub fn assert_error(&self, args: &[&str], expected: &str) -> process::ExitStatus {
  L300: fn run_command(&self, path: &Path, args: &[&str]) -> process::Output {
  L320: fn assert_error_subdirectory<P: AsRef<Path>>(
src/fmt/mod.rs:
  L46: pub enum FormatTemplate {
  L54: pub fn has_tokens(&self) -> bool {
  L58: pub fn parse(fmt: &str) -> Self {
  L112: pub fn generate(&self, path: impl AsRef<Path>, path_separator: Option<&str>) -> OsString {
  L147: fn replace_separator<'a>(path: &'a OsStr, path_separator: Option<&str>) -> Cow<'a, OsStr> {
  L18: pub enum Token {
  L28: fn fmt(&self, f: &mut Formatter) -> fmt::Result {
  L51: static PLACEHOLDERS: OnceLock<AhoCorasick> = OnceLock::new();
  L201: fn token_from_pattern_id(id: u32) -> Token {
  L219: fn parse_no_placeholders() {
  L228: fn parse_only_brace_escapes() {
  L237: fn all_placeholders() {
src/walk.rs:
  L28: enum ReceiverMode {
  L40: pub enum WorkerResult {
  L49: struct Batch {
  L54: fn new() -> Self {
  L60: fn lock(&self) -> MutexGuard<'_, Option<Vec<WorkerResult>>> {
  L66: type Item = WorkerResult;
  L67: type IntoIter = std::vec::IntoIter<WorkerResult>;
  L69: fn into_iter(self) -> Self::IntoIter {
  L75: struct BatchSender {
  L82: fn new(tx: Sender<Batch>, limit: usize) -> Self {
  L91: fn needs_flush(&self, batch: Option<&Vec<WorkerResult>>) -> bool {
  L101: fn send(&mut self, item: WorkerResult) -> Result<(), SendError<()>> {
  L125: const MAX_BUFFER_LENGTH: usize = 1000;
  L127: const DEFAULT_MAX_BUFFER_TIME: Duration = Duration::from_millis(100);
  L130: struct ReceiverBuffer<'a, W> {
  L153: fn new(state: &'a WorkerState, rx: Receiver<Batch>, stdout: W) -> Self {
  L174: fn process(&mut self) -> ExitCode {
  L184: fn recv(&self) -> Result<Batch, RecvTimeoutError> {
  L198: fn poll(&mut self) -> Result<(), ExitCode> {
  L252: fn print(&mut self, entry: &DirEntry) -> Result<(), ExitCode> {
  L270: fn stream(&mut self) -> Result<(), ExitCode> {
  L282: fn stop(&mut self) -> Result<(), ExitCode> {
  L296: fn flush(&mut self) -> Result<(), ExitCode> {
  L306: struct WorkerState {
  L318: fn new(patterns: Vec<Regex>, config: Config) -> Self {
  L330: fn build_overrides(&self, paths: &[PathBuf]) -> Result<Override> {
  L347: fn build_walker(&self, paths: &[PathBuf]) -> Result<WalkParallel> {
  L408: fn receive(&self, rx: Receiver<Batch>) -> ExitCode {
  L443: fn spawn_senders(&self, walker: WalkParallel, tx: Sender<Batch>) {
  L617: fn scan(&self, paths: &[PathBuf]) -> Result<ExitCode> {
  L656: fn search_str_for_entry<'a>(
  L685: pub fn scan(paths: &[PathBuf], patterns: Vec<Regex>, config: Config) -> Result<ExitCode> {
  L695: fn search_str_for_entry_with_relative_path() {
  L704: fn search_str_for_entry_strips_dot_prefix() {
  L713: fn search_str_for_entry_with_absolute_path() {
  L722: fn search_str_no_base_dir() {
  L730: fn search_str_no_base_dir_with_plain_relative_path() {
  L738: fn search_str_no_base_dir_with_file_in_current_dir() {
src/dir_entry.rs:
  L105: pub fn style(&self, ls_colors: &LsColors) -> Option<&Style> {
  L112: fn starts_with_dash(path: &Path) -> bool {
  L12: enum DirEntryInner {
  L18: pub struct DirEntry {
  L26: pub fn normal(e: ignore::DirEntry) -> Self {
  L34: pub fn broken_symlink(path: PathBuf) -> Self {
  L42: pub fn path(&self) -> &Path {
  L49: pub fn into_path(self) -> PathBuf {
  L59: pub fn stripped_path(&self, config: &Config) -> &Path {
  L74: pub fn into_stripped_path(self, config: &Config) -> PathBuf {
  L82: pub fn file_type(&self) -> Option<FileType> {
  L89: pub fn metadata(&self) -> Option<&Metadata> {
  L98: pub fn depth(&self) -> Option<usize> {
  L118: fn eq(&self, other: &Self) -> bool {
  L127: fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
  L134: fn cmp(&self, other: &Self) -> std::cmp::Ordering {
  L140: fn path(&self) -> PathBuf {
  L144: fn file_name(&self) -> OsString {
  L160: fn file_type(&self) -> Option<FileType> {
  L164: fn metadata(&self) -> Option<Metadata> {
  L175: fn dash_prefixed_paths_detected() {
  L182: fn safe_paths_not_flagged() {
src/exec/command.rs:
  L9: struct Outputs {
  L13: pub struct OutputBuffer {
  L19: pub fn new(null_separator: bool) -> Self {
  L26: fn push(&mut self, stdout: Vec<u8>, stderr: Vec<u8>) {
  L30: fn write(self) {
  L60: pub fn execute_commands<I: Iterator<Item = io::Result<Command>>>(
  L101: pub fn handle_cmd_error(cmd: Option<&Command>, err: io::Error) -> ExitCode {
src/exec/mod.rs:
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
  L369: fn tokens_with_literal_braces() {
  L379: fn tokens_with_literal_braces_and_placeholder() {
  L386: fn tokens_multiple() {
  L406: fn tokens_single_batch() {
  L422: fn tokens_multiple_batch() {
  L427: fn template_no_args() {
  L434: fn command_set_no_args() {
  L439: fn placeholder_as_executable_rejected() {
  L446: fn generate_custom_path_separator() {
  L461: fn generate_custom_path_separator_windows() {
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
