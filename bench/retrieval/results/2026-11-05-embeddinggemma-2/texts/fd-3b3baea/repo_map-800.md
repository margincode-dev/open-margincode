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
