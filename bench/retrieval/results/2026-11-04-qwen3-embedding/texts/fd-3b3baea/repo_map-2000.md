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
