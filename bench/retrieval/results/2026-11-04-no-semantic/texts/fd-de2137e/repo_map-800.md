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
