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
