src/exec/mod.rs:
  L164: fn new_command(pre_args: &[OsString]) -> io::Result<Command> {
  L30: pub struct CommandSet {
  L36: pub fn new<I, T, S>(input: I) -> Result<CommandSet>
  L51: pub fn new_batch<I, T, S>(input: I) -> Result<CommandSet>
  L72: pub fn in_batch_mode(&self) -> bool {
  L76: pub fn execute(
  L90: pub fn execute_batch<I>(&self, paths: I, limit: usize, path_separator: Option<&str>) -> ExitCode
  L125: struct CommandBuilder {
  L136: fn new(template: &CommandTemplate, limit: usize) -> io::Result<Self> {
  L173: fn push(&mut self, path: &Path, separator: Option<&str>) -> io::Result<()> {
  L191: fn finish(&mut self) -> io::Result<()> {
  L205: fn exit_code(&self) -> ExitCode {
  L215: struct CommandTemplate {
  L220: fn new<I, S>(input: I, mode: ExecutionMode) -> Result<CommandTemplate>
  L258: fn number_of_tokens(&self) -> usize {
  L266: fn generate(&self, input: &Path, path_separator: Option<&str>) -> io::Result<Command> {
  L434: fn command_set_no_args() {
  L22: pub enum ExecutionMode {
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
  L439: fn placeholder_as_executable_rejected() {
  L446: fn generate_custom_path_separator() {
  L461: fn generate_custom_path_separator_windows() {
src/walk.rs:
  L252: fn print(&mut self, entry: &DirEntry) -> Result<(), ExitCode> {
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
src/sanitize.rs:
  L49: pub fn maybe_sanitize<'a>(s: &'a str, is_terminal: bool) -> Cow<'a, str> {
  L159: fn maybe_sanitize_passthrough_when_not_terminal() {
  L28: pub fn sanitize_for_terminal(s: &str) -> Cow<'_, str> {
  L9: fn needs_escape(c: char) -> bool {
  L62: fn preserves_safe_content() {
  L80: fn strips_osc52_clipboard_payload() {
  L88: fn strips_cr_output_forgery() {
  L93: fn strips_osc8_hyperlink_injection() {
  L99: fn strips_del() {
  L104: fn strips_bel_and_null() {
  L110: fn strips_newline() {
  L115: fn escape_preserves_information() {
  L121: fn strips_c1_csi_and_osc_initiators() {
  L131: fn strips_bidi_overrides_and_zero_width() {
  L143: fn keeps_legitimate_unicode_features() {
src/exec/command.rs:
  L101: pub fn handle_cmd_error(cmd: Option<&Command>, err: io::Error) -> ExitCode {
  L9: struct Outputs {
  L13: pub struct OutputBuffer {
  L19: pub fn new(null_separator: bool) -> Self {
  L26: fn push(&mut self, stdout: Vec<u8>, stderr: Vec<u8>) {
  L30: fn write(self) {
  L60: pub fn execute_commands<I: Iterator<Item = io::Result<Command>>>(
src/error.rs:
  L5: pub fn print_error(msg: impl Into<String>) {
src/filter/owner.rs:
  L6: pub struct OwnerFilter {
  L12: enum Check<T> {
  L19: const IGNORE: Self = OwnerFilter {
  L27: pub fn from_string(input: &str) -> Result<Self> {
  L61: pub fn filter_ignore(self) -> Option<Self> {
  L69: pub fn matches(&self, md: &fs::Metadata) -> bool {
  L77: fn check(&self, v: T) -> bool {
  L85: fn parse<F>(s: Option<&str>, f: F) -> Result<Self>
src/dir_entry.rs:
  L175: fn dash_prefixed_paths_detected() {
  L182: fn safe_paths_not_flagged() {
  L12: enum DirEntryInner {
  L18: pub struct DirEntry {
  L26: pub fn normal(e: ignore::DirEntry) -> Self {
  L34: pub fn broken_symlink(path: PathBuf) -> Self {
  L42: pub fn path(&self) -> &Path {
  L49: pub fn into_path(self) -> PathBuf {
  L59: pub fn stripped_path(&self, config: &Config) -> &Path {
  L74: pub fn into_stripped_path(self, config: &Config) -> PathBuf {
  L82: pub fn file_type(&self) -> Option<FileType> {
