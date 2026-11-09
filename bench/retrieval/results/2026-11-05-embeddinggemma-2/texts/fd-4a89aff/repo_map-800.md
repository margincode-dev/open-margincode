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
