crates/ignore/src/walk.rs:
  L973: pub fn sort_by_file_path<F>(&mut self, cmp: F) -> &mut WalkBuilder
  L791: pub fn add_ignore<P: AsRef<Path>>(&mut self, path: P) -> Option<Error> {
  L820: pub fn add_custom_ignore_filename<S: AsRef<OsStr>>(
  L896: pub fn ignore(&mut self, yes: bool) -> &mut WalkBuilder {
  L921: pub fn git_ignore(&mut self, yes: bool) -> &mut WalkBuilder {
  L954: pub fn ignore_case_insensitive(&mut self, yes: bool) -> &mut WalkBuilder {
  L992: pub fn sort_by_file_name<F>(&mut self, cmp: F) -> &mut WalkBuilder
  L1008: pub fn same_file_system(&mut self, yes: bool) -> &mut WalkBuilder {
  L1104: fn build_ignore(&self) -> Ignore {
  L38: pub fn path(&self) -> &Path {
  L44: pub fn into_path(self) -> PathBuf {
  L49: pub fn path_is_symlink(&self) -> bool {
  L68: pub fn file_type(&self) -> Option<FileType> {
  L76: pub fn file_name(&self) -> &OsStr {
  L138: fn path(&self) -> &Path {
  L147: fn into_path(self) -> PathBuf {
  L156: fn path_is_symlink(&self) -> bool {
  L191: fn file_type(&self) -> Option<FileType> {
  L200: fn file_name(&self) -> &OsStr {
  L272: fn path(&self) -> &Path {
  L276: fn into_path(self) -> PathBuf {
  L280: fn path_is_symlink(&self) -> bool {
  L289: fn metadata_internal(&self) -> Result<fs::Metadata, Error> {
  L299: fn metadata_internal(&self) -> Result<fs::Metadata, Error> {
  L310: fn file_type(&self) -> FileType {
  L314: fn file_name(&self) -> &OsStr {
  L389: fn from_path(
  L406: fn from_path(
  L427: fn from_path(
  L488: pub struct WalkBuilder {
  L524: fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
  L552: pub fn new<P: AsRef<Path>>(path: P) -> WalkBuilder {
  L560: pub fn empty() -> WalkBuilder {
  L582: pub fn from_iter<P: AsRef<Path>, I: IntoIterator<Item = P>>(
  L593: pub fn build(&self) -> Walk {
  L675: pub fn build_matchers(&self) -> Vec<IncrementalIgnore> {
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
  L885: pub fn parents(&mut self, yes: bool) -> &mut WalkBuilder {
  L910: pub fn git_global(&mut self, yes: bool) -> &mut WalkBuilder {
  L932: pub fn git_exclude(&mut self, yes: bool) -> &mut WalkBuilder {
  L946: pub fn require_git(&mut self, yes: bool) -> &mut WalkBuilder {
  L1024: pub fn skip_stdout(&mut self, yes: bool) -> &mut WalkBuilder {
  L1046: pub fn filter_entry<P>(&mut self, filter: P) -> &mut WalkBuilder
  L1062: pub fn current_dir(
  L1080: fn get_or_set_current_dir(&self) -> Option<&Path> {
  L1117: pub struct Walk {
  L1134: pub fn new<P: AsRef<Path>>(path: P) -> Walk {
  L1143: pub fn from_iter<P: AsRef<Path>, I: IntoIterator<Item = P>>(
  L1149: fn skip_entry(&self, ent: &DirEntry) -> Result<bool, Error> {
  L1187: type Item = Result<DirEntry, Error>;
  L1190: fn next(&mut self) -> Option<Result<DirEntry, Error>> {
  L1270: struct WalkEventIter {
  L1277: enum WalkEvent {
  L1284: fn from(it: WalkDir) -> WalkEventIter {
  L1290: type Item = walkdir::Result<WalkEvent>;
  L1293: fn next(&mut self) -> Option<walkdir::Result<WalkEvent>> {
  L1325: pub enum WalkState {
  L1340: fn is_continue(&self) -> bool {
  L1344: fn is_quit(&self) -> bool {
  L1411: pub struct WalkParallel {
  L1428: pub fn run<'s, F>(self, mkf: F)
  L1452: pub fn visit(mut self, builder: &mut dyn ParallelVisitorBuilder<'_>) {
  L1537: fn threads(&self) -> usize {
  L1603: fn add_ignore(&mut self) {
  L2123: fn path_equals(dent: &DirEntry, handle: &Handle) -> Result<bool, Error> {
  L2164: fn is_same_file_system(root_device: u64, path: &Path) -> Result<bool, Error> {
  L2224: fn normal_path(unix: &str) -> String {
  L2228: fn walk_collect(prefix: &Path, builder: &WalkBuilder) -> Vec<String> {
  L2245: fn walk_collect_parallel(
  L2261: fn walk_collect_entries_parallel(builder: &WalkBuilder) -> Vec<DirEntry> {
  L2287: fn assert_paths(prefix: &Path, builder: &WalkBuilder, expected: &[&str]) {
  L2310: fn custom_ignore() {
  L2326: fn custom_ignore_exclusive_use() {
  L2364: fn explicit_ignore() {
  L2380: fn explicit_ignore_exclusive_use() {
  L2436: fn gitignore_parent() {
  L2473: fn max_depth_does_not_load_unreachable_ignore_files() {
  L2591: fn first_path_not_symlink() {
  L2628: fn same_file_system() {
  L31: pub struct DirEntry {
  L56: pub fn is_stdin(&self) -> bool {
  L61: pub fn metadata(&self) -> Result<Metadata, Error> {
  L81: pub fn depth(&self) -> usize {
  L89: pub fn ino(&self) -> Option<u64> {
  L98: pub fn error(&self) -> Option<&Error> {
  L103: pub(crate) fn is_dir(&self) -> bool {
  L107: fn new_stdin() -> DirEntry {
  L111: fn new_walkdir(dent: walkdir::DirEntry, err: Option<Error>) -> DirEntry {
  L115: fn new_raw(dent: DirEntryRaw, err: Option<Error>) -> DirEntry {
  L131: enum DirEntryInner {
  L165: fn is_stdin(&self) -> bool {
  L172: fn metadata(&self) -> Result<Metadata, Error> {
  L209: fn depth(&self) -> usize {
  L219: fn ino(&self) -> Option<u64> {
  L230: fn is_dir(&self) -> bool {
  L238: struct DirEntryRaw {
  L259: fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
  L284: fn metadata(&self) -> Result<Metadata, Error> {
  L318: fn depth(&self) -> usize {
  L323: fn ino(&self) -> u64 {
  L327: fn from_entry(
  L339: fn from_entry_os(
  L358: fn from_entry_os(
  L377: fn from_entry_os(
  L515: enum Sorter {
  L521: struct Filter(Arc<dyn Fn(&DirEntry) -> bool + Send + Sync + 'static>);
  L1353: pub trait ParallelVisitorBuilder<'s> {
