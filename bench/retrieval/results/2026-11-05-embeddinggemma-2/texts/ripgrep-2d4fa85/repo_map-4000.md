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
  L885: pub fn parents(&mut self, yes: bool) -> &mut WalkBuilder {
  L992: pub fn sort_by_file_name<F>(&mut self, cmp: F) -> &mut WalkBuilder
  L1008: pub fn same_file_system(&mut self, yes: bool) -> &mut WalkBuilder {
  L1046: pub fn filter_entry<P>(&mut self, filter: P) -> &mut WalkBuilder
  L1062: pub fn current_dir(
  L1080: fn get_or_set_current_dir(&self) -> Option<&Path> {
  L1117: pub struct Walk {
  L1134: pub fn new<P: AsRef<Path>>(path: P) -> Walk {
  L1143: pub fn from_iter<P: AsRef<Path>, I: IntoIterator<Item = P>>(
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
  L2063: fn skip_filesize(
  L2085: fn should_skip_entry(ig: &Ignore, dent: &DirEntry) -> bool {
  L2123: fn path_equals(dent: &DirEntry, handle: &Handle) -> Result<bool, Error> {
  L2224: fn normal_path(unix: &str) -> String {
  L2228: fn walk_collect(prefix: &Path, builder: &WalkBuilder) -> Vec<String> {
  L2245: fn walk_collect_parallel(
  L2261: fn walk_collect_entries_parallel(builder: &WalkBuilder) -> Vec<DirEntry> {
  L2310: fn custom_ignore() {
  L2326: fn custom_ignore_exclusive_use() {
  L2364: fn explicit_ignore() {
  L2380: fn explicit_ignore_exclusive_use() {
  L2436: fn gitignore_parent() {
  L2473: fn max_depth_does_not_load_unreachable_ignore_files() {
  L2591: fn first_path_not_symlink() {
  L2698: fn empty() {
  L31: pub struct DirEntry {
  L56: pub fn is_stdin(&self) -> bool {
  L61: pub fn metadata(&self) -> Result<Metadata, Error> {
  L68: pub fn file_type(&self) -> Option<FileType> {
  L76: pub fn file_name(&self) -> &OsStr {
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
  L191: fn file_type(&self) -> Option<FileType> {
  L200: fn file_name(&self) -> &OsStr {
  L209: fn depth(&self) -> usize {
  L219: fn ino(&self) -> Option<u64> {
  L230: fn is_dir(&self) -> bool {
  L238: struct DirEntryRaw {
  L259: fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
  L284: fn metadata(&self) -> Result<Metadata, Error> {
  L289: fn metadata_internal(&self) -> Result<fs::Metadata, Error> {
  L299: fn metadata_internal(&self) -> Result<fs::Metadata, Error> {
  L310: fn file_type(&self) -> FileType {
  L314: fn file_name(&self) -> &OsStr {
  L318: fn depth(&self) -> usize {
  L323: fn ino(&self) -> u64 {
  L327: fn from_entry(
  L339: fn from_entry_os(
  L358: fn from_entry_os(
  L377: fn from_entry_os(
  L515: enum Sorter {
  L521: struct Filter(Arc<dyn Fn(&DirEntry) -> bool + Send + Sync + 'static>);
  L1353: pub trait ParallelVisitorBuilder<'s> {
  L1355: fn build(&mut self) -> Box<dyn ParallelVisitor + 's>;
  L1361: fn build(&mut self) -> Box<dyn ParallelVisitor + 's> {
  L1371: pub trait ParallelVisitor: Send {
  L1374: fn visit(&mut self, entry: Result<DirEntry, Error>) -> WalkState;
  L1377: struct FnBuilder<F> {
  L1384: fn build(&mut self) -> Box<dyn ParallelVisitor + 's> {
  L1390: type FnVisitor<'s> =
  L1393: struct FnVisitorImp<'s> {
  L1398: fn visit(&mut self, entry: Result<DirEntry, Error>) -> WalkState {
  L1547: enum Message {
  L1560: struct Work {
  L1571: struct ReadDirResult {
  L1578: fn is_dir(&self) -> bool {
  L1583: fn is_symlink(&self) -> bool {
  L1591: fn add_parents(&mut self) -> Option<Error> {
  L1616: fn read_dir(&mut self) -> Result<ReadDirResult, Error> {
  L1649: struct Stack {
  L1662: fn new_for_each_thread(threads: usize, init: Vec<Message>) -> Vec<Stack> {
  L1692: fn push(&self, msg: Message) {
  L1697: fn pop(&self) -> Option<Message> {
  L1702: fn steal(&self) -> Option<Message> {
  L1721: struct Worker<'s> {
  L1761: fn run(mut self) {
  L1769: fn run_one(&mut self, mut work: Work) -> WalkState {
  L1874: fn generate_work(
  L1942: fn get_work(&mut self) -> Option<Work> {
  L1995: fn quit_now(&self) {
  L2000: fn is_quit_now(&self) -> bool {
  L2005: fn send(&self, work: Work) {
  L2010: fn send_quit(&self) {
  L2015: fn recv(&self) -> Option<Message> {
  L2020: fn deactivate_worker(&self) -> usize {
  L2025: fn activate_worker(&self) {
  L2031: fn drop(&mut self) {
  L2038: fn check_symlink_loop(
  L2105: fn stdout_handle() -> Option<Handle> {
  L2152: fn walkdir_is_dir(dent: &walkdir::DirEntry) -> bool {
  L2164: fn is_same_file_system(root_device: u64, path: &Path) -> Result<bool, Error> {
  L2171: fn device_num<P: AsRef<Path>>(path: P) -> io::Result<u64> {
  L2178: fn device_num<P: AsRef<Path>>(path: P) -> io::Result<u64> {
  L2186: fn device_num<P: AsRef<Path>>(_: P) -> io::Result<u64> {
  L2204: fn wfile<P: AsRef<Path>>(path: P, contents: &str) {
  L2209: fn wfile_size<P: AsRef<Path>>(path: P, size: u64) {
  L2215: fn symlink<P: AsRef<Path>, Q: AsRef<Path>>(src: P, dst: Q) {
  L2220: fn mkdirp<P: AsRef<Path>>(path: P) {
  L2277: fn mkpaths(paths: &[&str]) -> Vec<String> {
  L2283: fn tmpdir() -> TempDir {
  L2287: fn assert_paths(prefix: &Path, builder: &WalkBuilder, expected: &[&str]) {
  L2295: fn no_ignores() {
  L2346: fn gitignore() {
  L2406: fn hidden_with_trailing_dot() {
  L2449: fn max_depth() {
  L2494: fn min_depth() {
  L2539: fn max_filesize() {
  L2574: fn symlinks() {
  L2612: fn symlink_loop() {
  L2628: fn same_file_system() {
  L2658: fn no_read_permissions() {
  L2676: fn filter() {
  L2707: fn from_iter() {
  L2746: fn panic_in_parallel() {
  L2762: fn panic_in_parallel_builder() {
crates/ignore/src/dir.rs:
  L1427: fn absolute_parent_matchers_are_cached_across_roots() {
  L336: fn add_child_path_with_found_ignore_files(
  L466: fn has_any_ignore_rules(&self) -> bool {
  L721: pub(crate) fn is_absolute_parent(&self) -> bool {
  L966: pub(crate) fn git_ignore(&mut self, yes: bool) -> &mut IgnoreBuilder {
  L171: pub(crate) fn path(&self) -> &Path {
  L176: pub(crate) fn is_root(&self) -> bool {
  L181: pub(crate) fn parent(&self) -> Option<Ignore> {
  L300: fn add_child_path(&self, dir: &Path) -> (IgnoreInner, Option<Error>) {
  L304: fn collect_ignore_files(
  L544: pub(crate) fn matched_ignore<'a>(
  L690: fn absolute_base(&self) -> Option<&Path> {
  L717: pub(crate) fn path(&self) -> &Path {
  L893: pub(crate) fn add_ignore(&mut self, ig: Gitignore) -> &mut IgnoreBuilder {
  L904: pub(crate) fn add_custom_ignore_filename<S: AsRef<OsStr>>(
  L931: pub(crate) fn ignore(&mut self, yes: bool) -> &mut IgnoreBuilder {
  L955: pub(crate) fn git_global(&mut self, yes: bool) -> &mut IgnoreBuilder {
  L977: pub(crate) fn git_exclude(&mut self, yes: bool) -> &mut IgnoreBuilder {
  L987: pub(crate) fn require_git(&mut self, yes: bool) -> &mut IgnoreBuilder {
  L995: pub(crate) fn ignore_case_insensitive(
  L1251: fn custom_ignore_over_ignore() {
  L1384: fn absolute_parent() {
  L1407: fn absolute_parent_anchored() {
  L1523: fn multi_root_custom_ignore_order_independent() {
  L38: pub(crate) struct IgnoreMatch<'a>(IgnoreMatchInner<'a>);
  L44: enum IgnoreMatchInner<'a> {
  L52: fn overrides(x: overrides::Glob<'a>) -> IgnoreMatch<'a> {
  L56: fn gitignore(x: &'a gitignore::Glob) -> IgnoreMatch<'a> {
  L60: fn types(x: types::Glob<'a>) -> IgnoreMatch<'a> {
  L64: fn hidden() -> IgnoreMatch<'static> {
  L72: struct IgnoreOptions {
  L94: pub(crate) struct Ignore {
  L115: struct IgnoreInner {
  L192: pub(crate) fn add_parents<P: AsRef<Path>>(
  L264: pub(crate) fn add_child<P: AsRef<Path>>(
  L280: pub(crate) fn add_child_with_entries<P: AsRef<Path>>(
  L481: pub(crate) fn matched_dir_entry<'a>(
  L496: pub(crate) fn matched<'a, P: AsRef<Path>>(
  L684: pub(crate) fn parents(&self) -> Parents<'_> {
  L703: struct IgnoreFilesFound {
  L712: pub(crate) struct IgnoreRef<'a> {
  L749: pub(crate) struct IgnoreBuilder {
  L780: pub(crate) fn new() -> IgnoreBuilder {
  L805: pub(crate) fn build(&self) -> Ignore {
  L813: pub(crate) fn build_with_cwd(&self, cwd: Option<PathBuf>) -> Ignore {
  L861: pub(crate) fn current_dir(
  L874: pub(crate) fn overrides(
  L887: pub(crate) fn types(&mut self, types: Types) -> &mut IgnoreBuilder {
  L915: pub(crate) fn hidden(&mut self, yes: bool) -> &mut IgnoreBuilder {
  L921: pub(crate) fn is_hidden(&self) -> bool {
  L942: pub(crate) fn parents(&mut self, yes: bool) -> &mut IgnoreBuilder {
  L1057: fn resolve_git_commondir(
  L1141: fn explicit_ignore() {
  L1156: fn git_exclude() {
  L1195: fn gitignore_no_git() {
  L1207: fn gitignore_allowed_no_git() {
  L1222: fn ignore() {
  L1234: fn custom_ignore() {
  L1267: fn custom_ignore_precedence() {
  L1285: fn ignore_over_gitignore() {
  L1342: fn errored_partial_and_ignore() {
  L1353: fn not_present_empty() {
  L1361: fn stops_at_git_dir() {
  L1460: fn multi_root_gitignore_order_independent() {
  L1566: fn git_info_exclude_in_linked_worktree() {
  L727: pub(crate) struct Parents<'a>(Option<IgnoreRef<'a>>);
  L730: type Item = IgnoreRef<'a>;
