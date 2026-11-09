crates/ignore/src/walk.rs:
  L921: pub fn git_ignore(&mut self, yes: bool) -> &mut WalkBuilder {
  L560: pub fn empty() -> WalkBuilder {
  L675: pub fn build_matchers(&self) -> Vec<IncrementalIgnore> {
  L791: pub fn add_ignore<P: AsRef<Path>>(&mut self, path: P) -> Option<Error> {
  L820: pub fn add_custom_ignore_filename<S: AsRef<OsStr>>(
  L896: pub fn ignore(&mut self, yes: bool) -> &mut WalkBuilder {
  L910: pub fn git_global(&mut self, yes: bool) -> &mut WalkBuilder {
  L932: pub fn git_exclude(&mut self, yes: bool) -> &mut WalkBuilder {
  L946: pub fn require_git(&mut self, yes: bool) -> &mut WalkBuilder {
  L954: pub fn ignore_case_insensitive(&mut self, yes: bool) -> &mut WalkBuilder {
  L973: pub fn sort_by_file_path<F>(&mut self, cmp: F) -> &mut WalkBuilder
  L1024: pub fn skip_stdout(&mut self, yes: bool) -> &mut WalkBuilder {
  L1104: fn build_ignore(&self) -> Ignore {
  L1149: fn skip_entry(&self, ent: &DirEntry) -> Result<bool, Error> {
  L38: pub fn path(&self) -> &Path {
  L44: pub fn into_path(self) -> PathBuf {
  L49: pub fn path_is_symlink(&self) -> bool {
  L138: fn path(&self) -> &Path {
  L147: fn into_path(self) -> PathBuf {
  L156: fn path_is_symlink(&self) -> bool {
  L272: fn path(&self) -> &Path {
  L276: fn into_path(self) -> PathBuf {
  L280: fn path_is_symlink(&self) -> bool {
  L389: fn from_path(
  L406: fn from_path(
  L427: fn from_path(
  L488: pub struct WalkBuilder {
  L524: fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
  L552: pub fn new<P: AsRef<Path>>(path: P) -> WalkBuilder {
  L582: pub fn from_iter<P: AsRef<Path>, I: IntoIterator<Item = P>>(
  L593: pub fn build(&self) -> Walk {
  L701: pub fn build_parallel(&self) -> WalkParallel {
  L722: pub fn add<P: AsRef<Path>>(&mut self, path: P) -> &mut WalkBuilder {
  L730: pub fn max_depth(&mut self, depth: Option<usize>) -> &mut WalkBuilder {
  L744: pub fn min_depth(&mut self, depth: Option<usize>) -> &mut WalkBuilder {
  L756: pub fn follow_links(&mut self, yes: bool) -> &mut WalkBuilder {
  L762: pub fn max_filesize(&mut self, filesize: Option<u64>) -> &mut WalkBuilder {
  L773: pub fn threads(&mut self, n: usize) -> &mut WalkBuilder {
  L833: pub fn overrides(&mut self, overrides: Override) -> &mut WalkBuilder {
  L843: pub fn types(&mut self, types: Types) -> &mut WalkBuilder {
  L862: pub fn standard_filters(&mut self, yes: bool) -> &mut WalkBuilder {
  L874: pub fn hidden(&mut self, yes: bool) -> &mut WalkBuilder {
