src/dir_entry.rs:
  L36: pub fn broken_symlink(path: PathBuf, depth: usize) -> Self {
  L44: pub fn path(&self) -> &Path {
  L51: pub fn into_path(self) -> PathBuf {
  L61: pub fn stripped_path(&self, config: &Config) -> &Path {
  L76: pub fn into_stripped_path(self, config: &Config) -> PathBuf {
  L100: pub fn depth(&self) -> Option<usize> {
  L142: fn path(&self) -> PathBuf {
  L177: fn dash_prefixed_paths_detected() {
  L184: fn safe_paths_not_flagged() {
  L12: enum DirEntryInner {
  L20: pub struct DirEntry {
  L28: pub fn normal(e: ignore::DirEntry) -> Self {
  L84: pub fn file_type(&self) -> Option<FileType> {
  L91: pub fn metadata(&self) -> Option<&Metadata> {
  L107: pub fn style(&self, ls_colors: &LsColors) -> Option<&Style> {
  L114: fn starts_with_dash(path: &Path) -> bool {
  L120: fn eq(&self, other: &Self) -> bool {
  L129: fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
  L136: fn cmp(&self, other: &Self) -> std::cmp::Ordering {
  L146: fn file_name(&self) -> OsString {
  L162: fn file_type(&self) -> Option<FileType> {
  L166: fn metadata(&self) -> Option<Metadata> {
src/cli.rs:
  L696: pub fn search_paths(&self) -> anyhow::Result<Vec<PathBuf>> {
  L738: pub fn no_search_paths(&self) -> bool {
  L723: fn normalize_path(&self, path: &Path) -> PathBuf {
  L747: pub fn max_depth(&self) -> Option<usize> {
  L751: pub fn min_depth(&self) -> Option<usize> {
  L32: pub struct Opts {
  L743: pub fn rg_alias_ignore(&self) -> bool {
  L755: pub fn threads(&self) -> NonZeroUsize {
  L759: pub fn max_results(&self) -> Option<usize> {
  L765: pub fn strip_cwd_prefix<P: FnOnce() -> bool>(&self, auto_pred: P) -> bool {
  L776: pub fn gen_completions(&self) -> anyhow::Result<Option<Shell>> {
  L789: fn default_num_threads() -> NonZeroUsize {
  L803: pub enum FileType {
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
src/walk.rs:
  L665: fn depth_relative_to_roots(path: &Path, roots: &[PathBuf]) -> usize {
  L715: fn search_str_for_entry_with_relative_path() {
  L733: fn search_str_for_entry_with_absolute_path() {
  L750: fn search_str_no_base_dir_with_plain_relative_path() {
  L676: fn search_str_for_entry<'a>(
  L724: fn search_str_for_entry_strips_dot_prefix() {
  L742: fn search_str_no_base_dir() {
  L758: fn search_str_no_base_dir_with_file_in_current_dir() {
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
  L443: fn spawn_senders(&self, walker: WalkParallel, roots: &[PathBuf], tx: Sender<Batch>) {
  L618: fn scan(&self, paths: &[PathBuf]) -> Result<ExitCode> {
  L705: pub fn scan(paths: &[PathBuf], patterns: Vec<Regex>, config: Config) -> Result<ExitCode> {
tests/tests.rs:
  L32: fn get_absolute_root_path(env: &TestEnv) -> String {
  L1216: fn test_min_depth_broken_symlink() {
  L1606: fn test_symlink_and_absolute_path() {
  L1627: fn test_symlink_as_absolute_root() {
  L1669: fn test_symlink_and_full_path_abs_path() {
  L447: fn test_explicit_root_path() {
  L488: fn test_single_dash_root_path() {
  L1112: fn test_follow_broken_symlink() {
  L1297: fn test_absolute_path() {
  L1334: fn test_implicit_absolute_path() {
  L1353: fn test_normalized_absolute_path() {
  L1565: fn test_symlink_as_root() {
  L1644: fn test_symlink_and_full_path() {
  L49: fn get_test_env_with_abs_path(dirs: &[&'static str], files: &[&'static str]) -> (TestEnv, String) {
  L244: fn test_and_plus_ignore_case() {
  L255: fn test_and_plus_case_sensitive() {
  L265: fn test_and_plus_full_path() {
  L410: fn test_pattern_with_forward_slash_allowed_with_full_path() {
  L517: fn test_smart_case() {
  L541: fn test_case_sensitive() {
  L556: fn test_case_insensitive() {
  L604: fn test_full_path_glob_searches() {
  L626: fn test_smart_case_glob_searches() {
  L640: fn test_case_sensitive_glob_searches() {
  L667: fn test_full_path() {
  L1155: fn test_max_depth() {
  L1192: fn test_min_depth() {
  L1250: fn test_exact_depth() {
  L2416: fn remove_symlink<P: AsRef<Path>>(path: P) {
  L2428: fn test_modified_relative() {
  L2471: fn test_modified_absolute() {
  L2519: fn test_owner_root() {
  L2532: fn test_custom_path_separator() {
  L2893: fn test_ignore_contain_precedence_over_depth_check() {
  L2911: fn test_ignore_contain_precedence_over_root_check() {
  L17: static DEFAULT_DIRS: &[&str] = &["one/two/three", "one/two/three/directory_foo"];
  L19: static DEFAULT_FILES: &[&str] = &[
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
  L295: fn test_empty_pattern() {
  L316: fn test_multi_file() {
  L340: fn test_multi_file_with_missing() {
  L383: fn test_pattern_with_forward_slash_is_rejected() {
  L425: fn test_and_pattern_with_forward_slash_is_rejected() {
  L496: fn test_regex_searches() {
  L574: fn test_glob_searches() {
  L648: fn test_glob_searches_with_extension() {
  L659: fn test_regex_overrides_glob() {
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
  L1139: fn test_print0() {
  L1263: fn test_prune() {
  L1373: fn test_type() {
  L1411: fn test_type_executable() {
  L1453: fn test_type_empty() {
  L1480: fn test_extension() {
  L1537: fn test_no_extension() {
  L1689: fn test_excludes() {
  L1738: fn format() {
  L1799: fn test_exec() {
  L1883: fn test_exec_multi() {
  L1943: fn test_exec_nulls() {
  L1957: fn test_exec_batch() {
  L2034: fn test_exec_batch_multi() {
  L2102: fn test_exec_batch_with_limit() {
  L2140: fn test_exec_with_separator() {
  L2219: fn test_quiet() {
  L2232: fn test_fixed_strings() {
  L2268: fn test_exact_literal_nonsubstring() {
  L2306: fn test_invalid_utf8() {
  L2330: fn test_size() {
  L2408: fn create_file_with_modified<P: AsRef<Path>>(path: P, duration_in_secs: u64) {
  L2461: fn change_file_modified<P: AsRef<Path>>(path: P, iso_date: &str) {
  L2489: fn test_owner_ignore_all() {
  L2497: fn test_owner_current_user() {
  L2508: fn test_owner_current_group() {
  L2546: fn test_base_directory() {
  L2595: fn test_max_results() {
  L2638: fn test_exec_invalid_utf8() {
  L2676: fn test_list_details() {
  L2684: fn test_single_and_multithreaded_execution() {
  L2693: fn test_number_parsing_errors() {
  L2719: fn test_opposing(flag: &str, opposing_flags: &[&str]) {
  L2740: fn test_error_if_hidden_not_set_and_pattern_starts_with_dot() {
  L2752: fn test_strip_cwd_prefix() {
  L2775: fn test_invalid_cwd() {
  L2796: fn test_git_dir() {
  L2836: fn test_gitignore_parent() {
  L2849: fn test_hyperlink() {
  L2867: fn test_ignore_contain() {
tests/testenv/mod.rs:
  L179: pub fn create_broken_symlink<P: AsRef<Path>>(
  L200: pub fn test_root(&self) -> PathBuf {
  L211: pub fn system_root(&self) -> PathBuf {
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
  L206: pub fn test_exe(&self) -> &PathBuf {
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
src/exec/mod.rs:
  L446: fn generate_custom_path_separator() {
  L461: fn generate_custom_path_separator_windows() {
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
