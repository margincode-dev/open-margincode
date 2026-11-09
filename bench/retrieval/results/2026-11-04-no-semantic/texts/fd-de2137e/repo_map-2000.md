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
