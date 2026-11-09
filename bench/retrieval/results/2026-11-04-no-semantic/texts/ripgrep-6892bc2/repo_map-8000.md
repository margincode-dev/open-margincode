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
  L2063: fn skip_filesize(
  L2085: fn should_skip_entry(ig: &Ignore, dent: &DirEntry) -> bool {
  L2105: fn stdout_handle() -> Option<Handle> {
  L2152: fn walkdir_is_dir(dent: &walkdir::DirEntry) -> bool {
  L2171: fn device_num<P: AsRef<Path>>(path: P) -> io::Result<u64> {
  L2178: fn device_num<P: AsRef<Path>>(path: P) -> io::Result<u64> {
  L2186: fn device_num<P: AsRef<Path>>(_: P) -> io::Result<u64> {
  L2204: fn wfile<P: AsRef<Path>>(path: P, contents: &str) {
  L2209: fn wfile_size<P: AsRef<Path>>(path: P, size: u64) {
  L2215: fn symlink<P: AsRef<Path>, Q: AsRef<Path>>(src: P, dst: Q) {
  L2220: fn mkdirp<P: AsRef<Path>>(path: P) {
  L2277: fn mkpaths(paths: &[&str]) -> Vec<String> {
  L2283: fn tmpdir() -> TempDir {
  L2295: fn no_ignores() {
  L2346: fn gitignore() {
  L2406: fn hidden_with_trailing_dot() {
  L2449: fn max_depth() {
  L2494: fn min_depth() {
  L2539: fn max_filesize() {
  L2574: fn symlinks() {
  L2612: fn symlink_loop() {
  L2658: fn no_read_permissions() {
  L2676: fn filter() {
  L2698: fn empty() {
  L2707: fn from_iter() {
  L2746: fn panic_in_parallel() {
  L2762: fn panic_in_parallel_builder() {
crates/ignore/src/dir.rs:
  L344: fn add_child_path_with_found_ignore_files(
  L1261: fn custom_ignore_over_ignore() {
  L1486: fn absolute_parent_matchers_are_cached_across_roots() {
  L174: pub(crate) fn path(&self) -> &Path {
  L184: pub(crate) fn parent(&self) -> Option<Ignore> {
  L308: fn add_child_path(&self, dir: &Path) -> (IgnoreInner, Option<Error>) {
  L312: fn collect_ignore_files(
  L475: fn has_any_ignore_rules(&self) -> bool {
  L553: pub(crate) fn matched_ignore<'a>(
  L726: pub(crate) fn path(&self) -> &Path {
  L730: pub(crate) fn is_absolute_parent(&self) -> bool {
  L903: pub(crate) fn add_ignore(&mut self, ig: Gitignore) -> &mut IgnoreBuilder {
  L914: pub(crate) fn add_custom_ignore_filename<S: AsRef<OsStr>>(
  L941: pub(crate) fn ignore(&mut self, yes: bool) -> &mut IgnoreBuilder {
  L976: pub(crate) fn git_ignore(&mut self, yes: bool) -> &mut IgnoreBuilder {
  L1005: pub(crate) fn ignore_case_insensitive(
  L1295: fn ignore_over_gitignore() {
  L38: pub(crate) struct IgnoreMatch<'a>(IgnoreMatchInner<'a>);
  L44: enum IgnoreMatchInner<'a> {
  L52: fn overrides(x: overrides::Glob<'a>) -> IgnoreMatch<'a> {
  L56: fn gitignore(x: &'a gitignore::Glob) -> IgnoreMatch<'a> {
  L60: fn types(x: types::Glob<'a>) -> IgnoreMatch<'a> {
  L64: fn hidden() -> IgnoreMatch<'static> {
  L72: struct IgnoreOptions {
  L94: pub(crate) struct Ignore {
  L115: struct IgnoreInner {
  L179: pub(crate) fn is_root(&self) -> bool {
  L195: pub(crate) fn add_parents<P: AsRef<Path>>(
  L272: pub(crate) fn add_child<P: AsRef<Path>>(
  L288: pub(crate) fn add_child_with_entries<P: AsRef<Path>>(
  L490: pub(crate) fn matched_dir_entry<'a>(
  L505: pub(crate) fn matched<'a, P: AsRef<Path>>(
  L693: pub(crate) fn parents(&self) -> Parents<'_> {
  L699: fn absolute_base(&self) -> Option<&Path> {
  L712: struct IgnoreFilesFound {
  L721: pub(crate) struct IgnoreRef<'a> {
  L758: pub(crate) struct IgnoreBuilder {
  L789: pub(crate) fn new() -> IgnoreBuilder {
  L814: pub(crate) fn build(&self) -> Ignore {
  L822: pub(crate) fn build_with_cwd(&self, cwd: Option<PathBuf>) -> Ignore {
  L871: pub(crate) fn current_dir(
  L884: pub(crate) fn overrides(
  L897: pub(crate) fn types(&mut self, types: Types) -> &mut IgnoreBuilder {
  L925: pub(crate) fn hidden(&mut self, yes: bool) -> &mut IgnoreBuilder {
  L931: pub(crate) fn is_hidden(&self) -> bool {
  L952: pub(crate) fn parents(&mut self, yes: bool) -> &mut IgnoreBuilder {
  L965: pub(crate) fn git_global(&mut self, yes: bool) -> &mut IgnoreBuilder {
  L987: pub(crate) fn git_exclude(&mut self, yes: bool) -> &mut IgnoreBuilder {
  L997: pub(crate) fn require_git(&mut self, yes: bool) -> &mut IgnoreBuilder {
  L1151: fn explicit_ignore() {
  L1232: fn ignore() {
  L1244: fn custom_ignore() {
  L1277: fn custom_ignore_precedence() {
  L1352: fn errored_partial_and_ignore() {
  L1394: fn absolute_parent() {
  L1417: fn absolute_parent_anchored() {
  L1437: fn absolute_parent_rules_through_empty_directories() {
  L1468: fn absolute_parent_repository_without_rules() {
  L1582: fn multi_root_custom_ignore_order_independent() {
  L736: pub(crate) struct Parents<'a>(Option<IgnoreRef<'a>>);
  L739: type Item = IgnoreRef<'a>;
  L741: fn next(&mut self) -> Option<IgnoreRef<'a>> {
  L1022: pub(crate) fn create_gitignore<T: AsRef<OsStr>>(
  L1067: fn resolve_git_commondir(
  L1115: fn strip_if_is_prefix<'a, P: AsRef<Path> + ?Sized>(
  L1130: fn wfile<P: AsRef<Path>>(path: P, contents: &str) {
  L1135: fn mkdirp<P: AsRef<Path>>(path: P) {
  L1139: fn partial(err: Error) -> Vec<Error> {
  L1146: fn tmpdir() -> TempDir {
  L1166: fn git_exclude() {
  L1179: fn gitignore() {
  L1192: fn gitignore_with_jj() {
  L1205: fn gitignore_no_git() {
  L1217: fn gitignore_allowed_no_git() {
  L1307: fn exclude_lowest() {
  L1322: fn errored() {
  L1331: fn errored_both() {
  L1341: fn errored_partial() {
  L1363: fn not_present_empty() {
  L1371: fn stops_at_git_dir() {
  L1519: fn multi_root_gitignore_order_independent() {
  L1625: fn git_info_exclude_in_linked_worktree() {
crates/ignore/src/lib.rs:
  L263: fn with_path<P: AsRef<Path>>(self, path: P) -> Error {
  L380: fn push_ignore_io(&mut self, err: Error) {
  L392: fn maybe_push_ignore_io(&mut self, err: Option<Error>) {
  L437: pub fn is_ignore(&self) -> bool {
  L545: pub fn path(&self) -> &Path {
  L68: pub enum Error {
  L123: fn clone(&self) -> Error {
  L162: pub fn is_partial(&self) -> bool {
  L173: pub fn is_io(&self) -> bool {
  L207: pub fn io_error(&self) -> Option<&std::io::Error> {
  L232: pub fn into_io_error(self) -> Option<std::io::Error> {
  L254: pub fn depth(&self) -> Option<usize> {
  L271: fn with_depth(self, depth: usize) -> Error {
  L277: fn tagged<P: AsRef<Path>>(self, path: P, lineno: u64) -> Error {
  L287: fn from_walkdir(err: walkdir::Error) -> Error {
  L312: fn description(&self) -> &str {
  L328: fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
  L367: fn from(err: std::io::Error) -> Error {
  L373: struct PartialErrorBuilder(Vec<Error>);
  L376: fn push(&mut self, err: Error) {
  L386: fn maybe_push(&mut self, err: Option<Error>) {
  L398: fn into_error_option(mut self) -> Option<Error> {
  L416: pub enum Match<T> {
  L429: pub fn is_none(&self) -> bool {
  L446: pub fn is_whitelist(&self) -> bool {
  L455: pub fn invert(self) -> Match<T> {
  L464: pub fn inner(&self) -> Option<&T> {
  L475: pub fn map<U, F: FnOnce(T) -> U>(self, f: F) -> Match<U> {
  L484: pub fn or(self, other: Self) -> Self {
  L497: pub(crate) type Result<T> =
  L512: pub struct TempDir(PathBuf);
  L515: fn drop(&mut self) {
  L523: pub fn new() -> Result<TempDir> {
crates/core/flags/defs.rs:
  L3209: struct IgnoreFile;
  L3212: fn is_switch(&self) -> bool {
  L3215: fn name_long(&self) -> &'static str {
  L3218: fn doc_variable(&self) -> Option<&'static str> {
  L3221: fn doc_category(&self) -> Category {
  L3224: fn doc_short(&self) -> &'static str {
  L3227: fn doc_long(&self) -> &'static str {
  L3242: fn completion_type(&self) -> CompletionType {
  L3246: fn update(&self, v: FlagValue, args: &mut LowArgs) -> anyhow::Result<()> {
  L3255: fn test_ignore_file() {
  L3272: struct IgnoreFileCaseInsensitive;
  L3275: fn is_switch(&self) -> bool {
  L3278: fn name_long(&self) -> &'static str {
  L3281: fn name_negated(&self) -> Option<&'static str> {
  L3284: fn doc_category(&self) -> Category {
  L3287: fn doc_short(&self) -> &'static str {
  L3290: fn doc_long(&self) -> &'static str {
  L3298: fn update(&self, v: FlagValue, args: &mut LowArgs) -> anyhow::Result<()> {
  L3306: fn test_ignore_file_case_insensitive() {
  L4632: struct NoIgnoreParent;
  L4635: fn is_switch(&self) -> bool {
  L4638: fn name_long(&self) -> &'static str {
  L4641: fn name_negated(&self) -> Option<&'static str> {
  L4644: fn doc_category(&self) -> Category {
  L4647: fn doc_short(&self) -> &'static str {
  L4650: fn doc_long(&self) -> &'static str {
  L4659: fn update(&self, v: FlagValue, args: &mut LowArgs) -> anyhow::Result<()> {
  L4667: fn test_no_ignore_parent() {
  L341: struct BeforeContext;
  L344: fn is_switch(&self) -> bool {
  L347: fn name_short(&self) -> Option<u8> {
  L350: fn name_long(&self) -> &'static str {
  L353: fn doc_variable(&self) -> Option<&'static str> {
  L356: fn doc_category(&self) -> Category {
  L359: fn doc_short(&self) -> &'static str {
  L362: fn doc_long(&self) -> &'static str {
  L371: fn update(&self, v: FlagValue, args: &mut LowArgs) -> anyhow::Result<()> {
  L379: fn test_before_context() {
  L578: struct ByteOffset;
  L581: fn is_switch(&self) -> bool {
  L584: fn name_short(&self) -> Option<u8> {
  L587: fn name_long(&self) -> &'static str {
  L590: fn name_negated(&self) -> Option<&'static str> {
  L593: fn doc_category(&self) -> Category {
  L596: fn doc_short(&self) -> &'static str {
  L599: fn doc_long(&self) -> &'static str {
  L611: fn update(&self, v: FlagValue, args: &mut LowArgs) -> anyhow::Result<()> {
  L619: fn test_byte_offset() {
  L1987: struct File;
  L1990: fn is_switch(&self) -> bool {
  L1993: fn name_short(&self) -> Option<u8> {
  L1996: fn name_long(&self) -> &'static str {
  L1999: fn doc_variable(&self) -> Option<&'static str> {
  L2002: fn doc_category(&self) -> Category {
  L2005: fn doc_short(&self) -> &'static str {
  L2008: fn doc_long(&self) -> &'static str {
  L2024: fn completion_type(&self) -> CompletionType {
  L2028: fn update(&self, v: FlagValue, args: &mut LowArgs) -> anyhow::Result<()> {
  L2037: fn test_file() {
  L3148: struct IgnoreCase;
  L3151: fn is_switch(&self) -> bool {
  L3154: fn name_short(&self) -> Option<u8> {
  L3157: fn name_long(&self) -> &'static str {
  L3160: fn doc_category(&self) -> Category {
  L3163: fn doc_short(&self) -> &'static str {
  L3166: fn doc_long(&self) -> &'static str {
  L3181: fn update(&self, v: FlagValue, args: &mut LowArgs) -> anyhow::Result<()> {
  L3190: fn test_ignore_case() {
  L4321: struct NoIgnore;
  L4324: fn is_switch(&self) -> bool {
  L4327: fn name_long(&self) -> &'static str {
  L4330: fn name_negated(&self) -> Option<&'static str> {
  L4333: fn doc_category(&self) -> Category {
  L4336: fn doc_short(&self) -> &'static str {
  L4339: fn doc_long(&self) -> &'static str {
  L4355: fn update(&self, v: FlagValue, args: &mut LowArgs) -> anyhow::Result<()> {
  L4368: fn test_no_ignore() {
  L4393: struct NoIgnoreDot;
  L4396: fn is_switch(&self) -> bool {
  L4399: fn name_long(&self) -> &'static str {
  L4402: fn name_negated(&self) -> Option<&'static str> {
  L4405: fn doc_category(&self) -> Category {
  L4408: fn doc_short(&self) -> &'static str {
  L4411: fn doc_long(&self) -> &'static str {
  L4421: fn update(&self, v: FlagValue, args: &mut LowArgs) -> anyhow::Result<()> {
  L4429: fn test_no_ignore_dot() {
  L4442: struct NoIgnoreExclude;
  L4445: fn is_switch(&self) -> bool {
  L4448: fn name_long(&self) -> &'static str {
  L4451: fn name_negated(&self) -> Option<&'static str> {
  L4454: fn doc_category(&self) -> Category {
  L4457: fn doc_short(&self) -> &'static str {
  L4460: fn doc_long(&self) -> &'static str {
  L4467: fn update(&self, v: FlagValue, args: &mut LowArgs) -> anyhow::Result<()> {
  L4475: fn test_no_ignore_exclude() {
  L4489: struct NoIgnoreFiles;
  L4492: fn is_switch(&self) -> bool {
  L4495: fn name_long(&self) -> &'static str {
  L4498: fn name_negated(&self) -> Option<&'static str> {
  L4501: fn doc_category(&self) -> Category {
  L4504: fn doc_short(&self) -> &'static str {
  L4507: fn doc_long(&self) -> &'static str {
  L4514: fn update(&self, v: FlagValue, args: &mut LowArgs) -> anyhow::Result<()> {
  L4522: fn test_no_ignore_files() {
  L4535: struct NoIgnoreGlobal;
  L4538: fn is_switch(&self) -> bool {
  L4541: fn name_long(&self) -> &'static str {
  L4544: fn name_negated(&self) -> Option<&'static str> {
  L4547: fn doc_category(&self) -> Category {
  L4550: fn doc_short(&self) -> &'static str {
  L4553: fn doc_long(&self) -> &'static str {
  L4561: fn update(&self, v: FlagValue, args: &mut LowArgs) -> anyhow::Result<()> {
  L4569: fn test_no_ignore_global() {
  L4583: struct NoIgnoreMessages;
  L4586: fn is_switch(&self) -> bool {
  L4589: fn name_long(&self) -> &'static str {
  L4592: fn name_negated(&self) -> Option<&'static str> {
  L4595: fn doc_category(&self) -> Category {
  L4598: fn doc_short(&self) -> &'static str {
  L4601: fn doc_long(&self) -> &'static str {
  L4610: fn update(&self, v: FlagValue, args: &mut LowArgs) -> anyhow::Result<()> {
  L4618: fn test_no_ignore_messages() {
  L4681: struct NoIgnoreVcs;
  L4684: fn is_switch(&self) -> bool {
  L4687: fn name_long(&self) -> &'static str {
  L4690: fn name_negated(&self) -> Option<&'static str> {
  L4693: fn doc_category(&self) -> Category {
  L4696: fn doc_short(&self) -> &'static str {
  L4699: fn doc_long(&self) -> &'static str {
  L4716: fn update(&self, v: FlagValue, args: &mut LowArgs) -> anyhow::Result<()> {
  L4724: fn test_no_ignore_vcs() {
  L5079: struct OneFileSystem;
  L5082: fn is_switch(&self) -> bool {
  L5085: fn name_long(&self) -> &'static str {
  L5088: fn name_negated(&self) -> Option<&'static str> {
  L5091: fn doc_category(&self) -> Category {
  L5094: fn doc_short(&self) -> &'static str {
  L5097: fn doc_long(&self) -> &'static str {
  L5117: fn update(&self, v: FlagValue, args: &mut LowArgs) -> anyhow::Result<()> {
  L5125: fn test_one_file_system() {
  L5186: struct PathSeparator;
  L5189: fn is_switch(&self) -> bool {
  L5192: fn name_long(&self) -> &'static str {
  L5195: fn doc_variable(&self) -> Option<&'static str> {
  L5198: fn doc_category(&self) -> Category {
  L5201: fn doc_short(&self) -> &'static str {
  L5204: fn doc_long(&self) -> &'static str {
  L5216: fn update(&self, v: FlagValue, args: &mut LowArgs) -> anyhow::Result<()> {
  L5239: fn test_path_separator() {
  L44: pub(super) const FLAGS: &[&dyn Flag] = &[
  L156: struct AfterContext;
  L159: fn is_switch(&self) -> bool {
  L162: fn name_short(&self) -> Option<u8> {
  L165: fn name_long(&self) -> &'static str {
  L168: fn doc_variable(&self) -> Option<&'static str> {
  L171: fn doc_category(&self) -> Category {
  L174: fn doc_short(&self) -> &'static str {
  L177: fn doc_long(&self) -> &'static str {
  L186: fn update(&self, v: FlagValue, args: &mut LowArgs) -> anyhow::Result<()> {
  L194: fn test_after_context() {
  L242: struct AutoHybridRegex;
  L245: fn is_switch(&self) -> bool {
  L248: fn name_long(&self) -> &'static str {
  L251: fn name_negated(&self) -> Option<&'static str> {
  L254: fn doc_category(&self) -> Category {
  L257: fn doc_short(&self) -> &'static str {
  L260: fn doc_long(&self) -> &'static str {
  L290: fn update(&self, v: FlagValue, args: &mut LowArgs) -> anyhow::Result<()> {
  L303: fn test_auto_hybrid_regex() {
  L427: struct Binary;
  L430: fn is_switch(&self) -> bool {
  L433: fn name_long(&self) -> &'static str {
  L436: fn name_negated(&self) -> Option<&'static str> {
  L439: fn doc_category(&self) -> Category {
  L442: fn doc_short(&self) -> &'static str {
  L445: fn doc_long(&self) -> &'static str {
  L481: fn update(&self, v: FlagValue, args: &mut LowArgs) -> anyhow::Result<()> {
  L493: fn test_binary() {
  L518: struct BlockBuffered;
  L521: fn is_switch(&self) -> bool {
  L524: fn name_long(&self) -> &'static str {
  L527: fn name_negated(&self) -> Option<&'static str> {
  L530: fn doc_category(&self) -> Category {
  L533: fn doc_short(&self) -> &'static str {
  L536: fn doc_long(&self) -> &'static str {
  L549: fn update(&self, v: FlagValue, args: &mut LowArgs) -> anyhow::Result<()> {
  L561: fn test_block_buffered() {
  L638: struct CaseSensitive;
  L641: fn is_switch(&self) -> bool {
  L644: fn name_short(&self) -> Option<u8> {
  L647: fn name_long(&self) -> &'static str {
  L650: fn doc_category(&self) -> Category {
  L653: fn doc_short(&self) -> &'static str {
  L656: fn doc_long(&self) -> &'static str {
  L669: fn update(&self, v: FlagValue, args: &mut LowArgs) -> anyhow::Result<()> {
  L678: fn test_case_sensitive() {
  L691: struct Color;
  L694: fn is_switch(&self) -> bool {
  L697: fn name_long(&self) -> &'static str {
  L700: fn doc_variable(&self) -> Option<&'static str> {
  L703: fn doc_category(&self) -> Category {
  L706: fn doc_short(&self) -> &'static str {
  L709: fn doc_long(&self) -> &'static str {
  L760: fn doc_choices(&self) -> &'static [&'static str] {
