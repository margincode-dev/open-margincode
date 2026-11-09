crates/ignore/src/dir.rs:
  L181: pub(crate) fn parent(&self) -> Option<Ignore> {
  L308: fn collect_ignore_files(
  L340: fn add_child_path_with_found_ignore_files(
  L470: fn has_any_ignore_rules(&self) -> bool {
  L548: pub(crate) fn matched_ignore<'a>(
  L725: pub(crate) fn is_absolute_parent(&self) -> bool {
  L897: pub(crate) fn add_ignore(&mut self, ig: Gitignore) -> &mut IgnoreBuilder {
  L908: pub(crate) fn add_custom_ignore_filename<S: AsRef<OsStr>>(
  L935: pub(crate) fn ignore(&mut self, yes: bool) -> &mut IgnoreBuilder {
  L970: pub(crate) fn git_ignore(&mut self, yes: bool) -> &mut IgnoreBuilder {
  L999: pub(crate) fn ignore_case_insensitive(
  L1255: fn custom_ignore_over_ignore() {
  L38: pub(crate) struct IgnoreMatch<'a>(IgnoreMatchInner<'a>);
  L44: enum IgnoreMatchInner<'a> {
  L52: fn overrides(x: overrides::Glob<'a>) -> IgnoreMatch<'a> {
  L56: fn gitignore(x: &'a gitignore::Glob) -> IgnoreMatch<'a> {
  L60: fn types(x: types::Glob<'a>) -> IgnoreMatch<'a> {
  L64: fn hidden() -> IgnoreMatch<'static> {
  L72: struct IgnoreOptions {
  L94: pub(crate) struct Ignore {
  L115: struct IgnoreInner {
  L171: pub(crate) fn path(&self) -> &Path {
  L176: pub(crate) fn is_root(&self) -> bool {
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
  L1145: fn explicit_ignore() {
  L1226: fn ignore() {
  L1238: fn custom_ignore() {
  L1271: fn custom_ignore_precedence() {
  L1289: fn ignore_over_gitignore() {
  L1346: fn errored_partial_and_ignore() {
  L1388: fn absolute_parent() {
  L1411: fn absolute_parent_anchored() {
  L1431: fn absolute_parent_matchers_are_cached_across_roots() {
  L1527: fn multi_root_custom_ignore_order_independent() {
  L731: pub(crate) struct Parents<'a>(Option<IgnoreRef<'a>>);
  L734: type Item = IgnoreRef<'a>;
  L736: fn next(&mut self) -> Option<IgnoreRef<'a>> {
  L1016: pub(crate) fn create_gitignore<T: AsRef<OsStr>>(
  L1061: fn resolve_git_commondir(
  L1109: fn strip_if_is_prefix<'a, P: AsRef<Path> + ?Sized>(
  L1124: fn wfile<P: AsRef<Path>>(path: P, contents: &str) {
  L1129: fn mkdirp<P: AsRef<Path>>(path: P) {
  L1133: fn partial(err: Error) -> Vec<Error> {
  L1140: fn tmpdir() -> TempDir {
  L1160: fn git_exclude() {
  L1173: fn gitignore() {
  L1186: fn gitignore_with_jj() {
  L1199: fn gitignore_no_git() {
  L1211: fn gitignore_allowed_no_git() {
  L1301: fn exclude_lowest() {
  L1316: fn errored() {
  L1325: fn errored_both() {
  L1335: fn errored_partial() {
  L1357: fn not_present_empty() {
  L1365: fn stops_at_git_dir() {
  L1464: fn multi_root_gitignore_order_independent() {
  L1570: fn git_info_exclude_in_linked_worktree() {
crates/core/flags/defs.rs:
  L4632: struct NoIgnoreParent;
  L4635: fn is_switch(&self) -> bool {
  L4638: fn name_long(&self) -> &'static str {
  L4641: fn name_negated(&self) -> Option<&'static str> {
  L4644: fn doc_category(&self) -> Category {
  L4647: fn doc_short(&self) -> &'static str {
  L4650: fn doc_long(&self) -> &'static str {
  L4659: fn update(&self, v: FlagValue, args: &mut LowArgs) -> anyhow::Result<()> {
  L4667: fn test_no_ignore_parent() {
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
