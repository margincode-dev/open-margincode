src/dir_entry.rs:
  L36: pub fn broken_symlink(path: PathBuf, depth: usize) -> Self {
  L44: pub fn path(&self) -> &Path {
  L51: pub fn into_path(self) -> PathBuf {
  L61: pub fn stripped_path(&self, config: &Config) -> &Path {
  L76: pub fn into_stripped_path(self, config: &Config) -> PathBuf {
  L84: pub fn file_type(&self) -> Option<FileType> {
  L100: pub fn depth(&self) -> Option<usize> {
  L142: fn path(&self) -> PathBuf {
  L146: fn file_name(&self) -> OsString {
  L162: fn file_type(&self) -> Option<FileType> {
  L12: enum DirEntryInner {
  L20: pub struct DirEntry {
  L28: pub fn normal(e: ignore::DirEntry) -> Self {
  L91: pub fn metadata(&self) -> Option<&Metadata> {
  L107: pub fn style(&self, ls_colors: &LsColors) -> Option<&Style> {
  L114: fn starts_with_dash(path: &Path) -> bool {
  L120: fn eq(&self, other: &Self) -> bool {
  L129: fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
  L136: fn cmp(&self, other: &Self) -> std::cmp::Ordering {
  L166: fn metadata(&self) -> Option<Metadata> {
  L177: fn dash_prefixed_paths_detected() {
  L184: fn safe_paths_not_flagged() {
src/cli.rs:
  L751: pub fn min_depth(&self) -> Option<usize> {
  L696: pub fn search_paths(&self) -> anyhow::Result<Vec<PathBuf>> {
  L723: fn normalize_path(&self, path: &Path) -> PathBuf {
  L738: pub fn no_search_paths(&self) -> bool {
  L743: pub fn rg_alias_ignore(&self) -> bool {
  L747: pub fn max_depth(&self) -> Option<usize> {
  L803: pub enum FileType {
  L32: pub struct Opts {
  L755: pub fn threads(&self) -> NonZeroUsize {
  L759: pub fn max_results(&self) -> Option<usize> {
  L765: pub fn strip_cwd_prefix<P: FnOnce() -> bool>(&self, auto_pred: P) -> bool {
  L776: pub fn gen_completions(&self) -> anyhow::Result<Option<Shell>> {
  L789: fn default_num_threads() -> NonZeroUsize {
  L826: pub enum ColorWhen {
  L836: pub enum StripCwdWhen {
  L846: pub enum HyperlinkWhen {
  L857: pub struct Exec {
  L862: fn from_arg_matches(matches: &ArgMatches) -> clap::error::Result<Self> {
  L876: fn update_from_arg_matches(&mut self, matches: &ArgMatches) -> clap::error::Result<()> {
  L883: fn augment_args(cmd: Command) -> Command {
  L954: fn augment_args_for_update(cmd: Command) -> Command {
  L959: fn parse_millis(arg: &str) -> Result<Duration, std::num::ParseIntError> {
  L963: fn ensure_current_directory_exists(current_directory: &Path) -> anyhow::Result<()> {
tests/tests.rs:
  L1254: fn test_min_depth_broken_symlink_absolute_path() {
  L1216: fn test_min_depth_broken_symlink() {
  L488: fn test_single_dash_root_path() {
  L1294: fn test_max_depth_broken_symlink() {
  L1333: fn test_exact_depth_broken_symlink() {
  L1787: fn test_symlink_and_full_path_abs_path() {
  L32: fn get_absolute_root_path(env: &TestEnv) -> String {
  L244: fn test_and_plus_ignore_case() {
  L447: fn test_explicit_root_path() {
  L1112: fn test_follow_broken_symlink() {
  L1192: fn test_min_depth() {
  L1683: fn test_symlink_as_root() {
  L1724: fn test_symlink_and_absolute_path() {
  L1745: fn test_symlink_as_absolute_root() {
  L1762: fn test_symlink_and_full_path() {
  L3011: fn test_ignore_contain_precedence_over_depth_check() {
  L3029: fn test_ignore_contain_precedence_over_root_check() {
  L49: fn get_test_env_with_abs_path(dirs: &[&'static str], files: &[&'static str]) -> (TestEnv, String) {
  L56: fn create_file_with_size<P: AsRef<Path>>(path: P, size_in_bytes: usize) {
  L255: fn test_and_plus_case_sensitive() {
  L265: fn test_and_plus_full_path() {
  L316: fn test_multi_file() {
  L340: fn test_multi_file_with_missing() {
  L410: fn test_pattern_with_forward_slash_allowed_with_full_path() {
  L517: fn test_smart_case() {
  L541: fn test_case_sensitive() {
  L556: fn test_case_insensitive() {
  L604: fn test_full_path_glob_searches() {
  L626: fn test_smart_case_glob_searches() {
  L640: fn test_case_sensitive_glob_searches() {
  L667: fn test_full_path() {
  L700: fn test_hidden_file_attribute() {
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
  L1074: fn test_file_system_boundaries() {
  L1155: fn test_max_depth() {
  L1368: fn test_exact_depth() {
  L1415: fn test_absolute_path() {
  L1452: fn test_implicit_absolute_path() {
  L1471: fn test_normalized_absolute_path() {
  L2526: fn create_file_with_modified<P: AsRef<Path>>(path: P, duration_in_secs: u64) {
  L2534: fn remove_symlink<P: AsRef<Path>>(path: P) {
  L2579: fn change_file_modified<P: AsRef<Path>>(path: P, iso_date: &str) {
  L2607: fn test_owner_ignore_all() {
  L2637: fn test_owner_root() {
  L2650: fn test_custom_path_separator() {
  L2802: fn test_single_and_multithreaded_execution() {
  L2811: fn test_number_parsing_errors() {
  L2858: fn test_error_if_hidden_not_set_and_pattern_starts_with_dot() {
  L2985: fn test_ignore_contain() {
  L17: static DEFAULT_DIRS: &[&str] = &["one/two/three", "one/two/three/directory_foo"];
  L19: static DEFAULT_FILES: &[&str] = &[
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
  L295: fn test_empty_pattern() {
  L383: fn test_pattern_with_forward_slash_is_rejected() {
  L425: fn test_and_pattern_with_forward_slash_is_rejected() {
  L496: fn test_regex_searches() {
  L574: fn test_glob_searches() {
  L648: fn test_glob_searches_with_extension() {
  L659: fn test_regex_overrides_glob() {
  L682: fn test_hidden() {
  L752: fn test_gitignore_and_fdignore() {
  L1056: fn test_follow() {
  L1139: fn test_print0() {
  L1381: fn test_prune() {
  L1491: fn test_type() {
  L1529: fn test_type_executable() {
  L1571: fn test_type_empty() {
  L1598: fn test_extension() {
  L1655: fn test_no_extension() {
  L1807: fn test_excludes() {
  L1856: fn format() {
  L1917: fn test_exec() {
  L2001: fn test_exec_multi() {
  L2061: fn test_exec_nulls() {
  L2075: fn test_exec_batch() {
  L2152: fn test_exec_batch_multi() {
  L2220: fn test_exec_batch_with_limit() {
  L2258: fn test_exec_with_separator() {
  L2337: fn test_quiet() {
  L2350: fn test_fixed_strings() {
  L2386: fn test_exact_literal_nonsubstring() {
  L2424: fn test_invalid_utf8() {
  L2448: fn test_size() {
  L2546: fn test_modified_relative() {
  L2589: fn test_modified_absolute() {
  L2615: fn test_owner_current_user() {
  L2626: fn test_owner_current_group() {
  L2664: fn test_base_directory() {
  L2713: fn test_max_results() {
  L2756: fn test_exec_invalid_utf8() {
  L2794: fn test_list_details() {
  L2837: fn test_opposing(flag: &str, opposing_flags: &[&str]) {
  L2870: fn test_strip_cwd_prefix() {
  L2893: fn test_invalid_cwd() {
  L2914: fn test_git_dir() {
  L2954: fn test_gitignore_parent() {
  L2967: fn test_hyperlink() {
tests/testenv/mod.rs:
  L169: pub fn global_ignore_file(self, content: &str) -> TestEnv {
  L179: pub fn create_broken_symlink<P: AsRef<Path>>(
  L65: fn create_config_directory_with_global_ignore(ignore_file_content: &str) -> io::Result<TempDir> {
  L82: fn format_exit_error(args: &[&str], output: &process::Output) -> String {
  L92: fn format_output_error(args: &[&str], expected: &str, actual: &str) -> String {
  L200: pub fn test_root(&self) -> PathBuf {
  L211: pub fn system_root(&self) -> PathBuf {
  L280: pub fn assert_failure_with_error(&self, args: &[&str], expected: &str) {
  L296: pub fn assert_error(&self, args: &[&str], expected: &str) -> process::ExitStatus {
  L320: fn assert_error_subdirectory<P: AsRef<Path>>(
  L14: pub struct TestEnv {
  L29: fn create_working_directory(
  L76: fn find_fd_exe() -> PathBuf {
  L115: fn normalize_output(s: &str, trim_start: bool, normalize_line: bool) -> String {
  L137: fn trim_lines(s: &str) -> String {
  L148: pub fn new(directories: &[&'static str], files: &[&'static str]) -> TestEnv {
  L160: pub fn normalize_line(self, normalize: bool) -> TestEnv {
  L206: pub fn test_exe(&self) -> &PathBuf {
  L218: pub fn assert_success_and_get_output<P: AsRef<Path>>(
  L234: pub fn assert_success_and_get_normalized_output<P: AsRef<Path>>(
  L248: pub fn assert_output(&self, args: &[&str], expected: &str) {
  L254: pub fn assert_output_raw(&self, args: &[&str], expected: &[u8]) {
  L262: pub fn assert_output_subdirectory<P: AsRef<Path>>(
  L288: pub fn assert_failure(&self, args: &[&str]) {
  L300: fn run_command(&self, path: &Path, args: &[&str]) -> process::Output {
src/walk.rs:
  L670: fn depth_relative_to_roots(path: &Path, roots: &[PathBuf]) -> usize {
  L729: fn search_str_for_entry_with_relative_path() {
  L747: fn search_str_for_entry_with_absolute_path() {
  L764: fn search_str_no_base_dir_with_plain_relative_path() {
  L772: fn search_str_no_base_dir_with_file_in_current_dir() {
  L127: const DEFAULT_MAX_BUFFER_TIME: Duration = Duration::from_millis(100);
  L347: fn build_walker(&self, paths: &[PathBuf]) -> Result<WalkParallel> {
  L690: fn search_str_for_entry<'a>(
  L738: fn search_str_for_entry_strips_dot_prefix() {
  L756: fn search_str_no_base_dir() {
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
  L408: fn receive(&self, rx: Receiver<Batch>) -> ExitCode {
  L443: fn spawn_senders(&self, walker: WalkParallel, roots: &[PathBuf], tx: Sender<Batch>) {
  L618: fn scan(&self, paths: &[PathBuf]) -> Result<ExitCode> {
  L719: pub fn scan(paths: &[PathBuf], patterns: Vec<Regex>, config: Config) -> Result<ExitCode> {
src/filesystem.rs:
  L14: pub fn path_absolute_form(path: &Path) -> io::Result<PathBuf> {
  L23: pub fn absolute_path(path: &Path) -> io::Result<PathBuf> {
  L128: pub fn default_path_separator() -> Option<String> {
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
  L144: fn strip_current_dir_basic() {
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
