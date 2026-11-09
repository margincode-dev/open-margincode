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
