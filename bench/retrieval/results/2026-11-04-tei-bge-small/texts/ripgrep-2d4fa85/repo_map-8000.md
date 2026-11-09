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
  L732: fn next(&mut self) -> Option<IgnoreRef<'a>> {
  L1012: pub(crate) fn create_gitignore<T: AsRef<OsStr>>(
  L1105: fn strip_if_is_prefix<'a, P: AsRef<Path> + ?Sized>(
  L1120: fn wfile<P: AsRef<Path>>(path: P, contents: &str) {
  L1125: fn mkdirp<P: AsRef<Path>>(path: P) {
  L1129: fn partial(err: Error) -> Vec<Error> {
  L1136: fn tmpdir() -> TempDir {
  L1169: fn gitignore() {
  L1182: fn gitignore_with_jj() {
  L1297: fn exclude_lowest() {
  L1312: fn errored() {
  L1321: fn errored_both() {
  L1331: fn errored_partial() {
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
crates/printer/src/standard.rs:
  L229: pub fn path(&mut self, yes: bool) -> &mut StandardBuilder {
  L238: pub fn only_matching(&mut self, yes: bool) -> &mut StandardBuilder {
  L444: pub fn separator_path(&mut self, sep: Option<u8>) -> &mut StandardBuilder {
  L457: pub fn path_terminator(
  L541: pub fn sink_with_path<'p, 's, M, P>(
  L1064: fn sink_slow_multi_line_only_matching(&self) -> io::Result<()> {
  L1358: fn write_path_line(&self) -> io::Result<()> {
  L1441: fn write_path(&self, path: &PrinterPath) -> io::Result<()> {
  L1448: fn write_path_hyperlink(&self, path: &PrinterPath) -> io::Result<()> {
  L1546: fn path(&self) -> Option<&'a PrinterPath<'a>> {
  L1660: fn write_path(&mut self) -> io::Result<()> {
  L2025: fn path() {
  L2079: fn separator_path() {
  L2103: fn path_terminator() {
  L3002: fn only_matching() {
  L3028: fn only_matching_multi_line1() {
  L3056: fn only_matching_multi_line2() {
  L3085: fn only_matching_max_columns() {
  L3112: fn only_matching_max_columns_preview() {
  L3140: fn only_matching_max_columns_multi_line1() {
  L3173: fn only_matching_max_columns_preview_multi_line1() {
  L3207: fn only_matching_max_columns_multi_line2() {
  L3237: fn only_matching_max_columns_preview_multi_line2() {
  L3673: fn replacement_only_matching() {
  L3837: fn invert_context_only_matching() {
  L3868: fn invert_context_only_matching_multi_line() {
  L3900: fn regression_search_empty_with_crlf() {
  L36: struct Config {
  L60: fn default() -> Config {
  L99: pub struct StandardBuilder {
  L105: pub fn new() -> StandardBuilder {
  L127: pub fn build<W: WriteColor>(&self, wtr: W) -> Standard<W> {
  L140: pub fn build_no_color<W: io::Write>(
  L164: pub fn color_specs(&mut self, specs: ColorSpecs) -> &mut StandardBuilder {
  L180: pub fn hyperlink(
  L203: pub fn stats(&mut self, yes: bool) -> &mut StandardBuilder {
  L218: pub fn heading(&mut self, yes: bool) -> &mut StandardBuilder {
  L260: pub fn per_match(&mut self, yes: bool) -> &mut StandardBuilder {
  L275: pub fn per_match_one_line(&mut self, yes: bool) -> &mut StandardBuilder {
  L290: pub fn replacement(
  L306: pub fn max_columns(&mut self, limit: Option<u64>) -> &mut StandardBuilder {
  L321: pub fn max_columns_preview(&mut self, yes: bool) -> &mut StandardBuilder {
  L335: pub fn column(&mut self, yes: bool) -> &mut StandardBuilder {
  L347: pub fn byte_offset(&mut self, yes: bool) -> &mut StandardBuilder {
  L356: pub fn trim_ascii(&mut self, yes: bool) -> &mut StandardBuilder {
  L374: pub fn separator_search(
  L391: pub fn separator_context(
  L407: pub fn separator_field_match(
  L423: pub fn separator_field_context(
  L480: pub struct Standard<W> {
  L494: pub fn new(wtr: W) -> Standard<W> {
  L505: pub fn new_no_color(wtr: W) -> Standard<NoColor<W>> {
  L515: pub fn sink<'s, M: Matcher>(
  L578: fn needs_match_granularity(&self) -> bool {
  L600: pub fn has_written(&self) -> bool {
  L605: pub fn get_mut(&mut self) -> &mut W {
  L611: pub fn into_inner(self) -> W {
  L639: pub struct StandardSink<'p, 's, M: Matcher, W> {
  L658: pub fn has_match(&self) -> bool {
  L669: pub fn match_count(&self) -> u64 {
  L683: pub fn binary_byte_offset(&self) -> Option<u64> {
  L692: pub fn stats(&self) -> Option<&Stats> {
  L698: fn record_matches(
  L741: fn replace(
  L764: type Error = io::Error;
  L766: fn matched(
  L793: fn context(
  L815: fn context_break(
  L823: fn binary_data(
  L841: fn begin(&mut self, _searcher: &Searcher) -> Result<bool, io::Error> {
  L849: fn finish(
  L876: struct StandardImpl<'a, M: Matcher, W> {
  L886: fn new(
  L900: fn from_match(
  L915: fn from_context(
  L928: fn sink(&self) -> io::Result<()> {
  L951: fn sink_fast(&self) -> io::Result<()> {
  L969: fn sink_fast_multi_line(&self) -> io::Result<()> {
  L994: fn sink_slow(&self) -> io::Result<()> {
  L1029: fn sink_slow_multi_line(&self) -> io::Result<()> {
  L1115: fn sink_slow_multi_per_match(&self) -> io::Result<()> {
  L1176: fn write_prelude(
  L1192: fn write_line(&self, line: &[u8]) -> io::Result<()> {
  L1219: fn write_colored_line(
  L1247: fn write_colored_matches(
  L1290: fn write_exceeded_line(
  L1370: fn write_search_prelude(&self) -> io::Result<()> {
  L1388: fn write_binary_message(&self, offset: u64) -> io::Result<()> {
  L1421: fn write_context_separator(&self) -> io::Result<()> {
  L1429: fn write_line_term(&self) -> io::Result<()> {
  L1433: fn write_spec(&self, spec: &ColorSpec, buf: &[u8]) -> io::Result<()> {
  L1454: fn start_hyperlink(
  L1468: fn end_hyperlink(
  L1475: fn start_color_match(&self) -> io::Result<()> {
  L1484: fn end_color_match(&self) -> io::Result<()> {
  L1499: fn highlight_on(&self) -> bool {
  L1503: fn start_line_highlight(&self) -> io::Result<()> {
  L1512: fn end_line_highlight(&self) -> io::Result<()> {
  L1519: fn write(&self, buf: &[u8]) -> io::Result<()> {
  L1523: fn trim_line_terminator(&self, buf: &[u8], line: &mut Match) {
  L1527: fn has_line_terminator(&self, buf: &[u8]) -> bool {
  L1531: fn is_context(&self) -> bool {
  L1536: fn config(&self) -> &'a Config {
  L1541: fn wtr(&self) -> &'a RefCell<CounterWriter<W>> {
  L1552: fn separator_field(&self) -> &[u8] {
  L1562: fn exceeds_max_columns(&self, line: &[u8]) -> bool {
  L1573: fn multi_line(&self) -> bool {
  L1582: fn trim_ascii_prefix(&self, slice: &[u8], range: &mut Match) {
  L1594: struct PreludeWriter<'a, M: Matcher, W> {
  L1602: enum PreludeSeparator {
  L1614: fn new(std: &'a StandardImpl<'a, M, W>) -> PreludeWriter<'a, M, W> {
  L1630: fn start(
  L1647: fn end(&mut self) -> io::Result<()> {
  L1681: fn write_line_number(&mut self, line: Option<u64>) -> io::Result<()> {
  L1692: fn write_column_number(&mut self, column: Option<u64>) -> io::Result<()> {
  L1706: fn write_byte_offset(&mut self, offset: u64) -> io::Result<()> {
  L1722: fn write_separator(&mut self) -> io::Result<()> {
  L1739: fn config(&self) -> &Config {
  L1753: const SHERLOCK: &'static str = "\
  L1763: const SHERLOCK_CRLF: &'static str = "\
  L1772: fn printer_contents(printer: &mut Standard<NoColor<Vec<u8>>>) -> String {
  L1776: fn printer_contents_ansi(printer: &mut Standard<Ansi<Vec<u8>>>) -> String {
  L1781: fn reports_match() {
  L1804: fn reports_binary() {
  L1830: fn reports_stats() {
  L1857: fn reports_stats_multiple() {
  L1894: fn context_break() {
  L1924: fn context_break_multiple_no_heading() {
  L1974: fn context_break_multiple_heading() {
  L2048: fn separator_field() {
  L2127: fn heading() {
  L2151: fn no_heading() {
  L2174: fn no_heading_multiple() {
  L2210: fn heading_multiple() {
  L2248: fn trim_ascii() {
  L2271: fn trim_ascii_multi_line() {
  L2296: fn trim_ascii_with_line_term() {
  L2321: fn line_number() {
  L2343: fn line_number_multi_line() {
  L2369: fn column_number() {
  L2392: fn column_number_multi_line() {
  L2419: fn byte_offset() {
  L2443: fn byte_offset_multi_line() {
  L2471: fn max_columns() {
  L2495: fn max_columns_preview() {
  L2520: fn max_columns_with_count() {
  L2545: fn max_columns_with_count_preview_no_match() {
  L2571: fn max_columns_with_count_preview_one_match() {
  L2597: fn max_columns_with_count_preview_two_matches() {
  L2624: fn max_columns_multi_line() {
  L2649: fn max_columns_multi_line_preview() {
  L2679: fn max_matches() {
  L2701: fn max_matches_context() {
  L2800: fn max_matches_context_invert() {
  L2905: fn max_matches_multi_line1() {
  L2928: fn max_matches_multi_line2() {
  L2953: fn max_matches_multi_line3() {
  L2977: fn max_matches_multi_line4() {
  L3268: fn per_match() {
  L3294: fn per_match_multi_line1() {
  L3322: fn per_match_multi_line2() {
  L3351: fn per_match_multi_line3() {
  L3380: fn per_match_multi_line1_only_first_line() {
  L3409: fn per_match_multi_line2_only_first_line() {
  L3437: fn per_match_multi_line3_only_first_line() {
  L3465: fn replacement_passthru() {
  L3494: fn replacement() {
  L3523: fn replacement_multi_line() {
  L3545: fn replacement_multi_line_diff_line_term() {
  L3571: fn replacement_multi_line_combine_lines() {
  L3593: fn replacement_max_columns() {
  L3619: fn replacement_max_columns_preview1() {
  L3646: fn replacement_max_columns_preview2() {
  L3700: fn replacement_per_match() {
  L3727: fn invert() {
  L3752: fn invert_multi_line() {
  L3778: fn invert_context() {
  L3807: fn invert_context_multi_line() {
  L3917: fn regression_after_context_with_match() {
  L3952: fn regression_crlf_preserve() {
crates/ignore/src/incremental.rs:
  L413: fn root_ignore(
  L134: pub fn root(&self) -> &Path {
  L310: fn matched_with_errors_ignore(
  L393: fn match_path(
  L679: fn parent_ignore_and_custom_ignore() {
  L788: fn respects_git_repository_boundaries() {
  L848: fn cached_ignored_parent_matches_children() {
  L57: pub struct IncrementalIgnore {
  L73: pub(crate) struct IncrementalIgnoreOptions {
  L82: enum RootIgnore {
  L106: pub(crate) fn new(
  L155: pub fn normalize<P: AsRef<Path>>(&self, path: P) -> Option<PathBuf> {
  L194: pub fn matched<P: AsRef<Path>>(
  L213: pub fn matched_with_errors<P: AsRef<Path>>(
  L224: fn matched_with_errors_impl(
  L464: fn ignore() -> IncrementalMatch {
  L491: pub fn is_ignore(&self) -> bool {
  L767: fn ignored_ancestor_wins() {
  L833: fn cached_allowed_parent_matches_children() {
  L863: fn cached_parent_matcher_is_used_for_directory() {
  L1064: fn symlink_path_stays_under_lexical_root() {
  L94: enum CachedDir {
  L449: pub struct IncrementalMatch {
  L456: fn none(is_dir: bool) -> IncrementalMatch {
  L472: fn from_match(mat: Match<()>, is_dir: bool) -> IncrementalMatch {
  L477: fn no_descent(self) -> IncrementalMatch {
  L481: fn not_within_depth(self) -> IncrementalMatch {
  L486: pub fn is_none(&self) -> bool {
  L497: pub fn is_whitelist(&self) -> bool {
  L513: pub fn should_descend(&self) -> bool {
  L526: pub fn is_within_depth(&self) -> bool {
  L532: pub fn invert(self) -> IncrementalMatch {
  L542: fn normalize_absolute(path: &Path) -> Option<PathBuf> {
  L605: fn matchers(builder: &WalkBuilder) -> Vec<IncrementalIgnore> {
  L629: fn nested_parent_gitignores() {
  L648: fn anchored_child_rule_uses_child_root() {
  L662: fn leading_dot_slash_impacts_matching() {
  L751: fn directly_ignored_directory_is_not_descended() {
  L811: fn compiled_matchers_are_reused() {
  L881: fn compiled_multi_matchers_are_reused() {
  L901: fn compiled_multi_child_matchers_are_not_reused() {
  L941: fn partial_errors_keep_valid_rules() {
