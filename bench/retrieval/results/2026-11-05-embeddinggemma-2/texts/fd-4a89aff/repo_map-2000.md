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
