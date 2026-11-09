crates/ignore/src/dir.rs:
  L56: fn gitignore(x: &'a gitignore::Glob) -> IgnoreMatch<'a> {
  L308: fn collect_ignore_files(
  L340: fn add_child_path_with_found_ignore_files(
  L470: fn has_any_ignore_rules(&self) -> bool {
  L548: pub(crate) fn matched_ignore<'a>(
  L897: pub(crate) fn add_ignore(&mut self, ig: Gitignore) -> &mut IgnoreBuilder {
  L908: pub(crate) fn add_custom_ignore_filename<S: AsRef<OsStr>>(
  L935: pub(crate) fn ignore(&mut self, yes: bool) -> &mut IgnoreBuilder {
  L970: pub(crate) fn git_ignore(&mut self, yes: bool) -> &mut IgnoreBuilder {
  L999: pub(crate) fn ignore_case_insensitive(
  L1255: fn custom_ignore_over_ignore() {
  L1289: fn ignore_over_gitignore() {
  L38: pub(crate) struct IgnoreMatch<'a>(IgnoreMatchInner<'a>);
  L44: enum IgnoreMatchInner<'a> {
  L52: fn overrides(x: overrides::Glob<'a>) -> IgnoreMatch<'a> {
  L60: fn types(x: types::Glob<'a>) -> IgnoreMatch<'a> {
  L64: fn hidden() -> IgnoreMatch<'static> {
  L72: struct IgnoreOptions {
  L94: pub(crate) struct Ignore {
  L115: struct IgnoreInner {
  L171: pub(crate) fn path(&self) -> &Path {
  L176: pub(crate) fn is_root(&self) -> bool {
  L181: pub(crate) fn parent(&self) -> Option<Ignore> {
  L192: pub(crate) fn add_parents<P: AsRef<Path>>(
  L268: pub(crate) fn add_child<P: AsRef<Path>>(
  L284: pub(crate) fn add_child_with_entries<P: AsRef<Path>>(
  L304: fn add_child_path(&self, dir: &Path) -> (IgnoreInner, Option<Error>) {
  L485: pub(crate) fn matched_dir_entry<'a>(
  L500: pub(crate) fn matched<'a, P: AsRef<Path>>(
  L688: pub(crate) fn parents(&self) -> Parents<'_> {
  L694: fn absolute_base(&self) -> Option<&Path> {
  L707: struct IgnoreFilesFound {
  L716: pub(crate) struct IgnoreRef<'a> {
  L721: pub(crate) fn path(&self) -> &Path {
  L725: pub(crate) fn is_absolute_parent(&self) -> bool {
  L753: pub(crate) struct IgnoreBuilder {
  L784: pub(crate) fn new() -> IgnoreBuilder {
  L809: pub(crate) fn build(&self) -> Ignore {
  L817: pub(crate) fn build_with_cwd(&self, cwd: Option<PathBuf>) -> Ignore {
  L865: pub(crate) fn current_dir(
  L878: pub(crate) fn overrides(
  L891: pub(crate) fn types(&mut self, types: Types) -> &mut IgnoreBuilder {
  L919: pub(crate) fn hidden(&mut self, yes: bool) -> &mut IgnoreBuilder {
  L925: pub(crate) fn is_hidden(&self) -> bool {
  L946: pub(crate) fn parents(&mut self, yes: bool) -> &mut IgnoreBuilder {
  L959: pub(crate) fn git_global(&mut self, yes: bool) -> &mut IgnoreBuilder {
  L981: pub(crate) fn git_exclude(&mut self, yes: bool) -> &mut IgnoreBuilder {
  L991: pub(crate) fn require_git(&mut self, yes: bool) -> &mut IgnoreBuilder {
  L1016: pub(crate) fn create_gitignore<T: AsRef<OsStr>>(
  L1145: fn explicit_ignore() {
  L1173: fn gitignore() {
  L1186: fn gitignore_with_jj() {
  L1199: fn gitignore_no_git() {
  L1211: fn gitignore_allowed_no_git() {
  L1226: fn ignore() {
  L1238: fn custom_ignore() {
  L1271: fn custom_ignore_precedence() {
  L1346: fn errored_partial_and_ignore() {
  L1357: fn not_present_empty() {
  L1431: fn absolute_parent_matchers_are_cached_across_roots() {
  L1464: fn multi_root_gitignore_order_independent() {
  L1527: fn multi_root_custom_ignore_order_independent() {
  L731: pub(crate) struct Parents<'a>(Option<IgnoreRef<'a>>);
  L734: type Item = IgnoreRef<'a>;
  L736: fn next(&mut self) -> Option<IgnoreRef<'a>> {
  L1061: fn resolve_git_commondir(
  L1109: fn strip_if_is_prefix<'a, P: AsRef<Path> + ?Sized>(
  L1124: fn wfile<P: AsRef<Path>>(path: P, contents: &str) {
  L1129: fn mkdirp<P: AsRef<Path>>(path: P) {
  L1133: fn partial(err: Error) -> Vec<Error> {
  L1140: fn tmpdir() -> TempDir {
  L1160: fn git_exclude() {
  L1301: fn exclude_lowest() {
  L1316: fn errored() {
  L1325: fn errored_both() {
  L1335: fn errored_partial() {
  L1365: fn stops_at_git_dir() {
  L1388: fn absolute_parent() {
  L1411: fn absolute_parent_anchored() {
  L1570: fn git_info_exclude_in_linked_worktree() {
crates/ignore/src/lib.rs:
  L380: fn push_ignore_io(&mut self, err: Error) {
  L392: fn maybe_push_ignore_io(&mut self, err: Option<Error>) {
  L437: pub fn is_ignore(&self) -> bool {
  L68: pub enum Error {
  L123: fn clone(&self) -> Error {
  L162: pub fn is_partial(&self) -> bool {
  L173: pub fn is_io(&self) -> bool {
  L207: pub fn io_error(&self) -> Option<&std::io::Error> {
  L232: pub fn into_io_error(self) -> Option<std::io::Error> {
  L254: pub fn depth(&self) -> Option<usize> {
  L263: fn with_path<P: AsRef<Path>>(self, path: P) -> Error {
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
  L545: pub fn path(&self) -> &Path {
crates/core/flags/defs.rs:
  L3148: struct IgnoreCase;
  L3151: fn is_switch(&self) -> bool {
  L3154: fn name_short(&self) -> Option<u8> {
  L3157: fn name_long(&self) -> &'static str {
  L3160: fn doc_category(&self) -> Category {
  L3163: fn doc_short(&self) -> &'static str {
  L3166: fn doc_long(&self) -> &'static str {
  L3181: fn update(&self, v: FlagValue, args: &mut LowArgs) -> anyhow::Result<()> {
  L3190: fn test_ignore_case() {
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
  L4632: struct NoIgnoreParent;
  L4635: fn is_switch(&self) -> bool {
  L4638: fn name_long(&self) -> &'static str {
  L4641: fn name_negated(&self) -> Option<&'static str> {
  L4644: fn doc_category(&self) -> Category {
  L4647: fn doc_short(&self) -> &'static str {
  L4650: fn doc_long(&self) -> &'static str {
  L4659: fn update(&self, v: FlagValue, args: &mut LowArgs) -> anyhow::Result<()> {
  L4667: fn test_no_ignore_parent() {
  L4681: struct NoIgnoreVcs;
  L4684: fn is_switch(&self) -> bool {
  L4687: fn name_long(&self) -> &'static str {
  L4690: fn name_negated(&self) -> Option<&'static str> {
  L4693: fn doc_category(&self) -> Category {
  L4696: fn doc_short(&self) -> &'static str {
  L4699: fn doc_long(&self) -> &'static str {
  L4716: fn update(&self, v: FlagValue, args: &mut LowArgs) -> anyhow::Result<()> {
  L4724: fn test_no_ignore_vcs() {
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
  L764: fn update(&self, v: FlagValue, args: &mut LowArgs) -> anyhow::Result<()> {
  L778: fn test_color() {
  L814: struct Colors;
  L817: fn is_switch(&self) -> bool {
  L820: fn name_long(&self) -> &'static str {
  L823: fn doc_variable(&self) -> Option<&'static str> {
  L826: fn doc_category(&self) -> Category {
  L829: fn doc_short(&self) -> &'static str {
  L832: fn doc_long(&self) -> &'static str {
  L892: fn update(&self, v: FlagValue, args: &mut LowArgs) -> anyhow::Result<()> {
  L902: fn test_colors() {
  L945: struct Column;
  L948: fn is_switch(&self) -> bool {
  L951: fn name_long(&self) -> &'static str {
  L954: fn name_negated(&self) -> Option<&'static str> {
  L957: fn doc_category(&self) -> Category {
  L960: fn doc_short(&self) -> &'static str {
  L963: fn doc_long(&self) -> &'static str {
  L974: fn update(&self, v: FlagValue, args: &mut LowArgs) -> anyhow::Result<()> {
  L982: fn test_column() {
  L998: struct Context;
  L1001: fn is_switch(&self) -> bool {
  L1004: fn name_short(&self) -> Option<u8> {
  L1007: fn name_long(&self) -> &'static str {
  L1010: fn doc_variable(&self) -> Option<&'static str> {
  L1013: fn doc_category(&self) -> Category {
  L1016: fn doc_short(&self) -> &'static str {
  L1019: fn doc_long(&self) -> &'static str {
  L1031: fn update(&self, v: FlagValue, args: &mut LowArgs) -> anyhow::Result<()> {
  L1039: fn test_context() {
  L1138: struct ContextSeparator;
  L1141: fn is_switch(&self) -> bool {
  L1144: fn name_long(&self) -> &'static str {
  L1147: fn name_negated(&self) -> Option<&'static str> {
  L1150: fn doc_variable(&self) -> Option<&'static str> {
  L1153: fn doc_category(&self) -> Category {
  L1156: fn doc_short(&self) -> &'static str {
  L1159: fn doc_long(&self) -> &'static str {
  L1172: fn update(&self, v: FlagValue, args: &mut LowArgs) -> anyhow::Result<()> {
  L1188: fn test_context_separator() {
  L1247: struct Count;
  L1250: fn is_switch(&self) -> bool {
  L1253: fn name_short(&self) -> Option<u8> {
  L1256: fn name_long(&self) -> &'static str {
  L1259: fn doc_category(&self) -> Category {
  L1262: fn doc_short(&self) -> &'static str {
  L1265: fn doc_long(&self) -> &'static str {
  L1294: fn update(&self, v: FlagValue, args: &mut LowArgs) -> anyhow::Result<()> {
  L1303: fn test_count() {
  L1322: struct CountMatches;
  L1325: fn is_switch(&self) -> bool {
  L1328: fn name_long(&self) -> &'static str {
  L1331: fn doc_variable(&self) -> Option<&'static str> {
  L1334: fn doc_category(&self) -> Category {
  L1337: fn doc_short(&self) -> &'static str {
  L1340: fn doc_long(&self) -> &'static str {
  L1357: fn update(&self, v: FlagValue, args: &mut LowArgs) -> anyhow::Result<()> {
  L1366: fn test_count_matches() {
  L1382: struct Crlf;
  L1385: fn is_switch(&self) -> bool {
  L1388: fn name_long(&self) -> &'static str {
  L1391: fn name_negated(&self) -> Option<&'static str> {
  L1394: fn doc_category(&self) -> Category {
  L1397: fn doc_short(&self) -> &'static str {
  L1400: fn doc_long(&self) -> &'static str {
  L1418: fn update(&self, v: FlagValue, args: &mut LowArgs) -> anyhow::Result<()> {
  L1429: fn test_crlf() {
  L1456: struct Debug;
  L1459: fn is_switch(&self) -> bool {
  L1462: fn name_long(&self) -> &'static str {
  L1465: fn doc_category(&self) -> Category {
  L1468: fn doc_short(&self) -> &'static str {
  L1471: fn doc_long(&self) -> &'static str {
  L1484: fn update(&self, v: FlagValue, args: &mut LowArgs) -> anyhow::Result<()> {
  L1493: fn test_debug() {
  L1506: struct DfaSizeLimit;
  L1509: fn is_switch(&self) -> bool {
  L1512: fn name_long(&self) -> &'static str {
  L1515: fn doc_variable(&self) -> Option<&'static str> {
  L1518: fn doc_category(&self) -> Category {
  L1521: fn doc_short(&self) -> &'static str {
  L1524: fn doc_long(&self) -> &'static str {
  L1537: fn update(&self, v: FlagValue, args: &mut LowArgs) -> anyhow::Result<()> {
  L1546: fn test_dfa_size_limit() {
  L1582: struct Encoding;
  L1585: fn is_switch(&self) -> bool {
  L1588: fn name_short(&self) -> Option<u8> {
  L1591: fn name_long(&self) -> &'static str {
  L1594: fn name_negated(&self) -> Option<&'static str> {
  L1597: fn doc_variable(&self) -> Option<&'static str> {
  L1600: fn doc_category(&self) -> Category {
  L1603: fn doc_short(&self) -> &'static str {
  L1606: fn doc_long(&self) -> &'static str {
  L1626: fn completion_type(&self) -> CompletionType {
  L1630: fn update(&self, v: FlagValue, args: &mut LowArgs) -> anyhow::Result<()> {
  L1653: fn test_encoding() {
  L1691: struct Engine;
  L1694: fn is_switch(&self) -> bool {
  L1697: fn name_long(&self) -> &'static str {
  L1700: fn doc_variable(&self) -> Option<&'static str> {
  L1703: fn doc_category(&self) -> Category {
  L1706: fn doc_short(&self) -> &'static str {
  L1709: fn doc_long(&self) -> &'static str {
  L1731: fn doc_choices(&self) -> &'static [&'static str] {
  L1735: fn update(&self, v: FlagValue, args: &mut LowArgs) -> anyhow::Result<()> {
  L1750: fn test_engine() {
  L1783: struct FieldContextSeparator;
  L1786: fn is_switch(&self) -> bool {
  L1789: fn name_long(&self) -> &'static str {
  L1792: fn doc_variable(&self) -> Option<&'static str> {
  L1795: fn doc_category(&self) -> Category {
  L1798: fn doc_short(&self) -> &'static str {
  L1801: fn doc_long(&self) -> &'static str {
  L1812: fn update(&self, v: FlagValue, args: &mut LowArgs) -> anyhow::Result<()> {
  L1822: fn test_field_context_separator() {
  L1891: struct FieldMatchSeparator;
  L1894: fn is_switch(&self) -> bool {
  L1897: fn name_long(&self) -> &'static str {
  L1900: fn doc_variable(&self) -> Option<&'static str> {
  L1903: fn doc_category(&self) -> Category {
  L1906: fn doc_short(&self) -> &'static str {
  L1909: fn doc_long(&self) -> &'static str {
  L1920: fn update(&self, v: FlagValue, args: &mut LowArgs) -> anyhow::Result<()> {
  L1930: fn test_field_match_separator() {
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
  L2131: struct Files;
  L2134: fn is_switch(&self) -> bool {
  L2137: fn name_long(&self) -> &'static str {
  L2140: fn doc_category(&self) -> Category {
  L2143: fn doc_short(&self) -> &'static str {
  L2146: fn doc_long(&self) -> &'static str {
  L2155: fn update(&self, v: FlagValue, args: &mut LowArgs) -> anyhow::Result<()> {
  L2164: fn test_files() {
  L2174: struct FilesWithMatches;
  L2177: fn is_switch(&self) -> bool {
  L2180: fn name_short(&self) -> Option<u8> {
  L2183: fn name_long(&self) -> &'static str {
  L2186: fn doc_category(&self) -> Category {
  L2189: fn doc_short(&self) -> &'static str {
  L2192: fn doc_long(&self) -> &'static str {
  L2208: fn update(&self, v: FlagValue, args: &mut LowArgs) -> anyhow::Result<()> {
  L2217: fn test_files_with_matches() {
  L2230: struct FilesWithoutMatch;
  L2233: fn is_switch(&self) -> bool {
  L2236: fn name_long(&self) -> &'static str {
  L2239: fn doc_category(&self) -> Category {
  L2242: fn doc_short(&self) -> &'static str {
  L2245: fn doc_long(&self) -> &'static str {
  L2253: fn update(&self, v: FlagValue, args: &mut LowArgs) -> anyhow::Result<()> {
  L2265: fn test_files_without_match() {
  L2285: struct FixedStrings;
  L2288: fn is_switch(&self) -> bool {
  L2291: fn name_short(&self) -> Option<u8> {
  L2294: fn name_long(&self) -> &'static str {
  L2297: fn name_negated(&self) -> Option<&'static str> {
  L2300: fn doc_category(&self) -> Category {
  L2303: fn doc_short(&self) -> &'static str {
  L2306: fn doc_long(&self) -> &'static str {
  L2314: fn update(&self, v: FlagValue, args: &mut LowArgs) -> anyhow::Result<()> {
  L2322: fn test_fixed_strings() {
  L2341: struct Follow;
  L2344: fn is_switch(&self) -> bool {
  L2347: fn name_short(&self) -> Option<u8> {
  L2350: fn name_long(&self) -> &'static str {
  L2353: fn name_negated(&self) -> Option<&'static str> {
  L2356: fn doc_category(&self) -> Category {
  L2359: fn doc_short(&self) -> &'static str {
  L2362: fn doc_long(&self) -> &'static str {
  L2372: fn update(&self, v: FlagValue, args: &mut LowArgs) -> anyhow::Result<()> {
  L2380: fn test_follow() {
  L2399: struct Generate;
  L2402: fn is_switch(&self) -> bool {
  L2405: fn name_long(&self) -> &'static str {
  L2408: fn doc_variable(&self) -> Option<&'static str> {
  L2411: fn doc_category(&self) -> Category {
  L2414: fn doc_short(&self) -> &'static str {
  L2417: fn doc_long(&self) -> &'static str {
  L2442: fn doc_choices(&self) -> &'static [&'static str] {
  L2452: fn update(&self, v: FlagValue, args: &mut LowArgs) -> anyhow::Result<()> {
  L2468: fn test_generate() {
  L2504: struct Glob;
  L2507: fn is_switch(&self) -> bool {
  L2510: fn name_short(&self) -> Option<u8> {
  L2513: fn name_long(&self) -> &'static str {
  L2516: fn doc_variable(&self) -> Option<&'static str> {
  L2519: fn doc_category(&self) -> Category {
  L2522: fn doc_short(&self) -> &'static str {
  L2525: fn doc_long(&self) -> &'static str {
  L2554: fn update(&self, v: FlagValue, args: &mut LowArgs) -> anyhow::Result<()> {
  L2563: fn test_glob() {
  L2594: struct GlobCaseInsensitive;
  L2597: fn is_switch(&self) -> bool {
  L2600: fn name_long(&self) -> &'static str {
  L2603: fn name_negated(&self) -> Option<&'static str> {
  L2606: fn doc_category(&self) -> Category {
  L2609: fn doc_short(&self) -> &'static str {
  L2612: fn doc_long(&self) -> &'static str {
  L2619: fn update(&self, v: FlagValue, args: &mut LowArgs) -> anyhow::Result<()> {
  L2627: fn test_glob_case_insensitive() {
  L2651: struct Heading;
  L2654: fn is_switch(&self) -> bool {
  L2657: fn name_long(&self) -> &'static str {
  L2660: fn name_negated(&self) -> Option<&'static str> {
  L2663: fn doc_category(&self) -> Category {
  L2666: fn doc_short(&self) -> &'static str {
  L2669: fn doc_long(&self) -> &'static str {
  L2683: fn update(&self, v: FlagValue, args: &mut LowArgs) -> anyhow::Result<()> {
